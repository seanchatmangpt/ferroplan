//! HDDL text -> AST: a tokenizer + generic S-expression reader, then a
//! recursive-descent translation into `crate::ast` types.

use crate::ast::*;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Syntax(String),
    UnsupportedConstruct(String),
    MalformedOneof(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(msg) => write!(f, "syntax error: {msg}"),
            Self::UnsupportedConstruct(what) => write!(f, "unsupported construct: {what}"),
            Self::MalformedOneof(msg) => write!(f, "malformed oneof: {msg}"),
        }
    }
}
impl std::error::Error for ParseError {}

// ---------------------------------------------------------------------------
// Tokenizer + S-expression reader
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
}

fn tokenize(src: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        if c == ';' {
            while let Some(&c2) = chars.peek() {
                if c2 == '\n' {
                    break;
                }
                chars.next();
            }
            continue;
        }
        if c == '(' || c == ')' {
            tokens.push(c.to_string());
            chars.next();
            continue;
        }
        let mut s = String::new();
        while let Some(&c2) = chars.peek() {
            if c2.is_whitespace() || c2 == '(' || c2 == ')' || c2 == ';' {
                break;
            }
            s.push(c2);
            chars.next();
        }
        tokens.push(s);
    }
    tokens
}

fn read_one(tokens: &[String], pos: usize) -> Result<(Sexp, usize), ParseError> {
    match tokens.get(pos) {
        None => Err(ParseError::Syntax("unexpected end of input".to_owned())),
        Some(t) if t == "(" => {
            let mut items = Vec::new();
            let mut p = pos + 1;
            loop {
                match tokens.get(p) {
                    None => return Err(ParseError::Syntax("unbalanced parentheses".to_owned())),
                    Some(t2) if t2 == ")" => {
                        p += 1;
                        break;
                    }
                    _ => {
                        let (item, next) = read_one(tokens, p)?;
                        items.push(item);
                        p = next;
                    }
                }
            }
            Ok((Sexp::List(items), p))
        }
        Some(t) if t == ")" => Err(ParseError::Syntax("unexpected ')'".to_owned())),
        Some(t) => Ok((Sexp::Atom(t.clone()), pos + 1)),
    }
}

fn read_top(src: &str) -> Result<Sexp, ParseError> {
    let tokens = tokenize(src);
    if tokens.is_empty() {
        return Err(ParseError::Syntax("empty input".to_owned()));
    }
    let (sexp, _) = read_one(&tokens, 0)?;
    Ok(sexp)
}

/// Canonical re-serialization of an `Sexp`, used to preserve `:constraints`
/// entries verbatim (as `ConstraintDef::raw`) without needing a full nested
/// constraint-GD grammar in the AST.
fn sexp_to_string(s: &Sexp) -> String {
    match s {
        Sexp::Atom(a) => a.clone(),
        Sexp::List(items) => {
            let inner = items
                .iter()
                .map(sexp_to_string)
                .collect::<Vec<_>>()
                .join(" ");
            format!("({inner})")
        }
    }
}

/// Whether an atom lexes as a (possibly signed, possibly fractional) numeral,
/// per PDDL/HDDL numeric-literal syntax — used to tell a plain numeral value
/// apart from a fluent-reference term.
fn is_numeral(atom: &str) -> bool {
    !atom.is_empty() && atom.parse::<f64>().is_ok()
}

fn as_atom(s: &Sexp) -> Result<&str, ParseError> {
    match s {
        Sexp::Atom(a) => Ok(a.as_str()),
        Sexp::List(_) => Err(ParseError::Syntax(
            "expected an atom, found a list".to_owned(),
        )),
    }
}

fn as_list(s: &Sexp) -> Result<&[Sexp], ParseError> {
    match s {
        Sexp::List(items) => Ok(items.as_slice()),
        Sexp::Atom(a) => Err(ParseError::Syntax(format!(
            "expected a list, found atom '{a}'"
        ))),
    }
}

fn expect_atom(s: &Sexp, expected: &str) -> Result<(), ParseError> {
    let a = as_atom(s)?;
    if a != expected {
        return Err(ParseError::Syntax(format!(
            "expected '{expected}', found '{a}'"
        )));
    }
    Ok(())
}

/// A flat run of `name name ... - type` groups, as used by `:types`,
/// `:constants`/`:objects` and typed parameter lists. Names with no trailing
/// `- type` default to `object`.
fn parse_typed_group(items: &[Sexp]) -> Result<Vec<(String, String)>, ParseError> {
    let mut result = Vec::new();
    let mut pending = Vec::new();
    let mut i = 0;
    while i < items.len() {
        let atom = as_atom(&items[i])?;
        if atom == "-" {
            i += 1;
            let ty =
                as_atom(items.get(i).ok_or_else(|| {
                    ParseError::Syntax("expected a type name after '-'".to_owned())
                })?)?;
            for name in pending.drain(..) {
                result.push((name, ty.to_owned()));
            }
            i += 1;
        } else {
            pending.push(atom.to_owned());
            i += 1;
        }
    }
    for name in pending.drain(..) {
        result.push((name, "object".to_owned()));
    }
    Ok(result)
}

fn parse_typed_objects(items: &[Sexp]) -> Result<Vec<TypedObject>, ParseError> {
    Ok(parse_typed_group(items)?
        .into_iter()
        .map(|(name, type_name)| TypedObject { name, type_name })
        .collect())
}

fn parse_typed_params(items: &[Sexp]) -> Result<Vec<TypedParam>, ParseError> {
    parse_typed_group(items)?
        .into_iter()
        .map(|(var, type_name)| {
            let var = var.strip_prefix('?').ok_or_else(|| {
                ParseError::Syntax(format!("expected a variable ('?name'), found '{var}'"))
            })?;
            Ok(TypedParam {
                var: var.to_owned(),
                type_name,
            })
        })
        .collect()
}

fn parse_types(items: &[Sexp]) -> Result<TypeDef, ParseError> {
    let mut parent = BTreeMap::new();
    let mut declared = Vec::new();
    for (child, ty) in parse_typed_group(items)? {
        declared.push(child.clone());
        if ty != "object" {
            parent.insert(child, ty);
        }
    }
    Ok(TypeDef { parent, declared })
}

fn parse_term(s: &Sexp) -> Result<Term, ParseError> {
    let atom = as_atom(s)?;
    match atom.strip_prefix('?') {
        Some(v) => Ok(Term::Var(v.to_owned())),
        None => Ok(Term::Const(atom.to_owned())),
    }
}

fn parse_atomic(s: &Sexp) -> Result<AtomicFormula, ParseError> {
    let items = as_list(s)?;
    let predicate = as_atom(items.first().ok_or_else(|| {
        ParseError::Syntax("expected a predicate name in atomic formula".to_owned())
    })?)?
    .to_owned();
    let args = items[1..]
        .iter()
        .map(parse_term)
        .collect::<Result<_, _>>()?;
    Ok(AtomicFormula { predicate, args })
}

/// A numeric-fluent value: either a bare numeral (kept as source text) or a
/// reference to another fluent, `(fluent-name ?args...)`.
fn parse_numeric_value(s: &Sexp) -> Result<NumericValue, ParseError> {
    match s {
        Sexp::Atom(a) if is_numeral(a) => Ok(NumericValue::Number(a.clone())),
        Sexp::Atom(a) => Err(ParseError::Syntax(format!(
            "expected a numeral or a fluent reference, found '{a}'"
        ))),
        Sexp::List(_) => Ok(NumericValue::Fluent(parse_atomic(s)?)),
    }
}

/// `(= (fluent-name ?params...) value)`, as found nested inside `:predicates`.
fn parse_numeric_fluent_decl(items: &[Sexp]) -> Result<NumericFluentDecl, ParseError> {
    if items.len() != 3 {
        return Err(ParseError::Syntax(
            "expected '(= (fluent-name ?params...) value)'".to_owned(),
        ));
    }
    let sig = as_list(&items[1])?;
    let name = as_atom(sig.first().ok_or_else(|| {
        ParseError::Syntax("expected a fluent name in numeric fluent declaration".to_owned())
    })?)?
    .to_owned();
    let params = parse_typed_params(&sig[1..])?;
    let value = parse_numeric_value(&items[2])?;
    Ok(NumericFluentDecl {
        name,
        params,
        value,
    })
}

/// `:predicates` entries are either an ordinary predicate declaration
/// (`(name ?params...)`) or a numeric-fluent function declaration
/// (`(= (name ?params...) value)`) — the two are split apart here rather
/// than requiring a separate `:functions` section, matching real-world HDDL
/// files that declare numeric fluents this way.
fn parse_predicates(
    items: &[Sexp],
) -> Result<(Vec<PredicateDef>, Vec<NumericFluentDecl>), ParseError> {
    let mut predicates = Vec::new();
    let mut numeric_fluents = Vec::new();
    for s in items {
        let list = as_list(s)?;
        let head =
            as_atom(list.first().ok_or_else(|| {
                ParseError::Syntax("expected a predicate declaration".to_owned())
            })?)?;
        if head == "=" {
            numeric_fluents.push(parse_numeric_fluent_decl(list)?);
            continue;
        }
        let params = parse_typed_params(&list[1..])?;
        predicates.push(PredicateDef {
            name: head.to_owned(),
            params,
        });
    }
    Ok((predicates, numeric_fluents))
}

/// A single `:constraints` entry, e.g. `(always (connected ?a ?b))`. `kind`
/// is the modal-operator keyword; the whole sexp is also preserved verbatim
/// in `raw` — see `ConstraintDef`'s docs for why nothing deeper is parsed.
fn parse_constraint_entry(s: &Sexp) -> Result<ConstraintDef, ParseError> {
    let items = as_list(s)?;
    let kind = as_atom(
        items
            .first()
            .ok_or_else(|| ParseError::Syntax("expected a constraint entry".to_owned()))?,
    )?
    .to_owned();
    Ok(ConstraintDef {
        kind,
        raw: sexp_to_string(s),
    })
}

/// A `:constraints` section body: `(and c1 c2 ...)` flattens into one
/// `ConstraintDef` per conjunct; anything else is a single constraint entry.
fn parse_constraints(s: &Sexp) -> Result<Vec<ConstraintDef>, ParseError> {
    let items = as_list(s)?;
    if !items.is_empty() {
        if let Ok("and") = as_atom(&items[0]) {
            return items[1..].iter().map(parse_constraint_entry).collect();
        }
    }
    Ok(vec![parse_constraint_entry(s)?])
}

fn parse_goal(s: &Sexp) -> Result<GoalDesc, ParseError> {
    let items = as_list(s)?;
    if items.is_empty() {
        return Ok(GoalDesc::Empty);
    }
    let head = as_atom(&items[0])?;
    match head {
        "and" => Ok(GoalDesc::And(
            items[1..]
                .iter()
                .map(parse_goal)
                .collect::<Result<_, _>>()?,
        )),
        "not" => {
            if items.len() != 2 {
                return Err(ParseError::Syntax(
                    "'not' takes exactly one argument".to_owned(),
                ));
            }
            Ok(GoalDesc::Not(Box::new(parse_goal(&items[1])?)))
        }
        "or" => Ok(GoalDesc::Or(
            items[1..]
                .iter()
                .map(parse_goal)
                .collect::<Result<_, _>>()?,
        )),
        "imply" => {
            if items.len() != 3 {
                return Err(ParseError::Syntax(
                    "'imply' takes exactly two arguments".to_owned(),
                ));
            }
            Ok(GoalDesc::Imply(
                Box::new(parse_goal(&items[1])?),
                Box::new(parse_goal(&items[2])?),
            ))
        }
        "exists" | "forall" => Err(ParseError::UnsupportedConstruct(head.to_owned())),
        _ => {
            let args = items[1..]
                .iter()
                .map(parse_term)
                .collect::<Result<_, _>>()?;
            Ok(GoalDesc::Atom(AtomicFormula {
                predicate: head.to_owned(),
                args,
            }))
        }
    }
}

/// `allow_oneof` gates whether a `oneof` construct is legal at this position:
/// true only at the top of an action's `:effect`. A `oneof` nested inside
/// another `oneof` branch, or inside a `when`-effect, is a hard error.
fn parse_effect(s: &Sexp, allow_oneof: bool) -> Result<Effect, ParseError> {
    let items = as_list(s)?;
    if items.is_empty() {
        return Ok(Effect::Empty);
    }
    let head = as_atom(&items[0])?;
    match head {
        "and" => Ok(Effect::And(
            items[1..]
                .iter()
                .map(|e| parse_effect(e, allow_oneof))
                .collect::<Result<_, _>>()?,
        )),
        "not" => {
            if items.len() != 2 {
                return Err(ParseError::Syntax(
                    "'not' takes exactly one argument".to_owned(),
                ));
            }
            Ok(Effect::Literal(Literal::Neg(parse_atomic(&items[1])?)))
        }
        "when" => {
            if items.len() != 3 {
                return Err(ParseError::Syntax(
                    "'when' takes a condition and an effect".to_owned(),
                ));
            }
            let cond = parse_goal(&items[1])?;
            // oneof under when is out of scope regardless of the caller's own
            // allow_oneof — a when's effect is never itself a oneof position.
            let eff = parse_effect(&items[2], false)?;
            Ok(Effect::When(cond, Box::new(eff)))
        }
        "oneof" => {
            if !allow_oneof {
                return Err(ParseError::MalformedOneof(
                    "nested oneof or oneof under when is not permitted".to_owned(),
                ));
            }
            let branches = items[1..]
                .iter()
                .map(|e| parse_effect(e, false))
                .collect::<Result<_, _>>()?;
            Ok(Effect::Oneof(branches))
        }
        "increase" | "decrease" => {
            if items.len() != 3 {
                return Err(ParseError::Syntax(format!(
                    "'{head}' takes a fluent term and a value"
                )));
            }
            let fluent = parse_atomic(&items[1])?;
            let value = parse_numeric_value(&items[2])?;
            if head == "increase" {
                Ok(Effect::Increase(fluent, value))
            } else {
                Ok(Effect::Decrease(fluent, value))
            }
        }
        _ => Ok(Effect::Literal(Literal::Pos(parse_atomic(s)?))),
    }
}

/// Keyword/value pairs as used by `:action`, `:task`, `:method` and `:htn`
/// sections, e.g. `:parameters (...) :precondition (...) :effect (...)`.
fn keyed_map(items: &[Sexp]) -> Result<BTreeMap<String, &Sexp>, ParseError> {
    let mut map = BTreeMap::new();
    let mut i = 0;
    while i < items.len() {
        let key = as_atom(&items[i])?;
        if !key.starts_with(':') {
            return Err(ParseError::Syntax(format!(
                "expected a ':keyword', found '{key}'"
            )));
        }
        let value = items
            .get(i + 1)
            .ok_or_else(|| ParseError::Syntax(format!("missing value for '{key}'")))?;
        map.insert(key.to_owned(), value);
        i += 2;
    }
    Ok(map)
}

fn parse_task_call(s: &Sexp) -> Result<TaskCall, ParseError> {
    let items = as_list(s)?;
    let name = as_atom(
        items
            .first()
            .ok_or_else(|| ParseError::Syntax("expected a task name".to_owned()))?,
    )?
    .to_owned();
    let args = items[1..]
        .iter()
        .map(parse_term)
        .collect::<Result<_, _>>()?;
    Ok(TaskCall { name, args })
}

/// `(and (id1 (task args...)) (id2 (task args...)) ...)` — every subtask is
/// explicitly labeled in this frontend's grammar.
fn parse_labeled_subtasks(s: &Sexp) -> Result<Vec<Subtask>, ParseError> {
    let items = as_list(s)?;
    if !items.is_empty() {
        expect_atom(&items[0], "and")?;
    }
    let rest = if items.is_empty() { items } else { &items[1..] };
    rest.iter()
        .map(|entry| {
            let pair = as_list(entry)?;
            if pair.len() != 2 {
                return Err(ParseError::Syntax(
                    "expected '(id (task args...))' subtask entry".to_owned(),
                ));
            }
            let id = as_atom(&pair[0])?.to_owned();
            let task = parse_task_call(&pair[1])?;
            Ok(Subtask { id, task })
        })
        .collect()
}

/// `(and (before after) (before after) ...)` ordering-edge pairs.
fn parse_order_edges(s: &Sexp) -> Result<Vec<OrderEdge>, ParseError> {
    let items = as_list(s)?;
    if !items.is_empty() {
        expect_atom(&items[0], "and")?;
    }
    let rest = if items.is_empty() { items } else { &items[1..] };
    rest.iter()
        .map(|entry| {
            let pair = as_list(entry)?;
            if pair.len() != 2 {
                return Err(ParseError::Syntax(
                    "expected '(before after)' ordering edge".to_owned(),
                ));
            }
            Ok(OrderEdge {
                before: as_atom(&pair[0])?.to_owned(),
                after: as_atom(&pair[1])?.to_owned(),
            })
        })
        .collect()
}

fn parse_task_network(map: &BTreeMap<String, &Sexp>) -> Result<TaskNetwork, ParseError> {
    if let Some(s) = map
        .get(":ordered-subtasks")
        .or_else(|| map.get(":ordered-tasks"))
    {
        let subtasks = parse_labeled_subtasks(s)?;
        let order = subtasks
            .windows(2)
            .map(|w| OrderEdge {
                before: w[0].id.clone(),
                after: w[1].id.clone(),
            })
            .collect();
        return Ok(TaskNetwork { subtasks, order });
    }
    if let Some(s) = map.get(":subtasks").or_else(|| map.get(":tasks")) {
        let subtasks = parse_labeled_subtasks(s)?;
        let order = match map.get(":order").or_else(|| map.get(":ordering")) {
            Some(o) => parse_order_edges(o)?,
            None => vec![],
        };
        return Ok(TaskNetwork { subtasks, order });
    }
    Ok(TaskNetwork::default())
}

fn parse_task_def(rest: &[Sexp]) -> Result<TaskDef, ParseError> {
    let name = as_atom(
        rest.first()
            .ok_or_else(|| ParseError::Syntax("expected a task name".to_owned()))?,
    )?
    .to_owned();
    let map = keyed_map(&rest[1..])?;
    let params = match map.get(":parameters") {
        Some(s) => parse_typed_params(as_list(s)?)?,
        None => vec![],
    };
    Ok(TaskDef { name, params })
}

fn parse_action_def(rest: &[Sexp]) -> Result<ActionDef, ParseError> {
    let name = as_atom(
        rest.first()
            .ok_or_else(|| ParseError::Syntax("expected an action name".to_owned()))?,
    )?
    .to_owned();
    let map = keyed_map(&rest[1..])?;
    let params = match map.get(":parameters") {
        Some(s) => parse_typed_params(as_list(s)?)?,
        None => vec![],
    };
    let precondition = match map.get(":precondition") {
        Some(s) => parse_goal(s)?,
        None => GoalDesc::Empty,
    };
    let effect = match map.get(":effect") {
        Some(s) => parse_effect(s, true)?,
        None => Effect::Empty,
    };
    Ok(ActionDef {
        name,
        params,
        precondition,
        effect,
        // No `crate::probabilistic::preprocess` rewrite pass exists yet to
        // populate this from a `(:probabilistic ...)` block — see the field's
        // doc comment in `ast.rs`. `None` is the documented default for a
        // plain/deterministic effect and is what every current parse path
        // (this is the only `ActionDef` construction site) produces.
        probability_weights: None,
    })
}

fn parse_method_def(rest: &[Sexp]) -> Result<MethodDef, ParseError> {
    let name = as_atom(
        rest.first()
            .ok_or_else(|| ParseError::Syntax("expected a method name".to_owned()))?,
    )?
    .to_owned();
    let map = keyed_map(&rest[1..])?;
    let params = match map.get(":parameters") {
        Some(s) => parse_typed_params(as_list(s)?)?,
        None => vec![],
    };
    let task_sexp = map
        .get(":task")
        .ok_or_else(|| ParseError::Syntax("method is missing ':task'".to_owned()))?;
    let task = parse_task_call(task_sexp)?;
    let network = parse_task_network(&map)?;
    Ok(MethodDef {
        name,
        params,
        task,
        network,
    })
}

fn parse_htn(rest: &[Sexp]) -> Result<TaskNetwork, ParseError> {
    let map = keyed_map(rest)?;
    parse_task_network(&map)
}

pub fn parse_domain(src: &str) -> Result<Domain, ParseError> {
    // Pure text preprocessing pass, run before real tokenizing: rewrites any
    // koala-planner-style `(:probabilistic w1 e1 w2 e2 ...)` effect block
    // into a standard `(oneof e1 e2 ...)` block, handing back the declared
    // weights keyed by enclosing action name (see `probabilistic::preprocess`
    // for why the map is keyed that way). Everything below this line parses
    // `cleaned` exactly as before -- `:probabilistic` never reaches the
    // tokenizer/recursive-descent grammar at all.
    let (cleaned, weight_map) = crate::probabilistic::preprocess(src);
    let top = read_top(&cleaned)?;
    let items = as_list(&top)?;
    if items.is_empty() {
        return Err(ParseError::Syntax("empty 'define' form".to_owned()));
    }
    expect_atom(&items[0], "define")?;
    let header = as_list(
        items
            .get(1)
            .ok_or_else(|| ParseError::Syntax("missing '(domain name)' header".to_owned()))?,
    )?;
    expect_atom(&header[0], "domain")?;
    let name = as_atom(&header[1])?.to_owned();

    let mut domain = Domain {
        name,
        ..Domain::default()
    };
    for section in &items[2..] {
        let sec = as_list(section)?;
        let keyword = as_atom(
            sec.first()
                .ok_or_else(|| ParseError::Syntax("empty domain section".to_owned()))?,
        )?;
        match keyword {
            ":requirements" => {}
            ":types" => domain.types = parse_types(&sec[1..])?,
            ":constants" => domain.constants = parse_typed_objects(&sec[1..])?,
            ":predicates" => {
                let (predicates, numeric_fluents) = parse_predicates(&sec[1..])?;
                domain.predicates = predicates;
                domain.numeric_fluents = numeric_fluents;
            }
            ":task" => domain.tasks.push(parse_task_def(&sec[1..])?),
            ":action" => domain.actions.push(parse_action_def(&sec[1..])?),
            ":method" => domain.methods.push(parse_method_def(&sec[1..])?),
            ":constraints" => domain.constraints = parse_constraints(&sec[1])?,
            ":functions" => {
                return Err(ParseError::UnsupportedConstruct(keyword.to_owned()));
            }
            other => {
                return Err(ParseError::Syntax(format!(
                    "unknown domain section '{other}'"
                )))
            }
        }
    }

    // Attach each action's declared `:probabilistic` weights (extracted by
    // the preprocessing pass above, before this action's effect was even a
    // `oneof` yet) now that the action itself exists as a real `ActionDef`.
    for action in &mut domain.actions {
        if let Some(weights) = weight_map.get(&action.name) {
            action.probability_weights = Some(weights.clone());
        }
    }

    Ok(domain)
}

pub fn parse_problem(src: &str) -> Result<Problem, ParseError> {
    let top = read_top(src)?;
    let items = as_list(&top)?;
    if items.is_empty() {
        return Err(ParseError::Syntax("empty 'define' form".to_owned()));
    }
    expect_atom(&items[0], "define")?;
    let header = as_list(
        items
            .get(1)
            .ok_or_else(|| ParseError::Syntax("missing '(problem name)' header".to_owned()))?,
    )?;
    expect_atom(&header[0], "problem")?;
    let name = as_atom(&header[1])?.to_owned();

    let mut problem = Problem {
        name,
        ..Problem::default()
    };
    for section in &items[2..] {
        let sec = as_list(section)?;
        let keyword = as_atom(
            sec.first()
                .ok_or_else(|| ParseError::Syntax("empty problem section".to_owned()))?,
        )?;
        match keyword {
            ":domain" => problem.domain_name = as_atom(&sec[1])?.to_owned(),
            ":objects" => problem.objects = parse_typed_objects(&sec[1..])?,
            ":init" => {
                problem.init = sec[1..]
                    .iter()
                    .map(parse_atomic)
                    .collect::<Result<_, _>>()?;
            }
            ":goal" => problem.goal = parse_goal(&sec[1])?,
            ":htn" => problem.htn = parse_htn(&sec[1..])?,
            ":constraints" => problem.constraints = parse_constraints(&sec[1])?,
            other => {
                return Err(ParseError::Syntax(format!(
                    "unknown problem section '{other}'"
                )))
            }
        }
    }
    Ok(problem)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_A_DOMAIN: &str = include_str!("../fixtures/a/domain.hddl");
    const FIXTURE_A_PROBLEM: &str = include_str!("../fixtures/a/problem.hddl");
    const FIXTURE_B_DOMAIN: &str = include_str!("../fixtures/b/domain.hddl");
    const FIXTURE_C_DOMAIN: &str = include_str!("../fixtures/c/domain.hddl");
    const FIXTURE_E_DOMAIN: &str = include_str!("../fixtures/e/domain.hddl");

    #[test]
    fn fixture_e_probabilistic_effect_rewrites_to_oneof_with_weights() {
        let domain = parse_domain(FIXTURE_E_DOMAIN).expect("fixture E domain parses");
        let action = domain
            .actions
            .iter()
            .find(|a| a.name == "toss-coin")
            .expect("toss-coin action present");
        match &action.effect {
            Effect::Oneof(branches) => assert_eq!(branches.len(), 2),
            other => panic!("expected a oneof effect, got {other:?}"),
        }
        assert_eq!(
            action.probability_weights,
            Some(vec!["3".to_owned(), "1".to_owned()])
        );
    }

    #[test]
    fn fixture_a_domain_structure() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).expect("fixture A domain parses");
        assert_eq!(domain.name, "transport-a");
        assert_eq!(domain.actions.len(), 3);
        assert_eq!(domain.methods.len(), 1);
        let method = &domain.methods[0];
        assert_eq!(method.network.subtasks.len(), 3);
        assert_eq!(
            method.network.order,
            vec![
                OrderEdge {
                    before: "t1".to_owned(),
                    after: "t2".to_owned()
                },
                OrderEdge {
                    before: "t2".to_owned(),
                    after: "t3".to_owned()
                },
            ]
        );
    }

    #[test]
    fn fixture_a_problem_structure() {
        let problem = parse_problem(FIXTURE_A_PROBLEM).expect("fixture A problem parses");
        assert_eq!(problem.domain_name, "transport-a");
        assert_eq!(problem.objects.len(), 2);
        assert_eq!(problem.init.len(), 2);
        assert_eq!(problem.htn.subtasks.len(), 1);
        assert_eq!(problem.htn.order.len(), 0);
    }

    #[test]
    fn fixture_b_partial_order_structure() {
        let domain = parse_domain(FIXTURE_B_DOMAIN).expect("fixture B domain parses");
        assert_eq!(domain.methods.len(), 1);
        let method = &domain.methods[0];
        assert_eq!(method.network.subtasks.len(), 5);
        assert_eq!(method.network.order.len(), 4);
        let edges = method
            .network
            .order
            .iter()
            .map(|e| (e.before.clone(), e.after.clone()))
            .collect::<std::collections::BTreeSet<_>>();
        let expected = [("t1", "t3"), ("t2", "t4"), ("t3", "t5"), ("t4", "t5")]
            .into_iter()
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(edges, expected);
    }

    #[test]
    fn fixture_c_oneof_structure() {
        let domain = parse_domain(FIXTURE_C_DOMAIN).expect("fixture C domain parses");
        assert_eq!(domain.methods.len(), 2);
        let action = domain
            .actions
            .iter()
            .find(|a| a.name == "cross-bridge")
            .expect("cross-bridge action present");
        match &action.effect {
            Effect::Oneof(branches) => assert_eq!(branches.len(), 2),
            other => panic!("expected a oneof effect, got {other:?}"),
        }
    }

    #[test]
    fn rejects_forall_in_precondition() {
        let src = r#"(define (domain bad)
          (:predicates (p ?x - object))
          (:action a
            :parameters (?x - object)
            :precondition (forall (?y - object) (p ?y))
            :effect (and (p ?x))))"#;
        let err = parse_domain(src).unwrap_err();
        assert!(matches!(err, ParseError::UnsupportedConstruct(ref c) if c == "forall"));
    }

    #[test]
    fn rejects_nested_oneof() {
        let src = r#"(define (domain bad)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (and (p)) (oneof (and (q)) (and (p))))))"#;
        let err = parse_domain(src).unwrap_err();
        assert!(matches!(err, ParseError::MalformedOneof(_)));
    }

    const NUMERIC_DOMAIN: &str = r#"(define (domain numeric-d)
      (:types loc vehicle)
      (:predicates
        (at ?l - loc)
        (= (fuel-level ?v - vehicle) 0))
      (:constraints
        (and
          (always (at ?l))
          (sometime (at ?l))))
      (:action refuel
        :parameters (?v - vehicle)
        :precondition ()
        :effect (and
          (increase (fuel-level ?v) 10)
          (decrease (fuel-level ?v) 1))))"#;

    #[test]
    fn lexes_and_parses_numeric_fluent_declaration() {
        let domain = parse_domain(NUMERIC_DOMAIN).expect("numeric-fluent domain parses");
        assert_eq!(domain.predicates.len(), 1);
        assert_eq!(domain.predicates[0].name, "at");
        assert_eq!(domain.numeric_fluents.len(), 1);
        let fluent = &domain.numeric_fluents[0];
        assert_eq!(fluent.name, "fuel-level");
        assert_eq!(fluent.params.len(), 1);
        assert_eq!(fluent.params[0].var, "v");
        assert_eq!(fluent.params[0].type_name, "vehicle");
        assert_eq!(fluent.value, NumericValue::Number("0".to_owned()));
    }

    #[test]
    fn lexes_and_parses_domain_constraints_block() {
        let domain = parse_domain(NUMERIC_DOMAIN).expect("constraints domain parses");
        assert_eq!(domain.constraints.len(), 2);
        assert_eq!(domain.constraints[0].kind, "always");
        assert_eq!(domain.constraints[1].kind, "sometime");
        assert!(domain.constraints[0].raw.contains("at ?l"));
    }

    #[test]
    fn lexes_and_parses_problem_constraints_block() {
        let src = r#"(define (problem p)
          (:domain d)
          (:objects a b - object)
          (:init (p a))
          (:goal (p a))
          (:constraints (and (always (p a)) (sometime-after (p a) (p b)))))"#;
        let problem = parse_problem(src).expect("problem with :constraints parses");
        assert_eq!(problem.constraints.len(), 2);
        assert_eq!(problem.constraints[0].kind, "always");
        assert_eq!(problem.constraints[1].kind, "sometime-after");
    }

    #[test]
    fn lexes_and_parses_increase_and_decrease_effects() {
        let domain = parse_domain(NUMERIC_DOMAIN).expect("increase/decrease domain parses");
        let action = domain
            .actions
            .iter()
            .find(|a| a.name == "refuel")
            .expect("refuel action present");
        match &action.effect {
            Effect::And(effects) => {
                assert_eq!(effects.len(), 2);
                match &effects[0] {
                    Effect::Increase(fluent, value) => {
                        assert_eq!(fluent.predicate, "fuel-level");
                        assert_eq!(*value, NumericValue::Number("10".to_owned()));
                    }
                    other => panic!("expected an 'increase' effect, got {other:?}"),
                }
                match &effects[1] {
                    Effect::Decrease(fluent, value) => {
                        assert_eq!(fluent.predicate, "fuel-level");
                        assert_eq!(*value, NumericValue::Number("1".to_owned()));
                    }
                    other => panic!("expected a 'decrease' effect, got {other:?}"),
                }
            }
            other => panic!("expected an 'and' effect, got {other:?}"),
        }
    }

    #[test]
    fn increase_value_may_reference_another_fluent() {
        let src = r#"(define (domain d)
          (:predicates (= (a) 0) (= (b) 0))
          (:action bump
            :parameters ()
            :precondition ()
            :effect (and (increase (a) (b)))))"#;
        let domain = parse_domain(src).expect("fluent-valued increase parses");
        let action = &domain.actions[0];
        match &action.effect {
            Effect::And(effects) => match &effects[0] {
                Effect::Increase(fluent, NumericValue::Fluent(value_fluent)) => {
                    assert_eq!(fluent.predicate, "a");
                    assert_eq!(value_fluent.predicate, "b");
                }
                other => panic!("expected an increase-by-fluent effect, got {other:?}"),
            },
            other => panic!("expected an 'and' effect, got {other:?}"),
        }
    }

    #[test]
    fn parses_or_precondition() {
        let src = r#"(define (domain d)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition (or (p) (q))
            :effect (and (p))))"#;
        let domain = parse_domain(src).expect("'or' precondition parses");
        match &domain.actions[0].precondition {
            GoalDesc::Or(branches) => {
                assert_eq!(branches.len(), 2);
                assert!(matches!(&branches[0], GoalDesc::Atom(a) if a.predicate == "p"));
                assert!(matches!(&branches[1], GoalDesc::Atom(a) if a.predicate == "q"));
            }
            other => panic!("expected an 'or' goal description, got {other:?}"),
        }
    }

    #[test]
    fn parses_imply_precondition() {
        let src = r#"(define (domain d)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition (imply (p) (q))
            :effect (and (q))))"#;
        let domain = parse_domain(src).expect("'imply' precondition parses");
        match &domain.actions[0].precondition {
            GoalDesc::Imply(ante, conseq) => {
                assert!(matches!(ante.as_ref(), GoalDesc::Atom(a) if a.predicate == "p"));
                assert!(matches!(conseq.as_ref(), GoalDesc::Atom(a) if a.predicate == "q"));
            }
            other => panic!("expected an 'imply' goal description, got {other:?}"),
        }
    }

    #[test]
    fn imply_requires_exactly_two_arguments() {
        let src = r#"(define (domain d)
          (:predicates (p))
          (:action a
            :parameters ()
            :precondition (imply (p))
            :effect (and (p))))"#;
        let err = parse_domain(src).unwrap_err();
        assert!(matches!(err, ParseError::Syntax(_)));
    }

    #[test]
    fn rejects_exists_in_precondition() {
        let src = r#"(define (domain bad)
          (:predicates (p ?x - object))
          (:action a
            :parameters (?x - object)
            :precondition (exists (?y - object) (p ?y))
            :effect (and (p ?x))))"#;
        let err = parse_domain(src).unwrap_err();
        assert!(matches!(err, ParseError::UnsupportedConstruct(ref c) if c == "exists"));
    }

    // -- fixture F: real external IPC 2020 corpus domain --------------------

    const FIXTURE_F_DOMAIN: &str = include_str!("../fixtures/f/domain.hddl");

    /// Fixture F is fetched **verbatim** (byte-for-byte, via `curl`, no hand
    /// edits) from the real IPC 2020 HTN-planning benchmark corpus:
    /// `panda-planner-dev/ipc2020-domains`, `total-order/Blocksworld-HPDDL/`
    /// (`domain.hddl` + `pfile_005.hddl`) — see `fixtures/f/domain.hddl`'s
    /// and `fixtures/f/problem.hddl`'s content for the exact text; source
    /// URLs:
    /// `https://raw.githubusercontent.com/panda-planner-dev/ipc2020-domains/master/total-order/Blocksworld-HPDDL/domain.hddl`
    /// and `.../pfile_005.hddl`. This is a classical (non-FOND) total-order
    /// HTN domain — no `oneof` — chosen because it was the one domain in
    /// that research pass with full verbatim text and a source URL.
    ///
    /// It does NOT fit this crate's scope, but not for the reason expected
    /// going in (the `setdone` method's `(forall (?b - BLOCK) (done ?b))`
    /// precondition, which this crate's parser is built to hard-refuse via
    /// `ParseError::UnsupportedConstruct` — see the `rejects_forall_in_
    /// precondition` test above). That refusal is never reached: PANDA's own
    /// HDDL dialect allows a bare, unlabeled task call inside
    /// `:ordered-tasks`/`:ordered-subtasks` when no explicit ordering-by-id
    /// is needed (e.g. this domain's very first method body,
    /// `:ordered-tasks (and (mark_done ?b) (achieve-goals))`), whereas this
    /// crate's grammar requires every subtask to be an explicit
    /// `(id (task args...))` pair (`parse_labeled_subtasks`, "every subtask
    /// is explicitly labeled in this frontend's grammar"). The bare 2-token
    /// entry `(mark_done ?b)` is misread as such a pair (id = "mark_done",
    /// task-sexp = the atom `?b`), and `?b` is not a list, so parsing fails
    /// immediately with a real, correctly-typed `ParseError::Syntax` at that
    /// exact point — a loud, precise, non-panicking rejection, not a crash
    /// and not a wrong error, it just fires on a different construct (bare
    /// subtask calls, a total-order PANDA-dialect convention this frontend's
    /// stricter always-labeled-subtask grammar does not accept) than the one
    /// anticipated. As a secondary, independently real finding: even past
    /// that point, method-level `:precondition` (including the `forall`
    /// above) is never read by `parse_method_def` at all — `MethodDef` has
    /// no precondition field — so it would have been silently dropped rather
    /// than hitting the `forall`/`exists` hard-refusal in `parse_goal`,
    /// which only fires for action preconditions and `:goal`.
    #[test]
    fn fixture_f_real_ipc2020_blocksworld_rejects_unlabeled_subtask_call() {
        let err = parse_domain(FIXTURE_F_DOMAIN).unwrap_err();
        assert_eq!(
            err,
            ParseError::Syntax("expected a list, found atom '?b'".to_owned())
        );
    }
}
