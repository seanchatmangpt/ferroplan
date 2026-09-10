//! HDDL abstract syntax tree.
//!
//! Scope for this pass (see crate-level docs in `lib.rs` for the full boundary):
//! typed parameters/objects, a single-level type hierarchy (subtype -> parent,
//! transitively resolved during grounding), predicates, primitive actions with a
//! conjunctive precondition/effect, `oneof` non-deterministic effects, abstract
//! tasks, methods with totally- or partially-ordered subtask networks (explicit
//! `<` ordering edges), ground init facts, a `:goal` conjunction, and the
//! problem's initial `:htn` task network.
//!
//! `:constraints` blocks (domain and problem level), numeric-fluent function
//! declarations nested inside `:predicates` (the `(= (fluent-name ?params) value)`
//! form), and `increase`/`decrease` effects ARE lexed and parsed into real AST
//! nodes below (`ConstraintDef`, `NumericFluentDecl`, `Effect::Increase`/
//! `Effect::Decrease`, `NumericValue`) rather than failing at the tokenizer or
//! parser stage. Full numeric-planning and temporal-constraint semantics are
//! still out of scope: `grounder::ground` refuses any domain/problem carrying
//! one of these with a specific typed error
//! (`GroundError::UnsupportedConstraint` / `GroundError::UnsupportedNumericFluent`)
//! the moment grounding starts, rather than the parser rejecting the file
//! outright or silently misparsing/dropping the construct. This turns "silent
//! garbage" (a raw tokenizer failure, or worse, a mis-parse) into "loud,
//! precise refusal" at the correct pipeline stage.
//!
//! `:goal` may combine positive and negative ground literals, e.g. `(and (at
//! l2) (not (at l1)))` — `translate.rs` compiles each `(not P)` into a
//! synthetic marker fact rather than needing a negative-fact field on
//! `ferroplan`'s shared `Goal` type. `not` of anything but a single atom
//! (`(not (and ...))`, etc.) is out of scope for `:goal`, matching the
//! existing restriction on `not` inside preconditions/`when`-conditions.
//!
//! `or`/`imply` ARE represented in `GoalDesc` and grounded (see
//! `grounder::GroundGoal`/`grounder::evaluate_ground_goal`): both are
//! propositional connectives over already-fixed, already-typed parameters —
//! grounding one is just recursive substitution followed by boolean
//! evaluation against a ground fact set, the same shape as `and`/`not`, with
//! no new variable binding introduced.
//!
//! `forall`/`exists` ARE represented (`GoalDesc::Forall`/`GoalDesc::Exists`)
//! in action/method preconditions and in `:goal`, and grounded by
//! *quantifier expansion at grounding time*: since grounding already has
//! full access to the typed object universe
//! (`grounder::index_objects_by_type`), `(forall (?x - t) P(?x))` grounds to
//! the conjunction of `P(o)` for every object `o` of type `t`, and
//! `(exists (?x - t) P(?x))` grounds to the disjunction — reusing the exact
//! Cartesian-product binding enumeration (`grounder::enumerate_bindings`)
//! and term/atom substitution (`grounder::subst_term`/`grounder::subst_atom`)
//! already used for action/method parameter grounding. `grounder::ground_goal`
//! handles the precondition case directly (producing a `GroundGoal`);
//! `grounder::expand_goal_quantifiers` handles `:goal` (staying in
//! `ast::GoalDesc` shape, since `GroundedIR::goal`/`translate::flatten_goal`
//! need that shape) — two small parallel functions for the same reason this
//! crate already carries two parallel `flatten_goal` functions. `forall`/
//! `exists` remain out of scope inside a `when`-effect's condition
//! (`grounder::flatten_goal`'s flat pos/neg literal-set model can't
//! represent the `And`/`Or` a quantifier expands into) — refused there with
//! `GroundError::UnsupportedPrecondition`, the same refusal `or`/`imply`
//! already get in that position.
//!
//! Deliberately NOT represented at all (a parse of one of these must be a
//! hard error, never a silent drop): temporal/durative actions, and nested
//! `oneof` / `oneof` under `when`.

use std::collections::BTreeMap;

pub type Name = String;
/// Parameter/task variable name, WITHOUT the leading `?`.
pub type VarName = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(VarName),
    Const(Name),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedParam {
    pub var: VarName,
    pub type_name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedObject {
    pub name: Name,
    pub type_name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicFormula {
    pub predicate: Name,
    pub args: Vec<Term>,
}

/// Preconditions and `:goal` — and/or/not/imply/atom/forall/exists, see the
/// module docs above.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GoalDesc {
    #[default]
    Empty,
    Atom(AtomicFormula),
    Not(Box<GoalDesc>),
    And(Vec<GoalDesc>),
    /// `(or g1 g2 ...)` — holds when at least one disjunct holds.
    Or(Vec<GoalDesc>),
    /// `(imply antecedent consequent)` — classical material implication:
    /// holds whenever the antecedent is false OR the consequent is true
    /// (vacuously true if the antecedent doesn't hold). Grounded as sugar for
    /// `Or(Not(antecedent), consequent)` — see `grounder::ground_goal`.
    Imply(Box<GoalDesc>, Box<GoalDesc>),
    /// `(forall (?x - t ...) body)` — holds when `body` holds under every
    /// substitution of the bound variables (`vars`, one or more, each with
    /// its own declared type) to every object of its type. `vars` uses the
    /// same `Vec<TypedParam>` shape as `ActionDef::params`/`MethodDef::params`
    /// (parsed by the same `parse_typed_params`) so one node binds several
    /// variables at once, matching HDDL's `(forall (?x - t1 ?y - t2) ...)`
    /// surface syntax directly rather than requiring the parser to desugar
    /// it into nested single-variable `Forall`s. Grounded via quantifier
    /// expansion into a conjunction over every object combination — see
    /// `grounder::ground_goal`'s `Forall` arm (preconditions) and
    /// `grounder::expand_goal_quantifiers` (`:goal`).
    Forall(Vec<TypedParam>, Box<GoalDesc>),
    /// `(exists (?x - t ...) body)` — holds when `body` holds under at least
    /// one substitution of the bound variables to objects of their types.
    /// Same binding shape and grounding strategy as `Forall`, but expands to
    /// a disjunction instead of a conjunction.
    Exists(Vec<TypedParam>, Box<GoalDesc>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Pos(AtomicFormula),
    Neg(AtomicFormula),
}

/// The right-hand side of an `increase`/`decrease` effect: either a numeral
/// literal (kept as source text — this pass never evaluates numeric
/// expressions, only lexes/parses them, so no float parsing/`Eq` concerns
/// arise) or a reference to another numeric fluent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumericValue {
    Number(String),
    Fluent(AtomicFormula),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Empty,
    Literal(Literal),
    And(Vec<Effect>),
    When(GoalDesc, Box<Effect>),
    /// Exactly one of these sub-effects occurs at execution time — grounding
    /// turns each branch into one non-deterministic outcome of the action.
    Oneof(Vec<Effect>),
    /// `(increase (fluent ?args...) value)` — parsed, never grounded: see the
    /// module-level docs and `GroundError::UnsupportedNumericFluent`.
    Increase(AtomicFormula, NumericValue),
    /// `(decrease (fluent ?args...) value)` — parsed, never grounded: see the
    /// module-level docs and `GroundError::UnsupportedNumericFluent`.
    Decrease(AtomicFormula, NumericValue),
}

/// type_name -> parent type_name. A type with no declared parent in the source
/// (the untyped tail of a `:types` list, or "object" itself) has no entry here.
/// Note: this map alone is NOT the full set of declared type names — a type
/// declared with an implicit/explicit `object` parent (e.g. `(:types loc)`)
/// never appears as a key (its parent is "object", filtered out) or as a
/// value (nothing is a subtype of it). Use `TypeDef::declared` (below) plus
/// this map's values for that purpose — see `validate::declared_type_names`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeDef {
    pub parent: BTreeMap<Name, Name>,
    /// Every type name that appeared as a child in the `:types` list, in
    /// source order, WITH duplicates preserved (unlike `parent`, which is a
    /// map and silently collapses a repeated child onto its last-seen
    /// parent). This is what makes "same type name declared twice" or "type
    /// declared with an implicit `object` parent" detectable post-parse.
    pub declared: Vec<Name>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateDef {
    pub name: Name,
    pub params: Vec<TypedParam>,
}

/// A numeric-fluent function declaration nested inside `:predicates`, in the
/// `(= (fluent-name ?params...) value)` form. Lexed/parsed into a real AST
/// node so real-world HDDL files carrying this construct don't fail at the
/// tokenizer stage; `grounder::ground` refuses any domain declaring one of
/// these with `GroundError::UnsupportedNumericFluent` rather than silently
/// dropping it or attempting (unsupported) numeric grounding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumericFluentDecl {
    pub name: Name,
    pub params: Vec<TypedParam>,
    pub value: NumericValue,
}

/// One entry of a `:constraints` block (domain or problem level), e.g.
/// `(always (connected ?a ?b))` or `(sometime-after (p) (q))`. `kind` is the
/// constraint's modal-operator keyword (`always`, `sometime`,
/// `sometime-after`, `within`, `and`, ...), preserved verbatim; `raw` is a
/// canonical re-serialization of the full constraint sexp. Full PDDL 3.0
/// constraint-GD semantics are out of scope for this pass — the point of
/// this node is only to let real-world `:constraints` blocks lex/parse
/// cleanly instead of failing the tokenizer, while `grounder::ground` still
/// refuses them via `GroundError::UnsupportedConstraint`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintDef {
    pub kind: Name,
    pub raw: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDef {
    pub name: Name,
    pub params: Vec<TypedParam>,
    pub precondition: GoalDesc,
    pub effect: Effect,
    /// Weights declared by a `(:probabilistic w1 e1 w2 e2 ...)` effect block
    /// that `crate::probabilistic::preprocess` rewrote into this action's
    /// top-level `oneof` effect before real parsing, in declaration order
    /// (one weight per `effect`'s `Effect::Oneof` branch). Kept as raw source
    /// text rather than parsed to `f64` here -- the same "preserve source
    /// text, parse only where it's used" discipline `NumericValue::Number
    /// (String)` already follows elsewhere in this module, so no float
    /// `Eq`/`PartialEq` concerns leak into this `#[derive(... Eq)]` type.
    /// `None` for a plain (weightless) `oneof` or a deterministic effect.
    pub probability_weights: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDef {
    pub name: Name,
    pub params: Vec<TypedParam>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCall {
    pub name: Name,
    pub args: Vec<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subtask {
    pub id: Name,
    pub task: TaskCall,
}

/// `before` must be ordered strictly before `after` (subtask ids).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderEdge {
    pub before: Name,
    pub after: Name,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskNetwork {
    pub subtasks: Vec<Subtask>,
    pub order: Vec<OrderEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDef {
    pub name: Name,
    pub params: Vec<TypedParam>,
    pub task: TaskCall,
    pub network: TaskNetwork,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Domain {
    pub name: Name,
    pub types: TypeDef,
    pub constants: Vec<TypedObject>,
    pub predicates: Vec<PredicateDef>,
    /// Numeric-fluent function declarations found nested inside
    /// `:predicates` — see `NumericFluentDecl`. Always empty for a domain
    /// that doesn't use numeric fluents.
    pub numeric_fluents: Vec<NumericFluentDecl>,
    pub tasks: Vec<TaskDef>,
    pub actions: Vec<ActionDef>,
    pub methods: Vec<MethodDef>,
    /// `:constraints` block entries, if the domain declares one. Always
    /// empty for a domain with no `:constraints` section.
    pub constraints: Vec<ConstraintDef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Problem {
    pub name: Name,
    pub domain_name: Name,
    pub objects: Vec<TypedObject>,
    pub init: Vec<AtomicFormula>,
    pub goal: GoalDesc,
    pub htn: TaskNetwork,
    /// `:constraints` block entries, if the problem declares one. Always
    /// empty for a problem with no `:constraints` section.
    pub constraints: Vec<ConstraintDef>,
}
