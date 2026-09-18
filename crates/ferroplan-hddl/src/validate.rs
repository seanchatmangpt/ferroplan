//! Pre-grounding static checks for parsed HDDL domains and problems — a
//! deliberately bounded but real well-formedness gate (not a full HDDL
//! semantic verifier). `grounder::ground` runs both entry points before any
//! grounding work, so everything below is enforced on the solve path, not
//! merely available to callers.
//!
//! Error-level checks (`ValidationError`, first error wins):
//!
//! - Duplicate declarations: task/predicate/type (the original checks) plus
//!   action and method (domain level) and object (domain `:constants` and
//!   problem `:objects`) — `ValidationError::DuplicateDefinition`.
//! - Type system: every type referenced anywhere (typed parameters, numeric
//!   fluents, `:constants`/`:objects` entries, `forall`/`exists` binders in
//!   action *and* method preconditions and in `:goal`) was declared in
//!   `:types` or is the built-in `object` (`UndefinedType`); the `:types`
//!   parent relation is acyclic (`CyclicTypeHierarchy`).
//! - Predicates: every predicate referenced in an action precondition or
//!   effect (including `when` conditions), a method precondition or `:effect`
//!   (including its `when` conditions), `:init`, or
//!   `:goal` was declared — or is the built-in `=` (`UndefinedPredicate`);
//!   every such use passes the declared argument count
//!   (`PredicateArityMismatch`).
//! - Task references: a method head resolves to a declared *compound* task
//!   (`MethodHeadNotCompoundTask` when it names an action,
//!   `UnknownTaskOrAction` otherwise); method subtasks and the problem's root
//!   `:htn` subtasks resolve to a declared task or action
//!   (`UnknownTaskOrAction`); every task call passes the callee's declared
//!   parameter count (`ArityMismatch`).
//! - Arguments: a variable used in a method head, a method subtask argument,
//!   a method precondition, or a method `:effect` is one of that method's
//!   parameters
//!   (`UnknownMethodVariable`); a constant used in `:init`, `:goal`, or a
//!   root `:htn` subtask names a declared domain constant or problem object
//!   (`UnknownConstant`); an argument's type is the same as, or a subtype
//!   of, the callee's declared parameter type under the `:types` hierarchy
//!   (`ArgumentTypeMismatch`); `:init` atoms are ground
//!   (`NonGroundInitAtom`), and a root `:htn` subtask argument is either
//!   ground or a variable declared in the `:htn`'s own `:parameters` — the
//!   grounder existentially binds those over their declared types (an
//!   undeclared variable stays `NonGroundRootSubtaskArg`).
//! - Ordering constraints: every edge names subtask ids of its own network
//!   (`UndefinedOrderRef` — this closes the koala gap where a dangling order
//!   id is silently ignored), and the edges are acyclic — in method networks
//!   and in the problem's root `:htn` alike (`CyclicOrdering`).
//! - Problems: the root `:htn` network carries at least one subtask —
//!   `solve_hddl` has no model for a problem with neither `:htn` subtasks nor
//!   any other usable task network (`MissingTaskNetwork`).
//!
//! Warning-level checks (`ValidationWarning`, returned by
//! `validate_domain_with_warnings`/`validate_problem_with_warnings`; the
//! plain `validate_*` entry points run the identical error checks and simply
//! drop the warnings):
//!
//! - A compound task that is actually referenced (as a method head, a method
//!   network subtask, or — at problem level — a root `:htn` subtask) but has
//!   no method chain reducing it to primitive actions when preconditions are
//!   ignored (`ValidationWarning::UnrefinableCompoundTask`). This is a
//!   task-decomposition-graph reachability + nullability fixpoint,
//!   deliberately a WARNING: an unrefinable task is a modeling smell that
//!   dooms the search, not a syntactic ill-formedness.

use crate::ast::{
    AtomicFormula, Domain, Effect, GoalDesc, Literal, MethodDef, Problem, TaskNetwork, Term,
    TypedParam,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Which kind of name was found declared more than once — see
/// `ValidationError::DuplicateDefinition`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateKind {
    /// A task name (`:task`).
    Task,
    /// A predicate name (`:predicates`).
    Predicate,
    /// A type name (`:types`).
    Type,
    /// An action name (`:action`).
    Action,
    /// A method name (`:method`).
    Method,
    /// An object or constant name (`:objects`/`:constants`).
    Object,
}

impl fmt::Display for DuplicateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Task => "task",
            Self::Predicate => "predicate",
            Self::Type => "type",
            Self::Action => "action",
            Self::Method => "method",
            Self::Object => "object",
        };
        write!(f, "{s}")
    }
}

/// An error produced by `validate_domain`/`validate_problem` (and by the
/// `*_with_warnings` variants, which carry the same error set).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// A method's `:task`, or a subtask's task call, names something that is
    /// neither a declared `:task` nor a declared `:action`.
    UnknownTaskOrAction(String),
    /// A task call (a method head, a method subtask, or a root `:htn`
    /// subtask) passes a different number of arguments than the referenced
    /// task or action declares parameters.
    ArityMismatch {
        /// The task name involved.
        task: String,
        /// The number of parameters the task declares.
        expected: usize,
        /// The number of arguments the call actually passed.
        found: usize,
    },
    /// A predicate name appears in an action's `:precondition`/`:effect`, a
    /// method's `:precondition`, or the problem's `:init`/`:goal`, that was
    /// never declared in the domain's `:predicates` block.
    UndefinedPredicate(String),
    /// A type name appears in a typed parameter list (a predicate, numeric
    /// fluent, task, action, or method parameter; a `:constants`/`:objects`
    /// entry; or a `forall`/`exists` binder) that was never declared in the
    /// domain's `:types` block, and is not the built-in `object` type.
    UndefinedType(String),
    /// The same task, predicate, type, action, method, or object/constant
    /// name is declared more than once.
    DuplicateDefinition { kind: DuplicateKind, name: String },
    /// The `:types` parent relation contains a cycle, so some type is its
    /// own ancestor and no subtype walk can terminate on it.
    CyclicTypeHierarchy {
        /// A type participating in the cycle.
        type_name: String,
    },
    /// A declared predicate was used with a different number of arguments
    /// than its `:predicates` entry declares.
    PredicateArityMismatch {
        /// The predicate involved.
        predicate: String,
        /// The declared parameter count.
        expected: usize,
        /// The number of arguments actually supplied at the use site.
        found: usize,
    },
    /// A method's `:task` head names a declared *action*; methods decompose
    /// compound tasks only.
    MethodHeadNotCompoundTask {
        /// The method's name.
        method: String,
        /// The action name the method tried to decompose.
        name: String,
    },
    /// A method used a variable (in its precondition, head arguments, or
    /// subtask arguments) that is not one of its declared parameters.
    UnknownMethodVariable {
        /// The method's name.
        method: String,
        /// The undeclared variable name (without the leading `?`).
        var: String,
    },
    /// An ordering edge references a subtask id that does not exist in the
    /// same task network (a method network, or the problem's root `:htn`).
    UndefinedOrderRef {
        /// `Some(method name)` for a method network edge, `None` for an edge
        /// of the problem's root `:htn`.
        in_method: Option<String>,
        /// The dangling subtask id.
        id: String,
    },
    /// The ordering edges of a task network (method network or problem root
    /// `:htn`) contain a cycle, so no subtask can legally execute first.
    CyclicOrdering {
        /// `Some(method name)` for a method network, `None` for the problem's
        /// root `:htn`.
        in_method: Option<String>,
        /// The subtask ids along the cycle, in edge order (the last element
        /// precedes the first).
        cycle: Vec<String>,
    },
    /// The problem's root task network has no subtasks — there is no task
    /// network for `solve_hddl`'s decomposition model to run on.
    MissingTaskNetwork,
    /// A ground argument in `:init`, `:goal`, or a root `:htn` subtask names
    /// something that is neither a declared problem object nor a domain
    /// constant.
    UnknownConstant {
        /// The undeclared constant/object name.
        name: String,
    },
    /// An argument's declared type is not a subtype of (or equal to) the
    /// callee's declared parameter type at that position.
    ArgumentTypeMismatch {
        /// The task, action, or predicate being called.
        callee: String,
        /// The 0-based argument position.
        position: usize,
        /// The callee's declared parameter type.
        expected: String,
        /// The argument's own declared type.
        found: String,
    },
    /// An `:init` atom contains a variable; initial facts must be ground.
    NonGroundInitAtom {
        /// The predicate of the offending atom.
        predicate: String,
    },
    /// A root `:htn` subtask has a variable argument; the initial task
    /// network must be ground.
    NonGroundRootSubtaskArg {
        /// The subtask id of the offending root subtask.
        id: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTaskOrAction(name) => {
                write!(f, "'{name}' is neither a declared task nor an action")
            }
            Self::ArityMismatch {
                task,
                expected,
                found,
            } => write!(f, "'{task}' expects {expected} argument(s), found {found}"),
            Self::UndefinedPredicate(name) => {
                write!(
                    f,
                    "predicate '{name}' is used but never declared in :predicates"
                )
            }
            Self::UndefinedType(name) => {
                write!(f, "type '{name}' is used but never declared in :types")
            }
            Self::DuplicateDefinition { kind, name } => {
                write!(f, "{kind} '{name}' is declared more than once")
            }
            Self::CyclicTypeHierarchy { type_name } => {
                write!(f, "type hierarchy is cyclic: '{type_name}' is its own ancestor")
            }
            Self::PredicateArityMismatch {
                predicate,
                expected,
                found,
            } => write!(
                f,
                "predicate '{predicate}' expects {expected} argument(s), found {found}"
            ),
            Self::MethodHeadNotCompoundTask { method, name } => {
                write!(
                    f,
                    "method '{method}' decomposes '{name}', which is a declared action, not a compound task"
                )
            }
            Self::UnknownMethodVariable { method, var } => {
                write!(
                    f,
                    "method '{method}' uses variable '?{var}', which is not one of its parameters"
                )
            }
            Self::UndefinedOrderRef { in_method, id } => {
                let network = network_description(in_method.as_deref());
                write!(
                    f,
                    "ordering constraint references subtask id '{id}', which does not exist in {network}"
                )
            }
            Self::CyclicOrdering { in_method, cycle } => {
                let network = network_description(in_method.as_deref());
                let mut path = cycle.join(" -> ");
                if let Some(first) = cycle.first() {
                    path.push_str(" -> ");
                    path.push_str(first);
                }
                write!(f, "ordering constraints of {network} form a cycle: {path}")
            }
            Self::MissingTaskNetwork => write!(
                f,
                "problem declares no usable task network: ':htn' has no subtasks to decompose"
            ),
            Self::UnknownConstant { name } => write!(
                f,
                "'{name}' is used but is neither a declared problem object nor a domain constant"
            ),
            Self::ArgumentTypeMismatch {
                callee,
                position,
                expected,
                found,
            } => write!(
                f,
                "'{callee}' argument {position} has type '{found}', which is not a subtype of the declared parameter type '{expected}'"
            ),
            Self::NonGroundInitAtom { predicate } => {
                write!(
                    f,
                    ":init atom ({predicate} ...) contains a variable; init facts must be ground"
                )
            }
            Self::NonGroundRootSubtaskArg { id } => {
                write!(
                    f,
                    "root task-network subtask '{id}' has a variable argument; the initial task network must be ground"
                )
            }
        }
    }
}
impl std::error::Error for ValidationError {}

/// "`:htn`" network description used in ordering-error messages.
fn network_description(in_method: Option<&str>) -> String {
    match in_method {
        Some(method) => format!("method '{method}'"),
        None => "the problem's root task network".to_owned(),
    }
}

/// A non-fatal modeling problem found by the `*_with_warnings` entry points.
/// The plain `validate_*` entry points run the same error checks and drop
/// these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationWarning {
    /// A compound task is referenced (as a method head, a method network
    /// subtask, or a root `:htn` subtask) but no method chain reduces it to
    /// primitive actions when preconditions are ignored — any search that
    /// reaches it can never discharge it.
    UnrefinableCompoundTask {
        /// The unrefinable compound task's name.
        task: String,
    },
}

impl fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnrefinableCompoundTask { task } => write!(
                f,
                "compound task '{task}' is referenced but no method chain reduces it to primitive actions (preconditions ignored)"
            ),
        }
    }
}
impl std::error::Error for ValidationWarning {}

/// First name that occurs more than once in `names`, in iteration order, or
/// `None` if every name is unique.
fn first_duplicate<'a, I: IntoIterator<Item = &'a str>>(names: I) -> Option<&'a str> {
    let mut seen = BTreeSet::new();
    names.into_iter().find(|&name| !seen.insert(name))
}

/// The full set of type names the domain's `:types` block makes available:
/// every declared child name (`TypeDef::declared`, which — unlike
/// `TypeDef::parent` — includes types whose parent is the implicit `object`),
/// every name used as a parent (`TypeDef::parent`'s values, e.g. a type
/// referenced only as a supertype), and the always-available built-in
/// `object` type itself.
fn declared_type_names(domain: &Domain) -> BTreeSet<&str> {
    let mut names: BTreeSet<&str> = BTreeSet::new();
    names.insert("object");
    for name in &domain.types.declared {
        names.insert(name.as_str());
    }
    for parent in domain.types.parent.values() {
        names.insert(parent.as_str());
    }
    names
}

/// Declared predicate arities (parameter counts), plus the built-in `=`
/// (arity 2): `=` is the PDDL/HDDL term-equality predicate, universally
/// available without a `:predicates` declaration (used bare in real domains,
/// e.g. koala-planner/domains' Snake and Satellite). Doubles as the
/// "declared predicate names" set — presence in the map is declaredness.
fn predicate_arities(domain: &Domain) -> BTreeMap<&str, usize> {
    let mut arities: BTreeMap<&str, usize> = domain
        .predicates
        .iter()
        .map(|p| (p.name.as_str(), p.params.len()))
        .collect();
    arities.insert("=", 2);
    arities
}

/// domain constant name -> declared type name.
fn domain_constant_types(domain: &Domain) -> BTreeMap<&str, &str> {
    domain
        .constants
        .iter()
        .map(|c| (c.name.as_str(), c.type_name.as_str()))
        .collect()
}

/// Object name -> declared type name, from the domain's `:constants` and the
/// problem's `:objects` (the ground terms a problem file may legally name).
#[allow(clippy::needless_lifetimes)] // elision would tie outputs to `domain` only; they borrow both inputs
fn problem_ground_types<'a>(domain: &'a Domain, problem: &'a Problem) -> BTreeMap<&'a str, &'a str> {
    let mut names = domain_constant_types(domain);
    names.extend(
        problem
            .objects
            .iter()
            .map(|o| (o.name.as_str(), o.type_name.as_str())),
    );
    names
}

/// Whether `found` is `expected` itself or one of its (transitive) subtypes
/// under the domain's `:types` hierarchy. `object` is the top type: every
/// type is a subtype of `object`. Cycle-safe (a cyclic hierarchy is reported
/// separately by `check_cyclic_type_hierarchy`, and walks here simply stop
/// when they revisit a type).
fn is_subtype(domain: &Domain, found: &str, expected: &str) -> bool {
    if expected == "object" || found == expected {
        return true;
    }
    let mut visited = BTreeSet::new();
    visited.insert(found);
    let mut current = found;
    while let Some(parent) = domain.types.parent.get(current) {
        let parent = parent.as_str();
        if parent == expected {
            return true;
        }
        if !visited.insert(parent) {
            return false;
        }
        current = parent;
    }
    false
}

/// Every typed parameter list in the domain that can carry a type reference:
/// predicate params, numeric-fluent params, task params, action params, and
/// method params.
fn all_typed_params(domain: &Domain) -> impl Iterator<Item = &TypedParam> {
    domain
        .predicates
        .iter()
        .flat_map(|p| p.params.iter())
        .chain(domain.numeric_fluents.iter().flat_map(|f| f.params.iter()))
        .chain(domain.tasks.iter().flat_map(|t| t.params.iter()))
        .chain(domain.actions.iter().flat_map(|a| a.params.iter()))
        .chain(domain.methods.iter().flat_map(|m| m.params.iter()))
}

/// Every `Forall`/`Exists`-bound `TypedParam` found anywhere inside `goal`,
/// so their declared types can be checked the same way predicate/task/
/// action/method parameter types already are (`all_typed_params`).
fn goal_desc_quantifier_params<'a>(goal: &'a GoalDesc, out: &mut Vec<&'a TypedParam>) {
    match goal {
        GoalDesc::Empty | GoalDesc::Atom(_) => {}
        GoalDesc::Not(g) => goal_desc_quantifier_params(g, out),
        GoalDesc::And(gs) | GoalDesc::Or(gs) => {
            for g in gs {
                goal_desc_quantifier_params(g, out);
            }
        }
        GoalDesc::Imply(a, b) => {
            goal_desc_quantifier_params(a, out);
            goal_desc_quantifier_params(b, out);
        }
        GoalDesc::Forall(vars, body) | GoalDesc::Exists(vars, body) => {
            out.extend(vars.iter());
            goal_desc_quantifier_params(body, out);
        }
    }
}

/// Every atomic formula appearing anywhere inside `goal` (including inside
/// `not`/`or`/`imply` and quantifier bodies and `when` conditions), so
/// predicate declaredness, arity, and ground-argument checks see the full
/// use surface.
fn goal_desc_atoms<'a>(goal: &'a GoalDesc, out: &mut Vec<&'a AtomicFormula>) {
    match goal {
        GoalDesc::Empty => {}
        GoalDesc::Atom(a) => out.push(a),
        GoalDesc::Not(g) => goal_desc_atoms(g, out),
        GoalDesc::And(gs) | GoalDesc::Or(gs) => {
            for g in gs {
                goal_desc_atoms(g, out);
            }
        }
        GoalDesc::Imply(a, b) => {
            goal_desc_atoms(a, out);
            goal_desc_atoms(b, out);
        }
        GoalDesc::Forall(_, g) | GoalDesc::Exists(_, g) => goal_desc_atoms(g, out),
    }
}

/// Every atomic formula appearing anywhere inside `effect`: literal atoms
/// plus the atoms of `when` conditions (recursively). Numeric-fluent
/// references (`increase`/`decrease`) live in a separate namespace
/// (`:predicates`-nested `NumericFluentDecl`) and are already refused at
/// grounding time by `GroundError::UnsupportedNumericFluent` — not this
/// pass's concern.
fn effect_atoms<'a>(effect: &'a Effect, out: &mut Vec<&'a AtomicFormula>) {
    match effect {
        Effect::Empty => {}
        Effect::Literal(Literal::Pos(a)) | Effect::Literal(Literal::Neg(a)) => out.push(a),
        Effect::And(es) | Effect::Oneof(es) => {
            for e in es {
                effect_atoms(e, out);
            }
        }
        Effect::When(condition, effect) => {
            goal_desc_atoms(condition, out);
            effect_atoms(effect, out);
        }
        Effect::Increase(_, _) | Effect::Decrease(_, _) => {}
    }
}

/// Every atomic formula used anywhere in the domain's action preconditions/
/// effects and method preconditions/`:effect`s.
fn domain_atoms(domain: &Domain) -> Vec<&AtomicFormula> {
    let mut atoms = Vec::new();
    for action in &domain.actions {
        goal_desc_atoms(&action.precondition, &mut atoms);
        effect_atoms(&action.effect, &mut atoms);
    }
    for method in &domain.methods {
        goal_desc_atoms(&method.precondition, &mut atoms);
        effect_atoms(&method.effect, &mut atoms);
    }
    atoms
}

fn check_duplicate_definitions(domain: &Domain) -> Result<(), ValidationError> {
    // Checked in declaration-surface order: tasks, actions, methods,
    // predicates, types, constants.
    if let Some(name) = first_duplicate(domain.tasks.iter().map(|t| t.name.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Task,
            name: name.to_owned(),
        });
    }
    if let Some(name) = first_duplicate(domain.actions.iter().map(|a| a.name.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Action,
            name: name.to_owned(),
        });
    }
    if let Some(name) = first_duplicate(domain.methods.iter().map(|m| m.name.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Method,
            name: name.to_owned(),
        });
    }
    if let Some(name) = first_duplicate(domain.predicates.iter().map(|p| p.name.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Predicate,
            name: name.to_owned(),
        });
    }
    if let Some(name) = first_duplicate(domain.types.declared.iter().map(|s| s.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Type,
            name: name.to_owned(),
        });
    }
    if let Some(name) = first_duplicate(domain.constants.iter().map(|c| c.name.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Object,
            name: name.to_owned(),
        });
    }
    Ok(())
}

/// Walk the `:types` parent chain from every known type name; a chain that
/// revisits a type is a cycle in the hierarchy (e.g. `(:types a - b b - a)`
/// or the self-parent `(:types a - a)`).
fn check_cyclic_type_hierarchy(domain: &Domain) -> Result<(), ValidationError> {
    for name in declared_type_names(domain) {
        let mut visited = BTreeSet::new();
        visited.insert(name);
        let mut current = name;
        while let Some(parent) = domain.types.parent.get(current) {
            let parent = parent.as_str();
            if !visited.insert(parent) {
                return Err(ValidationError::CyclicTypeHierarchy {
                    type_name: parent.to_owned(),
                });
            }
            current = parent;
        }
    }
    Ok(())
}

fn check_undefined_types(domain: &Domain) -> Result<(), ValidationError> {
    let declared = declared_type_names(domain);
    for param in all_typed_params(domain) {
        if !declared.contains(param.type_name.as_str()) {
            return Err(ValidationError::UndefinedType(param.type_name.clone()));
        }
    }
    for constant in &domain.constants {
        if !declared.contains(constant.type_name.as_str()) {
            return Err(ValidationError::UndefinedType(constant.type_name.clone()));
        }
    }
    let mut quantifier_params = Vec::new();
    for action in &domain.actions {
        goal_desc_quantifier_params(&action.precondition, &mut quantifier_params);
    }
    for method in &domain.methods {
        goal_desc_quantifier_params(&method.precondition, &mut quantifier_params);
    }
    for param in quantifier_params {
        if !declared.contains(param.type_name.as_str()) {
            return Err(ValidationError::UndefinedType(param.type_name.clone()));
        }
    }
    Ok(())
}

/// Existence + arity for every atom in `atoms`, against `arities` (which
/// includes the built-in `=`).
fn check_atoms_declared_and_arity(
    arities: &BTreeMap<&str, usize>,
    atoms: &[&AtomicFormula],
) -> Result<(), ValidationError> {
    for atom in atoms {
        let Some(&expected) = arities.get(atom.predicate.as_str()) else {
            return Err(ValidationError::UndefinedPredicate(atom.predicate.clone()));
        };
        if atom.args.len() != expected {
            return Err(ValidationError::PredicateArityMismatch {
                predicate: atom.predicate.clone(),
                expected,
                found: atom.args.len(),
            });
        }
    }
    Ok(())
}

fn check_domain_predicates(domain: &Domain) -> Result<(), ValidationError> {
    let arities = predicate_arities(domain);
    check_atoms_declared_and_arity(&arities, &domain_atoms(domain))
}

/// The declared parameter list of the task or action named `name` (tasks
/// first, matching `is_known_task_or_action`'s lookup order).
fn callee_params<'a>(domain: &'a Domain, name: &str) -> Option<&'a [TypedParam]> {
    domain
        .tasks
        .iter()
        .find(|t| t.name == name)
        .map(|t| t.params.as_slice())
        .or_else(|| {
            domain
                .actions
                .iter()
                .find(|a| a.name == name)
                .map(|a| a.params.as_slice())
        })
}

/// One ground-constant argument check: the constant must name a known ground
/// term in `consts`, and (when the callee declares a parameter type there)
/// its type must be a subtype of `expected`.
fn check_const_arg(
    domain: &Domain,
    callee: &str,
    position: usize,
    expected: &str,
    constant: &str,
    consts: &BTreeMap<&str, &str>,
) -> Result<(), ValidationError> {
    let Some(&found) = consts.get(constant) else {
        return Err(ValidationError::UnknownConstant {
            name: constant.to_owned(),
        });
    };
    if !is_subtype(domain, found, expected) {
        return Err(ValidationError::ArgumentTypeMismatch {
            callee: callee.to_owned(),
            position,
            expected: expected.to_owned(),
            found: found.to_owned(),
        });
    }
    Ok(())
}

/// Argument checks for one task call inside a method (the method's head or
/// one of its network subtasks): arity against `callee_params`; variable
/// arguments must be declared method parameters and be a subtype of the
/// callee's declared parameter type at that position. Constant arguments are
/// deliberately NOT checked here (real-world HDDL method networks reference
/// bare constants without a `:constants` block — see `translate.rs`'s
/// shortcut test — and schema-level constant resolution is outside this
/// pass's ticketed scope); ground-constant checking happens at the problem
/// level (`:init`, `:goal`, root `:htn`).
fn check_method_task_call_args(
    domain: &Domain,
    method: &MethodDef,
    callee: &str,
    callee_params: &[TypedParam],
    args: &[Term],
    scope: &BTreeMap<&str, &str>,
) -> Result<(), ValidationError> {
    if args.len() != callee_params.len() {
        return Err(ValidationError::ArityMismatch {
            task: callee.to_owned(),
            expected: callee_params.len(),
            found: args.len(),
        });
    }
    for (position, (arg, param)) in args.iter().zip(callee_params.iter()).enumerate() {
        if let Term::Var(var) = arg {
            let Some(&found) = scope.get(var.as_str()) else {
                return Err(ValidationError::UnknownMethodVariable {
                    method: method.name.clone(),
                    var: var.clone(),
                });
            };
            let expected = param.type_name.as_str();
            if !is_subtype(domain, found, expected) {
                return Err(ValidationError::ArgumentTypeMismatch {
                    callee: callee.to_owned(),
                    position,
                    expected: expected.to_owned(),
                    found: found.to_owned(),
                });
            }
        }
    }
    Ok(())
}

/// All ordering-constraint checks for one task network: every edge endpoint
/// is a subtask id of `network`, and the edges are acyclic.
fn check_network_ordering(
    network: &TaskNetwork,
    in_method: Option<&str>,
) -> Result<(), ValidationError> {
    let ids: BTreeSet<&str> = network.subtasks.iter().map(|s| s.id.as_str()).collect();
    for edge in &network.order {
        for id in [&edge.before, &edge.after] {
            if !ids.contains(id.as_str()) {
                return Err(ValidationError::UndefinedOrderRef {
                    in_method: in_method.map(str::to_owned),
                    id: id.clone(),
                });
            }
        }
    }
    if let Some(cycle) = find_order_cycle(network, &ids) {
        return Err(ValidationError::CyclicOrdering {
            in_method: in_method.map(str::to_owned),
            cycle,
        });
    }
    Ok(())
}

/// Kahn's algorithm over the network's ordering edges. Returns `None` when
/// the edges are acyclic; otherwise returns the ids along one concrete cycle
/// (in edge order — the last element precedes the first), found by walking
/// predecessors inside the leftover (never-topologically-sortable) set, which
/// is exactly the set of nodes on or downstream of a cycle and where every
/// node has at least one predecessor still in the set.
fn find_order_cycle(network: &TaskNetwork, ids: &BTreeSet<&str>) -> Option<Vec<String>> {
    let mut successors: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    let mut predecessors: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for id in ids {
        successors.entry(id).or_default();
        predecessors.entry(id).or_default();
    }
    for edge in &network.order {
        successors
            .entry(edge.before.as_str())
            .or_default()
            .insert(edge.after.as_str());
        predecessors
            .entry(edge.after.as_str())
            .or_default()
            .insert(edge.before.as_str());
    }
    let mut remaining_predecessors: BTreeMap<&str, usize> = predecessors
        .iter()
        .map(|(&id, preds)| (id, preds.len()))
        .collect();
    let mut ready: Vec<&str> = remaining_predecessors
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&id, _)| id)
        .collect();
    let mut processed = 0;
    while let Some(id) = ready.pop() {
        processed += 1;
        for &succ in &successors[&id] {
            let count = remaining_predecessors.get_mut(succ).expect("known id");
            *count -= 1;
            if *count == 0 {
                ready.push(succ);
            }
        }
    }
    if processed == ids.len() {
        return None;
    }
    // Some ids never became ready: they sit on, or downstream of, a cycle.
    // Walk predecessors inside that leftover set — every leftover node has at
    // least one leftover predecessor (otherwise its count would have reached
    // 0) — until a node repeats; the slice between the two visits is a cycle.
    let leftover: BTreeSet<&str> = remaining_predecessors
        .iter()
        .filter(|(_, &count)| count > 0)
        .map(|(&id, _)| id)
        .collect();
    let start = *leftover.iter().next().expect("nonempty leftover set");
    let mut path = vec![start];
    let mut current = start;
    loop {
        current = *predecessors[&current]
            .iter()
            .find(|pred| leftover.contains(**pred))
            .expect("every leftover node has a leftover predecessor");
        if let Some(position) = path.iter().position(|&id| id == current) {
            // `path[position..]` walks the cycle backwards (predecessor
            // steps); reverse it to edge order for the error message.
            let mut cycle: Vec<String> = path[position..]
                .iter()
                .map(|id| (*id).to_owned())
                .collect();
            cycle.reverse();
            return Some(cycle);
        }
        path.push(current);
    }
}

/// Every variable used anywhere in `goal` (quantifier binders introduce
/// scope) must be one of the method's parameters.
fn check_method_precondition_vars(
    goal: &GoalDesc,
    scope: &BTreeMap<&str, &str>,
    method: &MethodDef,
) -> Result<(), ValidationError> {
    match goal {
        GoalDesc::Empty => Ok(()),
        GoalDesc::Atom(atom) => {
            for arg in &atom.args {
                if let Term::Var(var) = arg {
                    if !scope.contains_key(var.as_str()) {
                        return Err(ValidationError::UnknownMethodVariable {
                            method: method.name.clone(),
                            var: var.clone(),
                        });
                    }
                }
            }
            Ok(())
        }
        GoalDesc::Not(g) => check_method_precondition_vars(g, scope, method),
        GoalDesc::And(gs) | GoalDesc::Or(gs) => {
            for g in gs {
                check_method_precondition_vars(g, scope, method)?;
            }
            Ok(())
        }
        GoalDesc::Imply(a, b) => {
            check_method_precondition_vars(a, scope, method)?;
            check_method_precondition_vars(b, scope, method)
        }
        GoalDesc::Forall(vars, body) | GoalDesc::Exists(vars, body) => {
            let mut inner = scope.clone();
            for var in vars {
                inner.insert(var.var.as_str(), var.type_name.as_str());
            }
            check_method_precondition_vars(body, &inner, method)
        }
    }
}

/// Every variable used anywhere inside a method's `:effect` — literal atoms
/// and `when`-condition atoms alike — is one of the method's parameters (or
/// quantifier-bound inside a condition, via
/// `check_method_precondition_vars`). Structural mirror of
/// `check_method_precondition_vars`, which it reuses for `when` conditions.
/// `Effect::Oneof` cannot be produced for a method effect
/// (`parse_method_def` parses `:effect` in `EffectCtx::Nested`, which
/// refuses `oneof`) but is handled defensively by recursing into branches.
fn check_method_effect_vars(
    effect: &Effect,
    scope: &BTreeMap<&str, &str>,
    method: &MethodDef,
) -> Result<(), ValidationError> {
    match effect {
        Effect::Empty => Ok(()),
        Effect::Literal(Literal::Pos(atom)) | Effect::Literal(Literal::Neg(atom)) => {
            for arg in &atom.args {
                if let Term::Var(var) = arg {
                    if !scope.contains_key(var.as_str()) {
                        return Err(ValidationError::UnknownMethodVariable {
                            method: method.name.clone(),
                            var: var.clone(),
                        });
                    }
                }
            }
            Ok(())
        }
        Effect::And(es) | Effect::Oneof(es) => {
            for e in es {
                check_method_effect_vars(e, scope, method)?;
            }
            Ok(())
        }
        Effect::When(condition, inner) => {
            check_method_precondition_vars(condition, scope, method)?;
            check_method_effect_vars(inner, scope, method)
        }
        // Numeric effects are lexed/parsed but refused at grounding time
        // (`GroundError::UnsupportedNumericFluent`); nothing to check here.
        Effect::Increase(_, _) | Effect::Decrease(_, _) => Ok(()),
    }
}

/// All method-level checks for one method: the head names a declared
/// compound task with matching arity and well-typed arguments; every subtask
/// resolves, matches arity, and has well-declared, well-typed arguments; the
/// ordering constraints are well-founded; the precondition's and `:effect`'s
/// variables are the method's own (or quantifier-bound).
fn check_method(domain: &Domain, method: &MethodDef) -> Result<(), ValidationError> {
    let task_def = match domain.tasks.iter().find(|t| t.name == method.task.name) {
        Some(task) => task,
        None => {
            if domain.actions.iter().any(|a| a.name == method.task.name) {
                return Err(ValidationError::MethodHeadNotCompoundTask {
                    method: method.name.clone(),
                    name: method.task.name.clone(),
                });
            }
            return Err(ValidationError::UnknownTaskOrAction(
                method.task.name.clone(),
            ));
        }
    };
    let scope: BTreeMap<&str, &str> = method
        .params
        .iter()
        .map(|p| (p.var.as_str(), p.type_name.as_str()))
        .collect();
    check_method_task_call_args(
        domain,
        method,
        &method.task.name,
        &task_def.params,
        &method.task.args,
        &scope,
    )?;
    for subtask in &method.network.subtasks {
        let params = callee_params(domain, &subtask.task.name).ok_or_else(|| {
            ValidationError::UnknownTaskOrAction(subtask.task.name.clone())
        })?;
        check_method_task_call_args(
            domain,
            method,
            &subtask.task.name,
            params,
            &subtask.task.args,
            &scope,
        )?;
    }
    check_network_ordering(&method.network, Some(&method.name))?;
    check_method_precondition_vars(&method.precondition, &scope, method)?;
    check_method_effect_vars(&method.effect, &scope, method)
}

/// The compound tasks that have NO method chain reducing them to primitive
/// actions when preconditions are ignored — a least-fixpoint over the
/// task-decomposition graph: a task is refinable once some method for it has
/// a network whose every compound subtask is itself refinable (an empty
/// network is vacuously refinable: that is the nullability base case, so a
/// recursive task with a terminating method — fixture f's `setdone` — is
/// refinable). Everything left out of the fixpoint is unrefinable.
fn unrefinable_compound_tasks<'a>(
    domain: &'a Domain,
    task_names: &BTreeSet<&'a str>,
) -> BTreeSet<&'a str> {
    let mut refinable: BTreeSet<&str> = BTreeSet::new();
    loop {
        let mut changed = false;
        for &name in task_names {
            if refinable.contains(&name) {
                continue;
            }
            let has_viable_method = domain.methods.iter().any(|method| {
                method.task.name == name
                    && method.network.subtasks.iter().all(|subtask| {
                        !task_names.contains(subtask.task.name.as_str())
                            || refinable.contains(subtask.task.name.as_str())
                    })
            });
            if has_viable_method {
                refinable.insert(name);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    task_names.difference(&refinable).copied().collect()
}

/// `UnrefinableCompoundTask` warnings for every compound task that is BOTH
/// unrefinable and actually referenced — as a method head, a method network
/// subtask, or (when `problem` is given) a root `:htn` subtask. Declared but
/// never-referenced dead tasks produce no warning.
fn unrefinable_warnings(domain: &Domain, problem: Option<&Problem>) -> Vec<ValidationWarning> {
    let task_names: BTreeSet<&str> = domain.tasks.iter().map(|t| t.name.as_str()).collect();
    let mut referenced: BTreeSet<&str> = BTreeSet::new();
    for method in &domain.methods {
        if task_names.contains(method.task.name.as_str()) {
            referenced.insert(method.task.name.as_str());
        }
        for subtask in &method.network.subtasks {
            if task_names.contains(subtask.task.name.as_str()) {
                referenced.insert(subtask.task.name.as_str());
            }
        }
    }
    if let Some(problem) = problem {
        for subtask in &problem.htn.subtasks {
            if task_names.contains(subtask.task.name.as_str()) {
                referenced.insert(subtask.task.name.as_str());
            }
        }
    }
    let unrefinable = unrefinable_compound_tasks(domain, &task_names);
    referenced
        .intersection(&unrefinable)
        .map(|&task| ValidationWarning::UnrefinableCompoundTask {
            task: task.to_owned(),
        })
        .collect()
}

/// Run all domain-level static checks (see the module docs for the exact
/// list) and return the non-fatal warnings (currently: referenced compound
/// tasks no method chain can refine). The plain `validate_domain` is this
/// function with the warnings dropped.
///
/// # Errors
///
/// Returns the first `ValidationError` found; see the enum's variants for
/// exactly which conditions are checked.
///
/// # Examples
///
/// ```
/// use ferroplan_hddl::parser::parse_domain;
/// use ferroplan_hddl::validate::validate_domain_with_warnings;
///
/// let domain = parse_domain(r#"
///     (define (domain doors)
///       (:predicates (open ?d))
///       (:action open-door
///         :parameters (?d)
///         :precondition ()
///         :effect (open ?d)))
/// "#).unwrap();
///
/// let warnings = validate_domain_with_warnings(&domain).unwrap();
/// assert!(warnings.is_empty());
/// ```
pub fn validate_domain_with_warnings(
    domain: &Domain,
) -> Result<Vec<ValidationWarning>, ValidationError> {
    check_duplicate_definitions(domain)?;
    check_cyclic_type_hierarchy(domain)?;
    check_undefined_types(domain)?;
    check_domain_predicates(domain)?;

    for method in &domain.methods {
        check_method(domain, method)?;
    }
    Ok(unrefinable_warnings(domain, None))
}

/// Run all domain-level static checks (see the module docs for the exact
/// list): duplicate task/action/method/predicate/type/constant names, cyclic
/// type hierarchies, undefined types, undefined or mis-arity predicate uses,
/// and — for every declared method — that its head is a declared compound
/// task with matching arity and well-typed arguments, that every subtask
/// resolves with matching arity and well-declared, well-typed arguments,
/// that its ordering constraints name real subtask ids acyclically, and that
/// its precondition only uses declared variables.
///
/// # Errors
///
/// Returns the first `ValidationError` found; see the enum's variants for
/// exactly which conditions are checked.
///
/// # Examples
///
/// ```
/// use ferroplan_hddl::parser::parse_domain;
/// use ferroplan_hddl::validate::validate_domain;
///
/// let domain = parse_domain(r#"
///     (define (domain doors)
///       (:predicates (open ?d))
///       (:action open-door
///         :parameters (?d)
///         :precondition ()
///         :effect (open ?d)))
/// "#).unwrap();
///
/// assert!(validate_domain(&domain).is_ok());
/// ```
pub fn validate_domain(domain: &Domain) -> Result<(), ValidationError> {
    validate_domain_with_warnings(domain).map(|_| ())
}

fn check_duplicate_objects(problem: &Problem) -> Result<(), ValidationError> {
    if let Some(name) = first_duplicate(problem.objects.iter().map(|o| o.name.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Object,
            name: name.to_owned(),
        });
    }
    Ok(())
}

/// Existence + arity + ground, well-typed constant arguments for the
/// problem's `:init` atoms.
fn check_init_atoms(
    domain: &Domain,
    problem: &Problem,
    arities: &BTreeMap<&str, usize>,
    consts: &BTreeMap<&str, &str>,
) -> Result<(), ValidationError> {
    for atom in &problem.init {
        let Some(&expected) = arities.get(atom.predicate.as_str()) else {
            return Err(ValidationError::UndefinedPredicate(atom.predicate.clone()));
        };
        if atom.args.len() != expected {
            return Err(ValidationError::PredicateArityMismatch {
                predicate: atom.predicate.clone(),
                expected,
                found: atom.args.len(),
            });
        }
        let params = domain
            .predicates
            .iter()
            .find(|p| p.name == atom.predicate)
            .map(|p| &p.params);
        for (position, arg) in atom.args.iter().enumerate() {
            match arg {
                Term::Var(_) => {
                    return Err(ValidationError::NonGroundInitAtom {
                        predicate: atom.predicate.clone(),
                    })
                }
                Term::Const(constant) => {
                    let expected_type = params
                        .and_then(|params| params.get(position))
                        .map(|param| param.type_name.as_str())
                        .unwrap_or("object");
                    check_const_arg(
                        domain,
                        &atom.predicate,
                        position,
                        expected_type,
                        constant,
                        consts,
                    )?;
                }
            }
        }
    }
    Ok(())
}

/// Existence + arity for the problem's `:goal` atoms; constant arguments
/// must name declared ground terms with compatible types (variable arguments
/// are legal inside `forall`/`exists` bodies and are not analyzed here).
fn check_goal_atoms(
    domain: &Domain,
    problem: &Problem,
    arities: &BTreeMap<&str, usize>,
    consts: &BTreeMap<&str, &str>,
) -> Result<(), ValidationError> {
    let mut atoms = Vec::new();
    goal_desc_atoms(&problem.goal, &mut atoms);
    check_atoms_declared_and_arity(arities, &atoms)?;
    for atom in atoms {
        let params = domain
            .predicates
            .iter()
            .find(|p| p.name == atom.predicate)
            .map(|p| &p.params);
        for (position, arg) in atom.args.iter().enumerate() {
            if let Term::Const(constant) = arg {
                let expected_type = params
                    .and_then(|params| params.get(position))
                    .map(|param| param.type_name.as_str())
                    .unwrap_or("object");
                check_const_arg(
                    domain,
                    &atom.predicate,
                    position,
                    expected_type,
                    constant,
                    consts,
                )?;
            }
        }
    }
    Ok(())
}

/// Existence + arity + groundness + constant/type well-formedness for the
/// problem's root `:htn` subtask calls.
fn check_root_network(
    domain: &Domain,
    problem: &Problem,
    consts: &BTreeMap<&str, &str>,
) -> Result<(), ValidationError> {
    // The root `:htn` network's own `:parameters` (ticket fond-htn-24): a
    // subtask argument may be one of these variables, in which case the
    // grounder existentially binds it over its declared type (one ground
    // root network per admissible binding). A variable NOT declared there
    // stays the same `NonGroundRootSubtaskArg` refusal as before. Declared
    // variables get the same subtype check `check_method_task_call_args`
    // applies to method variables: the variable's declared type must be the
    // callee parameter type or a subtype of it, so EVERY object the
    // grounder can bind the variable to is a well-typed argument.
    let htn_params: BTreeMap<&str, &str> = problem
        .htn
        .params
        .iter()
        .map(|p| (p.var.as_str(), p.type_name.as_str()))
        .collect();
    for subtask in &problem.htn.subtasks {
        let params = callee_params(domain, &subtask.task.name).ok_or_else(|| {
            ValidationError::UnknownTaskOrAction(subtask.task.name.clone())
        })?;
        if subtask.task.args.len() != params.len() {
            return Err(ValidationError::ArityMismatch {
                task: subtask.task.name.clone(),
                expected: params.len(),
                found: subtask.task.args.len(),
            });
        }
        for (position, (arg, param)) in subtask.task.args.iter().zip(params).enumerate() {
            match arg {
                Term::Var(var) => {
                    let Some(&found) = htn_params.get(var.as_str()) else {
                        return Err(ValidationError::NonGroundRootSubtaskArg {
                            id: subtask.id.clone(),
                        });
                    };
                    let expected = param.type_name.as_str();
                    if !is_subtype(domain, found, expected) {
                        return Err(ValidationError::ArgumentTypeMismatch {
                            callee: subtask.task.name.clone(),
                            position,
                            expected: expected.to_owned(),
                            found: found.to_owned(),
                        });
                    }
                }
                Term::Const(constant) => {
                    check_const_arg(
                        domain,
                        &subtask.task.name,
                        position,
                        param.type_name.as_str(),
                        constant,
                        consts,
                    )?;
                }
            }
        }
    }
    Ok(())
}

/// Run all problem-level static checks against `domain` (see the module docs
/// for the exact list) and return the non-fatal warnings (currently:
/// referenced compound tasks no method chain can refine, considering both
/// the domain's method networks and the problem's root `:htn`). The plain
/// `validate_problem` is this function with the warnings dropped.
///
/// # Errors
///
/// Returns the first `ValidationError` found — starting with
/// `ValidationError::MissingTaskNetwork` when the root `:htn` carries no
/// subtasks at all, since there is no task network for `solve_hddl`'s model
/// to run on.
///
/// # Examples
///
/// ```
/// use ferroplan_hddl::parser::{parse_domain, parse_problem};
/// use ferroplan_hddl::validate::validate_problem;
///
/// let domain = parse_domain(r#"
///     (define (domain doors)
///       (:predicates (open ?d))
///       (:action open-door
///         :parameters (?d)
///         :precondition ()
///         :effect (open ?d)))
/// "#).unwrap();
/// let problem = parse_problem(r#"
///     (define (problem doors-p1)
///       (:domain doors)
///       (:objects d1)
///       (:init)
///       (:goal (open d1))
///       (:htn :ordered-subtasks (open-door d1)))
/// "#).unwrap();
///
/// assert!(validate_problem(&domain, &problem).is_ok());
/// ```
pub fn validate_problem_with_warnings(
    domain: &Domain,
    problem: &Problem,
) -> Result<Vec<ValidationWarning>, ValidationError> {
    if problem.htn.subtasks.is_empty() {
        return Err(ValidationError::MissingTaskNetwork);
    }
    check_duplicate_objects(problem)?;

    let declared = declared_type_names(domain);
    for object in &problem.objects {
        if !declared.contains(object.type_name.as_str()) {
            return Err(ValidationError::UndefinedType(object.type_name.clone()));
        }
    }
    let mut quantifier_params = Vec::new();
    goal_desc_quantifier_params(&problem.goal, &mut quantifier_params);
    for param in quantifier_params {
        if !declared.contains(param.type_name.as_str()) {
            return Err(ValidationError::UndefinedType(param.type_name.clone()));
        }
    }

    let arities = predicate_arities(domain);
    let consts = problem_ground_types(domain, problem);
    check_init_atoms(domain, problem, &arities, &consts)?;
    check_goal_atoms(domain, problem, &arities, &consts)?;
    check_root_network(domain, problem, &consts)?;
    check_network_ordering(&problem.htn, None)?;

    Ok(unrefinable_warnings(domain, Some(problem)))
}

/// Run problem-level static checks against `domain`: the root `:htn` network
/// is non-empty (`MissingTaskNetwork`), every root subtask resolves to a
/// declared task or action with matching arity and well-typed arguments —
/// each either ground or a variable declared in the `:htn`'s own
/// `:parameters` (existentially bound at grounding time) — every object was
/// declared with a declared type, every
/// `:init`/`:goal` atom uses a declared predicate with matching arity over
/// declared ground terms, the root ordering constraints name real subtask
/// ids acyclically, and every type referenced by a `forall`/`exists`
/// quantifier in `:goal` was declared in `:types`.
///
/// # Errors
///
/// Returns the first `ValidationError` found; see the enum's variants for
/// exactly which conditions are checked.
///
/// # Examples
///
/// ```
/// use ferroplan_hddl::parser::{parse_domain, parse_problem};
/// use ferroplan_hddl::validate::validate_problem;
///
/// let domain = parse_domain(r#"
///     (define (domain doors)
///       (:predicates (open ?d))
///       (:action open-door
///         :parameters (?d)
///         :precondition ()
///         :effect (open ?d)))
/// "#).unwrap();
/// let problem = parse_problem(r#"
///     (define (problem doors-p1)
///       (:domain doors)
///       (:objects d1)
///       (:init)
///       (:goal (open d1))
///       (:htn :ordered-subtasks (open-door d1)))
/// "#).unwrap();
///
/// assert!(validate_problem(&domain, &problem).is_ok());
/// ```
pub fn validate_problem(domain: &Domain, problem: &Problem) -> Result<(), ValidationError> {
    validate_problem_with_warnings(domain, problem).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_domain, parse_problem};

    const FIXTURE_A_DOMAIN: &str = include_str!("../fixtures/a/domain.hddl");
    const FIXTURE_A_PROBLEM: &str = include_str!("../fixtures/a/problem.hddl");
    const FIXTURE_B_DOMAIN: &str = include_str!("../fixtures/b/domain.hddl");
    const FIXTURE_C_DOMAIN: &str = include_str!("../fixtures/c/domain.hddl");
    const FIXTURE_D_DOMAIN: &str = include_str!("../fixtures/d/domain.hddl");
    const FIXTURE_F_DOMAIN: &str = include_str!("../fixtures/f/domain.hddl");
    const FIXTURE_G_DOMAIN: &str = include_str!("../fixtures/g/domain.hddl");
    const FIXTURE_G_PROBLEM: &str = include_str!("../fixtures/g/problem.hddl");

    #[test]
    fn fixture_a_validates_clean() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_A_PROBLEM).unwrap();
        assert!(validate_domain(&domain).is_ok());
        assert!(validate_problem(&domain, &problem).is_ok());
    }

    #[test]
    fn rejects_undeclared_subtask_reference() {
        let mut domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        domain.methods[0].network.subtasks[0].task.name = "no-such-task".to_owned();
        let err = validate_domain(&domain).unwrap_err();
        assert!(matches!(err, ValidationError::UnknownTaskOrAction(ref n) if n == "no-such-task"));
    }

    // -- undefined predicate references -------------------------------------

    const UNDEFINED_PREDICATE_DOMAIN: &str = r#"
        (define (domain undefined-predicate)
          (:types loc)
          (:predicates
            (at ?l - loc))
          (:action move
            :parameters (?l - loc)
            :precondition (at ?l)
            :effect (and (not (at ?l)) (holding ?l))))
    "#;

    #[test]
    fn rejects_undefined_predicate_in_effect() {
        let domain = parse_domain(UNDEFINED_PREDICATE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::UndefinedPredicate("holding".to_owned())
        );
    }

    // `=` is the PDDL/HDDL built-in term-equality predicate: real domains
    // (koala-planner/domains' Snake, e.g. `(= ?snakepos ?goalpos)`) use it
    // bare in preconditions/effects without ever declaring it in
    // `:predicates`, since it's a language built-in, not a domain-defined
    // predicate. Before this fix, `predicate_arities` (then
    // `declared_predicate_names`) had no built-in-`=` special case, so any
    // domain using it this way was wrongly rejected as
    // `UndefinedPredicate("=")`.
    const BUILTIN_EQUALITY_PREDICATE_DOMAIN: &str = r#"
        (define (domain builtin-equality)
          (:types loc)
          (:predicates
            (at ?l - loc))
          (:action move
            :parameters (?from ?to - loc)
            :precondition (and (at ?from) (not (= ?from ?to)))
            :effect (and (not (at ?from)) (at ?to))))
    "#;

    #[test]
    fn accepts_bare_builtin_equality_predicate_without_declaration() {
        let domain = parse_domain(BUILTIN_EQUALITY_PREDICATE_DOMAIN).unwrap();
        assert!(validate_domain(&domain).is_ok());
    }

    const DEFINED_PREDICATE_DOMAIN: &str = r#"
        (define (domain defined-predicate)
          (:types loc)
          (:predicates
            (at ?l - loc)
            (holding ?l - loc))
          (:action move
            :parameters (?l - loc)
            :precondition (at ?l)
            :effect (and (not (at ?l)) (holding ?l))))
    "#;

    #[test]
    fn accepts_action_using_only_declared_predicates() {
        let domain = parse_domain(DEFINED_PREDICATE_DOMAIN).unwrap();
        assert!(validate_domain(&domain).is_ok());
    }

    // -- undefined type references -------------------------------------------

    const UNDEFINED_TYPE_DOMAIN: &str = r#"
        (define (domain undefined-type)
          (:types loc)
          (:predicates
            (at ?l - loc)
            (holds ?v - vehicle))
          (:action noop
            :parameters ()
            :precondition ()
            :effect ()))
    "#;

    #[test]
    fn rejects_undefined_type_in_predicate_params() {
        let domain = parse_domain(UNDEFINED_TYPE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(err, ValidationError::UndefinedType("vehicle".to_owned()));
    }

    const DEFINED_TYPE_DOMAIN: &str = r#"
        (define (domain defined-type)
          (:types loc vehicle)
          (:predicates
            (at ?l - loc)
            (holds ?v - vehicle))
          (:action noop
            :parameters ()
            :precondition ()
            :effect ()))
    "#;

    #[test]
    fn accepts_predicate_params_using_only_declared_types() {
        let domain = parse_domain(DEFINED_TYPE_DOMAIN).unwrap();
        assert!(validate_domain(&domain).is_ok());
    }

    /// A type declared with an implicit `object` parent (no trailing
    /// `- type`) must still count as declared — regression guard for the gap
    /// where `TypeDef::parent` alone (which drops object-parented entries)
    /// would have wrongly flagged this as undefined.
    #[test]
    fn accepts_object_parented_type_used_in_action_params() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        assert!(check_undefined_types(&domain).is_ok());
    }

    // -- duplicate definitions ------------------------------------------------

    const DUPLICATE_TASK_DOMAIN: &str = r#"
        (define (domain duplicate-task)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task deliver :parameters (?l - loc))
          (:task deliver :parameters (?l - loc))
          (:action noop :parameters () :precondition () :effect ()))
    "#;

    #[test]
    fn rejects_duplicate_task_definition() {
        let domain = parse_domain(DUPLICATE_TASK_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::DuplicateDefinition {
                kind: DuplicateKind::Task,
                name: "deliver".to_owned(),
            }
        );
    }

    const DUPLICATE_PREDICATE_DOMAIN: &str = r#"
        (define (domain duplicate-predicate)
          (:types loc)
          (:predicates
            (at ?l - loc)
            (at ?l - loc))
          (:action noop :parameters () :precondition () :effect ()))
    "#;

    #[test]
    fn rejects_duplicate_predicate_definition() {
        let domain = parse_domain(DUPLICATE_PREDICATE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::DuplicateDefinition {
                kind: DuplicateKind::Predicate,
                name: "at".to_owned(),
            }
        );
    }

    const DUPLICATE_TYPE_DOMAIN: &str = r#"
        (define (domain duplicate-type)
          (:types loc loc)
          (:predicates (at ?l - loc))
          (:action noop :parameters () :precondition () :effect ()))
    "#;

    #[test]
    fn rejects_duplicate_type_definition() {
        let domain = parse_domain(DUPLICATE_TYPE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::DuplicateDefinition {
                kind: DuplicateKind::Type,
                name: "loc".to_owned(),
            }
        );
    }

    const DUPLICATE_ACTION_DOMAIN: &str = r#"
        (define (domain duplicate-action)
          (:types loc)
          (:predicates (at ?l - loc))
          (:action noop :parameters () :precondition () :effect ())
          (:action noop :parameters () :precondition () :effect ()))
    "#;

    #[test]
    fn rejects_duplicate_action_definition() {
        let domain = parse_domain(DUPLICATE_ACTION_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::DuplicateDefinition {
                kind: DuplicateKind::Action,
                name: "noop".to_owned(),
            }
        );
    }

    const DUPLICATE_METHOD_DOMAIN: &str = r#"
        (define (domain duplicate-method)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task go :parameters (?l - loc))
          (:action walk
            :parameters (?from - loc ?to - loc)
            :precondition ()
            :effect ())
          (:method m-go
            :parameters (?l - loc)
            :task (go ?l)
            :ordered-subtasks (and (t1 (walk ?l ?l))))
          (:method m-go
            :parameters (?l - loc)
            :task (go ?l)
            :ordered-subtasks (and (t1 (walk ?l ?l)))))
    "#;

    #[test]
    fn rejects_duplicate_method_definition() {
        let domain = parse_domain(DUPLICATE_METHOD_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::DuplicateDefinition {
                kind: DuplicateKind::Method,
                name: "m-go".to_owned(),
            }
        );
    }

    const DUPLICATE_OBJECT_DOMAIN: &str = r#"
        (define (domain duplicate-object)
          (:types loc)
          (:predicates (at ?l - loc))
          (:action noop :parameters () :precondition () :effect ()))
    "#;

    const DUPLICATE_OBJECT_PROBLEM: &str = r#"
        (define (problem duplicate-object-p)
          (:domain duplicate-object)
          (:objects x - loc x - loc)
          (:init)
          (:goal ())
          (:htn :ordered-subtasks (and (r1 (noop)))))
    "#;

    #[test]
    fn rejects_duplicate_object_declaration() {
        let domain = parse_domain(DUPLICATE_OBJECT_DOMAIN).unwrap();
        let problem = parse_problem(DUPLICATE_OBJECT_PROBLEM).unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(
            err,
            ValidationError::DuplicateDefinition {
                kind: DuplicateKind::Object,
                name: "x".to_owned(),
            }
        );
    }

    #[test]
    fn accepts_fixtures_b_c_d_with_no_new_false_positives() {
        for (name, src) in [
            ("b", FIXTURE_B_DOMAIN),
            ("c", FIXTURE_C_DOMAIN),
            ("d", FIXTURE_D_DOMAIN),
        ] {
            let domain = parse_domain(src)
                .unwrap_or_else(|e| panic!("fixture {name} domain should parse: {e}"));
            assert!(
                validate_domain(&domain).is_ok(),
                "fixture {name} domain should validate cleanly under the new checks"
            );
        }
    }

    // -- undefined type in forall/exists bound variables --------------------

    const UNDEFINED_TYPE_IN_PRECONDITION_FORALL_DOMAIN: &str = r#"
        (define (domain undefined-type-forall)
          (:types loc)
          (:predicates
            (at ?l - loc)
            (p ?x - object))
          (:action noop
            :parameters ()
            :precondition (forall (?y - typo-type) (p ?y))
            :effect ()))
    "#;

    #[test]
    fn rejects_undefined_type_in_precondition_forall() {
        let domain = parse_domain(UNDEFINED_TYPE_IN_PRECONDITION_FORALL_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(err, ValidationError::UndefinedType("typo-type".to_owned()));
    }

    const UNDEFINED_TYPE_IN_GOAL_FORALL_DOMAIN: &str = r#"
        (define (domain undefined-type-goal-forall)
          (:types loc)
          (:predicates (at ?l - loc))
          (:action noop
            :parameters (?l - loc)
            :precondition ()
            :effect (and (at ?l))))
    "#;

    #[test]
    fn rejects_undefined_type_in_goal_forall() {
        let domain = parse_domain(UNDEFINED_TYPE_IN_GOAL_FORALL_DOMAIN).unwrap();
        let src = r#"(define (problem undefined-type-goal-forall-p)
          (:domain undefined-type-goal-forall)
          (:objects l1 - loc)
          (:init (at l1))
          (:goal (forall (?y - typo-type) (at ?y)))
          (:htn :subtasks (and (r1 (noop l1)))))"#;
        let problem = parse_problem(src).expect("problem parses");
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(err, ValidationError::UndefinedType("typo-type".to_owned()));
    }

    // -- type hierarchy cycles -----------------------------------------------

    const CYCLIC_TYPE_HIERARCHY_DOMAIN: &str = r#"
        (define (domain cyclic-types)
          (:types a - b b - a)
          (:predicates (p ?x - a))
          (:action noop
            :parameters ()
            :precondition ()
            :effect ()))
    "#;

    #[test]
    fn rejects_cyclic_type_hierarchy() {
        let domain = parse_domain(CYCLIC_TYPE_HIERARCHY_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::CyclicTypeHierarchy { ref type_name } if type_name == "a" || type_name == "b"
        ));
    }

    const SELF_PARENT_TYPE_DOMAIN: &str = r#"
        (define (domain self-parent-type)
          (:types a - a)
          (:predicates (p ?x - a))
          (:action noop
            :parameters ()
            :precondition ()
            :effect ()))
    "#;

    #[test]
    fn rejects_self_parent_type() {
        let domain = parse_domain(SELF_PARENT_TYPE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::CyclicTypeHierarchy { ref type_name } if type_name == "a"
        ));
    }

    // -- predicate arity at use ----------------------------------------------

    const PREDICATE_ARITY_DOMAIN: &str = r#"
        (define (domain predicate-arity)
          (:types loc)
          (:predicates (at ?l - loc))
          (:action move
            :parameters (?l - loc)
            :precondition (at ?l ?l)
            :effect ()))
    "#;

    #[test]
    fn rejects_predicate_arity_mismatch_in_action_precondition() {
        let domain = parse_domain(PREDICATE_ARITY_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::PredicateArityMismatch {
                predicate: "at".to_owned(),
                expected: 1,
                found: 2,
            }
        );
    }

    const ARITY_OK_DOMAIN: &str = r#"
        (define (domain arity-ok)
          (:types loc)
          (:predicates (at ?l - loc) (goal ?a ?b - loc))
          (:action noop
            :parameters ()
            :precondition ()
            :effect ()))
    "#;

    #[test]
    fn rejects_predicate_arity_mismatch_in_init() {
        let domain = parse_domain(ARITY_OK_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem arity-ok-p)
              (:domain arity-ok)
              (:objects x - loc)
              (:init (at x x))
              (:goal ())
              (:htn :subtasks (and (r1 (noop)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(
            err,
            ValidationError::PredicateArityMismatch {
                predicate: "at".to_owned(),
                expected: 1,
                found: 2,
            }
        );
    }

    #[test]
    fn rejects_predicate_arity_mismatch_in_goal() {
        let domain = parse_domain(ARITY_OK_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem arity-ok-g)
              (:domain arity-ok)
              (:objects x - loc)
              (:init)
              (:goal (and (goal x)))
              (:htn :subtasks (and (r1 (noop)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(
            err,
            ValidationError::PredicateArityMismatch {
                predicate: "goal".to_owned(),
                expected: 2,
                found: 1,
            }
        );
    }

    // -- method head: compound task + arity ----------------------------------

    const METHOD_HEAD_ACTION_DOMAIN: &str = r#"
        (define (domain method-head-action)
          (:types loc)
          (:predicates (at ?l - loc))
          (:action walk
            :parameters (?l - loc)
            :precondition ()
            :effect (at ?l))
          (:method m
            :parameters (?l - loc)
            :task (walk ?l)
            :ordered-subtasks (and (t1 (walk ?l)))))
    "#;

    #[test]
    fn rejects_method_head_naming_an_action() {
        let domain = parse_domain(METHOD_HEAD_ACTION_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::MethodHeadNotCompoundTask {
                method: "m".to_owned(),
                name: "walk".to_owned(),
            }
        );
    }

    const METHOD_HEAD_ARITY_DOMAIN: &str = r#"
        (define (domain method-head-arity)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task go :parameters (?l - loc))
          (:action noop :parameters () :precondition () :effect ())
          (:method m
            :parameters (?l - loc ?other - loc)
            :task (go ?l ?other)
            :ordered-subtasks (and (t1 (noop)))))
    "#;

    #[test]
    fn rejects_method_head_arity_mismatch() {
        let domain = parse_domain(METHOD_HEAD_ARITY_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::ArityMismatch {
                task: "go".to_owned(),
                expected: 1,
                found: 2,
            }
        );
    }

    // -- subtask existence + arity + argument typing -------------------------

    const SUBTASK_ARITY_DOMAIN: &str = r#"
        (define (domain subtask-arity)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task go :parameters (?l - loc))
          (:action walk
            :parameters (?from - loc ?to - loc)
            :precondition ()
            :effect ())
          (:method m
            :parameters (?l - loc)
            :task (go ?l)
            :ordered-subtasks (and (t1 (walk ?l)))))
    "#;

    #[test]
    fn rejects_subtask_arity_mismatch() {
        let domain = parse_domain(SUBTASK_ARITY_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::ArityMismatch {
                task: "walk".to_owned(),
                expected: 2,
                found: 1,
            }
        );
    }

    const ARGUMENT_TYPE_DOMAIN: &str = r#"
        (define (domain argument-type)
          (:types loc - locatable vehicle - locatable)
          (:predicates (at ?x - locatable ?l - loc))
          (:task go :parameters (?x - locatable ?l - loc))
          (:action park
            :parameters (?v - vehicle ?l - loc)
            :precondition ()
            :effect ())
          (:method m-ok
            :parameters (?v - vehicle ?l - loc)
            :task (go ?v ?l)
            :ordered-subtasks (and (t1 (park ?v ?l))))
          (:method m-bad
            :parameters (?any - locatable ?l - loc)
            :task (go ?any ?l)
            :ordered-subtasks (and (t2 (park ?any ?l)))))
    "#;

    /// Positive subtyping case: passing a `vehicle` (a declared subtype of
    /// `locatable`) where `locatable` is expected is accepted.
    #[test]
    fn accepts_subtype_conforming_argument() {
        let domain = parse_domain(
            r#"
            (define (domain subtype-positive)
              (:types loc - locatable vehicle - locatable)
              (:predicates (at ?x - locatable ?l - loc))
              (:task go :parameters (?x - locatable ?l - loc))
              (:action park
                :parameters (?v - vehicle ?l - loc)
                :precondition ()
                :effect ())
              (:method m
                :parameters (?v - vehicle ?l - loc)
                :task (go ?v ?l)
                :ordered-subtasks (and (t1 (park ?v ?l)))))
        "#,
        )
        .unwrap();
        assert!(validate_domain(&domain).is_ok());
    }

    #[test]
    fn rejects_argument_type_mismatch_in_subtask() {
        let domain = parse_domain(ARGUMENT_TYPE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::ArgumentTypeMismatch {
                callee: "park".to_owned(),
                position: 0,
                expected: "vehicle".to_owned(),
                found: "locatable".to_owned(),
            }
        );
    }

    // -- method-precondition variables ---------------------------------------

    const UNKNOWN_METHOD_VARIABLE_DOMAIN: &str = r#"
        (define (domain unknown-method-variable)
          (:types loc)
          (:predicates (at ?x - loc ?y - loc))
          (:task go :parameters (?l - loc))
          (:action walk
            :parameters (?from - loc ?to - loc)
            :precondition ()
            :effect ())
          (:method m
            :parameters (?l - loc)
            :task (go ?l)
            :precondition (at ?mystery ?l)
            :ordered-subtasks (and (t1 (walk ?l ?l)))))
    "#;

    #[test]
    fn rejects_unknown_variable_in_method_precondition() {
        let domain = parse_domain(UNKNOWN_METHOD_VARIABLE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::UnknownMethodVariable {
                method: "m".to_owned(),
                var: "mystery".to_owned(),
            }
        );
    }

    #[test]
    fn rejects_unknown_variable_in_subtask_argument() {
        let domain = parse_domain(UNKNOWN_METHOD_VARIABLE_DOMAIN).unwrap();
        let mut domain = domain;
        // Remove the precondition variable use so the subtask-argument use is
        // what surfaces first.
        domain.methods[0].precondition = GoalDesc::Empty;
        domain.methods[0].network.subtasks[0].task.args[0] = Term::Var("mystery".to_owned());
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::UnknownMethodVariable {
                method: "m".to_owned(),
                var: "mystery".to_owned(),
            }
        );
    }

    #[test]
    fn accepts_quantifier_bound_variable_in_method_precondition() {
        // `?l` is not a method parameter here — it is forall-bound inside the
        // precondition, which is legal. (l1/l2 are declared constants, since
        // constant subtask arguments are checked against `:constants`.)
        let domain = parse_domain(
            r#"
            (define (domain forall-precondition)
              (:types loc)
              (:constants l1 l2 - loc)
              (:predicates (at ?l - loc) (done ?l - loc))
              (:task go :parameters ())
              (:action walk
                :parameters (?from - loc ?to - loc)
                :precondition ()
                :effect ())
              (:method m
                :parameters ()
                :task (go)
                :precondition (forall (?l - loc) (done ?l))
                :ordered-subtasks (and (t1 (walk l1 l2)))))
        "#,
        )
        .unwrap();
        assert!(validate_domain(&domain).is_ok());
    }

    // -- ordering constraints --------------------------------------------------

    const DANGLING_ORDER_DOMAIN: &str = r#"
        (define (domain dangling-order)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task go :parameters (?l - loc))
          (:action a1 :parameters (?l - loc) :precondition () :effect ())
          (:action a2 :parameters (?l - loc) :precondition () :effect ())
          (:method m
            :parameters (?l - loc)
            :task (go ?l)
            :subtasks (and (t1 (a1 ?l)) (t2 (a2 ?l)))
            :order (and (t1 t3))))
    "#;

    #[test]
    fn rejects_ordering_edge_referencing_unknown_subtask_id() {
        let domain = parse_domain(DANGLING_ORDER_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::UndefinedOrderRef {
                in_method: Some("m".to_owned()),
                id: "t3".to_owned(),
            }
        );
    }

    const CYCLIC_METHOD_ORDER_DOMAIN: &str = r#"
        (define (domain cyclic-method-order)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task go :parameters (?l - loc))
          (:action a1 :parameters (?l - loc) :precondition () :effect ())
          (:action a2 :parameters (?l - loc) :precondition () :effect ())
          (:action a3 :parameters (?l - loc) :precondition () :effect ())
          (:method m
            :parameters (?l - loc)
            :task (go ?l)
            :subtasks (and (t1 (a1 ?l)) (t2 (a2 ?l)) (t3 (a3 ?l)))
            :order (and (t1 t2) (t2 t3) (t3 t1))))
    "#;

    #[test]
    fn rejects_cyclic_ordering_in_method_network() {
        let domain = parse_domain(CYCLIC_METHOD_ORDER_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        let ValidationError::CyclicOrdering {
            in_method: Some(method),
            cycle,
        } = err
        else {
            panic!("expected CyclicOrdering, got {err:?}");
        };
        assert_eq!(method, "m");
        assert_eq!(cycle.len(), 3, "full cycle reported, got {cycle:?}");
        assert!(cycle.contains(&"t1".to_owned()));
        assert!(cycle.contains(&"t2".to_owned()));
        assert!(cycle.contains(&"t3".to_owned()));
    }

    const CYCLIC_ROOT_ORDER_DOMAIN: &str = r#"
        (define (domain cyclic-root-order)
          (:types loc)
          (:predicates (at ?l - loc))
          (:action a1 :parameters (?l - loc) :precondition () :effect ())
          (:action a2 :parameters (?l - loc) :precondition () :effect ()))
    "#;

    #[test]
    fn rejects_cyclic_ordering_in_problem_htn() {
        let domain = parse_domain(CYCLIC_ROOT_ORDER_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem cyclic-root-order-p)
              (:domain cyclic-root-order)
              (:objects x - loc)
              (:init)
              (:goal ())
              (:htn
                :subtasks (and (r1 (a1 x)) (r2 (a2 x)))
                :order (and (r1 r2) (r2 r1))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        let ValidationError::CyclicOrdering {
            in_method,
            cycle,
        } = err
        else {
            panic!("expected CyclicOrdering");
        };
        assert_eq!(in_method, None);
        assert_eq!(cycle.len(), 2, "both cycle ids reported, got {cycle:?}");
        assert!(cycle.contains(&"r1".to_owned()) && cycle.contains(&"r2".to_owned()));
    }

    #[test]
    fn rejects_dangling_order_id_in_problem_htn() {
        let domain = parse_domain(CYCLIC_ROOT_ORDER_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem dangling-root-order-p)
              (:domain cyclic-root-order)
              (:objects x - loc)
              (:init)
              (:goal ())
              (:htn
                :subtasks (and (r1 (a1 x)) (r2 (a2 x)))
                :order (and (r1 r9))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(
            err,
            ValidationError::UndefinedOrderRef {
                in_method: None,
                id: "r9".to_owned(),
            }
        );
    }

    // -- missing task network ---------------------------------------------------

    #[test]
    fn rejects_problem_with_empty_task_network() {
        let domain = parse_domain(CYCLIC_ROOT_ORDER_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem no-network-p)
              (:domain cyclic-root-order)
              (:objects x - loc)
              (:init)
              (:goal ())
              (:htn :subtasks ()))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(err, ValidationError::MissingTaskNetwork);
    }

    #[test]
    fn rejects_problem_without_any_htn_section() {
        let domain = parse_domain(CYCLIC_ROOT_ORDER_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem no-htn-p)
              (:domain cyclic-root-order)
              (:objects x - loc)
              (:init)
              (:goal ()))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(err, ValidationError::MissingTaskNetwork);
    }

    // -- init atoms: declared / typed / ground ----------------------------------

    #[test]
    fn rejects_undeclared_predicate_in_init() {
        let domain = parse_domain(ARITY_OK_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem undeclared-init-p)
              (:domain arity-ok)
              (:objects x - loc)
              (:init (mystery x))
              (:goal ())
              (:htn :subtasks (and (r1 (noop)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(
            err,
            ValidationError::UndefinedPredicate("mystery".to_owned())
        );
    }

    #[test]
    fn rejects_unknown_constant_in_init() {
        let domain = parse_domain(ARITY_OK_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem unknown-const-p)
              (:domain arity-ok)
              (:objects x - loc)
              (:init (at y))
              (:goal ())
              (:htn :subtasks (and (r1 (noop)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(err, ValidationError::UnknownConstant { name: "y".to_owned() });
    }

    #[test]
    fn rejects_nonground_init_atom() {
        let domain = parse_domain(ARITY_OK_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem nonground-init-p)
              (:domain arity-ok)
              (:objects x - loc)
              (:init (at ?x))
              (:goal ())
              (:htn :subtasks (and (r1 (noop)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(
            err,
            ValidationError::NonGroundInitAtom {
                predicate: "at".to_owned(),
            }
        );
    }

    #[test]
    fn rejects_argument_type_mismatch_in_init() {
        // `at` expects (locatable, loc); `any1` is a plain `locatable`, which
        // is NOT a subtype of `loc` — position 1 must be flagged.
        let domain = parse_domain(ARGUMENT_TYPE_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem bad-init-type-p)
              (:domain argument-type)
              (:objects l1 - loc any1 - locatable)
              (:init (at l1 any1))
              (:goal ())
              (:htn :subtasks (and (r1 (park l1 l1)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(
            err,
            ValidationError::ArgumentTypeMismatch {
                callee: "at".to_owned(),
                position: 1,
                expected: "loc".to_owned(),
                found: "locatable".to_owned(),
            }
        );
    }

    #[test]
    fn rejects_unknown_constant_in_root_subtask() {
        let domain = parse_domain(CYCLIC_ROOT_ORDER_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem unknown-root-const-p)
              (:domain cyclic-root-order)
              (:objects x - loc)
              (:init)
              (:goal ())
              (:htn :subtasks (and (r1 (a1 y)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(err, ValidationError::UnknownConstant { name: "y".to_owned() });
    }

    #[test]
    fn rejects_nonground_root_subtask_argument() {
        let domain = parse_domain(CYCLIC_ROOT_ORDER_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem nonground-root-p)
              (:domain cyclic-root-order)
              (:objects x - loc)
              (:init)
              (:goal ())
              (:htn :subtasks (and (r1 (a1 ?x)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(
            err,
            ValidationError::NonGroundRootSubtaskArg {
                id: "r1".to_owned(),
            }
        );
    }

    // -- undeclared types on constants/objects ---------------------------------

    const CONSTANT_TYPE_DOMAIN: &str = r#"
        (define (domain constant-type)
          (:types loc)
          (:constants hub - depot)
          (:predicates (at ?l - loc))
          (:action noop :parameters () :precondition () :effect ()))
    "#;

    #[test]
    fn rejects_undeclared_type_on_domain_constant() {
        let domain = parse_domain(CONSTANT_TYPE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(err, ValidationError::UndefinedType("depot".to_owned()));
    }

    #[test]
    fn rejects_undeclared_type_on_problem_object() {
        let domain = parse_domain(ARITY_OK_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem bad-object-type-p)
              (:domain arity-ok)
              (:objects x - typo)
              (:init)
              (:goal ())
              (:htn :subtasks (and (r1 (noop)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(err, ValidationError::UndefinedType("typo".to_owned()));
    }

    // -- unrefinable compound tasks (warnings) ---------------------------------

    const UNREFINABLE_DOMAIN: &str = r#"
        (define (domain unrefinable)
          (:predicates (p))
          (:task loop :parameters ())
          (:task stranded :parameters ())
          (:action ok :parameters () :precondition (p) :effect ())
          (:method m-recursive
            :parameters ()
            :task (loop)
            :ordered-subtasks (and (t1 (loop))))
          (:method m-uses-stranded
            :parameters ()
            :task (loop)
            :ordered-subtasks (and (t2 (stranded)))))
    "#;

    #[test]
    fn unrefinable_compound_task_is_a_warning_not_an_error() {
        let domain = parse_domain(UNREFINABLE_DOMAIN).unwrap();
        // Plain validate: no *error*.
        assert!(validate_domain(&domain).is_ok());
        // With warnings: both `loop` (referenced as a method head, and its
        // only methods recurse into themselves or into `stranded`) and
        // `stranded` (referenced by m-uses-stranded, has no methods) warn.
        let warnings = validate_domain_with_warnings(&domain).unwrap();
        assert_eq!(
            warnings,
            vec![
                ValidationWarning::UnrefinableCompoundTask {
                    task: "loop".to_owned()
                },
                ValidationWarning::UnrefinableCompoundTask {
                    task: "stranded".to_owned()
                },
            ]
        );
    }

    /// Fixture f is IPC-2020-derived (see its header): `achieve-goals` is
    /// recursive, but the empty-network `setdone` method is a nullability
    /// base case, so every compound task there is refinable and NO warning
    /// may fire.
    #[test]
    fn recursive_task_with_nullability_base_case_gets_no_warning() {
        let domain = parse_domain(FIXTURE_F_DOMAIN).unwrap();
        let warnings = validate_domain_with_warnings(&domain).unwrap();
        assert!(warnings.is_empty(), "unexpected warnings: {warnings:?}");
    }

    #[test]
    fn unrefinable_root_task_warns_at_problem_level() {
        let domain = parse_domain(UNREFINABLE_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem unrefinable-p)
              (:domain unrefinable)
              (:init)
              (:goal ())
              (:htn :subtasks (and (r1 (stranded)))))"#,
        )
        .unwrap();
        // Plain validate: no *error* (stranded exists; arity 0 = 0).
        assert!(validate_problem(&domain, &problem).is_ok());
        let warnings = validate_problem_with_warnings(&domain, &problem).unwrap();
        assert!(warnings.contains(&ValidationWarning::UnrefinableCompoundTask {
            task: "stranded".to_owned()
        }));
    }

    // -- transport accept case (hand-authored fixture g) ------------------------

    /// The accept case: a transport-style FOND-HTN domain+problem pair with
    /// typed hierarchies, a constant, method preconditions, explicit
    /// ordering edges, oneof effects, and a ground root network — the whole
    /// check surface must accept it with zero warnings.
    #[test]
    fn transport_fixture_validates_clean_with_no_warnings() {
        let domain = parse_domain(FIXTURE_G_DOMAIN)
            .unwrap_or_else(|e| panic!("fixture g domain should parse: {e}"));
        let problem = parse_problem(FIXTURE_G_PROBLEM)
            .unwrap_or_else(|e| panic!("fixture g problem should parse: {e}"));
        let domain_warnings = validate_domain_with_warnings(&domain)
            .unwrap_or_else(|e| panic!("fixture g domain should validate: {e}"));
        assert!(domain_warnings.is_empty(), "unexpected warnings: {domain_warnings:?}");
        let problem_warnings = validate_problem_with_warnings(&domain, &problem)
            .unwrap_or_else(|e| panic!("fixture g problem should validate: {e}"));
        assert!(problem_warnings.is_empty(), "unexpected warnings: {problem_warnings:?}");
    }

    #[test]
    fn transport_problem_rejects_when_given_the_wrong_cargo_type() {
        // Root subtask passes a location object where deliver's first
        // parameter is a package — the subtyping argument check must catch
        // it at the problem level.
        let domain = parse_domain(FIXTURE_G_DOMAIN).unwrap();
        let problem = parse_problem(
            r#"(define (problem transport-g-p1-bad)
              (:domain transport-g)
              (:objects truck1 - vehicle parcel1 - package depot - location)
              (:init
                (at truck1 hub)
                (at parcel1 hub)
                (connected hub depot))
              (:goal (and (delivered parcel1)))
              (:htn :ordered-subtasks (and (root (deliver depot depot)))))"#,
        )
        .unwrap();
        let err = validate_problem(&domain, &problem).unwrap_err();
        assert_eq!(
            err,
            ValidationError::ArgumentTypeMismatch {
                callee: "deliver".to_owned(),
                position: 0,
                expected: "package".to_owned(),
                found: "location".to_owned(),
            }
        );
    }
}
