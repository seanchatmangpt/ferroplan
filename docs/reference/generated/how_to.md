# How to: Using ferroplan

## Prerequisites


- crates/ferroplan-bevy/src/anim.rs::Plan (struct)

- crates/ferroplan-bevy/src/anim.rs::SolveJob (struct)

- crates/ferroplan-bevy/src/anim.rs::advance (function)

- crates/ferroplan-bevy/src/anim.rs::animate (function)

- crates/ferroplan-bevy/src/anim.rs::controls (function)

- crates/ferroplan-bevy/src/anim.rs::frac (function)

- crates/ferroplan-bevy/src/anim.rs::poll_solve (function)

- crates/ferroplan-bevy/src/anim.rs::span (function)

- crates/ferroplan-bevy/src/anim.rs::start_frac (function)

- crates/ferroplan-bevy/src/blocks.rs::Act (enum)

- crates/ferroplan-bevy/src/blocks.rs::Drag (struct)

- crates/ferroplan-bevy/src/blocks.rs::DragKind (enum)

- crates/ferroplan-bevy/src/blocks.rs::Editor (struct)

- crates/ferroplan-bevy/src/blocks.rs::EditorRoot (struct)

- crates/ferroplan-bevy/src/blocks.rs::Focus (enum)

- crates/ferroplan-bevy/src/blocks.rs::Ghost (struct)

- crates/ferroplan-bevy/src/blocks.rs::LitLoc (enum)

- crates/ferroplan-bevy/src/blocks.rs::Mode (enum)

- crates/ferroplan-bevy/src/blocks.rs::Zone (enum)

- crates/ferroplan-bevy/src/blocks.rs::editor_drag (function)

- crates/ferroplan-bevy/src/blocks.rs::handle_clicks (function)

- crates/ferroplan-bevy/src/blocks.rs::rebuild (function)

- crates/ferroplan-bevy/src/blocks.rs::scroll_editor (function)

- crates/ferroplan-bevy/src/blocks.rs::text_input (function)

- crates/ferroplan-bevy/src/blocks.rs::toggle_editor (function)

- crates/ferroplan-bevy/src/gantt.rs::GanttBar (struct)

- crates/ferroplan-bevy/src/gantt.rs::GanttNow (struct)

- crates/ferroplan-bevy/src/gantt.rs::GanttPanel (struct)

- crates/ferroplan-bevy/src/gantt.rs::GanttState (struct)

- crates/ferroplan-bevy/src/gantt.rs::GanttTrack (struct)

- crates/ferroplan-bevy/src/gantt.rs::gantt_now (function)

- crates/ferroplan-bevy/src/gantt.rs::gantt_visibility (function)

- crates/ferroplan-bevy/src/gantt.rs::rebuild_gantt (function)

- crates/ferroplan-bevy/src/gantt.rs::setup_gantt (function)

- crates/ferroplan-bevy/src/gantt.rs::toggle_gantt (function)

- crates/ferroplan-bevy/src/icons.rs::IconShape (enum)

- crates/ferroplan-bevy/src/icons.rs::color_for (function)

- crates/ferroplan-bevy/src/icons.rs::mat_handle (function)

- crates/ferroplan-bevy/src/icons.rs::mesh_handle (function)

- crates/ferroplan-bevy/src/icons.rs::shape_for (function)


## Steps


1. Use `advance` from `crates/ferroplan-bevy/src/anim.rs`.

2. Use `animate` from `crates/ferroplan-bevy/src/anim.rs`.

3. Use `controls` from `crates/ferroplan-bevy/src/anim.rs`.

4. Use `frac` from `crates/ferroplan-bevy/src/anim.rs`.

5. Use `poll_solve` from `crates/ferroplan-bevy/src/anim.rs`.

6. Use `span` from `crates/ferroplan-bevy/src/anim.rs`.

7. Use `start_frac` from `crates/ferroplan-bevy/src/anim.rs`.

8. Use `Plan` from `crates/ferroplan-bevy/src/anim.rs`.

9. Use `SolveJob` from `crates/ferroplan-bevy/src/anim.rs`.

10. Use `Act` from `crates/ferroplan-bevy/src/blocks.rs`.

11. Use `DragKind` from `crates/ferroplan-bevy/src/blocks.rs`.

12. Use `Focus` from `crates/ferroplan-bevy/src/blocks.rs`.


## Verified snippet

<!-- The snippet slot carries code copied from the extracted code surface -->
<!-- (doc:Claim rows whose doc:attribute is "snippet"), never agent prose. -->

```rust
// crates/ferroplan-bevy/src/anim.rs :: advance
advance(time: Res<Time>, mut plan: ResMut<Plan>)
```

<!-- AGENT-COMMENTARY-BEGIN -->
<!-- The ONLY region an agent may write into. Bounds: <= 12 lines,    -->
<!-- <= 100 chars/line, no new code facts (any new symbol mentioned   -->
<!-- must exist in queries/ast_extract.rq output; the doc_quality     -->
<!-- court fails Phi_halluc > 0.001 otherwise). No tables, no         -->
<!-- signatures, no parameters, no error lists — AGENT-FORBIDDEN      -->
<!-- everywhere.                                                      -->
<!-- AGENT-COMMENTARY-END -->
