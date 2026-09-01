//! Persistent session authority, driven through the real merged MCP stdio
//! server.
//!
//! The authoritative surface grounds one named repository mind, admits
//! observations and goal changes, follows or replans under a declared budget,
//! advances a plan cursor, reports standing, and closes the session. Forking,
//! list-all sessions, ambient state mutation, and implicit handle minting are
//! intentionally not part of this authority.

mod common;

use common::{Client, DOM, PROB};
use serde_json::{json, Value};

fn open(c: &mut Client, session_id: &str) -> Value {
    c.call_json(
        "session_open",
        json!({
            "session_id": session_id,
            "domain": DOM,
            "problem": PROB
        }),
    )
}

fn reported_plan_length(think: &Value) -> usize {
    think["plan"]["steps"]
        .as_array()
        .map(Vec::len)
        .or_else(|| think["solution"]["plan"]["steps"].as_array().map(Vec::len))
        .unwrap_or(0)
}

fn assert_blake3_hex(value: &str) {
    assert_eq!(value.len(), 64, "BLAKE3 receipt must be 32 bytes in hex");
    assert!(
        value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')),
        "BLAKE3 receipt must be canonical lowercase hex: {value}"
    );
}

#[test]
fn open_observe_then_think_replans_from_the_admitted_state() {
    let mut c = Client::start();
    let sid = "stateful-replan";
    let opened = open(&mut c, sid);
    assert_eq!(opened["schema"], "urn:chatman:ferroplan-session-open:v1");
    assert_eq!(opened["session_id"], sid);
    assert_eq!(opened["goal_met"], false);

    let first = c.call_json(
        "session_think",
        json!({"session_id": sid, "max_evaluated": 10_000}),
    );
    assert_eq!(first["decision"], "replan");
    assert_eq!(reported_plan_length(&first), 2, "initial plan is A then B");

    // The external world reports that A already happened. Observation is the
    // only state-mutation path; completed plan steps are not ambiently applied.
    let observed = c.call_json(
        "session_observe",
        json!({
            "session_id": sid,
            "facts": [
                {"fact": "(P)", "value": false},
                {"fact": "(Q)", "value": true}
            ],
            "fluents": []
        }),
    );
    assert_eq!(observed["schema"], "urn:chatman:ferroplan-observation:v1");
    assert_eq!(observed["fact_surprises"].as_array().unwrap().len(), 2);
    assert_eq!(observed["replan_required"], true);

    let second = c.call_json(
        "session_think",
        json!({"session_id": sid, "max_evaluated": 10_000}),
    );
    assert_eq!(second["decision"], "replan");
    assert_eq!(reported_plan_length(&second), 1, "only B remains");
    assert_eq!(second["solution"]["plan"]["steps"][0]["action"], "B");
    c.finish();
}

#[test]
fn session_id_is_caller_owned_and_replace_is_explicit() {
    let mut c = Client::start();
    let sid = "caller-owned-id";
    let first = open(&mut c, sid);
    let first_receipt = first["receipt"].as_str().unwrap_or_default();
    assert_blake3_hex(first_receipt);

    let (text, error) = c.call_text(
        "session_open",
        json!({"session_id": sid, "domain": DOM, "problem": PROB}),
    );
    assert!(error, "duplicate open must be refused");
    assert!(
        text.contains("already exists"),
        "unexpected refusal: {text}"
    );

    let replacement = c.call_json(
        "session_open",
        json!({
            "session_id": sid,
            "domain": DOM,
            "problem": PROB,
            "replace": true
        }),
    );
    assert_eq!(replacement["session_id"], sid);
    assert_blake3_hex(
        replacement["receipt"]
            .as_str()
            .expect("replacement receipt"),
    );
    c.finish();
}

#[test]
fn observe_reports_only_contradictions_and_updates_the_receipt_head() {
    let mut c = Client::start();
    let sid = "surprise-boundary";
    let opened = open(&mut c, sid);
    let open_receipt = opened["receipt"].as_str().expect("open receipt").to_owned();
    assert_blake3_hex(&open_receipt);

    let quiet = c.call_json(
        "session_observe",
        json!({
            "session_id": sid,
            "facts": [{"fact": "(P)", "value": true}],
            "fluents": []
        }),
    );
    assert!(quiet["fact_surprises"].as_array().unwrap().is_empty());
    assert_eq!(quiet["epoch"], 0);
    let quiet_receipt = quiet["receipt"]
        .as_str()
        .expect("quiet observation receipt")
        .to_owned();
    assert_blake3_hex(&quiet_receipt);
    assert_ne!(quiet_receipt, open_receipt);

    let news = c.call_json(
        "session_observe",
        json!({
            "session_id": sid,
            "facts": [{"fact": "(Q)", "value": true}],
            "fluents": []
        }),
    );
    assert_eq!(news["fact_surprises"].as_array().unwrap().len(), 1);
    assert_eq!(news["epoch"], 1);
    let news_receipt = news["receipt"]
        .as_str()
        .expect("surprise receipt")
        .to_owned();
    assert_blake3_hex(&news_receipt);
    assert_ne!(news_receipt, quiet_receipt);

    let status = c.call_json("session_status", json!({"session_id": sid}));
    assert_eq!(status["receipt_chain_head"], news_receipt);
    c.finish();
}

#[test]
fn think_budget_is_a_hard_bounded_contract() {
    let mut c = Client::start();
    let sid = "bounded-think";
    open(&mut c, sid);

    let (zero_text, zero_error) = c.call_text(
        "session_think",
        json!({"session_id": sid, "max_evaluated": 0}),
    );
    assert!(zero_error);
    assert!(zero_text.contains("greater than zero"));

    let (ceiling_text, ceiling_error) = c.call_text(
        "session_think",
        json!({"session_id": sid, "max_evaluated": 10_000_001}),
    );
    assert!(ceiling_error);
    assert!(ceiling_text.contains("at most 10000000"));

    let admitted = c.call_json(
        "session_think",
        json!({"session_id": sid, "max_evaluated": 10_000}),
    );
    assert!(matches!(
        admitted["decision"].as_str(),
        Some("follow" | "replan" | "bounded-refusal")
    ));
    c.finish();
}

#[test]
fn status_close_and_unknown_handle_refusals_preserve_the_server() {
    let mut c = Client::start();
    let sid = "close-boundary";
    open(&mut c, sid);

    let status = c.call_json("session_status", json!({"session_id": sid}));
    assert_eq!(status["schema"], "urn:chatman:ferroplan-session-status:v1");
    assert_eq!(status["session_id"], sid);
    assert_eq!(status["cursor"], 0);

    let (unknown_text, unknown_error) =
        c.call_text("session_status", json!({"session_id": "never-opened"}));
    assert!(unknown_error);
    assert!(unknown_text.contains("unknown session `never-opened`"));

    let closed = c.call_json("session_close", json!({"session_id": sid}));
    assert_eq!(closed["closed"], true);
    let closed_again = c.call_json("session_close", json!({"session_id": sid}));
    assert_eq!(
        closed_again["closed"], false,
        "close is observable and idempotent"
    );

    let (after_text, after_error) = c.call_text("session_status", json!({"session_id": sid}));
    assert!(after_error);
    assert!(after_text.contains("unknown session"));

    // A refused session lookup does not terminate the merged server.
    let report = c.call_json("parse", json!({"pddl": DOM}));
    assert_eq!(report["ok"], true);
    c.finish();
}

// ---- 0.24 Phase 5: the budget-stamped think contract on the wire -----------

/// The farm from the library's session tests: three steps of work, so a
/// 1-eval think honestly fails and a fed one solves.
const FARM_DOM: &str = "
(define (domain farm) (:requirements :strips :typing :numeric-fluents)
  (:types agent place)
  (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
  (:functions (grain))
  (:action walk :parameters (?a - agent ?from ?to - place)
    :precondition (and (at ?a ?from) (road ?from ?to))
    :effect (and (not (at ?a ?from)) (at ?a ?to)))
  (:action harvest :parameters (?a - agent ?p - place)
    :precondition (and (at ?a ?p) (fertile ?p))
    :effect (increase (grain) 1)))";
const FARM_PRB: &str = "
(define (problem p) (:domain farm)
  (:objects v1 - agent hut field - place)
  (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
  (:goal (>= (grain) 2)))";

/// Two interchangeable balls — the orbit-aware replan's wire witness (the
/// unary-goal SOLO1 shape the library fixture pins).
const ORB_DOM: &str = "
(define (domain rollers) (:requirements :strips :typing)
  (:types ball room)
  (:predicates (at ?b - ball ?r - room) (link ?x ?y - room)
               (goal-room ?r - room) (home ?b - ball))
  (:action roll :parameters (?b - ball ?from ?to - room)
    :precondition (and (at ?b ?from) (link ?from ?to))
    :effect (and (not (at ?b ?from)) (at ?b ?to)))
  (:action park :parameters (?b - ball ?r - room)
    :precondition (and (at ?b ?r) (goal-room ?r))
    :effect (home ?b)))";
const ORB_PRB: &str = "
(define (problem p) (:domain rollers)
  (:objects b1 b2 - ball ra rb - room)
  (:init (at b1 ra) (at b2 ra) (link ra rb) (link rb ra) (goal-room rb))
  (:goal (and (home b1) (home b2))))";

/// 0.24: every replan is budget-stamped — capped, spent_ms, spent_evals,
/// verdict ride alongside the unchanged Solution fields, and the memory
/// split stays honest across a stamped think.
#[test]
fn a_replan_is_budget_stamped_on_the_wire() {
    let mut c = Client::start();
    let s = c.call_json("session_open", json!({"domain": DOM, "problem": PROB}));
    let sid = s["session_id"].as_str().unwrap().to_string();
    let before = c.call_json("session_state", json!({"session_id": sid}));

    let sol = c.call_json(
        "session_replan",
        json!({"session_id": sid, "max_evaluated": 10000, "wall_ms": 60000, "memory_mb": 64}),
    );
    assert_eq!(sol["solved"], true);
    assert_eq!(sol["plan"]["length"], 2, "the Solution shape is unchanged");
    assert_eq!(sol["capped"], false);
    assert_eq!(sol["verdict"], "solved");
    assert!(sol["spent_evals"].as_u64().unwrap() >= 1, "{sol}");
    assert!(sol["spent_ms"].is_u64(), "{sol}");

    let after = c.call_json("session_state", json!({"session_id": sid}));
    assert_eq!(before["world_bytes"], after["world_bytes"]);
    assert_eq!(before["mind_bytes"], after["mind_bytes"]);
    assert!(after["world_bytes"].as_u64().unwrap() > 0);
    c.finish();
}

/// The capped-search honesty, verbatim on the wire: a budget-starved think
/// says `capped`, never anything an agent could read as "unsolvable".
#[test]
fn a_capped_think_never_reads_unsolvable_on_the_wire() {
    let mut c = Client::start();
    let s = c.call_json(
        "session_open",
        json!({"domain": FARM_DOM, "problem": FARM_PRB}),
    );
    let sid = s["session_id"].as_str().unwrap().to_string();

    let (text, err) = c.call_text(
        "session_replan",
        json!({"session_id": sid, "max_evaluated": 1, "memory_mb": 1}),
    );
    assert!(!err, "a capped think is an answer, not an error: {text}");
    let sol: serde_json::Value = serde_json::from_str(&text).expect("stamped JSON");
    assert_eq!(sol["solved"], false);
    assert_eq!(sol["capped"], true);
    assert_eq!(sol["verdict"], "capped");
    assert!(
        !text.to_lowercase().contains("unsolvable"),
        "the cap honesty must reach the wire verbatim: {text}"
    );

    // Round trip: the same session, properly fed, solves.
    let sol = c.call_json(
        "session_replan",
        json!({"session_id": sid, "max_evaluated": 100000, "wall_ms": 60000}),
    );
    assert_eq!(sol["solved"], true);
    assert_eq!(sol["verdict"], "solved");
    c.finish();
}

/// Orbit-aware replans reach the wire: a symmetric world's think narrates
/// the re-detected orbit in its notes.
#[test]
fn an_orbit_aware_replan_narrates_itself() {
    let mut c = Client::start();
    let s = c.call_json(
        "session_open",
        json!({"domain": ORB_DOM, "problem": ORB_PRB}),
    );
    let sid = s["session_id"].as_str().unwrap().to_string();
    let sol = c.call_json(
        "session_replan",
        json!({"session_id": sid, "max_evaluated": 10000}),
    );
    assert_eq!(sol["solved"], true);
    let notes = sol["notes"].as_array().unwrap();
    assert!(
        notes.iter().any(|n| n.as_str().unwrap().contains("orbit")),
        "{notes:?}"
    );
    c.finish();
}

#[test]
fn cursor_advance_refuses_unobserved_execution_beyond_the_plan() {
    let mut c = Client::start();
    let sid = "cursor-boundary";
    open(&mut c, sid);
    let think = c.call_json(
        "session_think",
        json!({"session_id": sid, "max_evaluated": 10_000}),
    );
    let length = reported_plan_length(&think);
    assert_eq!(length, 2);

    let (text, error) = c.call_text(
        "session_advance",
        json!({"session_id": sid, "completed_steps": length + 1}),
    );
    assert!(error);
    assert!(text.contains("beyond plan length"));

    let status = c.call_json("session_status", json!({"session_id": sid}));
    assert_eq!(
        status["cursor"], 0,
        "refused advance must not mutate cursor"
    );
    c.finish();
}
