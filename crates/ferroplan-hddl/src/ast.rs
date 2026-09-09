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
//! Deliberately NOT represented (a parse of one of these must be a hard error,
//! never a silent drop): numeric fluents, temporal/durative actions,
//! `:constraints`, `exists`/`forall`/`or`/`imply` in goal descriptions, negative
//! goal literals in `:goal` (rejected at the translation boundary — see
//! `translate.rs`), and nested `oneof` / `oneof` under `when`.

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

/// Preconditions and `:goal` — the scoped subset (and/not/atom only).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GoalDesc {
    #[default]
    Empty,
    Atom(AtomicFormula),
    Not(Box<GoalDesc>),
    And(Vec<GoalDesc>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Pos(AtomicFormula),
    Neg(AtomicFormula),
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
}

/// type_name -> parent type_name. A type with no declared parent in the source
/// (the untyped tail of a `:types` list, or "object" itself) has no entry here.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeDef {
    pub parent: BTreeMap<Name, Name>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateDef {
    pub name: Name,
    pub params: Vec<TypedParam>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDef {
    pub name: Name,
    pub params: Vec<TypedParam>,
    pub precondition: GoalDesc,
    pub effect: Effect,
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
    pub tasks: Vec<TaskDef>,
    pub actions: Vec<ActionDef>,
    pub methods: Vec<MethodDef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Problem {
    pub name: Name,
    pub domain_name: Name,
    pub objects: Vec<TypedObject>,
    pub init: Vec<AtomicFormula>,
    pub goal: GoalDesc,
    pub htn: TaskNetwork,
}
