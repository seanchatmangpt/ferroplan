//! A dead-drop from the Solver page. Before it routes here, `ferroplan-wasm/web/`
//! stashes the domain, the problem, and a plan ALREADY SOLVED into
//! `localStorage['ferroplan.handoff']` — so a click on "Animate" plays back the
//! exact run instead of falling to the embedded demo, or worse, re-solving and
//! risking a mismatch if search options ever drift apart.
//!
//! wasm32-only. `main.rs` calls [`try_load`] at startup and falls back to the
//! embedded demo the moment it comes back `false` — no drop found, or it wouldn't parse.

use bevy::prelude::*;

use crate::anim::{load_result, result_from_solution, Plan};
use crate::scene::Scene;

const KEY: &str = "ferroplan.handoff";

/// Pull the drop, crack it open, wire it in — if it's there at all. Returns `true`
/// on a clean hit, scene and plan both loaded, telling the caller to skip the demo.
pub(crate) fn try_load(scene: &mut Scene, plan: &mut Plan) -> bool {
    let Some(raw) = read_local_storage(KEY) else {
        return false;
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) else {
        web_sys::console::warn_1(&"ferroplan.handoff: invalid JSON, ignoring".into());
        return false;
    };
    let (Some(domain), Some(problem)) = (
        v.get("domain").and_then(|x| x.as_str()),
        v.get("problem").and_then(|x| x.as_str()),
    ) else {
        web_sys::console::warn_1(&"ferroplan.handoff: missing domain/problem, ignoring".into());
        return false;
    };
    scene.load_src(domain);
    scene.load_src(problem);

    // The solved plan is optional in principle (a future caller might hand off just
    // a domain+problem to load and let the user press S) — apply it if present and
    // parses as a Solution; a plan-less handoff still counts as a successful load
    // since the scene came through.
    if let Some(sol_v) = v.get("solution") {
        match serde_json::from_value::<ferroplan::Solution>(sol_v.clone()) {
            Ok(sol) => {
                let res = result_from_solution(domain, problem, sol);
                load_result(plan, res, true); // autoplay: they clicked "Animate"
            }
            Err(e) => web_sys::console::warn_1(
                &format!("ferroplan.handoff: solution didn't parse ({e}), scene loaded anyway")
                    .into(),
            ),
        }
    }
    true
}

fn read_local_storage(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()??
        .get_item(key)
        .ok()?
}
