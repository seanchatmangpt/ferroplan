//! Target-independent admission guards for counterfactual session probes.
//!
//! Both probe surfaces (`wasi_abi::op_session_probe` for beam4pm and
//! `WasmSession::probe_json` for the browser) are compile-time exclusive, so
//! the guards they must share live here, compiled on every target and unit
//! tested natively. Each guard returns evidence (the offending value), never
//! authority: a refusal only stops a candidate from being forked.

use std::collections::BTreeSet;

/// Maximum candidate id length (bytes) accepted by either probe surface.
pub const MAX_PROBE_ID_BYTES: usize = 256;

/// Canonical key for a fact display: ASCII-uppercased with whitespace runs
/// collapsed, matching how the grounder canonicalises `(at a)` / `(AT  A)`.
pub fn fact_key(fact: &str) -> String {
    fact.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_uppercase()
}

/// First candidate id that is empty or longer than [`MAX_PROBE_ID_BYTES`].
pub fn malformed_id<'a>(ids: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    ids.into_iter()
        .find(|id| id.is_empty() || id.len() > MAX_PROBE_ID_BYTES)
}

/// First candidate id delivered more than once in one probe request.
/// Duplicate ids make the result set ambiguous for a consumer that keys
/// results by id, so the whole request is refused before any fork.
pub fn duplicate_id<'a>(ids: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    let mut seen = BTreeSet::new();
    ids.into_iter().find(|id| !seen.insert(*id))
}

/// First fact observed with both `true` and `false` in one candidate's
/// sight. `Session::observe` is last-write-wins, so a contradictory sight
/// would silently depend on list order; it is refused instead.
pub fn contradictory_fact(sight: &[(String, bool)]) -> Option<&str> {
    let mut seen: std::collections::BTreeMap<String, bool> = std::collections::BTreeMap::new();
    for (fact, value) in sight {
        match seen.insert(fact_key(fact), *value) {
            Some(previous) if previous != *value => return Some(fact.as_str()),
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fact_key_collapses_case_and_whitespace() {
        assert_eq!(fact_key("(at  a)"), "(AT A)");
        assert_eq!(fact_key(" (AT A) "), "(AT A)");
    }

    #[test]
    fn malformed_id_refuses_empty_and_oversized_ids() {
        assert_eq!(malformed_id(["a", "", "b"]), Some(""));
        let long = "x".repeat(MAX_PROBE_ID_BYTES + 1);
        assert_eq!(malformed_id(["a", long.as_str()]), Some(long.as_str()));
        let edge = "x".repeat(MAX_PROBE_ID_BYTES);
        assert_eq!(malformed_id(["a", edge.as_str()]), None);
    }

    #[test]
    fn duplicate_id_finds_the_second_delivery() {
        assert_eq!(duplicate_id(["a", "b", "a"]), Some("a"));
        assert_eq!(duplicate_id(["a", "b", "c"]), None);
        assert_eq!(duplicate_id(Vec::<&str>::new()), None);
    }

    #[test]
    fn contradictory_fact_is_order_and_spelling_independent() {
        let sight = vec![("(at a)".to_string(), false), ("(AT  A)".to_string(), true)];
        assert_eq!(contradictory_fact(&sight), Some("(AT  A)"));
        let reordered = vec![("(AT A)".to_string(), true), ("(at a)".to_string(), false)];
        assert_eq!(contradictory_fact(&reordered), Some("(at a)"));
        let repeated = vec![("(at a)".to_string(), true), ("(AT A)".to_string(), true)];
        assert_eq!(contradictory_fact(&repeated), None);
    }
}
