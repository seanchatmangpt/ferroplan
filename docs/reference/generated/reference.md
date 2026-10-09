# ferroplan reference

<!-- ============================================================= -->
<!-- AGENT-FORBIDDEN-BEGIN: reference body is RIGID                -->
<!-- Every row below is rendered from queries/ast_extract.rq.      -->
<!-- Agents MUST NOT add, edit, reorder, or remove any row or      -->
<!-- table cell. Prose outside the fenced slot below is refused    -->
<!-- by the doc_quality court.                                     -->
<!-- ============================================================= -->

## Modules


### crates/ferroplan-bevy/src/anim.rs

| `advance` | function | advance(time: Res<Time>, mut plan: ResMut<Plan>) |  |  |  |  |

| `animate` | function | animate( plan: Res<Plan>, scene: Res<Scene>, nodes: Query<(&NodeObj, &Transform)>, mut mobiles: Query<(&MobileObj, &FanOffset, &mut Transform), Without<NodeObj>>, ) |  |  |  |  |

| `controls` | function | controls( keys: Res<ButtonInput<KeyCode>>, scene: Res<Scene>, editor: Res<crate::blocks::Editor>, mut plan: ResMut<Plan>, mut job: ResMut<SolveJob>, ) |  |  |  |  |

| `frac` | function | frac(&self) -> f32 |  |  |  |  |

| `poll_solve` | function | poll_solve(mut job: ResMut<SolveJob>, mut plan: ResMut<Plan>) |  |  |  |  |

| `span` | function | span(&self) -> f32 |  |  |  |  |

| `start_frac` | function | start_frac(&self, step: &Step, idx: usize) -> f32 |  |  |  |  |

| `Plan` | struct | Plan { pub steps: Vec<Step>, pub snapshots: Vec<StateSnapshot>, pub t: f32, pub playing: bool, pub status: String, pub temporal: bool, pub makespan: f32 } |  |  |  |  |

| `SolveJob` | struct | SolveJob { Option<Task<SolveResult>> } |  |  |  |  |


### crates/ferroplan-bevy/src/blocks.rs

| `Act` | enum | Act { AddObject, RemoveObject(usize), CycleType(usize), AddFact(bool), RemoveFact(bool, usize), CyclePred(bool, usize), CycleArg(bool, usize, usize), AddType, RemoveType(usize), CycleSuper(usize), AddPred, RemovePred(usize), AddArg(usize), RemoveArg(usize), CycleArgType(usize, usize), AddAction, RemoveAction(usize), AddParam(usize), RemoveParam(usize), CycleParamType(usize, usize), TogglePreKind(usize), AddWhen(usize), RemoveWhen(usize, usize), AddLit(usize, LitLoc), RemoveLit(usize, LitLoc, usize), CycleLitPred(usize, LitLoc, usize), CycleLitArg(usize, LitLoc, usize, usize), ToggleNeg(usize, LitLoc, usize), SetFocus(Focus), ToggleMode, Apply, Export, Close } |  |  |  |  |

| `DragKind` | enum | DragKind { Init(usize), Goal(usize) } |  |  |  |  |

| `Focus` | enum | Focus { DomainName, TypeName(usize), PredName(usize), ActionName(usize) } |  |  |  |  |

| `LitLoc` | enum | LitLoc { Pre, Eff, WhenCond(usize), WhenEff(usize) } |  |  |  |  |

| `Mode` | enum | Mode { Problem, Domain } |  |  |  |  |

| `Zone` | enum | Zone { Init, Goal } |  |  |  |  |

| `editor_drag` | function | editor_drag( mouse: Res<ButtonInput<MouseButton>>, windows: Query<&Window>, mut drag: ResMut<Drag>, mut editor: ResMut<Editor>, grips: Query<(&DragKind, &RelativeCursorPosition)>, zones: Query<(&Zone, &RelativeCursorPosition)>, mut ghosts: Query<&mut Node, With<Ghost>>, mut commands: Commands, ) |  |  |  |  |

| `handle_clicks` | function | handle_clicks( interactions: Query<(&Interaction, &Act), (Changed<Interaction>, With<Button>)>, mut editor: ResMut<Editor>, mut scene: ResMut<Scene>, ) |  |  |  |  |

| `rebuild` | function | rebuild( mut commands: Commands, mut editor: ResMut<Editor>, roots: Query<Entity, With<EditorRoot>>, ) |  |  |  |  |

| `scroll_editor` | function | scroll_editor( mut wheel: MessageReader<MouseWheel>, keys: Res<ButtonInput<KeyCode>>, editor: Res<Editor>, mut q: Query<&mut ScrollPosition, With<EditorRoot>>, ) |  |  |  |  |

| `text_input` | function | text_input(mut evr: MessageReader<KeyboardInput>, mut editor: ResMut<Editor>) |  |  |  |  |

| `toggle_editor` | function | toggle_editor( keys: Res<ButtonInput<KeyCode>>, scene: Res<Scene>, mut editor: ResMut<Editor>, ) |  |  |  |  |

| `../demo/domain.pddl` | str_key | DOMAIN = "../demo/domain.pddl" |  |  |  |  |

| `../demo/problem.pddl` | str_key | PROBLEM = "../demo/problem.pddl" |  |  |  |  |

| `Drag` | struct | Drag { held: Option<DragKind>, ghost: Option<Entity> } |  |  |  |  |

| `Editor` | struct | Editor { pub open: bool, pub focus: Option<Focus>, mode: Mode, dirty: bool, status: String, problem_name: String, objects: Vec<(String, String)>, init: Vec<(String, Vec<String>)>, goal: Vec<(String, Vec<String>)>, counters: HashMap<String, u32>, seeded: bool, dname: String, requirements: String, types: Vec<(String, String)>, dpreds: Vec<(String, Vec<String>)>, actions: Vec<EdAction>, dseeded: bool } |  |  |  |  |

| `EditorRoot` | struct |  |  |  |  |  |

| `Ghost` | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/gantt.rs

| `gantt_now` | function | gantt_now(plan: Res<Plan>, mut now: Query<&mut Node, With<GanttNow>>) |  |  |  |  |

| `gantt_visibility` | function | gantt_visibility( plan: Res<Plan>, state: Res<GanttState>, editor: Res<crate::blocks::Editor>, mut panel: Query<&mut Visibility, With<GanttPanel>>, ) |  |  |  |  |

| `rebuild_gantt` | function | rebuild_gantt( mut commands: Commands, plan: Res<Plan>, mut state: ResMut<GanttState>, track: Query<Entity, With<GanttTrack>>, bars: Query<Entity, With<GanttBar>>, ) |  |  |  |  |

| `setup_gantt` | function | setup_gantt(mut commands: Commands) |  |  |  |  |

| `toggle_gantt` | function | toggle_gantt( keys: Res<ButtonInput<KeyCode>>, editor: Res<crate::blocks::Editor>, mut state: ResMut<GanttState>, ) |  |  |  |  |

| `GanttBar` | struct |  |  |  |  |  |

| `GanttNow` | struct |  |  |  |  |  |

| `GanttPanel` | struct |  |  |  |  |  |

| `GanttState` | struct | GanttState { pub open: bool, built_for: usize, built_span: f32 } |  |  |  |  |

| `GanttTrack` | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/icons.rs

| `IconShape` | enum | IconShape { Circle, Truck, Box, Person, Robot, Machine, Diamond } |  |  |  |  |

| `color_for` | function | color_for(ty: &str) -> Color |  |  |  |  |

| `mat_handle` | function | mat_handle( materials: &mut Assets<ColorMaterial>, cache: &mut MatCache, color: Color, ) -> Handle<ColorMaterial> |  |  |  |  |

| `mesh_handle` | function | mesh_handle( meshes: &mut Assets<Mesh>, cache: &mut MeshCache, shape: IconShape, size: f32, ) -> Handle<Mesh> |  |  |  |  |

| `shape_for` | function | shape_for(ty: &str) -> IconShape |  |  |  |  |


### crates/ferroplan-bevy/src/interact.rs

| `draw_selection` | function | draw_selection( mut gizmos: Gizmos, selected: Res<Selected>, nodes: Query<(&NodeObj, &Transform)>, mobiles: Query<(&MobileObj, &Transform), Without<NodeObj>>, ) |  |  |  |  |

| `interact` | function | interact( mouse: Res<ButtonInput<MouseButton>>, windows: Query<&Window>, editor: Res<crate::blocks::Editor>, cam_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>, mut nodes: Query<(Entity, &NodeObj, &mut Transform)>, mobiles: Query<(&MobileObj, &Transform), Without<NodeObj>>, transport: Res<crate::transport::Transport>, mut selected: ResMut<Selected>, mut drag: ResMut<DragState>, ) |  |  |  |  |

| `DragState` | struct | DragState { node: Option<Entity> } |  |  |  |  |

| `Selected` | struct | Selected { pub Option<String> } |  |  |  |  |


### crates/ferroplan-bevy/src/palette.rs

| `ACC` | const | ACC: Color |  |  |  |  |

| `BG` | const | BG: Color |  |  |  |  |

| `BG2` | const | BG2: Color |  |  |  |  |

| `CRATE_AMBER` | const | CRATE_AMBER: Color |  |  |  |  |

| `CY` | const | CY: Color |  |  |  |  |

| `EDGE` | const | EDGE: Color |  |  |  |  |

| `EDGE2` | const | EDGE2: Color |  |  |  |  |

| `FAINT` | const | FAINT: Color |  |  |  |  |

| `GREY_NODE` | const | GREY_NODE: Color |  |  |  |  |

| `INK` | const | INK: Color |  |  |  |  |

| `MUT` | const | MUT: Color |  |  |  |  |

| `NODE_PURPLE` | const | NODE_PURPLE: Color |  |  |  |  |

| `PANEL` | const | PANEL: Color |  |  |  |  |

| `PANEL2` | const | PANEL2: Color |  |  |  |  |

| `PANEL_BLUR` | const | PANEL_BLUR: Color |  |  |  |  |

| `RIG_GREEN` | const | RIG_GREEN: Color |  |  |  |  |

| `ZONE` | const | ZONE: Color |  |  |  |  |


### crates/ferroplan-bevy/src/scene.rs

| `MOBILE_SIZE` | const | MOBILE_SIZE: f32 |  |  |  |  |

| `NODE_SIZE` | const | NODE_SIZE: f32 |  |  |  |  |

| `camera_nav` | function | camera_nav( mouse: Res<ButtonInput<MouseButton>>, editor: Res<crate::blocks::Editor>, mut motion: MessageReader<bevy::input::mouse::MouseMotion>, mut wheel: MessageReader<MouseWheel>, mut cam: Query<(&mut Transform, &mut Projection), With<MainCamera>>, ) |  |  |  |  |

| `draw_edges` | function | draw_edges( mut gizmos: Gizmos, scene: Res<Scene>, plan: Res<crate::anim::Plan>, nodes: Query<(&NodeObj, &Transform)>, ) |  |  |  |  |

| `handle_drops` | function | handle_drops(mut drops: MessageReader<FileDragAndDrop>, mut scene: ResMut<Scene>) |  |  |  |  |

| `load_src` | function | load_src(&mut self, src: &str) |  |  |  |  |

| `respawn_graph` | function | respawn_graph( mut commands: Commands, mut scene: ResMut<Scene>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, existing: Query<Entity, With<GraphItem>>, ) |  |  |  |  |

| `setup` | function | setup(mut commands: Commands) |  |  |  |  |

| `FanOffset` | struct | FanOffset { pub Vec2 } |  |  |  |  |

| `GraphItem` | struct |  |  |  |  |  |

| `MainCamera` | struct |  |  |  |  |  |

| `MobileObj` | struct | MobileObj { pub String } |  |  |  |  |

| `NodeObj` | struct | NodeObj { pub String } |  |  |  |  |

| `Scene` | struct | Scene { pub domain: Option<Domain>, pub problem: Option<Problem>, pub domain_src: String, pub problem_src: String, pub graph: VizGraph, pub dirty: bool, pub status: String } |  |  |  |  |


### crates/ferroplan-bevy/src/transport.rs

| `rebuild_notches` | function | rebuild_notches( mut commands: Commands, plan: Res<Plan>, mut state: ResMut<Transport>, track: Query<Entity, With<ScrubTrack>>, notches: Query<Entity, With<StepNotch>>, ) |  |  |  |  |

| `setup_transport` | function | setup_transport(mut commands: Commands) |  |  |  |  |

| `transport_input` | function | transport_input( mouse: Res<ButtonInput<MouseButton>>, mut transport: ResMut<Transport>, mut plan: ResMut<Plan>, play_btn: Query<&Interaction, (With<PlayButton>, Changed<Interaction>)>, track: Query<(&Interaction, &RelativeCursorPosition), With<ScrubTrack>>, ) |  |  |  |  |

| `transport_sync` | function | transport_sync( plan: Res<Plan>, mut fill: Query<&mut Node, (With<ScrubFill>, Without<Playhead>)>, mut head: Query<&mut Node, (With<Playhead>, Without<ScrubFill>)>, mut icon: Query<&mut Text, (With<PlayIcon>, Without<TransportLabel>)>, mut label: Query<&mut Text, (With<TransportLabel>, Without<PlayIcon>)>, ) |  |  |  |  |

| `transport_visibility` | function | transport_visibility(plan: Res<Plan>, mut bar: Query<&mut Visibility, With<TransportBar>>) |  |  |  |  |

| `PlayButton` | struct |  |  |  |  |  |

| `PlayIcon` | struct |  |  |  |  |  |

| `Playhead` | struct |  |  |  |  |  |

| `ScrubFill` | struct |  |  |  |  |  |

| `ScrubTrack` | struct |  |  |  |  |  |

| `StepNotch` | struct |  |  |  |  |  |

| `Transport` | struct | Transport { pub hovering: bool, built_for: usize } |  |  |  |  |

| `TransportBar` | struct |  |  |  |  |  |

| `TransportLabel` | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/ui.rs

| `setup_ui` | function | setup_ui(mut commands: Commands) |  |  |  |  |

| `update_info` | function | update_info( scene: Res<Scene>, selected: Res<Selected>, plan: Res<Plan>, mut q: Query<&mut Text, With<InfoText>>, ) |  |  |  |  |

| `InfoText` | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/webhandoff.rs

| `ferroplan.handoff` | str_key | KEY = "ferroplan.handoff" |  |  |  |  |


### crates/ferroplan-cli/src/generated/options.rs

| `FF_THREADS` | const | FF_THREADS: usize |  |  |  |  |

| `FF_WEIGHT_G` | const | FF_WEIGHT_G: f64 |  |  |  |  |

| `FF_WEIGHT_H` | const | FF_WEIGHT_H: f64 |  |  |  |  |

| `PPDDL_DISCOUNT` | const | PPDDL_DISCOUNT: f64 |  |  |  |  |

| `PPDDL_EPISODES` | const | PPDDL_EPISODES: usize |  |  |  |  |

| `PPDDL_EPSILON` | const | PPDDL_EPSILON: f64 |  |  |  |  |

| `PPDDL_HORIZON` | const | PPDDL_HORIZON: usize |  |  |  |  |

| `PPDDL_MAX_INITIAL_OUTCOMES` | const | PPDDL_MAX_INITIAL_OUTCOMES: usize |  |  |  |  |

| `PPDDL_MAX_ITERATIONS` | const | PPDDL_MAX_ITERATIONS: usize |  |  |  |  |

| `PPDDL_MAX_OUTCOMES_PER_ACTION` | const | PPDDL_MAX_OUTCOMES_PER_ACTION: usize |  |  |  |  |

| `PPDDL_MAX_POLICY_ENTRIES` | const | PPDDL_MAX_POLICY_ENTRIES: usize |  |  |  |  |

| `PPDDL_MAX_STATES` | const | PPDDL_MAX_STATES: usize |  |  |  |  |

| `PPDDL_MAX_TRANSITIONS` | const | PPDDL_MAX_TRANSITIONS: usize |  |  |  |  |

| `PPDDL_MAX_VALUE_CELLS` | const | PPDDL_MAX_VALUE_CELLS: usize |  |  |  |  |

| `PPDDL_SEED` | const | PPDDL_SEED: u64 |  |  |  |  |

| `PPDDL_SIMULATION_MAX_STEPS` | const | PPDDL_SIMULATION_MAX_STEPS: usize |  |  |  |  |

| `PPDDL_THREADS` | const | PPDDL_THREADS: usize |  |  |  |  |

| `ModeArg` | enum | ModeArg { Auto, Ff, Partition, Pddl3, Temporal, Portfolio, Optimal, Sat } |  |  |  |  |

| `ObjectiveArg` | enum | ObjectiveArg { Auto, MaximizeGoalProbability, MinimizeGoalProbability, MaximizeExpectedReward, MinimizeExpectedReward, MaximizeExpectedMetric, MinimizeExpectedMetric } |  |  |  |  |

| `SearchArg` | enum | SearchArg { Auto, Ehc, BestFirst, EhcThenBestFirst } |  |  |  |  |


### crates/ferroplan-cli/src/harvest/compile.rs

| `compile_pack` | function | compile_pack(pack: &ObservationPack, output_dir: &Path) -> Result<HarvestReceipt> |  |  |  |  |

| `replay_pack` | function | replay_pack( pack: &ObservationPack, expected: &HarvestReceipt, output_dir: &Path, ) -> Result<HarvestReceipt> |  |  |  |  |


### crates/ferroplan-cli/src/harvest/extract.rs

| `extract_operators` | function | extract_operators(report: &AdmissionReport) -> Vec<PlanningOperator> |  |  |  |  |


### crates/ferroplan-cli/src/harvest/gh.rs

| `collect_with_gh` | function | collect_with_gh( repositories: &[String], window: ObservationWindow, max_pages: usize, ) -> Result<ObservationPack> |  |  |  |  |


### crates/ferroplan-cli/src/harvest/mod.rs

| `admit` | function | admit(pack: &ObservationPack) -> AdmissionReport |  |  |  |  |

| `digest_bytes` | function | digest_bytes(bytes: &[u8]) -> String |  |  |  |  |

| `load_observation_pack` | function | load_observation_pack(path: &Path) -> Result<ObservationPack> |  |  |  |  |

| `load_receipt` | function | load_receipt(path: &Path) -> Result<HarvestReceipt> |  |  |  |  |

| `receipt_exit_code` | function | receipt_exit_code(receipt: &HarvestReceipt) -> i32 |  |  |  |  |

| `save_observation_pack` | function | save_observation_pack(path: &Path, pack: &ObservationPack) -> Result<()> |  |  |  |  |

| `validate_pack` | function | validate_pack(pack: &ObservationPack) -> Result<()> |  |  |  |  |

| `validate_window` | function | validate_window(window: &ObservationWindow) -> Result<()> |  |  |  |  |

| `compile::{compile_pack, replay_pack}` | use | compile::{compile_pack, replay_pack} |  |  |  |  |

| `extract::extract_operators` | use | extract::extract_operators |  |  |  |  |

| `gh::collect_with_gh` | use | gh::collect_with_gh |  |  |  |  |

| model::* (glob re-export; see prose note) | use | model::* |  |  |  |  |


### crates/ferroplan-cli/src/harvest/model.rs

| `ADMISSION_SCHEMA` | const | ADMISSION_SCHEMA: &str |  |  |  |  |

| `CATALOG_SCHEMA` | const | CATALOG_SCHEMA: &str |  |  |  |  |

| `OBSERVATION_SCHEMA` | const | OBSERVATION_SCHEMA: &str |  |  |  |  |

| `RECEIPT_SCHEMA` | const | RECEIPT_SCHEMA: &str |  |  |  |  |

| `ActuationClass` | enum | ActuationClass { Select, Construct, Do, HookIntent } |  |  |  |  |

| `AdmissionLevel` | enum | AdmissionLevel { Observed, IdentityResolved, ExecutionObserved, ResultCorroborated, ReceiptVerified, ReplayVerified } |  |  |  |  |

| `ExecutionResult` | enum | ExecutionResult { Pass, Fail, Cancelled, Pending, Unknown } |  |  |  |  |

| `FinalState` | enum | FinalState { PartialAlive, Alive, Blocked, BuildBroken, Unknown, Unsupported } |  |  |  |  |

| `GallCheckpoint` | enum | GallCheckpoint { G0Orient, G1Fence, G2Observe, G3Admit, G4Plan, G5Manufacture, G6Verify, G7Replay, G8ReleaseAdmission, G9SunsetAdmission } |  |  |  |  |

| `RefusalCode` | enum | RefusalCode { MissingExactSourceIdentity, OutsideObservationWindow, MissingChangedPaths, ExecutionNotObserved, WorkflowRunNotBoundToHead, ProbabilityEvidenceMissing, InvalidProbability, ProbabilityMassExceeded, OperatorBoundExceeded } |  |  |  |  |

| `ReplayState` | enum | ReplayState { NotExecuted, ReplayMatch, ReplayMismatch } |  |  |  |  |

| `ferroplan-harvest-admission/v1` | str_key | ADMISSION_SCHEMA = "ferroplan-harvest-admission/v1" |  |  |  |  |

| `ferroplan-harvest-receipt/v1` | str_key | RECEIPT_SCHEMA = "ferroplan-harvest-receipt/v1" |  |  |  |  |

| `ferroplan-method-catalog/v1` | str_key | CATALOG_SCHEMA = "ferroplan-method-catalog/v1" |  |  |  |  |

| `ferroplan-observation-pack/v1` | str_key | OBSERVATION_SCHEMA = "ferroplan-observation-pack/v1" |  |  |  |  |

| `AdmissionReport` | struct | AdmissionReport { pub schema: String, pub admitted: Vec<AdmittedWork>, pub excluded: Vec<ExcludedWork>, pub unresolved_transport_failures: Vec<TransportFailure> } |  |  |  |  |

| `AdmittedWork` | struct | AdmittedWork { pub identity: String, pub level: AdmissionLevel, pub work: ObservedWorkItem, pub evidence: Vec<EvidenceRef> } |  |  |  |  |

| `ArtifactEvidence` | struct | ArtifactEvidence { pub name: String, pub source_sha: String, pub evidence_url: String, pub digest: Option<String>, pub size_bytes: Option<u64> } |  |  |  |  |

| `EvidenceRef` | struct | EvidenceRef { pub kind: String, pub identity: String, pub location: String } |  |  |  |  |

| `ExcludedWork` | struct | ExcludedWork { pub identity: String, pub code: RefusalCode, pub detail: String } |  |  |  |  |

| `ExecutionEvidence` | struct | ExecutionEvidence { pub surface: String, pub command: String, pub source_sha: String, pub result: ExecutionResult, pub exit_code: Option<i32>, pub observed_at_utc: String, pub evidence_url: String } |  |  |  |  |

| `HarvestReceipt` | struct | HarvestReceipt { pub schema: String, pub run_id: String, pub receipt_digest: String, pub source_pack_digest: String, pub catalog_digest: String, pub source_revisions: Vec<SourceRevision>, pub source_work: Vec<String>, pub admitted_work: Vec<String>, pub excluded_work: Vec<ExcludedWork>, pub operators_added: Vec<String>, pub operators_deduplicated: usize, pub probabilistic_operators: usize, pub outputs: Vec<OutputArtifact>, pub validation: ValidationSummary, pub replay: ReplayState, pub generated_outputs_hand_edited: bool, pub transport_failures: Vec<TransportFailure>, pub failures: Vec<String>, pub exclusions: Vec<String>, pub final_state: FinalState } |  |  |  |  |

| `MethodCatalog` | struct | MethodCatalog { pub schema: String, pub run_id: String, pub source_pack_digest: String, pub raw_operator_count: usize, pub operator_count: usize, pub operators: Vec<PlanningOperator> } |  |  |  |  |

| `ObservationPack` | struct | ObservationPack { pub schema: String, pub run_id: String, pub window: ObservationWindow, pub repositories: Vec<String>, pub work_items: Vec<ObservedWorkItem>, pub transport_failures: Vec<TransportFailure> } |  |  |  |  |

| `ObservationWindow` | struct | ObservationWindow { pub start_utc: String, pub end_exclusive_utc: String, pub timezone: String } |  |  |  |  |

| `ObservedOutcome` | struct | ObservedOutcome { pub label: String, pub probability: f64, pub success: bool, pub evidence: Vec<EvidenceRef> } |  |  |  |  |

| `ObservedWorkItem` | struct | ObservedWorkItem { pub repository: String, pub sha: String, pub parent_sha: Option<String>, pub message: String, pub committed_at_utc: String, pub source_url: String, pub changed_paths: Vec<String>, pub executions: Vec<ExecutionEvidence>, pub artifacts: Vec<ArtifactEvidence>, pub probabilistic_outcomes: Vec<ObservedOutcome> } |  |  |  |  |

| `OperatorOutcome` | struct | OperatorOutcome { pub label: String, pub probability: f64, pub success: bool, pub evidence: Vec<EvidenceRef> } |  |  |  |  |

| `OutputArtifact` | struct | OutputArtifact { pub path: String, pub bytes: usize, pub blake3: String } |  |  |  |  |

| `PlanningOperator` | struct | PlanningOperator { pub id: String, pub name: String, pub signature: String, pub checkpoint: GallCheckpoint, pub actuation_class: ActuationClass, pub preconditions: Vec<String>, pub effects: Vec<String>, pub invariants: Vec<String>, pub failures: Vec<String>, pub refusals: Vec<String>, pub receipt_hook: bool, pub replay_hook: bool, pub probabilistic_outcomes: Vec<OperatorOutcome>, pub evidence: Vec<EvidenceRef>, pub source_work: Vec<String> } |  |  |  |  |

| `SourceRevision` | struct | SourceRevision { pub repository: String, pub base_sha: Option<String>, pub head_sha: String } |  |  |  |  |

| `TransportFailure` | struct | TransportFailure { pub repository: String, pub operation: String, pub state: String, pub detail: String } |  |  |  |  |

| `ValidationRecord` | struct | ValidationRecord { pub command: String, pub result: String, pub detail: Option<String> } |  |  |  |  |

| `ValidationSummary` | struct | ValidationSummary { pub parse_ok: bool, pub parse_error: Option<String>, pub solve_attempted: bool, pub solved: Option<bool>, pub initial_value: Option<f64>, pub policy_valid: Option<bool>, pub policy_errors: Vec<String>, pub records: Vec<ValidationRecord> } |  |  |  |  |


### crates/ferroplan-hddl/examples/fixture_f_stats.rs

| `FIXTURE_F_WALL_SECS` | env_key | std::env::var("FIXTURE_F_WALL_SECS") |  |  |  |  |


### crates/ferroplan-hddl/examples/validate_files.rs

| `validate_pair` | function | validate_pair(domain_src: &str, problem_src: &str) -> Result<Summary, String> |  |  |  |  |

| `validate_paths` | function | validate_paths(domain_path: &str, problem_path: &str) -> Result<Summary, String> |  |  |  |  |

| `Summary` | struct | Summary { pub domain: String, pub problem: String, pub actions: usize, pub tasks: usize, pub methods: usize, pub ground_methods: usize } |  |  |  |  |


### crates/ferroplan-hddl/src/ast.rs

| `Effect` | enum | Effect { Empty, Literal(Literal), And(Vec<Effect>), When(GoalDesc, Box<Effect>), Oneof(Vec<Effect>), Increase(AtomicFormula, NumericValue), Decrease(AtomicFormula, NumericValue) } |  |  |  |  |

| `GoalDesc` | enum | GoalDesc { Empty, Atom(AtomicFormula), Not(Box<GoalDesc>), And(Vec<GoalDesc>), Or(Vec<GoalDesc>), Imply(Box<GoalDesc>, Box<GoalDesc>), Forall(Vec<TypedParam>, Box<GoalDesc>), Exists(Vec<TypedParam>, Box<GoalDesc>) } |  |  |  |  |

| `Literal` | enum | Literal { Pos(AtomicFormula), Neg(AtomicFormula) } |  |  |  |  |

| `NumericValue` | enum | NumericValue { Number(String), Fluent(AtomicFormula) } |  |  |  |  |

| `Term` | enum | Term { Var(VarName), Const(Name) } |  |  |  |  |

| `ActionDef` | struct | ActionDef { pub name: Name, pub params: Vec<TypedParam>, pub precondition: GoalDesc, pub effect: Effect, pub probability_weights: Option<Vec<String>> } |  |  |  |  |

| `AtomicFormula` | struct | AtomicFormula { pub predicate: Name, pub args: Vec<Term> } |  |  |  |  |

| `ConstraintDef` | struct | ConstraintDef { pub kind: Name, pub raw: String } |  |  |  |  |

| `Domain` | struct | Domain { pub name: Name, pub types: TypeDef, pub constants: Vec<TypedObject>, pub predicates: Vec<PredicateDef>, pub numeric_fluents: Vec<NumericFluentDecl>, pub tasks: Vec<TaskDef>, pub actions: Vec<ActionDef>, pub methods: Vec<MethodDef>, pub constraints: Vec<ConstraintDef> } |  |  |  |  |

| `MethodDef` | struct | MethodDef { pub name: Name, pub params: Vec<TypedParam>, pub task: TaskCall, pub precondition: GoalDesc, pub effect: Effect, pub network: TaskNetwork } |  |  |  |  |

| `NumericFluentDecl` | struct | NumericFluentDecl { pub name: Name, pub params: Vec<TypedParam>, pub value: NumericValue } |  |  |  |  |

| `OrderEdge` | struct | OrderEdge { pub before: Name, pub after: Name } |  |  |  |  |

| `PredicateDef` | struct | PredicateDef { pub name: Name, pub params: Vec<TypedParam> } |  |  |  |  |

| `Problem` | struct | Problem { pub name: Name, pub domain_name: Name, pub objects: Vec<TypedObject>, pub init: Vec<AtomicFormula>, pub goal: GoalDesc, pub htn: TaskNetwork, pub constraints: Vec<ConstraintDef> } |  |  |  |  |

| `Subtask` | struct | Subtask { pub id: Name, pub task: TaskCall } |  |  |  |  |

| `TaskCall` | struct | TaskCall { pub name: Name, pub args: Vec<Term> } |  |  |  |  |

| `TaskDef` | struct | TaskDef { pub name: Name, pub params: Vec<TypedParam> } |  |  |  |  |

| `TaskNetwork` | struct | TaskNetwork { pub params: Vec<TypedParam>, pub subtasks: Vec<Subtask>, pub order: Vec<OrderEdge> } |  |  |  |  |

| `TypeDef` | struct | TypeDef { pub parents: BTreeMap<Name, BTreeSet<Name>>, pub declared: Vec<Name> } |  |  |  |  |

| `TypedObject` | struct | TypedObject { pub name: Name, pub type_name: Name } |  |  |  |  |

| `TypedParam` | struct | TypedParam { pub var: VarName, pub type_name: Name } |  |  |  |  |


### crates/ferroplan-hddl/src/grounder.rs

| `DEFAULT_MAX_GOAL_DEPTH` | const | DEFAULT_MAX_GOAL_DEPTH: usize |  |  |  |  |

| `GroundError` | enum | GroundError { Validation(validate::ValidationError), TypeCycle(String), UnboundVariable(String), UnsupportedPrecondition(String), LimitExceeded(String), UnsupportedNumericFluent(String), UnsupportedConstraint(String), GoalTooDeep { depth: usize, budget: usize }, Timeout { elapsed_ms: u128, limit_ms: u128 } } |  |  |  |  |

| `GroundGoal` | enum | GroundGoal { Empty, Atom(String), Eq(String, String), Not(Box<GroundGoal>), And(Vec<GroundGoal>), Or(Vec<GroundGoal>) } |  |  |  |  |

| `action_applicable` | function | action_applicable( action: &GroundAction, facts: &BTreeSet<String>, ) -> Result<bool, GroundError> |  |  |  |  |

| `build_type_closure` | function | build_type_closure( domain: &Domain, ) -> Result<BTreeMap<String, BTreeSet<String>>, GroundError> |  |  |  |  |

| `compute_reachability` | function | compute_reachability( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, initial_facts: &BTreeSet<String>, limits: &GroundingLimits, ) -> Result<ReachabilityInfo, GroundError> |  |  |  |  |

| `compute_task_relevance` | function | compute_task_relevance( domain: &Domain, problem: &Problem, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, ) -> Result<TaskRelevanceInfo, GroundError> |  |  |  |  |

| `evaluate_ground_goal` | function | evaluate_ground_goal( goal: &GroundGoal, facts: &BTreeSet<String>, ) -> Result<bool, GroundError> |  |  |  |  |

| `evaluate_ground_goal_with_budget` | function | evaluate_ground_goal_with_budget( goal: &GroundGoal, facts: &BTreeSet<String>, budget: usize, ) -> Result<bool, GroundError> |  |  |  |  |

| `ground` | function | ground( domain: &Domain, problem: &Problem, limits: &GroundingLimits, ) -> Result<GroundedIR, GroundError> |  |  |  |  |

| `ground_actions` | function | ground_actions( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, ) -> Result<Vec<GroundAction>, GroundError> |  |  |  |  |

| `ground_actions_reachable` | function | ground_actions_reachable( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, reachability: &ReachabilityInfo, ) -> Result<Vec<GroundAction>, GroundError> |  |  |  |  |

| `ground_actions_relevant` | function | ground_actions_relevant( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, relevance: &TaskRelevanceInfo, reachability: Option<&ReachabilityInfo>, ) -> Result<Vec<GroundAction>, GroundError> |  |  |  |  |

| `ground_initial_facts` | function | ground_initial_facts(problem: &Problem) -> Result<BTreeSet<String>, GroundError> |  |  |  |  |

| `ground_methods` | function | ground_methods( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, ) -> Result<Vec<GroundMethod>, GroundError> |  |  |  |  |

| `ground_methods_reachable` | function | ground_methods_reachable( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, reachability: &ReachabilityInfo, ) -> Result<Vec<GroundMethod>, GroundError> |  |  |  |  |

| `ground_methods_relevant` | function | ground_methods_relevant( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, relevance: &TaskRelevanceInfo, reachability: Option<&ReachabilityInfo>, ) -> Result<Vec<GroundMethod>, GroundError> |  |  |  |  |

| `ground_root_network` | function | ground_root_network( problem: &Problem, objects_by_type: &BTreeMap<String, Vec<String>>, ) -> Result<Vec<GroundRootNetwork>, GroundError> |  |  |  |  |

| `index_objects_by_type` | function | index_objects_by_type( domain: &Domain, problem: &Problem, closure: &BTreeMap<String, BTreeSet<String>>, ) -> BTreeMap<String, Vec<String>> |  |  |  |  |

| `../fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../fixtures/a/domain.hddl" |  |  |  |  |

| `../fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../fixtures/a/problem.hddl" |  |  |  |  |

| `../fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/c/problem.hddl` | str_key | FIXTURE_C_PROBLEM = "../fixtures/c/problem.hddl" |  |  |  |  |

| `../fixtures/d/domain.hddl` | str_key | FIXTURE_D_DOMAIN = "../fixtures/d/domain.hddl" |  |  |  |  |

| `../fixtures/d/problem.hddl` | str_key | FIXTURE_D_PROBLEM = "../fixtures/d/problem.hddl" |  |  |  |  |

| `../fixtures/e/domain.hddl` | str_key | FIXTURE_E_DOMAIN = "../fixtures/e/domain.hddl" |  |  |  |  |

| `../fixtures/e/problem.hddl` | str_key | FIXTURE_E_PROBLEM = "../fixtures/e/problem.hddl" |  |  |  |  |

| `GroundAction` | struct | GroundAction { pub name: String, pub precondition: GroundGoal, pub outcomes: Vec<GroundEffectBranch> } |  |  |  |  |

| `GroundConditional` | struct | GroundConditional { pub pos_cond: BTreeSet<String>, pub neg_cond: BTreeSet<String>, pub add: BTreeSet<String>, pub del: BTreeSet<String> } |  |  |  |  |

| `GroundEffectBranch` | struct | GroundEffectBranch { pub add: BTreeSet<String>, pub del: BTreeSet<String>, pub conditional: Vec<GroundConditional>, pub probability_weight: Option<String> } |  |  |  |  |

| `GroundMethod` | struct | GroundMethod { pub name: String, pub task_name: String, pub precondition: GroundGoal, pub effect: GroundEffectBranch, pub subtasks: Vec<GroundSubtask>, pub order: Vec<(String, String)> } |  |  |  |  |

| `GroundRootNetwork` | struct | GroundRootNetwork { pub subtasks: Vec<GroundSubtask>, pub order: Vec<(String, String)> } |  |  |  |  |

| `GroundSubtask` | struct | GroundSubtask { pub id: String, pub task_name: String } |  |  |  |  |

| `GroundedIR` | struct | GroundedIR { pub actions: Vec<GroundAction>, pub methods: Vec<GroundMethod>, pub root_networks: Vec<GroundRootNetwork>, pub initial_facts: BTreeSet<String>, pub goal: GoalDesc } |  |  |  |  |

| `GroundingLimits` | struct | GroundingLimits { pub max_ground_actions: usize, pub max_ground_methods: usize, pub prune_unreachable: bool, pub prune_irrelevant: bool, pub max_wall: Option<Duration> } |  |  |  |  |

| `ReachabilityInfo` | struct | ReachabilityInfo { pub facts: BTreeSet<String>, pub reachable_actions: BTreeSet<String> } |  |  |  |  |

| `TaskRelevanceInfo` | struct | TaskRelevanceInfo { pub relevant_tasks: BTreeSet<String> } |  |  |  |  |


### crates/ferroplan-hddl/src/parser.rs

| `DEFAULT_MAX_PARSE_DEPTH` | const | DEFAULT_MAX_PARSE_DEPTH: usize |  |  |  |  |

| `ParseError` | enum | ParseError { Syntax(String), UnsupportedConstruct(String), MalformedOneof(String), NestedProbabilisticBlock(String), NestingTooDeep { line: usize, column: usize, budget: usize, } } |  |  |  |  |

| `parse_domain` | function | parse_domain(src: &str) -> Result<Domain, ParseError> |  |  |  |  |

| `parse_domain_with_budget` | function | parse_domain_with_budget(src: &str, max_depth: usize) -> Result<Domain, ParseError> |  |  |  |  |

| `parse_problem` | function | parse_problem(src: &str) -> Result<Problem, ParseError> |  |  |  |  |

| `parse_problem_with_budget` | function | parse_problem_with_budget(src: &str, max_depth: usize) -> Result<Problem, ParseError> |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (and (p) (oneof (q) (r)))))` | str_key | DOMAIN = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (and (p) (oneof (q) (r)))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (and (when (r) (p))) (q))))` | str_key | UNDER_AND = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (and (when (r) (p))) (q))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (when (r) (p)) (q))))` | str_key | DIRECT = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (when (r) (p)) (q))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (when (r) (oneof (p) (q)))))` | str_key | DOMAIN = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (when (r) (oneof (p) (q)))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition (oneof (p) (q))
            :effect (and (p))))` | str_key | DOMAIN = "(define (domain bad)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition (oneof (p) (q))
            :effect (and (p))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof)))` | str_key | DOMAIN = "(define (domain bad)
          (:predicates (p))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof)))" |  |  |  |  |

| `(define (domain childsnack-style)
          (:types child)
          (:predicates (served ?c - child) (dirty ?c - child))
          (:action putdown
            :parameters (?c - child)
            :precondition ()
            :effect (oneof
              (and (served ?c))
              (and (served ?c) (not (dirty ?c))))))` | str_key | DOMAIN = "(define (domain childsnack-style)
          (:types child)
          (:predicates (served ?c - child) (dirty ?c - child))
          (:action putdown
            :parameters (?c - child)
            :precondition ()
            :effect (oneof
              (and (served ?c))
              (and (served ?c) (not (dirty ?c))))))" |  |  |  |  |

| `(define (domain drop-d)
          (:types loc truck)
          (:predicates (at ?t - truck ?l - loc))
          (:action drop
            :parameters (?t - truck ?from - loc ?to - loc)
            :precondition (at ?t ?from)
            :effect (oneof
              (and (not (at ?t ?from)) (at ?t ?to))
              ())))` | str_key | DOMAIN = "(define (domain drop-d)
          (:types loc truck)
          (:predicates (at ?t - truck ?l - loc))
          (:action drop
            :parameters (?t - truck ?from - loc ?to - loc)
            :precondition (at ?t ?from)
            :effect (oneof
              (and (not (at ?t ?from)) (at ?t ?to))
              ())))" |  |  |  |  |

| `(define (domain one-way)
          (:predicates (p) (q))
          (:action once
            :parameters ()
            :precondition ()
            :effect (oneof (and (p) (q)))))` | str_key | DOMAIN = "(define (domain one-way)
          (:predicates (p) (q))
          (:action once
            :parameters ()
            :precondition ()
            :effect (oneof (and (p) (q)))))" |  |  |  |  |

| `(define (domain ordering-operator)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task deliver :parameters (?l - loc))
          (:method m-deliver
            :parameters (?l - loc)
            :task (deliver ?l)
            :subtasks (and
              (t1 (deliver ?l))
              (t2 (deliver ?l))
              (t3 (deliver ?l)))
            :ordering (and
              (< t1 t2)
              (t2 < t3))))` | str_key | DOMAIN = "(define (domain ordering-operator)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task deliver :parameters (?l - loc))
          (:method m-deliver
            :parameters (?l - loc)
            :task (deliver ?l)
            :subtasks (and
              (t1 (deliver ?l))
              (t2 (deliver ?l))
              (t3 (deliver ?l)))
            :ordering (and
              (< t1 t2)
              (t2 < t3))))" |  |  |  |  |

| `(define (domain temporal-d)
  (:durative-action fly
    :parameters (?a ?b)
    :duration (= ?duration 10)
    :condition (at start (at ?a))
    :effect (at end (at ?b))))` | str_key | DOMAIN = "(define (domain temporal-d)
  (:durative-action fly
    :parameters (?a ?b)
    :duration (= ?duration 10)
    :condition (at start (at ?a))
    :effect (at end (at ?b))))" |  |  |  |  |

| `(define (domain three-way)
          (:predicates (p) (q) (r))
          (:action tri
            :parameters ()
            :precondition ()
            :effect (oneof (p) (q) (r))))` | str_key | DOMAIN = "(define (domain three-way)
          (:predicates (p) (q) (r))
          (:action tri
            :parameters ()
            :precondition ()
            :effect (oneof (p) (q) (r))))" |  |  |  |  |

| `../fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../fixtures/a/domain.hddl" |  |  |  |  |

| `../fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../fixtures/a/problem.hddl" |  |  |  |  |

| `../fixtures/b/domain.hddl` | str_key | FIXTURE_B_DOMAIN = "../fixtures/b/domain.hddl" |  |  |  |  |

| `../fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/e/domain.hddl` | str_key | FIXTURE_E_DOMAIN = "../fixtures/e/domain.hddl" |  |  |  |  |

| `../fixtures/f/domain.hddl` | str_key | FIXTURE_F_DOMAIN = "../fixtures/f/domain.hddl" |  |  |  |  |

| `../fixtures/f/problem.hddl` | str_key | FIXTURE_F_PROBLEM = "../fixtures/f/problem.hddl" |  |  |  |  |


### crates/ferroplan-hddl/src/probabilistic.rs

| `has_probabilistic` | function | has_probabilistic(src: &str) -> bool |  |  |  |  |

| `preprocess` | function | preprocess(src: &str) -> Result<(String, BTreeMap<String, Vec<String>>), ParseError> |  |  |  |  |


### crates/ferroplan-hddl/src/translate.rs

| `TranslateError` | enum | TranslateError { UnsupportedNegativeGoal, UnsupportedGoalConnective(String), MalformedTermEquality { found: usize, }, UnboundVariable(String), TaskNetworkDepthExceeded { addr: String, limit: usize, }, Timeout { elapsed_ms: u128, limit_ms: u128, }, MemoryLimitExceeded { states: usize, limit: usize, }, Ground(GroundError) } |  |  |  |  |

| `translate` | function | translate( ir: &GroundedIR, limits: &TranslateLimits, ) -> Result<PlanningProblem, TranslateError> |  |  |  |  |

| `(define (domain coin-empty)
  (:predicates (heads))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof () (heads)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))` | str_key | DOMAIN = "(define (domain coin-empty)
  (:predicates (heads))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof () (heads)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))" |  |  |  |  |

| `(define (domain eq-gate)
  (:requirements :typing :equality :method-preconditions)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p) (q))
  (:task go :parameters (?x - loc))
  (:action mark-p :parameters () :precondition () :effect (p))
  (:action mark-q :parameters () :precondition () :effect (q))
  (:method m-eq
    :parameters (?x - loc)
    :task (go ?x)
    :precondition (= ?x a)
    :ordered-subtasks (and (t1 (mark-p))))
  (:method m-neq
    :parameters (?x - loc)
    :task (go ?x)
    :precondition (not (= ?x a))
    :ordered-subtasks (and (t1 (mark-q)))))` | str_key | DOMAIN = "(define (domain eq-gate)
  (:requirements :typing :equality :method-preconditions)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p) (q))
  (:task go :parameters (?x - loc))
  (:action mark-p :parameters () :precondition () :effect (p))
  (:action mark-q :parameters () :precondition () :effect (q))
  (:method m-eq
    :parameters (?x - loc)
    :task (go ?x)
    :precondition (= ?x a)
    :ordered-subtasks (and (t1 (mark-p))))
  (:method m-neq
    :parameters (?x - loc)
    :task (go ?x)
    :precondition (not (= ?x a))
    :ordered-subtasks (and (t1 (mark-q)))))" |  |  |  |  |

| `(define (domain eq-goal)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p))
  (:task go :parameters ())
  (:method m-go
    :task (go)
    :ordered-subtasks ()))` | str_key | DOMAIN = "(define (domain eq-goal)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p))
  (:task go :parameters ())
  (:method m-go
    :task (go)
    :ordered-subtasks ()))" |  |  |  |  |

| `(define (domain eq-when)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p) (q))
  (:task go :parameters (?x - loc))
  (:action probe :parameters (?x - loc)
    :precondition ()
    :effect (and
      (when (= ?x a) (and (not (q)) (p)))
      (when (not (= ?x a)) (and (not (p)) (q)))))
  (:method m-go
    :parameters (?x - loc)
    :task (go ?x)
    :ordered-subtasks (and (t1 (probe ?x)))))` | str_key | DOMAIN = "(define (domain eq-when)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p) (q))
  (:task go :parameters (?x - loc))
  (:action probe :parameters (?x - loc)
    :precondition ()
    :effect (and
      (when (= ?x a) (and (not (q)) (p)))
      (when (not (= ?x a)) (and (not (p)) (q)))))
  (:method m-go
    :parameters (?x - loc)
    :task (go ?x)
    :ordered-subtasks (and (t1 (probe ?x)))))" |  |  |  |  |

| `(define (domain gated-tri)
  (:predicates (ready) (p) (q) (r))
  (:task go :parameters ())
  (:action tri
    :parameters ()
    :precondition (ready)
    :effect (oneof (p) (q) (r)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (tri)))))` | str_key | DOMAIN = "(define (domain gated-tri)
  (:predicates (ready) (p) (q) (r))
  (:task go :parameters ())
  (:action tri
    :parameters ()
    :precondition (ready)
    :effect (oneof (p) (q) (r)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (tri)))))" |  |  |  |  |

| `(define (domain method-precond-g)
  (:predicates (ready) (done-a) (done-b))
  (:task run :parameters ())
  (:method use-a
    :parameters ()
    :task (run)
    :precondition (ready)
    :ordered-subtasks (mark-a))
  (:method use-b
    :parameters ()
    :task (run)
    :precondition (not (ready))
    :ordered-subtasks (mark-b))
  (:action mark-a
    :effect (done-a))
  (:action mark-b
    :effect (done-b)))` | str_key | DOMAIN = "(define (domain method-precond-g)
  (:predicates (ready) (done-a) (done-b))
  (:task run :parameters ())
  (:method use-a
    :parameters ()
    :task (run)
    :precondition (ready)
    :ordered-subtasks (mark-a))
  (:method use-b
    :parameters ()
    :task (run)
    :precondition (not (ready))
    :ordered-subtasks (mark-b))
  (:action mark-a
    :effect (done-a))
  (:action mark-b
    :effect (done-b)))" |  |  |  |  |

| `(define (domain noop-then-move)
  (:predicates (at-a) (at-b))
  (:task go :parameters ())
  (:action pause
    :parameters ()
    :precondition ()
    :effect (and))
  (:action move
    :parameters ()
    :precondition (at-a)
    :effect (and (not (at-a)) (at-b)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (pause)) (t2 (move)))))` | str_key | DOMAIN = "(define (domain noop-then-move)
  (:predicates (at-a) (at-b))
  (:task go :parameters ())
  (:action pause
    :parameters ()
    :precondition ()
    :effect (and))
  (:action move
    :parameters ()
    :precondition (at-a)
    :effect (and (not (at-a)) (at-b)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (pause)) (t2 (move)))))" |  |  |  |  |

| `(define (domain shortcut-g)
  (:types loc)
  (:predicates (at ?l - loc) (cheated))
  (:task run :parameters ())
  (:action drive
    :parameters (?a - loc ?b - loc)
    :precondition (at ?a)
    :effect (and (not (at ?a)) (at ?b)))
  (:action special-action
    :parameters (?a - loc)
    :precondition (at ?a)
    :effect (and (cheated)))
  (:method m-run
    :parameters ()
    :task (run)
    :ordered-subtasks (and (t1 (drive l1 l2)))))` | str_key | DOMAIN = "(define (domain shortcut-g)
  (:types loc)
  (:predicates (at ?l - loc) (cheated))
  (:task run :parameters ())
  (:action drive
    :parameters (?a - loc ?b - loc)
    :precondition (at ?a)
    :effect (and (not (at ?a)) (at ?b)))
  (:action special-action
    :parameters (?a - loc)
    :precondition (at ?a)
    :effect (and (cheated)))
  (:method m-run
    :parameters ()
    :task (run)
    :ordered-subtasks (and (t1 (drive l1 l2)))))" |  |  |  |  |

| `(define (domain three-way)
  (:predicates (p) (q) (r))
  (:task go :parameters ())
  (:action tri
    :parameters ()
    :precondition ()
    :effect (oneof (p) (q) (r)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (tri)))))` | str_key | DOMAIN = "(define (domain three-way)
  (:predicates (p) (q) (r))
  (:task go :parameters ())
  (:action tri
    :parameters ()
    :precondition ()
    :effect (oneof (p) (q) (r)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (tri)))))" |  |  |  |  |

| `(define (problem coin-empty-p1)
  (:domain coin-empty)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem coin-empty-p1)
  (:domain coin-empty)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem eq-when-p)
  (:domain eq-when)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go a)) (g2 (go b))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem eq-when-p)
  (:domain eq-when)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go a)) (g2 (go b))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem gated-tri-p1)
  (:domain gated-tri)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem gated-tri-p1)
  (:domain gated-tri)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem method-precond-g-p1)
  (:domain method-precond-g)
  (:objects)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (ready))
  (:goal (and (done-a))))` | str_key | PROBLEM = "(define (problem method-precond-g-p1)
  (:domain method-precond-g)
  (:objects)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (ready))
  (:goal (and (done-a))))" |  |  |  |  |

| `(define (problem noop-then-move-p1)
  (:domain noop-then-move)
  (:objects)
  (:init (at-a))
  (:goal (at-b))
  (:htn :ordered-subtasks (and (g1 (go)))))` | str_key | PROBLEM = "(define (problem noop-then-move-p1)
  (:domain noop-then-move)
  (:objects)
  (:init (at-a))
  (:goal (at-b))
  (:htn :ordered-subtasks (and (g1 (go)))))" |  |  |  |  |

| `(define (problem shortcut-g-p1)
  (:domain shortcut-g)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (at l1))
  (:goal (and (cheated))))` | str_key | PROBLEM = "(define (problem shortcut-g-p1)
  (:domain shortcut-g)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (at l1))
  (:goal (and (cheated))))" |  |  |  |  |

| `(define (problem three-way-p1)
  (:domain three-way)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem three-way-p1)
  (:domain three-way)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem transport-a-p1-neg)
  (:domain transport-a)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (and (at l2) (not (has-package)))))` | str_key | PROBLEM = "(define (problem transport-a-p1-neg)
  (:domain transport-a)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (and (at l2) (not (has-package)))))" |  |  |  |  |

| `(define (problem transport-a-p1-or)
  (:domain transport-a)
  (:objects l1 l2 l3 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (or (at l2) (at l3))))` | str_key | PROBLEM = "(define (problem transport-a-p1-or)
  (:domain transport-a)
  (:objects l1 l2 l3 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (or (at l2) (at l3))))" |  |  |  |  |

| `../fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../fixtures/a/domain.hddl" |  |  |  |  |

| `../fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../fixtures/a/problem.hddl" |  |  |  |  |

| `../fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/c/problem.hddl` | str_key | FIXTURE_C_PROBLEM = "../fixtures/c/problem.hddl" |  |  |  |  |

| `../fixtures/e/domain.hddl` | str_key | FIXTURE_E_DOMAIN = "../fixtures/e/domain.hddl" |  |  |  |  |

| `../fixtures/e/problem.hddl` | str_key | FIXTURE_E_PROBLEM = "../fixtures/e/problem.hddl" |  |  |  |  |

| `../fixtures/f/domain.hddl` | str_key | FIXTURE_F_DOMAIN = "../fixtures/f/domain.hddl" |  |  |  |  |

| `../fixtures/f/problem.hddl` | str_key | FIXTURE_F_PROBLEM = "../fixtures/f/problem.hddl" |  |  |  |  |

| `Goal` | struct | Goal { pub facts: BTreeSet<String> } |  |  |  |  |

| `Method` | struct | Method { pub id: String, pub task: String, pub subtasks: Vec<String> } |  |  |  |  |

| `PlanningProblem` | struct | PlanningProblem { pub states: Vec<State>, pub initial_states: Vec<String>, pub goal: Goal, pub transitions: Vec<Transition>, pub tasks: Vec<Task>, pub root_tasks: Vec<String>, pub methods: Vec<Method> } |  |  |  |  |

| `State` | struct | State { pub id: String, pub facts: BTreeSet<String> } |  |  |  |  |

| `Task` | struct | Task { pub id: String, pub primitive_action: Option<String> } |  |  |  |  |

| `Transition` | struct | Transition { pub action: String, pub from: String, pub to: String, pub probability_ppm: u32 } |  |  |  |  |

| `TranslateLimits` | struct | TranslateLimits { pub max_task_network_depth: usize, pub max_wall: Option<std::time::Duration>, pub max_states: Option<usize> } |  |  |  |  |


### crates/ferroplan-hddl/src/validate.rs

| `DuplicateKind` | enum | DuplicateKind { Task, Predicate, Action, Method, Object } |  |  |  |  |

| `ValidationError` | enum | ValidationError { UnknownTaskOrAction(String), ArityMismatch { task: String, expected: usize, found: usize, }, UndefinedPredicate(String), UndefinedType(String), DuplicateDefinition { kind: DuplicateKind, name: String }, CyclicTypeHierarchy { type_name: String, }, PredicateArityMismatch { predicate: String, expected: usize, found: usize, }, MethodHeadNotCompoundTask { method: String, name: String, }, UnknownMethodVariable { method: String, var: String, }, UndefinedOrderRef { in_method: Option<String>, id: String, }, CyclicOrdering { in_method: Option<String>, cycle: Vec<String>, }, MissingTaskNetwork, UnknownConstant { name: String, }, ArgumentTypeMismatch { callee: String, position: usize, expected: String, found: String, }, NonGroundInitAtom { predicate: String, }, NonGroundRootSubtaskArg { id: String, } } |  |  |  |  |

| `ValidationWarning` | enum | ValidationWarning { UnrefinableCompoundTask { task: String, }, DuplicateTypeDeclaration { name: String, } } |  |  |  |  |

| `validate_domain` | function | validate_domain(domain: &Domain) -> Result<(), ValidationError> |  |  |  |  |

| `validate_domain_with_warnings` | function | validate_domain_with_warnings( domain: &Domain, ) -> Result<Vec<ValidationWarning>, ValidationError> |  |  |  |  |

| `validate_problem` | function | validate_problem(domain: &Domain, problem: &Problem) -> Result<(), ValidationError> |  |  |  |  |

| `validate_problem_with_warnings` | function | validate_problem_with_warnings( domain: &Domain, problem: &Problem, ) -> Result<Vec<ValidationWarning>, ValidationError> |  |  |  |  |

| `../fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../fixtures/a/domain.hddl" |  |  |  |  |

| `../fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../fixtures/a/problem.hddl" |  |  |  |  |

| `../fixtures/b/domain.hddl` | str_key | FIXTURE_B_DOMAIN = "../fixtures/b/domain.hddl" |  |  |  |  |

| `../fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/d/domain.hddl` | str_key | FIXTURE_D_DOMAIN = "../fixtures/d/domain.hddl" |  |  |  |  |

| `../fixtures/f/domain.hddl` | str_key | FIXTURE_F_DOMAIN = "../fixtures/f/domain.hddl" |  |  |  |  |

| `../fixtures/g/domain.hddl` | str_key | FIXTURE_G_DOMAIN = "../fixtures/g/domain.hddl" |  |  |  |  |

| `../fixtures/g/problem.hddl` | str_key | FIXTURE_G_PROBLEM = "../fixtures/g/problem.hddl" |  |  |  |  |


### crates/ferroplan-hddl/tests/solve_x.rs

| `CARGO_MANIFEST_DIR` | str_key | DOMAIN_PATH = "CARGO_MANIFEST_DIR" |  |  |  |  |

| `preserve` | str_key | ORDER = "preserve" |  |  |  |  |


### crates/ferroplan-hddl/tests/validate_files.rs

| `../fixtures/c/domain.hddl` | str_key | C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/c/problem.hddl` | str_key | C_PROBLEM = "../fixtures/c/problem.hddl" |  |  |  |  |


### crates/ferroplan-mcp/src/admission.rs

| `canonical_digest` | str_key | RESOURCE_TOOLS = "canonical_digest" |  |  |  |  |

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |

| `urn:chatman:claude-code-admission:v1` | str_key | RECEIPT_DOMAIN = "urn:chatman:claude-code-admission:v1" |  |  |  |  |


### crates/ferroplan-mcp/src/experience.rs

| `dx_manifest` | str_key | RESOURCE_TOOLS = "dx_manifest" |  |  |  |  |

| `plugins/chatman-ecosystem/ontology/ferroplan-experience.ttl` | str_key | ONTOLOGY_SOURCE = "plugins/chatman-ecosystem/ontology/ferroplan-experience.ttl" |  |  |  |  |

| `solve` | str_key | CAPABILITIES = "solve" |  |  |  |  |


### crates/ferroplan-mcp/src/generated/tool_ontology.rs

| `Applies fact and/or fluent observations to a session's belief (via Session::observe for facts, Session::set_fluent per changed fluent), bumping epoch if anything surprised, and reports whether the currently-stored plan is still valid from the current cursor. Fluent comparison against the prior value is an exact to_bits() equality check, not epsilon-tolerant.` | str_key | OBSERVE_ONTOLOGY = "Applies fact and/or fluent observations to a session's belief (via Session::observe for facts, Session::set_fluent per changed fluent), bumping epoch if anything surprised, and reports whether the currently-stored plan is still valid from the current cursor. Fluent comparison against the prior value is an exact to_bits() equality check, not epsilon-tolerant." |  |  |  |  |

| `Apply a bounded heterogeneous session transaction on a staged fork and commit exactly once, or refuse without partial mutation.` | str_key | QOL_BATCH_ONTOLOGY = "Apply a bounded heterogeneous session transaction on a staged fork and commit exactly once, or refuse without partial mutation." |  |  |  |  |

| `Atomically manufacture a ready persistent planning mind from domain, problem, goal, authority scope, and bounded search settings.` | str_key | WIZARD_BOOTSTRAP_ONTOLOGY = "Atomically manufacture a ready persistent planning mind from domain, problem, goal, authority scope, and bounded search settings." |  |  |  |  |

| `Classify a tool or protocol failure into a typed cause with bounded confidence, corrective actions, and refusal-preserving recovery guidance.` | str_key | DOCTOR_EXPLAIN_ONTOLOGY = "Classify a tool or protocol failure into a typed cause with bounded confidence, corrective actions, and refusal-preserving recovery guidance." |  |  |  |  |

| `Compile a high-level operator intent into an ordered, inspectable Ferroplan tool recipe with preflight, rollback, and receipt checkpoints.` | str_key | WIZARD_RECIPE_ONTOLOGY = "Compile a high-level operator intent into an ordered, inspectable Ferroplan tool recipe with preflight, rollback, and receipt checkpoints." |  |  |  |  |

| `Diagnose global or per-session health, assign typed findings, calculate standing, and emit executable remediation hints without mutating state.` | str_key | DOCTOR_SCAN_ONTOLOGY = "Diagnose global or per-session health, assign typed findings, calculate standing, and emit executable remediation hints without mutating state." |  |  |  |  |

| `Enumerate a bounded combinatorial capability lattice, minimal reachability depths, dependency edges, blocked frontiers, and theoretical composition capacity.` | str_key | VISION_LATTICE_ONTOLOGY = "Enumerate a bounded combinatorial capability lattice, minimal reachability depths, dependency edges, blocked frontiers, and theoretical composition capacity." |  |  |  |  |

| `Gall Checkpoint 9 (Recursive Multifractal Allocation): runs cmca_allocate's exact admission law at a root frontier, then descends into zero or more selected admitted nodes, each with a fresh local N=8/F=10 frontier, chaining every depth's payload_digest into the next depth's envelope server-side (never caller-supplied/trusted). Refuses the whole call -- no partial chain -- on: selected_parent_node naming an id that was not an admitted candidate at the immediately preceding depth (ParentNodeUnknown-shaped refusal); selected_parent_node repeating an id already used to enter an earlier depth on the same chain (cyclic-ancestry refusal); or any depth's own admission failing cmca_allocate's underlying candidate/forest/factor law. Same-input calls are byte-identical (deterministic replay) since every depth is a pure function of its own input plus the previous depth's real digest.` | str_key | CMCA_RECURSIVE_ONTOLOGY = "Gall Checkpoint 9 (Recursive Multifractal Allocation): runs cmca_allocate's exact admission law at a root frontier, then descends into zero or more selected admitted nodes, each with a fresh local N=8/F=10 frontier, chaining every depth's payload_digest into the next depth's envelope server-side (never caller-supplied/trusted). Refuses the whole call -- no partial chain -- on: selected_parent_node naming an id that was not an admitted candidate at the immediately preceding depth (ParentNodeUnknown-shaped refusal); selected_parent_node repeating an id already used to enter an earlier depth on the same chain (cyclic-ancestry refusal); or any depth's own admission failing cmca_allocate's underlying candidate/forest/factor law. Same-input calls are byte-identical (deterministic replay) since every depth is a pure function of its own input plus the previous depth's real digest." |  |  |  |  |

| `Inferred: a validation tool beyond plain syntax parsing — most plausibly checking a domain+problem pair grounds successfully (i.e. exercising ferroplan::api's grounding path without necessarily searching for a full plan), used as a pre-flight check before an expensive solve/decompose call. No struct named Validate*/ValidationReport was present in the extracted api.rs public-surface listing, so this tool's exact backing type and field schema is UNVERIFIED against the source excerpts available to this ontology; modeled at the tool-name/purpose level only.` | str_key | VALIDATE_ONTOLOGY = "Inferred: a validation tool beyond plain syntax parsing — most plausibly checking a domain+problem pair grounds successfully (i.e. exercising ferroplan::api's grounding path without necessarily searching for a full plan), used as a pre-flight check before an expensive solve/decompose call. No struct named Validate*/ValidationReport was present in the extracted api.rs public-surface listing, so this tool's exact backing type and field schema is UNVERIFIED against the source excerpts available to this ontology; modeled at the tool-name/purpose level only." |  |  |  |  |

| `Inferred: binds a CMCA/BCINR allocation payload (such as cmca_allocate's output) into a chained receipt, analogous in spirit to the session tools' chain_receipt over SESSION_RECEIPT_DOMAIN, but scoped to allocation payloads rather than session events. Exact input/output field schema not captured at fine grain in the extraction available.` | str_key | BIND_ALLOC_ONTOLOGY = "Inferred: binds a CMCA/BCINR allocation payload (such as cmca_allocate's output) into a chained receipt, analogous in spirit to the session tools' chain_receipt over SESSION_RECEIPT_DOMAIN, but scoped to allocation payloads rather than session events. Exact input/output field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Inferred: binds a ferroplan Plan/Solution payload into a chained receipt, so a downstream actuation broker (per chatman-ecosystem.ttl's ce:BRCE) can require ce:requiresReceipt before treating a candidate plan as admissible. Exact input/output field schema not captured at fine grain in the extraction available.` | str_key | BIND_PLAN_ONTOLOGY = "Inferred: binds a ferroplan Plan/Solution payload into a chained receipt, so a downstream actuation broker (per chatman-ecosystem.ttl's ce:BRCE) can require ce:requiresReceipt before treating a candidate plan as admissible. Exact input/output field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Inferred: computes a canonical (deterministic-serialization) blake3 digest over an arbitrary JSON payload, for use as a stable content-identity input to receipt chaining elsewhere. Exact input/output field schema not captured at fine grain in the extraction available; documented at the tool-name/purpose level only.` | str_key | DIGEST_ONTOLOGY = "Inferred: computes a canonical (deterministic-serialization) blake3 digest over an arbitrary JSON payload, for use as a stable content-identity input to receipt chaining elsewhere. Exact input/output field schema not captured at fine grain in the extraction available; documented at the tool-name/purpose level only." |  |  |  |  |

| `Inferred: verifies a previously-bound receipt (allocation or plan) against its claimed chain head / digest, returning whether the chain is intact. Exact input/output field schema not captured at fine grain in the extraction available.` | str_key | VERIFY_ONTOLOGY = "Inferred: verifies a previously-bound receipt (allocation or plan) against its claimed chain head / digest, returning whether the chain is intact. Exact input/output field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Manufacture a deterministic transport-neutral BLAKE3 integrity envelope with correlation, causation, idempotency, predecessor, and expiry fields; it performs no network operation.` | str_key | TELCO_ENVELOPE_ONTOLOGY = "Manufacture a deterministic transport-neutral BLAKE3 integrity envelope with correlation, causation, idempotency, predecessor, and expiry fields; it performs no network operation." |  |  |  |  |

| `Moves the session's cursor forward by completed_steps against the stored last_plan's length; errors if that would run past the plan's end. Advancing the cursor does not itself apply any world effects — effects still enter belief only via session_observe (or set_fact/elapse on the underlying Session), per the source comment at line 5 and the tool description at line 288.` | str_key | ADVANCE_ONTOLOGY = "Moves the session's cursor forward by completed_steps against the stored last_plan's length; errors if that would run past the plan's end. Advancing the cursor does not itself apply any world effects — effects still enter belief only via session_observe (or set_fact/elapse on the underlying Session), per the source comment at line 5 and the tool description at line 288." |  |  |  |  |

| `Opens (grounds) a new Session under session_id and stores it server-side, chaining an 'opened' receipt. Rejects if the session_id already exists unless replace is true. Rejects non-canonical session_ids (must be alphanumeric/-/_/./:).` | str_key | OPEN_ONTOLOGY = "Opens (grounds) a new Session under session_id and stores it server-side, chaining an 'opened' receipt. Rejects if the session_id already exists unless replace is true. Rejects non-canonical session_ids (must be alphanumeric/-/_/./:)." |  |  |  |  |

| `Read session state, selected facts and fluents, plan standing, diagnostics, memory, lineage, and recent history in one round trip.` | str_key | QOL_SNAPSHOT_ONTOLOGY = "Read session state, selected facts and fluents, plan standing, diagnostics, memory, lineage, and recent history in one round trip." |  |  |  |  |

| `Read-only status snapshot of a session — no receipt is chained (nothing mutated).` | str_key | STATUS_ONTOLOGY = "Read-only status snapshot of a session — no receipt is chained (nothing mutated)." |  |  |  |  |

| `Removes the session from server state (frees its grounded world if no other session shares it). No receipt chaining; no error if the session_id doesn't exist.` | str_key | CLOSE_ONTOLOGY = "Removes the session from server state (frees its grounded world if no other session shares it). No receipt chaining; no error if the session_id doesn't exist." |  |  |  |  |

| `Retargets a session's goal via Session::set_goal, resets cursor to 0, and bumps epoch. remaining_plan_valid in the response checks plan_still_valid(last_plan, 0) — against the just-reset cursor, not whatever cursor held before the call.` | str_key | SET_GOAL_ONTOLOGY = "Retargets a session's goal via Session::set_goal, resets cursor to 0, and bumps epoch. remaining_plan_valid in the response checks plan_still_valid(last_plan, 0) — against the just-reset cursor, not whatever cursor held before the call." |  |  |  |  |

| `Return the complete self-describing Ferroplan capability manifest, including authority categories, contracts, effects, reversibility, receipt behavior, and composition examples.` | str_key | DX_MANIFEST_ONTOLOGY = "Return the complete self-describing Ferroplan capability manifest, including authority categories, contracts, effects, reversibility, receipt behavior, and composition examples." |  |  |  |  |

| `Runs the bcinr_cmca (Chatman Multifractal Cascade Allocator) fixed-size allocation over exactly N=8 candidates, validating each candidate's id (non-empty, unique), parent (forest structure: exactly one root, no cycles), fixed-point factors (F entries, finite, in [0, u32::MAX/65536]), and cost. Note: this tool is CMCA/BCINR allocation surface bundled onto the session server, not itself a Session operation — it is the only tool here with no session_id and no receipt chaining on the ManagedSession scheme (it returns a self-contained payload_digest instead).` | str_key | CMCA_ONTOLOGY = "Runs the bcinr_cmca (Chatman Multifractal Cascade Allocator) fixed-size allocation over exactly N=8 candidates, validating each candidate's id (non-empty, unique), parent (forest structure: exactly one root, no cycles), fixed-point factors (F entries, finite, in [0, u32::MAX/65536]), and cost. Note: this tool is CMCA/BCINR allocation surface bundled onto the session server, not itself a Session operation — it is the only tool here with no session_id and no receipt chaining on the ManagedSession scheme (it returns a self-contained payload_digest instead)." |  |  |  |  |

| `Search the bounded capability graph for a minimal deterministic tool sequence from admitted starting atoms to requested outcome atoms.` | str_key | DX_COMPOSE_ONTOLOGY = "Search the bounded capability graph for a minimal deterministic tool sequence from admitted starting atoms to requested outcome atoms." |  |  |  |  |

| `The think step: if the stored last_plan is still valid from the current cursor, short-circuits to a 'follow' decision (searched:false) without invoking any planner, regardless of prefer_follow. Otherwise searches: prefer_follow only changes behavior when a (now-invalid) prior plan exists — then it chooses between Session::replan_following (bias toward the prior plan's structure) and Session::replan_budgeted (from-scratch bounded search); with no prior plan at all it always uses replan_budgeted regardless of prefer_follow. Cursor is always reset to 0 after any search path (both branches). decision is 'replan' if solved else 'bounded-refusal'.` | str_key | THINK_ONTOLOGY = "The think step: if the stored last_plan is still valid from the current cursor, short-circuits to a 'follow' decision (searched:false) without invoking any planner, regardless of prefer_follow. Otherwise searches: prefer_follow only changes behavior when a (now-invalid) prior plan exists — then it chooses between Session::replan_following (bias toward the prior plan's structure) and Session::replan_budgeted (from-scratch bounded search); with no prior plan at all it always uses replan_budgeted regardless of prefer_follow. Cursor is always reset to 0 after any search path (both branches). decision is 'replan' if solved else 'bounded-refusal'." |  |  |  |  |

| `Verify a transport envelope's schema, payload identity, envelope identity, routing expectations, predecessor, and expiry without treating integrity as authentication.` | str_key | TELCO_VERIFY_ONTOLOGY = "Verify a transport envelope's schema, payload identity, envelope identity, routing expectations, predecessor, and expiry without treating integrity as authentication." |  |  |  |  |

| `Wraps ferroplan::api::decompose(domain_src, problem_src, opts) -> Result<Decomposition, SolveError>: decomposes a temporal goal into solvable contracts, solves and stitches them, and returns the inspectable fp:Decomposition. Inferred: MCP wrapper's exact JSON field schema not captured at fine grain in the extraction available.` | str_key | DECOMPOSE_ONTOLOGY = "Wraps ferroplan::api::decompose(domain_src, problem_src, opts) -> Result<Decomposition, SolveError>: decomposes a temporal goal into solvable contracts, solves and stitches them, and returns the inspectable fp:Decomposition. Inferred: MCP wrapper's exact JSON field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Wraps ferroplan::api::parse(src) -> ParseReport: validates PDDL syntax and returns a structure summary without grounding or solving, auto-detecting domain vs problem. Inferred: MCP wrapper's exact JSON field schema not captured at fine grain in the extraction available.` | str_key | PARSE_ONTOLOGY = "Wraps ferroplan::api::parse(src) -> ParseReport: validates PDDL syntax and returns a structure summary without grounding or solving, auto-detecting domain vs problem. Inferred: MCP wrapper's exact JSON field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Wraps ferroplan::api::solve(domain_src, problem_src, opts) -> Result<Solution, SolveError>: parses domain+problem PDDL, grounds, and searches for a plan under the given fp:Options, returning the fp:Solution shape. Inferred: field-by-field JSON request/response schema for the MCP wrapper itself (as opposed to the underlying api::solve signature, which is documented in api.rs) was not captured at fine grain in the extraction available.` | str_key | SOLVE_ONTOLOGY = "Wraps ferroplan::api::solve(domain_src, problem_src, opts) -> Result<Solution, SolveError>: parses domain+problem PDDL, grounds, and searches for a plan under the given fp:Options, returning the fp:Solution shape. Inferred: field-by-field JSON request/response schema for the MCP wrapper itself (as opposed to the underlying api::solve signature, which is documented in api.rs) was not captured at fine grain in the extraction available." |  |  |  |  |


### crates/ferroplan-mcp/src/main.rs

| `solve` | str_key | MAIN_RESOURCE_TOOLS = "solve" |  |  |  |  |


### crates/ferroplan-mcp/src/mcp_plus_ready.rs

| `ferroplan-mcp-plus/1.0` | str_key | PROFILE = "ferroplan-mcp-plus/1.0" |  |  |  |  |


### crates/ferroplan-mcp/src/session.rs

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |

| `session_open` | str_key | RESOURCE_TOOLS = "session_open" |  |  |  |  |

| `urn:chatman:ferroplan-session-chain:v1` | str_key | SESSION_RECEIPT_DOMAIN = "urn:chatman:ferroplan-session-chain:v1" |  |  |  |  |


### crates/ferroplan-mcp/src/session_control.rs

| `domain_digest` | str_key | KEYS = "domain_digest" |  |  |  |  |

| `session_list` | str_key | RESOURCE_TOOLS = "session_list" |  |  |  |  |


### crates/ferroplan-mcp/tests/admission_protocol.rs

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |


### crates/ferroplan-mcp/tests/checkpoint8_refusals.rs

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |


### crates/ferroplan-mcp/tests/common/mod.rs

| `DOM` | const | DOM: &str |  |  |  |  |

| `PROB` | const | PROB: &str |  |  |  |  |

| `call` | function | call(&mut self, tool: &str, args: Value) -> Value |  |  |  |  |

| `call_json` | function | call_json(&mut self, tool: &str, args: Value) -> Value |  |  |  |  |

| `call_text` | function | call_text(&mut self, tool: &str, args: Value) -> (String, bool) |  |  |  |  |

| `finish` | function | finish(mut self) |  |  |  |  |

| `notify` | function | notify(&mut self, method: &str) |  |  |  |  |

| `request` | function | request(&mut self, method: &str, params: Value) -> Value |  |  |  |  |

| `start` | function | start() -> Client |  |  |  |  |

| `(define (domain d) (:requirements :strips) (:predicates (p) (q) (r)) ` | str_key | DOM = "(define (domain d) (:requirements :strips) (:predicates (p) (q) (r)) " |  |  |  |  |

| `(define (problem pr) (:domain d) (:init (p)) (:goal (r)))` | str_key | PROB = "(define (problem pr) (:domain d) (:init (p)) (:goal (r)))" |  |  |  |  |

| `Client` | struct | Client { child: Child, stdin: Option<ChildStdin>, stdout: BufReader<ChildStdout>, next_id: i64 } |  |  |  |  |


### crates/ferroplan-mcp/tests/dogfood_chain.rs

| `(define (domain loc) (:requirements :strips) ` | str_key | DOM = "(define (domain loc) (:requirements :strips) " |  |  |  |  |

| `(define (problem locp) (:domain loc) (:init (at-a)) (:goal (at-c)))` | str_key | PROB = "(define (problem locp) (:domain loc) (:init (at-a)) (:goal (at-c)))" |  |  |  |  |


### crates/ferroplan-mcp/tests/mcp_ontology_drift.rs

| `../../plugins/chatman-ecosystem/ontology/ferroplan-domain.ttl` | str_key | ONTOLOGIES = "../../plugins/chatman-ecosystem/ontology/ferroplan-domain.ttl" |  |  |  |  |

| `solve` | str_key | MAIN_STRUCTS = "solve" |  |  |  |  |

| `src/generated/tool_ontology.rs` | str_key | GENERATED_REL = "src/generated/tool_ontology.rs" |  |  |  |  |

| `src/main.rs` | str_key | ROUTER_SOURCES = "src/main.rs" |  |  |  |  |


### crates/ferroplan-mcp/tests/merged_server.rs

| `solve` | str_key | ALL_42_TOOLS = "solve" |  |  |  |  |


### crates/ferroplan-mcp/tests/recursive_admission_protocol.rs

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |


### crates/ferroplan-mcp/tests/session.rs

| `
(define (domain farm) (:requirements :strips :typing :numeric-fluents)
  (:types agent place)
  (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
  (:functions (grain))
  (:action walk :parameters (?a - agent ?from ?to - place)
    :precondition (and (at ?a ?from) (road ?from ?to))
    :effect (and (not (at ?a ?from)) (at ?a ?to)))
  (:action harvest :parameters (?a - agent ?p - place)
    :precondition (and (at ?a ?p) (fertile ?p))
    :effect (increase (grain) 1)))` | str_key | FARM_DOM = "
(define (domain farm) (:requirements :strips :typing :numeric-fluents)
  (:types agent place)
  (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
  (:functions (grain))
  (:action walk :parameters (?a - agent ?from ?to - place)
    :precondition (and (at ?a ?from) (road ?from ?to))
    :effect (and (not (at ?a ?from)) (at ?a ?to)))
  (:action harvest :parameters (?a - agent ?p - place)
    :precondition (and (at ?a ?p) (fertile ?p))
    :effect (increase (grain) 1)))" |  |  |  |  |

| `
(define (domain rollers) (:requirements :strips :typing)
  (:types ball room)
  (:predicates (at ?b - ball ?r - room) (link ?x ?y - room)
               (goal-room ?r - room) (home ?b - ball))
  (:action roll :parameters (?b - ball ?from ?to - room)
    :precondition (and (at ?b ?from) (link ?from ?to))
    :effect (and (not (at ?b ?from)) (at ?b ?to)))
  (:action park :parameters (?b - ball ?r - room)
    :precondition (and (at ?b ?r) (goal-room ?r))
    :effect (home ?b)))` | str_key | ORB_DOM = "
(define (domain rollers) (:requirements :strips :typing)
  (:types ball room)
  (:predicates (at ?b - ball ?r - room) (link ?x ?y - room)
               (goal-room ?r - room) (home ?b - ball))
  (:action roll :parameters (?b - ball ?from ?to - room)
    :precondition (and (at ?b ?from) (link ?from ?to))
    :effect (and (not (at ?b ?from)) (at ?b ?to)))
  (:action park :parameters (?b - ball ?r - room)
    :precondition (and (at ?b ?r) (goal-room ?r))
    :effect (home ?b)))" |  |  |  |  |

| `
(define (problem p) (:domain farm)
  (:objects v1 - agent hut field - place)
  (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
  (:goal (>= (grain) 2)))` | str_key | FARM_PRB = "
(define (problem p) (:domain farm)
  (:objects v1 - agent hut field - place)
  (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
  (:goal (>= (grain) 2)))" |  |  |  |  |

| `
(define (problem p) (:domain rollers)
  (:objects b1 b2 - ball ra rb - room)
  (:init (at b1 ra) (at b2 ra) (link ra rb) (link rb ra) (goal-room rb))
  (:goal (and (home b1) (home b2))))` | str_key | ORB_PRB = "
(define (problem p) (:domain rollers)
  (:objects b1 b2 - ball ra rb - room)
  (:init (at b1 ra) (at b2 ra) (link ra rb) (link rb ra) (goal-room rb))
  (:goal (and (home b1) (home b2))))" |  |  |  |  |


### crates/ferroplan-mcp/tests/session_goal_advance.rs

| `(define (domain d3) (:requirements :strips) ` | str_key | DOM = "(define (domain d3) (:requirements :strips) " |  |  |  |  |

| `(define (problem pr3) (:domain d3) (:init (p)) (:goal (s)))` | str_key | PROB = "(define (problem pr3) (:domain d3) (:init (p)) (:goal (s)))" |  |  |  |  |


### crates/ferroplan-mcp/tests/session_lifecycle_bookends.rs

| `(define (domain d) (:requirements :strips) (:predicates (p) (q)) ` | str_key | DOM = "(define (domain d) (:requirements :strips) (:predicates (p) (q)) " |  |  |  |  |

| `(define (problem pr) (:domain d) (:init (p)) (:goal (q)))` | str_key | PROB = "(define (problem pr) (:domain d) (:init (p)) (:goal (q)))" |  |  |  |  |


### crates/ferroplan-mcp/tests/session_protocol.rs

| `(define (domain d) (:requirements :strips) (:predicates (p) (q)) ` | str_key | DOM = "(define (domain d) (:requirements :strips) (:predicates (p) (q)) " |  |  |  |  |

| `(define (problem pr) (:domain d) (:init (p)) (:goal (q)))` | str_key | PROB = "(define (problem pr) (:domain d) (:init (p)) (:goal (q)))" |  |  |  |  |


### crates/ferroplan-runtime/src/authority.rs

| `Authority` | enum | Authority { Observe, Select, Construct, Do } |  |  |  |  |

| `permits` | function | permits(g: Authority, n: Authority) -> bool |  |  |  |  |


### crates/ferroplan-runtime/src/backoff.rs

| `exponential` | function | exponential(base: u64, attempt: u32, cap: u64) -> u64 |  |  |  |  |


### crates/ferroplan-runtime/src/budget.rs

| `new` | function | new(n: u32) -> Self |  |  |  |  |

| `remaining` | function | remaining(&self) -> u32 |  |  |  |  |

| `take` | function | take(&mut self) -> bool |  |  |  |  |

| `Budget` | struct | Budget { remaining: u32 } |  |  |  |  |


### crates/ferroplan-runtime/src/capability.rs

| `compatible` | function | compatible(available: &[Capability], required: &str) -> bool |  |  |  |  |

| `Capability` | struct | Capability { pub &'static str } |  |  |  |  |


### crates/ferroplan-runtime/src/circuit.rs

| `open` | function | open(&self) -> bool |  |  |  |  |

| `record_failure` | function | record_failure(&mut self) |  |  |  |  |

| `Circuit` | struct | Circuit { pub failures: u32, pub threshold: u32 } |  |  |  |  |


### crates/ferroplan-runtime/src/context.rs

| `bounded` | function | bounded(x: &[T], n: usize) -> Vec<T> |  |  |  |  |


### crates/ferroplan-runtime/src/coordinator.rs

| `fail` | function | fail(&mut self, id: &str) |  |  |  |  |

| `Coordinator` | struct | Coordinator { pub graph: Graph, pub excluded: Vec<String> } |  |  |  |  |


### crates/ferroplan-runtime/src/deadline.rs

| `after` | function | after(d: Duration) -> Self |  |  |  |  |

| `expired` | function | expired(&self) -> bool |  |  |  |  |

| `Deadline` | struct | Deadline { pub Instant } |  |  |  |  |


### crates/ferroplan-runtime/src/dispatcher.rs

| `dispatch` | function | dispatch(p: &P, s: &str) -> Outcome<Vec<String>> |  |  |  |  |


### crates/ferroplan-runtime/src/edge.rs

| `exclude` | function | exclude(&mut self) |  |  |  |  |

| `Edge` | struct | Edge { pub id: String, pub provider: String, pub enabled: bool } |  |  |  |  |


### crates/ferroplan-runtime/src/epoch.rs

| `next` | function | next(self) -> Self |  |  |  |  |

| `Epoch` | struct | Epoch { pub u64 } |  |  |  |  |


### crates/ferroplan-runtime/src/evidence.rs

| `admitted` | function | admitted(xs: &[Observation]) -> bool |  |  |  |  |


### crates/ferroplan-runtime/src/exact_subject.rs

| `admitted` | function | admitted(&self) -> bool |  |  |  |  |

| `ExactSubject` | struct | ExactSubject { pub repo: String, pub sha: String } |  |  |  |  |


### crates/ferroplan-runtime/src/failure.rs

| `FailureClass` | enum | FailureClass { Local, Edge, Authority, Unknown } |  |  |  |  |

| `classify` | function | classify(o: &Outcome<T>) -> Option<FailureClass> |  |  |  |  |


### crates/ferroplan-runtime/src/fond.rs

| `reselect` | function | reselect(g: &'a Graph, x: &[String]) -> Option<&'a str> |  |  |  |  |


### crates/ferroplan-runtime/src/graph.rs

| `exclude` | function | exclude(&mut self, id: &str) |  |  |  |  |

| `lawful` | function | lawful(&self) -> impl Iterator<Item = &Edge> |  |  |  |  |

| `Graph` | struct | Graph { pub edges: Vec<Edge> } |  |  |  |  |


### crates/ferroplan-runtime/src/hddl.rs

| `leaves` | function | leaves(&'a self, out: &mut Vec<&'a str>) |  |  |  |  |

| `Task` | struct | Task { pub id: String, pub children: Vec<Task> } |  |  |  |  |


### crates/ferroplan-runtime/src/health.rs

| `Health` | enum | Health { Healthy, Degraded, Open } |  |  |  |  |

| `eligible` | function | eligible(h: Health) -> bool |  |  |  |  |


### crates/ferroplan-runtime/src/idempotency.rs

| `key` | function | key(subject: &str, epoch: u64, provider: &str) -> u64 |  |  |  |  |


### crates/ferroplan-runtime/src/lease.rs

| `valid_for` | function | valid_for(&self, o: &str, e: u64) -> bool |  |  |  |  |

| `Lease` | struct | Lease { pub owner: String, pub epoch: u64 } |  |  |  |  |


### crates/ferroplan-runtime/src/observation.rs

| `Observation` | struct | Observation { pub key: String, pub value: String, pub source: String } |  |  |  |  |


### crates/ferroplan-runtime/src/ocel.rs

| `bound` | function | bound(&self) -> bool |  |  |  |  |

| `Event` | struct | Event { pub id: String, pub activity: String, pub objects: Vec<String> } |  |  |  |  |


### crates/ferroplan-runtime/src/outcome.rs

| `Outcome` | enum | Outcome { Success(T), Retryable(String), Permanent(String), Refused(String), Unknown(String) } |  |  |  |  |

| `is_success` | function | is_success(&self) -> bool |  |  |  |  |


### crates/ferroplan-runtime/src/plan.rs

| `valid` | function | valid(&self) -> bool |  |  |  |  |

| `Plan` | struct | Plan { pub provider: String, pub steps: Vec<String>, pub cost: u64 } |  |  |  |  |


### crates/ferroplan-runtime/src/policy.rs

| `select` | function | select(e: &'a [Edge], x: &[String]) -> Option<&'a Edge> |  |  |  |  |


### crates/ferroplan-runtime/src/poly_evidence.rs

| `exact` | function | exact(&self) -> bool |  |  |  |  |

| `EvidenceSet` | struct | EvidenceSet { pub subject: String, pub sources: Vec<String> } |  |  |  |  |


### crates/ferroplan-runtime/src/portfolio.rs

| `ranked` | function | ranked(mut ps: Vec<Plan>) -> Vec<Plan> |  |  |  |  |


### crates/ferroplan-runtime/src/powl.rs

| `acyclic` | function | acyclic(e: &[Order]) -> bool |  |  |  |  |

| `Order` | struct | Order { pub before: String, pub after: String } |  |  |  |  |


### crates/ferroplan-runtime/src/provider.rs

| `Provider` | trait |  |  |  |  |  |


### crates/ferroplan-runtime/src/ptd.rs

| `changed` | function | changed(a: &EpochArtifact, b: &EpochArtifact) -> bool |  |  |  |  |

| `EpochArtifact` | struct | EpochArtifact { pub epoch: Epoch, pub digest: u64 } |  |  |  |  |


### crates/ferroplan-runtime/src/receipt.rs

| `exact` | function | exact(&self) -> bool |  |  |  |  |

| `Receipt` | struct | Receipt { pub subject_sha: String, pub epoch: u64, pub provider: String, pub edge: String, pub outcome: String } |  |  |  |  |


### crates/ferroplan-runtime/src/reconcile.rs

| `reconcile` | function | reconcile(g: &mut Graph, s: &[(String, Health)]) |  |  |  |  |


### crates/ferroplan-runtime/src/recovery.rs

| `exclude_failed` | function | exclude_failed(g: &mut Graph, edge: &str) -> usize |  |  |  |  |


### crates/ferroplan-runtime/src/registry.rs

| `contains` | function | contains(&self, id: &str) -> bool |  |  |  |  |

| `register` | function | register(&mut self, id: impl Into<String>) |  |  |  |  |

| `Registry` | struct | Registry { ids: BTreeSet<String> } |  |  |  |  |


### crates/ferroplan-runtime/src/replay.rs

| `same_decision` | function | same_decision(a: &Receipt, b: &Receipt) -> bool |  |  |  |  |


### crates/ferroplan-runtime/src/runtime.rs

| `next_edge` | function | next_edge(&self) -> Option<&str> |  |  |  |  |


### crates/ferroplan-runtime/src/scheduler.rs

| `is_empty` | function | is_empty(&self) -> bool |  |  |  |  |

| `len` | function | len(&self) -> usize |  |  |  |  |

| `pop_next` | function | pop_next(&mut self) -> Option<T> |  |  |  |  |

| `push` | function | push(&mut self, x: T) |  |  |  |  |

| `Scheduler` | struct | Scheduler { q: VecDeque<T> } |  |  |  |  |


### crates/ferroplan-runtime/src/state.rs

| `State` | enum | State { Bound, Planning, Executing, Recovering, Succeeded, Failed } |  |  |  |  |

| `allowed` | function | allowed(a: State, b: State) -> bool |  |  |  |  |


### crates/ferroplan-runtime/src/supervision.rs

| `Restart` | enum | Restart { Permanent, Transient, Temporary } |  |  |  |  |

| `should_restart` | function | should_restart(r: Restart, abnormal: bool) -> bool |  |  |  |  |


### crates/ferroplan-runtime/src/switch.rs

| `next` | function | next(ps: &'a [Plan], failed: &str) -> Option<&'a Plan> |  |  |  |  |


### crates/ferroplan-runtime/src/telemetry.rs

| `success_rate` | function | success_rate(&self) -> f64 |  |  |  |  |

| `Counters` | struct | Counters { pub attempts: u64, pub recoveries: u64, pub successes: u64 } |  |  |  |  |


### crates/ferroplan-runtime/src/tla_evidence.rs

| `actionable` | function | actionable(&self) -> bool |  |  |  |  |

| `Counterexample` | struct | Counterexample { pub invariant: String, pub trace: Vec<String> } |  |  |  |  |


### crates/ferroplan-runtime/src/trimtab.rs

| `ModelRole` | struct | ModelRole { pub context: bool, pub select: bool, pub construct: bool, pub do_act: bool } |  |  |  |  |


### crates/ferroplan-sat/src/analyze_conflict.rs

| `add` | function | add(&mut self, level: usize) |  |  |  |  |

| `analyze_conflict` | function | analyze_conflict(ctx: &mut Context, conflict: Conflict) -> usize |  |  |  |  |

| `clause` | function | clause(&self) -> &[Lit] |  |  |  |  |

| `involved` | function | involved(&self) -> &[ClauseRef] |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `test` | function | test(&self, level: usize) -> bool |  |  |  |  |

| `AnalyzeConflict` | struct | AnalyzeConflict { clause: Vec<Lit>, current_level_count: usize, var_flags: Vec<bool>, to_clean: Vec<Var>, involved: Vec<ClauseRef>, stack: Vec<Lit> } |  |  |  |  |


### crates/ferroplan-sat/src/assumptions.rs

| `EnqueueAssumption` | enum | EnqueueAssumption { Done, Enqueued, Conflict } |  |  |  |  |

| `assumption_levels` | function | assumption_levels(&self) -> usize |  |  |  |  |

| `enqueue_assumption` | function | enqueue_assumption(ctx: &mut Context) -> EnqueueAssumption |  |  |  |  |

| `full_restart` | function | full_restart(&mut self) |  |  |  |  |

| `set_assumptions` | function | set_assumptions(ctx: &mut Context, user_assumptions: &[Lit]) |  |  |  |  |

| `user_failed_core` | function | user_failed_core(&self) -> &[Lit] |  |  |  |  |

| `Assumptions` | struct | Assumptions { assumptions: Vec<Lit>, failed_core: Vec<Lit>, user_failed_core: Vec<Lit>, assumption_levels: usize } |  |  |  |  |


### crates/ferroplan-sat/src/binary.rs

| `add_binary_clause` | function | add_binary_clause(&mut self, lits: [Lit; 2]) |  |  |  |  |

| `count` | function | count(&self) -> usize |  |  |  |  |

| `implied` | function | implied(&self, lit: Lit) -> &[Lit] |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `simplify_binary` | function | simplify_binary(ctx: &mut Context) |  |  |  |  |

| `BinaryClauses` | struct | BinaryClauses { by_lit: Vec<Vec<Lit>>, count: usize } |  |  |  |  |


### crates/ferroplan-sat/src/cdcl.rs

| `conflict_step` | function | conflict_step(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/clause.rs

| `header` | function | header(&self) -> &ClauseHeader |  |  |  |  |

| `header_mut` | function | header_mut(&mut self) -> &mut ClauseHeader |  |  |  |  |

| `lits` | function | lits(&self) -> &[Lit] |  |  |  |  |

| `lits_mut` | function | lits_mut(&mut self) -> &mut [Lit] |  |  |  |  |

| `Clause` | struct | Clause { data: [LitIdx] } |  |  |  |  |

| `activity::{decay_clause_activities, ClauseActivity}` | use | activity::{decay_clause_activities, ClauseActivity} |  |  |  |  |

| `alloc::{ClauseAlloc, ClauseRef}` | use | alloc::{ClauseAlloc, ClauseRef} |  |  |  |  |

| `db::{ClauseDb, Tier}` | use | db::{ClauseDb, Tier} |  |  |  |  |

| `gc::collect_garbage` | use | gc::collect_garbage |  |  |  |  |

| `header::ClauseHeader` | use | header::ClauseHeader |  |  |  |  |


### crates/ferroplan-sat/src/clause/activity.rs

| `bump_clause_activity` | function | bump_clause_activity(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| `decay_clause_activities` | function | decay_clause_activities(ctx: &mut Context) |  |  |  |  |

| `ClauseActivity` | struct | ClauseActivity { bump: f32, inv_decay: f32 } |  |  |  |  |


### crates/ferroplan-sat/src/clause/alloc.rs

| `add_clause` | function | add_clause(&mut self, mut header: ClauseHeader, lits: &[Lit]) -> ClauseRef |  |  |  |  |

| `buffer_size` | function | buffer_size(&self) -> usize |  |  |  |  |

| `check_bounds` | function | check_bounds(&self, cref: ClauseRef, len: usize) |  |  |  |  |

| `clause` | function | clause(&self, cref: ClauseRef) -> &Clause |  |  |  |  |

| `clause_mut` | function | clause_mut(&mut self, cref: ClauseRef) -> &mut Clause |  |  |  |  |

| `header` | function | header(&self, cref: ClauseRef) -> &ClauseHeader |  |  |  |  |

| `header_mut` | function | header_mut(&mut self, cref: ClauseRef) -> &mut ClauseHeader |  |  |  |  |

| `header_unchecked_mut` | function | header_unchecked_mut(&mut self, cref: ClauseRef) -> &mut ClauseHeader |  |  |  |  |

| `lits_ptr_mut_unchecked` | function | lits_ptr_mut_unchecked(&mut self, cref: ClauseRef) -> *mut Lit |  |  |  |  |

| `with_capacity` | function | with_capacity(capacity: usize) -> ClauseAlloc |  |  |  |  |

| `ClauseAlloc` | struct | ClauseAlloc { buffer: Vec<LitIdx> } |  |  |  |  |

| `ClauseRef` | struct | ClauseRef { offset: ClauseOffset } |  |  |  |  |


### crates/ferroplan-sat/src/clause/assess.rs

| `assess_learned_clause` | function | assess_learned_clause(ctx: &mut Context, lits: &[Lit]) -> ClauseHeader |  |  |  |  |

| `bump_clause` | function | bump_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |


### crates/ferroplan-sat/src/clause/db.rs

| `Tier` | enum | Tier { Irred = 0, Core = 1, Mid = 2, Local = 3 } |  |  |  |  |

| `add_clause` | function | add_clause(ctx: &mut Context, header: ClauseHeader, lits: &[Lit]) -> ClauseRef |  |  |  |  |

| `clauses_iter` | function | clauses_iter( db: &'a ClauseDb, alloc: &'a ClauseAlloc, ) -> impl Iterator<Item = ClauseRef> + 'a |  |  |  |  |

| `count` | function | count() -> usize |  |  |  |  |

| `count_by_tier` | function | count_by_tier(&self, tier: Tier) -> usize |  |  |  |  |

| `delete_clause` | function | delete_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| `filter_clauses` | function | filter_clauses( alloc: &mut ClauseAlloc, db: &mut ClauseDb, watchlists: &mut crate::prop::Watchlists, mut filter: F, ) |  |  |  |  |

| `from_index` | function | from_index(index: usize) -> Tier |  |  |  |  |

| `set_clause_tier` | function | set_clause_tier(ctx: &mut Context, cref: ClauseRef, tier: Tier) |  |  |  |  |

| `try_delete_clause` | function | try_delete_clause(ctx: &mut Context, cref: ClauseRef) -> bool |  |  |  |  |

| `ClauseDb` | struct | ClauseDb { pub(crate) clauses: Vec<ClauseRef>, pub(super) by_tier: [Vec<ClauseRef>; Tier::count()], pub(super) count_by_tier: [usize; Tier::count()], pub(super) garbage_size: usize } |  |  |  |  |


### crates/ferroplan-sat/src/clause/gc.rs

| `collect_garbage` | function | collect_garbage(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/clause/header.rs

| `active` | function | active(&self) -> bool |  |  |  |  |

| `activity` | function | activity(&self) -> f32 |  |  |  |  |

| `deleted` | function | deleted(&self) -> bool |  |  |  |  |

| `glue` | function | glue(&self) -> usize |  |  |  |  |

| `len` | function | len(&self) -> usize |  |  |  |  |

| `mark` | function | mark(&self) -> bool |  |  |  |  |

| `new` | function | new() -> ClauseHeader |  |  |  |  |

| `set_active` | function | set_active(&mut self, active: bool) |  |  |  |  |

| `set_activity` | function | set_activity(&mut self, activity: f32) |  |  |  |  |

| `set_deleted` | function | set_deleted(&mut self, deleted: bool) |  |  |  |  |

| `set_glue` | function | set_glue(&mut self, glue: usize) |  |  |  |  |

| `set_len` | function | set_len(&mut self, length: usize) |  |  |  |  |

| `set_mark` | function | set_mark(&mut self, mark: bool) |  |  |  |  |

| `set_tier` | function | set_tier(&mut self, tier: Tier) |  |  |  |  |

| `tier` | function | tier(&self) -> Tier |  |  |  |  |

| `ClauseHeader` | struct | ClauseHeader { pub(super) data: [LitIdx; HEADER_LEN] } |  |  |  |  |


### crates/ferroplan-sat/src/clause/reduce.rs

| `dedup_and_mark_by_tier` | function | dedup_and_mark_by_tier(ctx: &mut Context, tier: Tier) |  |  |  |  |

| `reduce_locals` | function | reduce_locals(ctx: &mut Context) |  |  |  |  |

| `reduce_mids` | function | reduce_mids(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/cnf.rs

| `is_empty` | function | is_empty(&self) -> bool |  |  |  |  |

| `iter` | function | iter(&self) -> impl Iterator<Item = &[Lit]> |  |  |  |  |

| `len` | function | len(&self) -> usize |  |  |  |  |

| `new` | function | new() -> CnfFormula |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `var_count` | function | var_count(&self) -> usize |  |  |  |  |

| `CnfFormula` | struct | CnfFormula { var_count: usize, literals: Vec<Lit>, clause_ranges: Vec<Range<usize>> } |  |  |  |  |

| `ExtendFormula` | trait |  |  |  |  |  |


### crates/ferroplan-sat/src/config.rs

| `SolverConfig` | struct | SolverConfig { pub vsids_decay: f32, pub clause_activity_decay: f32, pub reduce_locals_interval: u64, pub reduce_mids_interval: u64, pub luby_restart_interval_scale: u64 } |  |  |  |  |


### crates/ferroplan-sat/src/context.rs

| `set_var_count` | function | set_var_count(ctx: &mut Context, count: usize) |  |  |  |  |

| `Context` | struct | Context { pub analyze_conflict: AnalyzeConflict, pub assignment: Assignment, pub assumptions: Assumptions, pub binary_clauses: BinaryClauses, pub clause_activity: ClauseActivity, pub clause_alloc: ClauseAlloc, pub clause_db: ClauseDb, pub impl_graph: ImplGraph, pub model: Model, pub schedule: Schedule, pub solver_config: SolverConfig, pub solver_state: SolverState, pub tmp_data: TmpData, pub tmp_flags: TmpFlags, pub trail: Trail, pub variables: Variables, pub vsids: Vsids, pub watchlists: Watchlists } |  |  |  |  |


### crates/ferroplan-sat/src/decision.rs

| `make_decision` | function | make_decision(ctx: &mut Context) -> bool |  |  |  |  |


### crates/ferroplan-sat/src/decision/vsids.rs

| `bump` | function | bump(&mut self, var: Var) |  |  |  |  |

| `decay` | function | decay(&mut self) |  |  |  |  |

| `make_available` | function | make_available(&mut self, var: Var) |  |  |  |  |

| `make_unavailable` | function | make_unavailable(&mut self, var: Var) |  |  |  |  |

| `reset` | function | reset(&mut self, var: Var) |  |  |  |  |

| `seed` | function | seed(&mut self, var: Var, activity: f32) |  |  |  |  |

| `set_decay` | function | set_decay(&mut self, decay: f32) |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `var_count` | function | var_count(&self) -> usize |  |  |  |  |

| `Vsids` | struct | Vsids { activity: Vec<f32>, heap: Vec<Var>, position: Vec<Option<usize>>, bump: f32, inv_decay: f32 } |  |  |  |  |


### crates/ferroplan-sat/src/dimacs.rs

| `DimacsError` | enum | DimacsError { Io(io::Error), Parse { line: usize, msg: String } } |  |  |  |  |

| `parse_dimacs` | function | parse_dimacs(input: impl io::BufRead) -> Result<CnfFormula, DimacsError> |  |  |  |  |

| `parse_dimacs_str` | function | parse_dimacs_str(input: &str) -> Result<CnfFormula, DimacsError> |  |  |  |  |

| `write_dimacs` | function | write_dimacs(target: &mut impl io::Write, formula: &CnfFormula) -> io::Result<()> |  |  |  |  |


### crates/ferroplan-sat/src/glue.rs

| `compute_glue` | function | compute_glue(tmp_flags: &mut TmpFlags, impl_graph: &ImplGraph, lits: &[Lit]) -> usize |  |  |  |  |


### crates/ferroplan-sat/src/lib.rs

| `cnf::{CnfFormula, ExtendFormula}` | use | cnf::{CnfFormula, ExtendFormula} |  |  |  |  |

| `lit::{Lit, Var}` | use | lit::{Lit, Var} |  |  |  |  |

| `solver::{Solver, SolverError}` | use | solver::{Solver, SolverError} |  |  |  |  |


### crates/ferroplan-sat/src/lit.rs

| `code` | function | code(self) -> usize |  |  |  |  |

| `from_code` | function | from_code(code: usize) -> Lit |  |  |  |  |

| `from_dimacs` | function | from_dimacs(number: isize) -> Var |  |  |  |  |

| `from_index` | function | from_index(index: usize) -> Var |  |  |  |  |

| `from_var` | function | from_var(var: Var, polarity: bool) -> Lit |  |  |  |  |

| `index` | function | index(self) -> usize |  |  |  |  |

| `is_negative` | function | is_negative(self) -> bool |  |  |  |  |

| `is_positive` | function | is_positive(self) -> bool |  |  |  |  |

| `lit` | function | lit(self, polarity: bool) -> Lit |  |  |  |  |

| `map_var` | function | map_var(self, f: impl FnOnce(Var) -> Var) -> Lit |  |  |  |  |

| `max_count` | function | max_count() -> usize |  |  |  |  |

| `max_var` | function | max_var() -> Var |  |  |  |  |

| `negative` | function | negative(self) -> Lit |  |  |  |  |

| `positive` | function | positive(self) -> Lit |  |  |  |  |

| `to_dimacs` | function | to_dimacs(self) -> isize |  |  |  |  |

| `var` | function | var(self) -> Var |  |  |  |  |

| `Lit` | struct | Lit { code: LitIdx } |  |  |  |  |

| `Var` | struct | Var { index: LitIdx } |  |  |  |  |


### crates/ferroplan-sat/src/load.rs

| `load_clause` | function | load_clause(ctx: &mut Context, user_lits: &[Lit]) |  |  |  |  |


### crates/ferroplan-sat/src/model.rs

| `assignment` | function | assignment(&self) -> &[Option<bool>] |  |  |  |  |

| `reconstruct_global_model` | function | reconstruct_global_model(ctx: &mut Context) |  |  |  |  |

| `Model` | struct | Model { assignment: Vec<Option<bool>> } |  |  |  |  |


### crates/ferroplan-sat/src/prop.rs

| `propagate` | function | propagate(ctx: &mut Context) -> Result<(), Conflict> |  |  |  |  |

| `assignment::{backtrack, enqueue_assignment, full_restart, restart, Assignment, Trail}` | use | assignment::{backtrack, enqueue_assignment, full_restart, restart, Assignment, Trail} |  |  |  |  |

| `graph::{Conflict, ImplGraph, Reason}` | use | graph::{Conflict, ImplGraph, Reason} |  |  |  |  |

| `watch::{enable_watchlists, Watch, Watchlists}` | use | watch::{enable_watchlists, Watch, Watchlists} |  |  |  |  |


### crates/ferroplan-sat/src/prop/assignment.rs

| `assign_lit` | function | assign_lit(&mut self, lit: Lit) |  |  |  |  |

| `assignment` | function | assignment(&self) -> &[Option<bool>] |  |  |  |  |

| `backtrack` | function | backtrack(ctx: &mut Context, level: usize) |  |  |  |  |

| `clear` | function | clear(&mut self) |  |  |  |  |

| `current_level` | function | current_level(&self) -> usize |  |  |  |  |

| `enqueue_assignment` | function | enqueue_assignment( assignment: &mut Assignment, impl_graph: &mut ImplGraph, trail: &mut Trail, lit: Lit, reason: Reason, ) |  |  |  |  |

| `fast_option_eq` | function | fast_option_eq(a: Option<bool>, b: Option<bool>) -> bool |  |  |  |  |

| `full_restart` | function | full_restart(ctx: &mut Context) |  |  |  |  |

| `last_var_value` | function | last_var_value(&self, var: Var) -> bool |  |  |  |  |

| `lit_is_false` | function | lit_is_false(&self, lit: Lit) -> bool |  |  |  |  |

| `lit_is_true` | function | lit_is_true(&self, lit: Lit) -> bool |  |  |  |  |

| `lit_is_unk` | function | lit_is_unk(&self, lit: Lit) -> bool |  |  |  |  |

| `lit_value` | function | lit_value(&self, lit: Lit) -> Option<bool> |  |  |  |  |

| `new_decision_level` | function | new_decision_level(&mut self) |  |  |  |  |

| `pop_queue` | function | pop_queue(&mut self) -> Option<Lit> |  |  |  |  |

| `queue_head` | function | queue_head(&self) -> Option<Lit> |  |  |  |  |

| `restart` | function | restart(ctx: &mut Context) |  |  |  |  |

| `set_var` | function | set_var(&mut self, var: Var, assignment: Option<bool>) |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `trail` | function | trail(&self) -> &[Lit] |  |  |  |  |

| `unassign_var` | function | unassign_var(&mut self, var: Var) |  |  |  |  |

| `var_value` | function | var_value(&self, var: Var) -> Option<bool> |  |  |  |  |

| `Assignment` | struct | Assignment { assignment: Vec<Option<bool>>, last_value: Vec<bool> } |  |  |  |  |

| `Trail` | struct | Trail { trail: Vec<Lit>, queue_head_pos: usize, decisions: Vec<LitIdx>, units_removed: usize } |  |  |  |  |


### crates/ferroplan-sat/src/prop/binary.rs

| `propagate_binary` | function | propagate_binary(ctx: &mut Context, lit: Lit) -> Result<(), Conflict> |  |  |  |  |


### crates/ferroplan-sat/src/prop/graph.rs

| `Conflict` | enum | Conflict { Binary([Lit; 2]), Long(ClauseRef) } |  |  |  |  |

| `Reason` | enum | Reason { Unit, Binary([Lit; 1]), Long(ClauseRef) } |  |  |  |  |

| `is_removed_unit` | function | is_removed_unit(&self, var: Var) -> bool |  |  |  |  |

| `is_unit` | function | is_unit(&self) -> bool |  |  |  |  |

| `level` | function | level(&self, var: Var) -> usize |  |  |  |  |

| `lits` | function | lits(&'a self, alloc: &'a ClauseAlloc) -> &'a [Lit] |  |  |  |  |

| `reason` | function | reason(&self, var: Var) -> &Reason |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `update_reason` | function | update_reason(&mut self, var: Var, reason: Reason) |  |  |  |  |

| `update_removed_unit` | function | update_removed_unit(&mut self, var: Var) |  |  |  |  |

| `ImplGraph` | struct | ImplGraph { pub nodes: Vec<ImplNode> } |  |  |  |  |

| `ImplNode` | struct | ImplNode { pub reason: Reason, pub level: LitIdx, pub depth: LitIdx } |  |  |  |  |


### crates/ferroplan-sat/src/prop/long.rs

| `propagate_long` | function | propagate_long(ctx: &mut Context, lit: Lit) -> Result<(), Conflict> |  |  |  |  |


### crates/ferroplan-sat/src/prop/watch.rs

| `add_watch` | function | add_watch(&mut self, lit: Lit, watch: Watch) |  |  |  |  |

| `disable` | function | disable(&mut self) |  |  |  |  |

| `enable_watchlists` | function | enable_watchlists(ctx: &mut Context) |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `watch_clause` | function | watch_clause(&mut self, cref: ClauseRef, lits: [Lit; 2]) |  |  |  |  |

| `watched_by_mut` | function | watched_by_mut(&mut self, lit: Lit) -> &mut Vec<Watch> |  |  |  |  |

| `Watch` | struct | Watch { pub cref: ClauseRef, pub blocking: Lit } |  |  |  |  |

| `Watchlists` | struct | Watchlists { watches: Vec<Vec<Watch>>, enabled: bool } |  |  |  |  |


### crates/ferroplan-sat/src/schedule.rs

| `schedule_step` | function | schedule_step(ctx: &mut Context) -> bool |  |  |  |  |

| `Schedule` | struct | Schedule { conflicts: u64, next_restart: u64, restarts: u64, luby: LubySequence, pub conflict_limit: Option<u64>, pub conflicts_this_solve: u64 } |  |  |  |  |


### crates/ferroplan-sat/src/schedule/luby.rs

| `advance` | function | advance(&mut self) -> u64 |  |  |  |  |

| `LubySequence` | struct | LubySequence { u: u64, v: u64 } |  |  |  |  |


### crates/ferroplan-sat/src/solver.rs

| `SolverError` | enum | SolverError { Interrupted } |  |  |  |  |

| `add_formula` | function | add_formula(&mut self, formula: &CnfFormula) |  |  |  |  |

| `assume` | function | assume(&mut self, assumptions: &[Lit]) |  |  |  |  |

| `failed_core` | function | failed_core(&self) -> Option<&[Lit]> |  |  |  |  |

| `is_recoverable` | function | is_recoverable(&self) -> bool |  |  |  |  |

| `model` | function | model(&self) -> Option<Vec<Lit>> |  |  |  |  |

| `new` | function | new() -> Solver |  |  |  |  |

| `seed_activity` | function | seed_activity(&mut self, seeds: &[(usize, f32)]) |  |  |  |  |

| `set_conflict_limit` | function | set_conflict_limit(&mut self, limit: Option<u64>) |  |  |  |  |

| `solve` | function | solve(&mut self) -> Result<bool, SolverError> |  |  |  |  |

| `Solver` | struct | Solver { ctx: Box<Context> } |  |  |  |  |


### crates/ferroplan-sat/src/state.rs

| `SatState` | enum | SatState { Unknown, Sat, Unsat, UnsatUnderAssumptions } |  |  |  |  |

| `SolverState` | struct | SolverState { pub sat_state: SatState } |  |  |  |  |


### crates/ferroplan-sat/src/tmp.rs

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `TmpData` | struct | TmpData { pub lits: Vec<Lit>, pub lits_2: Vec<Lit> } |  |  |  |  |

| `TmpFlags` | struct | TmpFlags { pub flags: Vec<bool> } |  |  |  |  |


### crates/ferroplan-sat/src/unit_simplify.rs

| `prove_units` | function | prove_units(ctx: &mut Context) -> bool |  |  |  |  |

| `resurrect_unit` | function | resurrect_unit(ctx: &mut Context, lit: Lit) |  |  |  |  |

| `unit_simplify` | function | unit_simplify(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/variables.rs

| `existing_user_from_solver` | function | existing_user_from_solver(&self, solver: Var) -> Var |  |  |  |  |

| `global_from_solver` | function | global_from_solver(&self) -> &VarMap |  |  |  |  |

| `global_from_solver_mut` | function | global_from_solver_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `global_from_user` | function | global_from_user(&self) -> &VarMap |  |  |  |  |

| `global_from_user_mut` | function | global_from_user_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `global_var_iter` | function | global_var_iter(&self) -> impl Iterator<Item = Var> + '_ |  |  |  |  |

| `global_watermark` | function | global_watermark(&self) -> usize |  |  |  |  |

| `initialize_solver_var` | function | initialize_solver_var(ctx: &mut Context, solver: Var, global: Var) |  |  |  |  |

| `new_user_var` | function | new_user_var(ctx: &mut Context) -> Var |  |  |  |  |

| `next_unmapped_solver` | function | next_unmapped_solver(&self) -> Var |  |  |  |  |

| `next_unmapped_user` | function | next_unmapped_user(&self) -> Var |  |  |  |  |

| `remove_solver_var` | function | remove_solver_var(ctx: &mut Context, solver: Var) |  |  |  |  |

| `solver_from_global` | function | solver_from_global(&self) -> &VarMap |  |  |  |  |

| `solver_from_global_mut` | function | solver_from_global_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `solver_from_user` | function | solver_from_user(ctx: &mut Context, user: Var) -> Var |  |  |  |  |

| `solver_from_user_lits` | function | solver_from_user_lits(ctx: &mut Context, solver_lits: &mut Vec<Lit>, user_lits: &[Lit]) |  |  |  |  |

| `solver_var_present` | function | solver_var_present(&self, solver: Var) -> bool |  |  |  |  |

| `solver_watermark` | function | solver_watermark(&self) -> usize |  |  |  |  |

| `user_from_global` | function | user_from_global(&self) -> &VarMap |  |  |  |  |

| `user_var_iter` | function | user_var_iter(&self) -> impl Iterator<Item = Var> + '_ |  |  |  |  |

| `user_watermark` | function | user_watermark(&self) -> usize |  |  |  |  |

| `var_data_global` | function | var_data_global(&self, global: Var) -> &VarData |  |  |  |  |

| `var_data_global_mut` | function | var_data_global_mut(&mut self, global: Var) -> &mut VarData |  |  |  |  |

| `var_data_solver_mut` | function | var_data_solver_mut(&mut self, solver: Var) -> &mut VarData |  |  |  |  |

| `Variables` | struct | Variables { global_from_user: VarBiMap, solver_from_global: VarBiMap, solver_freelist: HashSet<Var>, var_data: Vec<VarData> } |  |  |  |  |


### crates/ferroplan-sat/src/variables/data.rs

| `user_default` | function | user_default() -> VarData |  |  |  |  |

| `VarData` | struct | VarData { pub unit: Option<bool>, pub isolated: bool, pub assumed: bool, pub deleted: bool } |  |  |  |  |


### crates/ferroplan-sat/src/variables/var_map.rs

| `bwd` | function | bwd(&self) -> &VarMap |  |  |  |  |

| `bwd_mut` | function | bwd_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `fwd` | function | fwd(&self) -> &VarMap |  |  |  |  |

| `fwd_mut` | function | fwd_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `get` | function | get(&self, from: Var) -> Option<Var> |  |  |  |  |

| `insert` | function | insert(&mut self, into: Var, from: Var) |  |  |  |  |

| `remove` | function | remove(&mut self, from: Var) |  |  |  |  |

| `watermark` | function | watermark(&self) -> usize |  |  |  |  |

| `VarBiMap` | struct | VarBiMap { fwd: VarMap, bwd: VarMap } |  |  |  |  |

| `VarBiMapMut` | struct | VarBiMapMut { fwd: &'a mut VarMap, bwd: &'a mut VarMap } |  |  |  |  |

| `VarMap` | struct | VarMap { mapping: Vec<LitIdx> } |  |  |  |  |


### crates/ferroplan-wasm/src/dfcm_route.rs

| `CORRIDOR_DOMAIN` | const | CORRIDOR_DOMAIN: &str |  |  |  |  |

| `MAX_PROBE_CANDIDATES` | const | MAX_PROBE_CANDIDATES: usize |  |  |  |  |

| `MAX_PROBE_FACT_BYTES` | const | MAX_PROBE_FACT_BYTES: usize |  |  |  |  |

| `MAX_PROBE_OBSERVATIONS` | const | MAX_PROBE_OBSERVATIONS: usize |  |  |  |  |

| `admit_candidates` | function | admit_candidates( candidates: &[ProbeCandidate], goal_limit: usize, surface: &str, ) -> Result<(), Refusal> |  |  |  |  |

| `corridor_problem` | function | corridor_problem(n: usize) -> String |  |  |  |  |

| `probe_all` | function | probe_all( parent: &Session, candidates: &[ProbeCandidate], evals: usize, mem_mb: usize, ) -> Vec<Value> |  |  |  |  |

| `probe_candidate` | function | probe_candidate( parent: &Session, candidate: &ProbeCandidate, evals: usize, mem_mb: usize, ) -> Value |  |  |  |  |

| `repair` | function | repair( inner: &Session, plan: &mut Option<Plan>, cursor: &mut usize, evals: usize, mem_mb: usize, ) -> Value |  |  |  |  |

| `ProbeCandidate` | struct | ProbeCandidate { pub id: String, pub goal: Option<String>, pub sight: Vec<(String, bool)>, pub restrict_contains: Option<String> } |  |  |  |  |


### crates/ferroplan-wasm/src/lib.rs

| `advance` | function | advance(&mut self) |  |  |  |  |

| `apply_start` | function | apply_start(&mut self, name: &str) -> Result<(), JsValue> |  |  |  |  |

| `drop_plan` | function | drop_plan(&mut self) |  |  |  |  |

| `elapse` | function | elapse(&mut self, dt: f64) -> Result<String, JsValue> |  |  |  |  |

| `explain` | function | explain(domain: &str, problem: &str, plan_json: &str) -> String |  |  |  |  |

| `fact` | function | fact(&self, name: &str) -> JsValue |  |  |  |  |

| `fluent` | function | fluent(&self, name: &str) -> JsValue |  |  |  |  |

| `fond_validate` | function | fond_validate(problem_json: &str, plan_json: &str) -> String |  |  |  |  |

| `fork` | function | fork(&self) -> WasmSession |  |  |  |  |

| `goal_met` | function | goal_met(&self) -> bool |  |  |  |  |

| `has_plan` | function | has_plan(&self) -> bool |  |  |  |  |

| `mind_bytes` | function | mind_bytes(&self) -> usize |  |  |  |  |

| `new` | function | new(domain: &str, problem: &str) -> Result<WasmSession, JsValue> |  |  |  |  |

| `observe` | function | observe(&mut self, sight_json: &str) -> Result<String, JsValue> |  |  |  |  |

| `plan` | function | plan( domain: &str, problem: &str, mode: Option<String>, flags: Option<String>, search: Option<String>, ) -> String |  |  |  |  |

| `plan_production` | function | plan_production( domain: &str, problem: &str, mode: Option<String>, search: Option<String>, max_evaluated: Option<usize>, max_plan_steps: Option<usize>, max_output_bytes: Option<usize>, request_id: Option<String>, ) -> String |  |  |  |  |

| `plan_valid_json` | function | plan_valid_json(&self, plan_json: &str, from: usize) -> bool |  |  |  |  |

| `probe_json` | function | probe_json(&self, candidates_json: &str, evals: usize, mem_mb: usize) -> String |  |  |  |  |

| `readiness` | function | readiness() -> String |  |  |  |  |

| `repair` | function | repair(&mut self, evals: usize, mem_mb: usize) -> String |  |  |  |  |

| `replan_following` | function | replan_following(&mut self, evals: usize, mem_mb: usize) -> String |  |  |  |  |

| `restrict_contains` | function | restrict_contains(&mut self, filter: String) |  |  |  |  |

| `restrict_prefix_claims` | function | restrict_prefix_claims(&mut self, prefix: String, claimed: String) |  |  |  |  |

| `set_fact` | function | set_fact(&mut self, name: &str, value: bool) -> Result<(), JsValue> |  |  |  |  |

| `set_fluent` | function | set_fluent(&mut self, name: &str, value: f64) -> Result<(), JsValue> |  |  |  |  |

| `set_goal` | function | set_goal(&mut self, goal: &str) -> Result<(), JsValue> |  |  |  |  |

| `set_timed_fact` | function | set_timed_fact(&mut self, dt: f64, name: &str, value: bool) -> Result<(), JsValue> |  |  |  |  |

| `step_json` | function | step_json(&self) -> String |  |  |  |  |

| `suffix_json` | function | suffix_json(&self) -> String |  |  |  |  |

| `think` | function | think(&mut self, evals: usize, mem_mb: usize) -> String |  |  |  |  |

| `valid` | function | valid(&self) -> bool |  |  |  |  |

| `version` | function | version() -> String |  |  |  |  |

| `world_bytes` | function | world_bytes(&self) -> usize |  |  |  |  |

| `WasmSession` | struct | WasmSession { inner: ferroplan::Session, plan: Option<ferroplan::api::Plan>, cursor: usize } |  |  |  |  |

| `browser_impl::{ explain, fond_validate, plan, plan_production, readiness, version, WasmSession, }` | use | browser_impl::{ explain, fond_validate, plan, plan_production, readiness, version, WasmSession, } |  |  |  |  |


### crates/ferroplan-wasm/src/probe_guard.rs

| `MAX_PROBE_ID_BYTES` | const | MAX_PROBE_ID_BYTES: usize |  |  |  |  |

| `contradictory_fact` | function | contradictory_fact(sight: &[(String, bool)]) -> Option<&str> |  |  |  |  |

| `duplicate_id` | function | duplicate_id(ids: impl IntoIterator<Item = &'a str>) -> Option<&'a str> |  |  |  |  |

| `fact_key` | function | fact_key(fact: &str) -> String |  |  |  |  |

| `malformed_id` | function | malformed_id(ids: impl IntoIterator<Item = &'a str>) -> Option<&'a str> |  |  |  |  |


### crates/ferroplan-wasm/src/wasi_abi.rs

| `fp_alloc` | function | fp_alloc(len: usize) -> *mut u8 |  |  |  |  |

| `fp_call` | function | fp_call(ptr: *mut u8, len: usize) -> u64 |  |  |  |  |

| `fp_dealloc` | function | fp_dealloc(ptr: *mut u8, len: usize) |  |  |  |  |

| `(define (domain rooms)
      (:requirements :strips :typing)
      (:types room)
      (:predicates (at ?r - room) (link ?a - room ?b - room))
      (:action go
        :parameters (?a - room ?b - room)
        :precondition (and (at ?a) (link ?a ?b))
        :effect (and (at ?b) (not (at ?a)))))` | str_key | CORRIDOR_DOMAIN = "(define (domain rooms)
      (:requirements :strips :typing)
      (:types room)
      (:predicates (at ?r - room) (link ?a - room ?b - room))
      (:action go
        :parameters (?a - room ?b - room)
        :precondition (and (at ?a) (link ?a ?b))
        :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (problem corridor)
      (:domain rooms)
      (:objects a b c d - room)
      (:init (at a) (link a b) (link b c) (link c d))
      (:goal (at d)))` | str_key | CORRIDOR_PROBLEM = "(define (problem corridor)
      (:domain rooms)
      (:objects a b c d - room)
      (:init (at a) (link a b) (link b c) (link c d))
      (:goal (at d)))" |  |  |  |  |

| `(define (problem dead-end)
      (:domain rooms)
      (:objects a b c d - room)
      (:init (at a) (link a b) (link b c))
      (:goal (at d)))` | str_key | CORRIDOR_DEAD_END_PROBLEM = "(define (problem dead-end)
      (:domain rooms)
      (:objects a b c d - room)
      (:init (at a) (link a b) (link b c))
      (:goal (at d)))" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../../ferroplan-hddl/fixtures/c/domain.hddl" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/c/problem.hddl` | str_key | FIXTURE_C_PROBLEM = "../../ferroplan-hddl/fixtures/c/problem.hddl" |  |  |  |  |


### crates/ferroplan-wasm/tests/abi_ontology_drift.rs

| `../../ontology/ferroplan-wasm.ttl` | str_key | ONTOLOGY_REL = "../../ontology/ferroplan-wasm.ttl" |  |  |  |  |

| `fp_alloc` | str_key | EXPECTED_EXPORTS = "fp_alloc" |  |  |  |  |

| `fp_dealloc` | str_key | FREE_SYMBOL = "fp_dealloc" |  |  |  |  |

| `ontology/contract.ttl` | str_key | CONTRACTS_REL = "ontology/contract.ttl" |  |  |  |  |

| `registry/capability-registry.json` | str_key | REGISTRY_REL = "registry/capability-registry.json" |  |  |  |  |

| `src/wasi_abi.rs` | str_key | WASI_ABI_REL = "src/wasi_abi.rs" |  |  |  |  |


### crates/ferroplan-wasm/tests/browser.rs

| `(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))` | str_key | DOMAIN = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (problem two-room)
  (:domain rooms)
  (:objects a b - room)
  (:init (at a) (link a b))
  (:goal (at b)))` | str_key | PROBLEM = "(define (problem two-room)
  (:domain rooms)
  (:objects a b - room)
  (:init (at a) (link a b))
  (:goal (at b)))" |  |  |  |  |


### crates/ferroplan-wasm/tests/copy_drift.rs

| `../../../ggen-marketplace/packs/qri-qualification-profile-pack` | str_key | PACK_REL = "../../../ggen-marketplace/packs/qri-qualification-profile-pack" |  |  |  |  |


### crates/ferroplan-wasm/tests/dfcm_browser_parity.rs

| `(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))` | str_key | DOMAIN = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (problem repair)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c b))
  (:goal (at b)))` | str_key | PROBLEM = "(define (problem repair)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c b))
  (:goal (at b)))" |  |  |  |  |


### crates/ferroplan-wasm/tests/pin_drift.rs

| `# c` | str_key | REG = "# c" |  |  |  |  |

| `x` | str_key | ONT = "x" |  |  |  |  |


### crates/ferroplan/benches/fond.rs

| `drop-retry` | str_key | DIRS = "drop-retry" |  |  |  |  |


### crates/ferroplan/benches/session_fork.rs

| `(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))` | str_key | DOMAIN = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |


### crates/ferroplan/examples/bazaar_live.rs

| `../../../benchmarks/bench/bazaar-chain-domain.pddl` | str_key | DOM = "../../../benchmarks/bench/bazaar-chain-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain-x2m.pddl` | str_key | PRB_X2M = "../../../benchmarks/bench/bazaar-chain-x2m.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain.pddl` | str_key | PRB = "../../../benchmarks/bench/bazaar-chain.pddl" |  |  |  |  |


### crates/ferroplan/examples/bazaar_thinks.rs

| `../../../benchmarks/bench/bazaar-chain-domain.pddl` | str_key | DOM = "../../../benchmarks/bench/bazaar-chain-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain-x2.pddl` | str_key | PRB_X2 = "../../../benchmarks/bench/bazaar-chain-x2.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain.pddl` | str_key | PRB = "../../../benchmarks/bench/bazaar-chain.pddl" |  |  |  |  |


### crates/ferroplan/examples/game_think.rs

| `
(define (domain homestead) (:requirements :strips :typing :numeric-fluents)
  (:types agent place)
  (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
  (:functions (grain))
  (:action walk :parameters (?a - agent ?from ?to - place)
    :precondition (and (at ?a ?from) (road ?from ?to))
    :effect (and (not (at ?a ?from)) (at ?a ?to)))
  (:action harvest :parameters (?a - agent ?p - place)
    :precondition (and (at ?a ?p) (fertile ?p))
    :effect (increase (grain) 1)))` | str_key | DOM = "
(define (domain homestead) (:requirements :strips :typing :numeric-fluents)
  (:types agent place)
  (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
  (:functions (grain))
  (:action walk :parameters (?a - agent ?from ?to - place)
    :precondition (and (at ?a ?from) (road ?from ?to))
    :effect (and (not (at ?a ?from)) (at ?a ?to)))
  (:action harvest :parameters (?a - agent ?p - place)
    :precondition (and (at ?a ?p) (fertile ?p))
    :effect (increase (grain) 1)))" |  |  |  |  |

| `
(define (problem morning) (:domain homestead)
  (:objects vera - agent hut field barn - place)
  (:init (at vera hut) (road hut field) (road field hut)
         (road field barn) (road barn field) (fertile field) (= (grain) 0))
  (:goal (>= (grain) 3)))` | str_key | PRB = "
(define (problem morning) (:domain homestead)
  (:objects vera - agent hut field barn - place)
  (:init (at vera hut) (road hut field) (road field hut)
         (road field barn) (road barn field) (fertile field) (= (grain) 0))
  (:goal (>= (grain) 3)))" |  |  |  |  |


### crates/ferroplan/examples/json_api.rs

| `(define (domain gripper)
 (:requirements :strips :typing)
 (:types room ball gripper)
 (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
              (free ?g - gripper) (carry ?b - ball ?g - gripper))
 (:action move :parameters (?from ?to - room)
   :precondition (at-robby ?from) :effect (and (at-robby ?to) (not (at-robby ?from))))
 (:action pick :parameters (?b - ball ?r - room ?g - gripper)
   :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
   :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
 (:action drop :parameters (?b - ball ?r - room ?g - gripper)
   :precondition (and (carry ?b ?g) (at-robby ?r))
   :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))` | str_key | DOMAIN = "(define (domain gripper)
 (:requirements :strips :typing)
 (:types room ball gripper)
 (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
              (free ?g - gripper) (carry ?b - ball ?g - gripper))
 (:action move :parameters (?from ?to - room)
   :precondition (at-robby ?from) :effect (and (at-robby ?to) (not (at-robby ?from))))
 (:action pick :parameters (?b - ball ?r - room ?g - gripper)
   :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
   :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
 (:action drop :parameters (?b - ball ?r - room ?g - gripper)
   :precondition (and (carry ?b ?g) (at-robby ?r))
   :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))" |  |  |  |  |

| `(define (problem g1) (:domain gripper)
 (:objects rooma roomb - room  ball1 ball2 - ball  left right - gripper)
 (:init (at-robby rooma) (free left) (free right)
        (at ball1 rooma) (at ball2 rooma))
 (:goal (and (at ball1 roomb) (at ball2 roomb))))` | str_key | PROBLEM = "(define (problem g1) (:domain gripper)
 (:objects rooma roomb - room  ball1 ball2 - ball  left right - gripper)
 (:init (at-robby rooma) (free left) (free right)
        (at ball1 rooma) (at ball2 rooma))
 (:goal (and (at ball1 roomb) (at ball2 roomb))))" |  |  |  |  |


### crates/ferroplan/examples/many_minds.rs

| `../../../benchmarks/bench/bazaar-redistribution.pddl` | str_key | PRB = "../../../benchmarks/bench/bazaar-redistribution.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar.pddl` | str_key | DOM = "../../../benchmarks/bench/bazaar.pddl" |  |  |  |  |


### crates/ferroplan/examples/validate_plan.rs

| `(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move :parameters (?from ?to - room)
    :precondition (at-robby ?from)
    :effect (and (not (at-robby ?from)) (at-robby ?to)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))` | str_key | DOMAIN = "(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move :parameters (?from ?to - room)
    :precondition (at-robby ?from)
    :effect (and (not (at-robby ?from)) (at-robby ?to)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))" |  |  |  |  |

| `(define (problem gripper-1) (:domain gripper)
  (:objects rooma roomb - room b1 - ball left - gripper)
  (:init (at-robby rooma) (at b1 rooma) (free left))
  (:goal (at b1 roomb)))` | str_key | PROBLEM = "(define (problem gripper-1) (:domain gripper)
  (:objects rooma roomb - room b1 - ball left - gripper)
  (:init (at-robby rooma) (at b1 rooma) (free left))
  (:goal (at b1 roomb)))" |  |  |  |  |


### crates/ferroplan/examples/village.rs

| `THINK_EVALS` | env_key | std::env::var("THINK_EVALS") |  |  |  |  |

| `../../../benchmarks/village/domain.pddl` | str_key | DOM = "../../../benchmarks/village/domain.pddl" |  |  |  |  |

| `../../../benchmarks/village/pair.pddl` | str_key | PRB = "../../../benchmarks/village/pair.pddl" |  |  |  |  |


### crates/ferroplan/examples/village_live.rs

| `../../../benchmarks/village/domain.pddl` | str_key | DOM = "../../../benchmarks/village/domain.pddl" |  |  |  |  |

| `../../../benchmarks/village/pair.pddl` | str_key | PRB = "../../../benchmarks/village/pair.pddl" |  |  |  |  |


### crates/ferroplan/src/api.rs

| `Mode` | enum | Mode { Auto, Ff, Partition, Pddl3, Temporal, Portfolio, Optimal, Sat } |  |  |  |  |

| `Search` | enum | Search { Auto, Ehc, BestFirst, EhcThenBestFirst } |  |  |  |  |

| `SolveError` | enum | SolveError { DomainParse(crate::types::ParseError), ProblemParse(crate::types::ParseError), EmptyType { kind: String, pred: String, ty: String, }, Derived(String), Unsupported(String) } |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_SAT_CLASSICAL` | env_key | std::env::var("FF_SAT_CLASSICAL") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `decompose` | function | decompose( domain_src: &str, problem_src: &str, opts: &Options, ) -> Result<Decomposition, SolveError> |  |  |  |  |

| `parse` | function | parse(src: &str) -> ParseReport |  |  |  |  |

| `solve` | function | solve(domain_src: &str, problem_src: &str, opts: &Options) -> Result<Solution, SolveError> |  |  |  |  |

| `Contract` | struct | Contract { pub index: usize, pub goal: String, pub steps: Vec<Step>, pub makespan: f64, pub offset: f64 } |  |  |  |  |

| `Decomposition` | struct | Decomposition { pub solved: bool, pub contracts: Vec<Contract>, pub plan: Option<Plan>, pub monolithic: bool, pub notes: Vec<String> } |  |  |  |  |

| `DomainSummary` | struct | DomainSummary { pub types: Vec<String>, pub predicates: Vec<String>, pub functions: Vec<String>, pub actions: Vec<String>, pub durative_actions: Vec<String>, pub derived: usize } |  |  |  |  |

| `Options` | struct | Options { pub mode: Mode, pub search: Search, pub helpful_actions: bool, pub weight_g: f64, pub weight_h: f64, pub threads: usize, pub max_evaluated: Option<usize>, pub optimize: bool, pub wall_ms: Option<u64>, pub should_continue: Option<std::sync::Arc<std::sync::atomic::AtomicBool>> } |  |  |  |  |

| `ParseReport` | struct | ParseReport { pub ok: bool, pub kind: Option<String>, pub name: Option<String>, pub requirements: Vec<String>, pub error: Option<String>, pub domain: Option<DomainSummary>, pub problem: Option<ProblemSummary> } |  |  |  |  |

| `Plan` | struct | Plan { pub steps: Vec<Step>, pub length: usize, pub metric: Option<f64>, pub makespan: Option<f64> } |  |  |  |  |

| `ProblemSummary` | struct | ProblemSummary { pub domain: String, pub objects: usize, pub init_facts: usize, pub init_fluents: usize, pub timed_initial_literals: usize, pub has_goal: bool, pub has_metric: bool } |  |  |  |  |

| `Solution` | struct | Solution { pub solved: bool, pub mode: Mode, pub plan: Option<Plan>, pub statistics: Statistics, pub notes: Vec<String> } |  |  |  |  |

| `Statistics` | struct | Statistics { pub grounded_facts: usize, pub grounded_actions: usize, pub evaluated_states: usize, pub threads: usize } |  |  |  |  |

| `Step` | struct | Step { pub index: usize, pub action: String, pub args: Vec<String>, pub time: Option<f64>, pub duration: Option<f64> } |  |  |  |  |


### crates/ferroplan/src/bitset.rs

| `clear` | function | clear(w: &mut [u64], i: usize) |  |  |  |  |

| `count` | function | count(w: &[u64]) -> usize |  |  |  |  |

| `set` | function | set(w: &mut [u64], i: usize) |  |  |  |  |

| `test` | function | test(w: &[u64], i: usize) -> bool |  |  |  |  |

| `words_for` | function | words_for(n_bits: usize) -> usize |  |  |  |  |


### crates/ferroplan/src/clock.rs

| `elapsed_ms` | function | elapsed_ms(&self) -> u128 |  |  |  |  |

| `elapsed_secs` | function | elapsed_secs(&self) -> f64 |  |  |  |  |

| `elapsed_us` | function | elapsed_us(&self) -> u128 |  |  |  |  |

| `now` | function | now() -> Self |  |  |  |  |

| `Clock` | struct | Clock { t0: std::time::Instant } |  |  |  |  |


### crates/ferroplan/src/constraints.rs

| `END_ACTION` | const | END_ACTION: &str |  |  |  |  |

| `Traj` | enum | Traj { Always(Formula), Sometime(Formula), AtMostOnce(Formula), SometimeAfter(Formula, Formula), SometimeBefore(Formula, Formula), AtEnd(Formula), Within(f64, Formula), AlwaysWithin(f64, Formula, Formula) } |  |  |  |  |

| `FF_CONSTRAINTS_REJECT` | env_key | std::env::var("FF_CONSTRAINTS_REJECT") |  |  |  |  |

| `FF_NO_COND_SHARE` | env_key | std::env::var("FF_NO_COND_SHARE") |  |  |  |  |

| `FF_NO_TRAJ_END` | env_key | std::env::var("FF_NO_TRAJ_END") |  |  |  |  |

| `FF_PREF_NO_STATIC` | env_key | std::env::var("FF_PREF_NO_STATIC") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `accepted` | function | accepted(&self) -> bool |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> |  |  |  |  |

| `expand` | function | expand(domain: &Domain, problem: &Problem) -> Result<Expanded, String> |  |  |  |  |

| `gate` | function | gate(domain: &Domain, problem: &Problem) -> Result<Option<(Domain, Problem)>, String> |  |  |  |  |

| `hard_only_gated` | function | hard_only_gated( domain: &Domain, problem: &Problem, ) -> Result<Option<(Domain, Problem)>, String> |  |  |  |  |

| `new` | function | new(traj: &'a Traj) -> Self |  |  |  |  |

| `op_name` | function | op_name(&self) -> &'static str |  |  |  |  |

| `step` | function | step(&mut self, holds: &mut dyn FnMut(&Formula) -> bool) |  |  |  |  |

| `step_at` | function | step_at(&mut self, time: f64, holds: &mut dyn FnMut(&Formula) -> bool) |  |  |  |  |

| `TRAJ-CLOCK` | str_key | CLOCK_FLUENT = "TRAJ-CLOCK" |  |  |  |  |

| `TRAJ-END` | str_key | END_ACTION = "TRAJ-END" |  |  |  |  |

| `Expanded` | struct | Expanded { pub hard: Vec<Traj>, pub soft: Vec<(String, Vec<Traj>)> } |  |  |  |  |

| `Fold` | struct | Fold { traj: &'a Traj, ok: bool, seen: bool, holding: bool, pending: bool, safe: bool, last: bool, due: f64 } |  |  |  |  |


### crates/ferroplan/src/costs.rs

| `FF_COST_SWEEP_EVALS` | env_key | std::env::var("FF_COST_SWEEP_EVALS") |  |  |  |  |

| `FF_LEN_SWEEP_EVALS` | env_key | std::env::var("FF_LEN_SWEEP_EVALS") |  |  |  |  |

| `improve` | function | improve( task: &PackedTask, cf: usize, ops: Vec<usize>, first_cost: f64, threads: usize, base: SearchCfg, spent: usize, orbit: Option<&crate::orbits::OrbitMap>, ) -> CostOutcome |  |  |  |  |

| `improve_length` | function | improve_length( task: &PackedTask, ops: Vec<usize>, threads: usize, base: SearchCfg, spent: usize, ) -> (Vec<usize>, usize, bool) |  |  |  |  |

| `metric_fluent` | function | metric_fluent(problem: &Problem) -> Option<String> |  |  |  |  |

| `optimize_text` | function | optimize_text( problem: &Problem, task: &PackedTask, optimize: bool, threads: usize, cfg: SearchCfg, ops: &mut Vec<usize>, orbit: Option<&crate::orbits::OrbitMap>, ) -> Option<(f64, &'static str)> |  |  |  |  |

| `plan_cost` | function | plan_cost(task: &PackedTask, cf: usize, ops: &[usize]) -> Option<f64> |  |  |  |  |

| `CostOutcome` | struct | CostOutcome { pub ops: Vec<usize>, pub cost: f64, pub improved: bool, pub proven: bool, pub evaluated: usize } |  |  |  |  |


### crates/ferroplan/src/derived.rs

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> |  |  |  |  |

| `(define (domain g) (:requirements :typing :adl)
      (:types node)
      (:predicates (link ?a ?b - node) (reachable ?a ?b - node) (at ?n - node) (visited ?n - node))
      (:derived (reachable ?a ?b - node)
        (or (link ?a ?b)
            (exists (?c - node) (and (link ?a ?c) (reachable ?c ?b)))))
      (:action go :parameters (?from ?to - node)
        :precondition (and (at ?from) (reachable ?from ?to))
        :effect (and (not (at ?from)) (at ?to) (visited ?to))))` | str_key | DOM = "(define (domain g) (:requirements :typing :adl)
      (:types node)
      (:predicates (link ?a ?b - node) (reachable ?a ?b - node) (at ?n - node) (visited ?n - node))
      (:derived (reachable ?a ?b - node)
        (or (link ?a ?b)
            (exists (?c - node) (and (link ?a ?c) (reachable ?c ?b)))))
      (:action go :parameters (?from ?to - node)
        :precondition (and (at ?from) (reachable ?from ?to))
        :effect (and (not (at ?from)) (at ?to) (visited ?to))))" |  |  |  |  |


### crates/ferroplan/src/espc.rs

| `FF_ESPC_MONO` | env_key | std::env::var("FF_ESPC_MONO") |  |  |  |  |

| `FF_ESPC_TIME_MS` | env_key | std::env::var("FF_ESPC_TIME_MS") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `espc_optimize` | function | espc_optimize( task: &PackedTask, cost_fluent: usize, sat: &mut SatGuidance, seed: Option<(Vec<usize>, f64)>, part: Option<EspcPartition>, threads: usize, cfg: SearchCfg, ) -> Option<EspcResult> |  |  |  |  |

| `EspcPartition` | struct | EspcPartition { pub comps: Vec<Subgoal>, pub tail: PhaseTail, pub assoc: FxHashMap<u32, Vec<u32>> } |  |  |  |  |

| `EspcResult` | struct | EspcResult { pub ops: Vec<usize>, pub cost: f64, pub iterations: usize } |  |  |  |  |


### crates/ferroplan/src/eve.rs

| `MAX_PRIMARY_ACTIVATORS` | const | MAX_PRIMARY_ACTIVATORS: usize |  |  |  |  |

| `EveError` | enum | EveError { Missing { field: String }, SplitRequired { directive: SplitDirective } } |  |  |  |  |

| `EveStage` | enum | EveStage { GroundHumanPurpose, ProjectGenesis, DecomposeHddl, GovernUncertaintyPpddl, ManufactureGgen, ExposeMcpPlus, ActuateBrce, ObserveOcel2, ConformTruexKernel, AdmitReceipt, ReplayTruex } |  |  |  |  |

| `PlanningRegime` | enum | PlanningRegime { Deterministic, Probabilistic } |  |  |  |  |

| `enter` | function | enter(request: EveRequest) -> Result<EveHandoff, EveError> |  |  |  |  |

| `ferroplan.eve-genesis.v1` | str_key | EVE_PROTOCOL = "ferroplan.eve-genesis.v1" |  |  |  |  |

| `Activator` | struct | Activator { pub name: String, pub value: String } |  |  |  |  |

| `CapabilityTarget` | struct | CapabilityTarget { pub capability: String, pub route: String, pub authority_scopes: Vec<String> } |  |  |  |  |

| `Eve` | struct |  |  |  |  |  |

| `EveHandoff` | struct | EveHandoff { pub protocol: String, pub closure_id: String, pub planning_regime: PlanningRegime, pub stages: Vec<EveStage>, pub goal: GroundedGoal, pub genesis: GenesisProjection, pub hddl: HddlDecompositionRequest, pub ppddl: Option<PpddlPolicyRequest>, pub ggen: GgenManufacturingRequest, pub mcp_plus: McpPlusHandoff, pub truex: TruexContinuation } |  |  |  |  |

| `EveRequest` | struct | EveRequest { pub purpose: HumanPurpose, pub genesis: GenesisWorld, pub manufacture: ManufactureTarget, pub capability: CapabilityTarget } |  |  |  |  |

| `GenesisProjection` | struct | GenesisProjection { pub ontology_rdf: String, pub construct_query: String } |  |  |  |  |

| `GenesisWorld` | struct | GenesisWorld { pub ontology_rdf: String, pub construct_query: String, pub hddl: HddlSurface, pub ppddl: Option<PpddlSurface> } |  |  |  |  |

| `GgenManufacturingRequest` | struct | GgenManufacturingRequest { pub target: ManufactureTarget, pub closure_id: String, pub candidate_only: bool } |  |  |  |  |

| `GroundedGoal` | struct | GroundedGoal { pub statement: String, pub desired_consequence: String, pub actor: Option<String>, pub root_task: String, pub activators: Vec<Activator> } |  |  |  |  |

| `HddlDecompositionRequest` | struct | HddlDecompositionRequest { pub domain: String, pub problem: String, pub root_task: String } |  |  |  |  |

| `HddlSurface` | struct | HddlSurface { pub domain: String, pub problem: String, pub root_task: String } |  |  |  |  |

| `HumanPurpose` | struct | HumanPurpose { pub statement: String, pub desired_consequence: String, pub actor: Option<String>, pub activators: Vec<Activator> } |  |  |  |  |

| `ManufactureTarget` | struct | ManufactureTarget { pub name: String, pub template: String, pub artifact_kind: String, pub output: String } |  |  |  |  |

| `McpPlusHandoff` | struct | McpPlusHandoff { pub target: CapabilityTarget, pub closure_id: String, pub ambient_authority: bool, pub brce_required: bool, pub receipt_obligations: Vec<String> } |  |  |  |  |

| `PpddlPolicyRequest` | struct | PpddlPolicyRequest { pub domain: String, pub problem: String } |  |  |  |  |

| `PpddlSurface` | struct | PpddlSurface { pub domain: String, pub problem: String } |  |  |  |  |

| `SplitDirective` | struct | SplitDirective { pub provided: usize, pub maximum: usize, pub groups: Vec<Vec<Activator>> } |  |  |  |  |

| `TruexContinuation` | struct | TruexContinuation { pub expected_process_geometry: String, pub observed_path_format: String, pub conformance_engine: String, pub terminal_authority: String, pub replay_required: bool } |  |  |  |  |


### crates/ferroplan/src/features.rs

| `DemandMode` | enum | DemandMode { Off, Numeric, Full } |  |  |  |  |

| `FF_NO_ESCALATE` | env_key | std::env::var("FF_NO_ESCALATE") |  |  |  |  |

| `FF_NO_ESPC` | env_key | std::env::var("FF_NO_ESPC") |  |  |  |  |

| `FF_NO_TDEMAND` | env_key | std::env::var("FF_NO_TDEMAND") |  |  |  |  |

| `FF_TCONC` | env_key | std::env::var("FF_TCONC") |  |  |  |  |

| `FF_TDECOMP` | env_key | std::env::var("FF_TDECOMP") |  |  |  |  |

| `FF_TDEMAND` | env_key | std::env::var("FF_TDEMAND") |  |  |  |  |

| `clear_overrides` | function | clear_overrides() |  |  |  |  |

| `demand_mode` | function | demand_mode() -> DemandMode |  |  |  |  |

| `escalate` | function | escalate() -> bool |  |  |  |  |

| `espc` | function | espc() -> bool |  |  |  |  |

| `set_escalate_override` | function | set_escalate_override(on: bool) |  |  |  |  |

| `set_espc_override` | function | set_espc_override(on: bool) |  |  |  |  |

| `set_overrides` | function | set_overrides(tdemand: bool, tdecomp: bool, tconc: bool) |  |  |  |  |

| `tconc` | function | tconc() -> bool |  |  |  |  |

| `tdecomp` | function | tdecomp() -> bool |  |  |  |  |

| `tdemand` | function | tdemand() -> bool |  |  |  |  |


### crates/ferroplan/src/ground.rs

| `Outcome` | enum | Outcome { Task(PackedTask), GoalTrue, GoalFalse(String), GoalUndefinedFluent(String), EmptyType { kind: &'static str, pred: String, ty: String, }, WallExhausted(String) } |  |  |  |  |

| `FF_GROUND_PHASES` | env_key | std::env::var("FF_GROUND_PHASES") |  |  |  |  |

| `FF_NO_DNF_STATIC` | env_key | std::env::var("FF_NO_DNF_STATIC") |  |  |  |  |

| `FF_NO_FACT_COMPACT` | env_key | std::env::var("FF_NO_FACT_COMPACT") |  |  |  |  |

| `FF_NO_FIXPOINT_GROUND` | env_key | std::env::var("FF_NO_FIXPOINT_GROUND") |  |  |  |  |

| `FF_NO_FLUENT_COMPACT` | env_key | std::env::var("FF_NO_FLUENT_COMPACT") |  |  |  |  |

| `FF_NO_FLUENT_FOLD` | env_key | std::env::var("FF_NO_FLUENT_FOLD") |  |  |  |  |

| `FF_NO_GOAL_FACTOR` | env_key | std::env::var("FF_NO_GOAL_FACTOR") |  |  |  |  |

| `FF_NO_JOIN_INDEX` | env_key | std::env::var("FF_NO_JOIN_INDEX") |  |  |  |  |

| `FF_NO_MCV_JOIN` | env_key | std::env::var("FF_NO_MCV_JOIN") |  |  |  |  |

| `FF_NO_STRAT_GROUND` | env_key | std::env::var("FF_NO_STRAT_GROUND") |  |  |  |  |

| `FF_NUMPRE_TEMPORAL` | env_key | std::env::var("FF_NUMPRE_TEMPORAL") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `ground` | function | ground(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_fixpoint` | function | ground_fixpoint(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_stratified` | function | ground_stratified(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_stratified_walled` | function | ground_stratified_walled(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_task` | function | ground_task(domain: &Domain, problem: &Problem, threads: usize) -> Option<PackedTask> |  |  |  |  |

| `initial_state` | function | initial_state(t: &PackedTask) -> State |  |  |  |  |

| `objects_by_type` | function | objects_by_type(domain: &Domain, problem: &Problem) -> HashMap<Sym, Vec<Sym>> |  |  |  |  |

| crate::packed::PackedTask as Task (alias re-export; see prose note) | use | crate::packed::PackedTask as Task |  |  |  |  |


### crates/ferroplan/src/hash.rs

| `FxHasher` | struct | FxHasher { hash: u64 } |  |  |  |  |


### crates/ferroplan/src/hddl.rs

| `HddlError` | enum | HddlError { Parse(String), Ground(String), Translate(String), Planner(PlannerError), RootTaskMismatch { root_task: String, problem_root_network: Vec<String>, }, Timeout { elapsed_ms: u128, limit_ms: u128, }, WorkerPanicked(String) } |  |  |  |  |

| `adapt_problem` | function | adapt_problem(p: ferroplan_hddl::translate::PlanningProblem) -> PlanningProblem |  |  |  |  |

| `solve_hddl` | function | solve_hddl( domain_src: &str, problem_src: &str, limits: &PlannerLimits, ) -> Result<UniversalPlan, HddlError> |  |  |  |  |

| `solve_hddl_from_eve` | function | solve_hddl_from_eve( handoff: &EveHandoff, limits: &PlannerLimits, ) -> Result<UniversalPlan, HddlError> |  |  |  |  |

| `(define (domain coin)
  (:predicates (heads) (tails))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof (heads) (tails)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))` | str_key | DOMAIN = "(define (domain coin)
  (:predicates (heads) (tails))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof (heads) (tails)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))" |  |  |  |  |

| `(define (domain coin-empty)
  (:predicates (heads))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof () (heads)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))` | str_key | DOMAIN = "(define (domain coin-empty)
  (:predicates (heads))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof () (heads)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))" |  |  |  |  |

| `(define (problem coin-empty-p1)
  (:domain coin-empty)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem coin-empty-p1)
  (:domain coin-empty)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem coin-p1)
  (:domain coin)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal (or (heads) (tails))))` | str_key | PROBLEM = "(define (problem coin-p1)
  (:domain coin)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal (or (heads) (tails))))" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../../ferroplan-hddl/fixtures/a/domain.hddl" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../../ferroplan-hddl/fixtures/a/problem.hddl" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../../ferroplan-hddl/fixtures/c/domain.hddl" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/c/problem.hddl` | str_key | FIXTURE_C_PROBLEM = "../../ferroplan-hddl/fixtures/c/problem.hddl" |  |  |  |  |


### crates/ferroplan/src/heuristic.rs

| `T_BUILD` | const | T_BUILD: std::sync::atomic::AtomicU64 |  |  |  |  |

| `T_EXTRACT` | const | T_EXTRACT: std::sync::atomic::AtomicU64 |  |  |  |  |

| `T_RESET` | const | T_RESET: std::sync::atomic::AtomicU64 |  |  |  |  |

| `FF_NO_NEED_DIRS` | env_key | std::env::var("FF_NO_NEED_DIRS") |  |  |  |  |

| `FF_NO_NUMH` | env_key | std::env::var("FF_NO_NUMH") |  |  |  |  |

| `FF_NO_NUMPRE` | env_key | std::env::var("FF_NO_NUMPRE") |  |  |  |  |

| `FF_NO_NUMPRE_CHAIN` | env_key | std::env::var("FF_NO_NUMPRE_CHAIN") |  |  |  |  |

| `FF_NUMPRE_DEPTH` | env_key | std::env::var("FF_NUMPRE_DEPTH") |  |  |  |  |

| `FF_NUMPRE_NODAMP` | env_key | std::env::var("FF_NUMPRE_NODAMP") |  |  |  |  |

| `FF_NUMPRE_NOSKIP` | env_key | std::env::var("FF_NUMPRE_NOSKIP") |  |  |  |  |

| `FF_NUMPRE_NOSUM` | env_key | std::env::var("FF_NUMPRE_NOSUM") |  |  |  |  |

| `extraction_need_facts` | function | extraction_need_facts(sc: &Scratch) -> Vec<(u32, u32)> |  |  |  |  |

| `helpful_needed_adders` | function | helpful_needed_adders( task: &PackedTask, sc: &Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> Vec<u32> |  |  |  |  |

| `new` | function | new(task: &PackedTask) -> Self |  |  |  |  |

| `reachability_layers` | function | reachability_layers( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> (Vec<u32>, Vec<u32>) |  |  |  |  |

| `relaxed` | function | relaxed( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> Option<i32> |  |  |  |  |

| `relaxed_costed` | function | relaxed_costed( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, ) -> Option<i32> |  |  |  |  |

| `relaxed_helpful` | function | relaxed_helpful( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<(i32, Vec<u32>)> |  |  |  |  |

| `relaxed_plan_cost` | function | relaxed_plan_cost( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, ) -> Option<f64> |  |  |  |  |

| `relaxed_to` | function | relaxed_to( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<i32> |  |  |  |  |

| `
    (define (domain watering-mini)
      (:requirements :typing :numeric-fluents)
      (:types agent plant - object)
      (:functions (x ?o - object) (carrying ?a - agent)
                  (poured ?p - plant) (maxx))
      (:action move_right :parameters (?a - agent)
        :precondition (<= (+ (x ?a) 1) (maxx))
        :effect (increase (x ?a) 1))
      (:action move_left :parameters (?a - agent)
        :precondition (>= (- (x ?a) 1) 0)
        :effect (decrease (x ?a) 1))
      (:action pour :parameters (?a - agent ?p - plant)
        :precondition (and (= (x ?a) (x ?p)) (>= (carrying ?a) 1))
        :effect (and (decrease (carrying ?a) 1) (increase (poured ?p) 1))))` | str_key | WATER_DOM = "
    (define (domain watering-mini)
      (:requirements :typing :numeric-fluents)
      (:types agent plant - object)
      (:functions (x ?o - object) (carrying ?a - agent)
                  (poured ?p - plant) (maxx))
      (:action move_right :parameters (?a - agent)
        :precondition (<= (+ (x ?a) 1) (maxx))
        :effect (increase (x ?a) 1))
      (:action move_left :parameters (?a - agent)
        :precondition (>= (- (x ?a) 1) 0)
        :effect (decrease (x ?a) 1))
      (:action pour :parameters (?a - agent ?p - plant)
        :precondition (and (= (x ?a) (x ?p)) (>= (carrying ?a) 1))
        :effect (and (decrease (carrying ?a) 1) (increase (poured ?p) 1))))" |  |  |  |  |

| `
    (define (problem watering-mini-1) (:domain watering-mini)
      (:objects a1 - agent pa pb - plant)
      (:init (= (x a1) 0) (= (x pa) 1) (= (x pb) 9)
             (= (carrying a1) 5) (= (poured pa) 0) (= (poured pb) 0)
             (= (maxx) 12))
      (:goal (and (= (poured pa) 1) (= (poured pb) 1))))` | str_key | WATER_PRB = "
    (define (problem watering-mini-1) (:domain watering-mini)
      (:objects a1 - agent pa pb - plant)
      (:init (= (x a1) 0) (= (x pa) 1) (= (x pb) 9)
             (= (carrying a1) 5) (= (poured pa) 0) (= (poured pb) 0)
             (= (maxx) 12))
      (:goal (and (= (poured pa) 1) (= (poured pb) 1))))" |  |  |  |  |

| `(define (domain drain)
      (:requirements :fluents)
      (:predicates (there) (idle))
      (:functions (energy))
      (:action drive :parameters ()
        :precondition (>= (energy) 8)
        :effect (and (there) (decrease (energy) 8)))
      (:action wander :parameters ()
        :precondition (idle)
        :effect (decrease (energy) 1)))` | str_key | DRAIN_DOM = "(define (domain drain)
      (:requirements :fluents)
      (:predicates (there) (idle))
      (:functions (energy))
      (:action drive :parameters ()
        :precondition (>= (energy) 8)
        :effect (and (there) (decrease (energy) 8)))
      (:action wander :parameters ()
        :precondition (idle)
        :effect (decrease (energy) 1)))" |  |  |  |  |

| `(define (problem d1) (:domain drain)
      (:init (idle) (= (energy) 5)) (:goal (there)))` | str_key | DRAIN_PRB = "(define (problem d1) (:domain drain)
      (:init (idle) (= (energy) 5)) (:goal (there)))" |  |  |  |  |

| `Scratch` | struct | Scratch { reached: Vec<bool>, fact_layer: Vec<u32>, op_layer: Vec<u32>, gen: u32, op_stamp: Vec<u32>, applicable: Vec<u32>, lb: Vec<f64>, ub: Vec<f64>, selected: Vec<u32>, need_fact: Vec<u32>, queue: Vec<u32>, num_applied: Vec<u32>, cond_ops: Vec<u32>, helpful: Vec<u32>, fact_time: Vec<f64>, op_time: Vec<f64> } |  |  |  |  |

| `TrpgInfo` | struct | TrpgInfo { pub start_of: Vec<u32>, pub lag: Vec<f64>, pub floor: Vec<f64>, pub windows: Vec<Vec<TrpgWindow>> } |  |  |  |  |

| `TrpgWindow` | struct | TrpgWindow { pub fact: u32, pub providers: Vec<(u32, f64)>, pub close: f64 } |  |  |  |  |


### crates/ferroplan/src/introspect.rs

| `explain` | function | explain(domain_src: &str, problem_src: &str, plan: &Plan) -> Result<Explanation, String> |  |  |  |  |

| `(define (domain chain)
      (:requirements :strips)
      (:predicates (a) (b) (c) (d))
      (:action MK-B :parameters () :precondition (a) :effect (b))
      (:action MK-C :parameters () :precondition (b) :effect (c)))` | str_key | CHAIN_DOM = "(define (domain chain)
      (:requirements :strips)
      (:predicates (a) (b) (c) (d))
      (:action MK-B :parameters () :precondition (a) :effect (b))
      (:action MK-C :parameters () :precondition (b) :effect (c)))" |  |  |  |  |

| `(define (problem chain-1) (:domain chain)
      (:init (a)) (:goal (c)))` | str_key | CHAIN_PRB = "(define (problem chain-1) (:domain chain)
      (:init (a)) (:goal (c)))" |  |  |  |  |

| `CausalLink` | struct | CausalLink { pub provider: Option<usize>, pub consumer: usize, pub fact: String } |  |  |  |  |

| `Explanation` | struct | Explanation { pub kind: String, pub causal_links: Vec<CausalLink>, pub invariant_spans: Vec<InvariantSpan>, pub preferences: Vec<PrefReport> } |  |  |  |  |

| `InvariantSpan` | struct | InvariantSpan { pub step: usize, pub action: String, pub start: f64, pub end: f64, pub conditions: Vec<String> } |  |  |  |  |

| `PrefReport` | struct | PrefReport { pub name: String, pub satisfied: bool, pub weight: f64 } |  |  |  |  |


### crates/ferroplan/src/invariants.rs

| `synthesize` | function | synthesize(domain: &Domain, task: &PackedTask) -> Vec<Vec<u32>> |  |  |  |  |

| `(define (domain blocks)
      (:requirements :strips :typing)
      (:types block)
      (:predicates (on ?x ?y - block) (ontable ?x - block) (clear ?x - block)
                   (handempty) (holding ?x - block))
      (:action pickup :parameters (?x - block)
        :precondition (and (clear ?x) (ontable ?x) (handempty))
        :effect (and (not (ontable ?x)) (not (clear ?x)) (not (handempty)) (holding ?x)))
      (:action putdown :parameters (?x - block)
        :precondition (holding ?x)
        :effect (and (not (holding ?x)) (clear ?x) (handempty) (ontable ?x)))
      (:action stack :parameters (?x ?y - block)
        :precondition (and (holding ?x) (clear ?y))
        :effect (and (not (holding ?x)) (not (clear ?y)) (clear ?x) (handempty) (on ?x ?y)))
      (:action unstack :parameters (?x ?y - block)
        :precondition (and (on ?x ?y) (clear ?x) (handempty))
        :effect (and (holding ?x) (clear ?y) (not (clear ?x)) (not (on ?x ?y)) (not (handempty)))))` | str_key | BLOCKS = "(define (domain blocks)
      (:requirements :strips :typing)
      (:types block)
      (:predicates (on ?x ?y - block) (ontable ?x - block) (clear ?x - block)
                   (handempty) (holding ?x - block))
      (:action pickup :parameters (?x - block)
        :precondition (and (clear ?x) (ontable ?x) (handempty))
        :effect (and (not (ontable ?x)) (not (clear ?x)) (not (handempty)) (holding ?x)))
      (:action putdown :parameters (?x - block)
        :precondition (holding ?x)
        :effect (and (not (holding ?x)) (clear ?x) (handempty) (ontable ?x)))
      (:action stack :parameters (?x ?y - block)
        :precondition (and (holding ?x) (clear ?y))
        :effect (and (not (holding ?x)) (not (clear ?y)) (clear ?x) (handempty) (on ?x ?y)))
      (:action unstack :parameters (?x ?y - block)
        :precondition (and (on ?x ?y) (clear ?x) (handempty))
        :effect (and (holding ?x) (clear ?y) (not (clear ?x)) (not (on ?x ?y)) (not (handempty)))))" |  |  |  |  |

| `(define (domain gripper)
      (:requirements :strips :typing)
      (:types room ball)
      (:predicates (at-robby ?r - room) (ball-at ?b - ball ?r - room) (carry ?b - ball))
      (:action move :parameters (?from ?to - room)
        :precondition (at-robby ?from)
        :effect (and (not (at-robby ?from)) (at-robby ?to)))
      (:action pick :parameters (?b - ball ?r - room)
        :precondition (and (ball-at ?b ?r) (at-robby ?r))
        :effect (and (not (ball-at ?b ?r)) (carry ?b)))
      (:action drop :parameters (?b - ball ?r - room)
        :precondition (and (carry ?b) (at-robby ?r))
        :effect (and (not (carry ?b)) (ball-at ?b ?r))))` | str_key | GRIPPER = "(define (domain gripper)
      (:requirements :strips :typing)
      (:types room ball)
      (:predicates (at-robby ?r - room) (ball-at ?b - ball ?r - room) (carry ?b - ball))
      (:action move :parameters (?from ?to - room)
        :precondition (at-robby ?from)
        :effect (and (not (at-robby ?from)) (at-robby ?to)))
      (:action pick :parameters (?b - ball ?r - room)
        :precondition (and (ball-at ?b ?r) (at-robby ?r))
        :effect (and (not (ball-at ?b ?r)) (carry ?b)))
      (:action drop :parameters (?b - ball ?r - room)
        :precondition (and (carry ?b) (at-robby ?r))
        :effect (and (not (carry ?b)) (ball-at ?b ?r))))" |  |  |  |  |

| `(define (domain log)
      (:requirements :strips :typing)
      (:types vehicle package location)
      (:predicates (at ?x - object ?l - location) (in ?p - package ?v - vehicle)
                   (road ?a ?b - location))
      (:action drive :parameters (?v - vehicle ?from ?to - location)
        :precondition (and (at ?v ?from) (road ?from ?to))
        :effect (and (not (at ?v ?from)) (at ?v ?to)))
      (:action load :parameters (?p - package ?v - vehicle ?l - location)
        :precondition (and (at ?p ?l) (at ?v ?l))
        :effect (and (not (at ?p ?l)) (in ?p ?v)))
      (:action unload :parameters (?p - package ?v - vehicle ?l - location)
        :precondition (and (in ?p ?v) (at ?v ?l))
        :effect (and (not (in ?p ?v)) (at ?p ?l))))` | str_key | LOGISTICS = "(define (domain log)
      (:requirements :strips :typing)
      (:types vehicle package location)
      (:predicates (at ?x - object ?l - location) (in ?p - package ?v - vehicle)
                   (road ?a ?b - location))
      (:action drive :parameters (?v - vehicle ?from ?to - location)
        :precondition (and (at ?v ?from) (road ?from ?to))
        :effect (and (not (at ?v ?from)) (at ?v ?to)))
      (:action load :parameters (?p - package ?v - vehicle ?l - location)
        :precondition (and (at ?p ?l) (at ?v ?l))
        :effect (and (not (at ?p ?l)) (in ?p ?v)))
      (:action unload :parameters (?p - package ?v - vehicle ?l - location)
        :precondition (and (in ?p ?v) (at ?v ?l))
        :effect (and (not (in ?p ?v)) (at ?p ?l))))" |  |  |  |  |

| `(define (problem p) (:domain blocks)
      (:objects a b c - block)
      (:init (ontable a) (on b a) (on c b) (clear c) (handempty))
      (:goal (on a b)))` | str_key | BLOCKS_PROB = "(define (problem p) (:domain blocks)
      (:objects a b c - block)
      (:init (ontable a) (on b a) (on c b) (clear c) (handempty))
      (:goal (on a b)))" |  |  |  |  |

| `(define (problem p) (:domain gripper)
      (:objects ra rb - room b1 - ball)
      (:init (at-robby ra) (ball-at b1 ra))
      (:goal (at-robby rb)))` | str_key | GRIP_PROB = "(define (problem p) (:domain gripper)
      (:objects ra rb - room b1 - ball)
      (:init (at-robby ra) (ball-at b1 ra))
      (:goal (at-robby rb)))" |  |  |  |  |

| `(define (problem p) (:domain log)
      (:objects v1 - vehicle pk1 - package a b c - location)
      (:init (at v1 a) (at pk1 b) (road a b) (road b c) (road a c))
      (:goal (at pk1 c)))` | str_key | LOG_PROB = "(define (problem p) (:domain log)
      (:objects v1 - vehicle pk1 - package a b c - location)
      (:init (at v1 a) (at pk1 b) (road a b) (road b c) (road a c))
      (:goal (at pk1 c)))" |  |  |  |  |


### crates/ferroplan/src/lama.rs

| `FF_LAMA_EXT_ARRIVAL` | env_key | std::env::var("FF_LAMA_EXT_ARRIVAL") |  |  |  |  |

| `FF_LEN_ANYTIME` | env_key | std::env::var("FF_LEN_ANYTIME") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `search` | function | search( task: &PackedTask, threads: usize, max_eval: usize, forbidden: &[bool], slice: Option<(crate::clock::Clock, f64)>, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `search_subgoal` | function | search_subgoal( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[crate::types::NumPre], threads: usize, max_eval: usize, forbidden: &[bool], len_anytime: bool, slice: Option<(crate::clock::Clock, f64)>, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |


### crates/ferroplan/src/landmarks.rs

| `goal_landmarks` | function | goal_landmarks(task: &PackedTask) -> Vec<u32> |  |  |  |  |

| `landmarks_for` | function | landmarks_for( task: &PackedTask, start: &crate::packed::State, goal_pos: &[u32], ) -> Vec<u32> |  |  |  |  |


### crates/ferroplan/src/lexer.rs

| `Tok` | enum | Tok { LParen, RParen, Dash, Var(String), Name(String), Num(f64), Op(String) } |  |  |  |  |

| `lex` | function | lex(input: &str) -> Result<(Vec<Tok>, Vec<u32>), crate::types::ParseError> |  |  |  |  |


### crates/ferroplan/src/lib.rs

| `api::{ decompose, parse, solve, Contract, Decomposition, DomainSummary, Metric, Mode, Options, ParseReport, Plan, ProblemSummary, Search, Solution, SolveError, Statistics, Step, }` | use | api::{ decompose, parse, solve, Contract, Decomposition, DomainSummary, Metric, Mode, Options, ParseReport, Plan, ProblemSummary, Search, Solution, SolveError, Statistics, Step, } |  |  |  |  |

| `eve::{ Activator, CapabilityTarget, Eve, EveError, EveHandoff, EveRequest, EveStage, GenesisProjection, GenesisWorld, GgenManufacturingRequest, GroundedGoal, HddlDecompositionRequest, HddlSurface, HumanPurpose, ManufactureTarget, McpPlusHandoff, PlanningRegime, PpddlPolicyRequest, PpddlSurface, SplitDirective, TruexContinuation, MAX_PRIMARY_ACTIVATORS, }` | use | eve::{ Activator, CapabilityTarget, Eve, EveError, EveHandoff, EveRequest, EveStage, GenesisProjection, GenesisWorld, GgenManufacturingRequest, GroundedGoal, HddlDecompositionRequest, HddlSurface, HumanPurpose, ManufactureTarget, McpPlusHandoff, PlanningRegime, PpddlPolicyRequest, PpddlSurface, SplitDirective, TruexContinuation, MAX_PRIMARY_ACTIVATORS, } |  |  |  |  |

| `hddl::{solve_hddl, HddlError}` | use | hddl::{solve_hddl, HddlError} |  |  |  |  |

| `operator_compiler::{ compile_operator, CompiledOperator, OperatorCompileError, OperatorEffects, OperatorSpec, }` | use | operator_compiler::{ compile_operator, CompiledOperator, OperatorCompileError, OperatorEffects, OperatorSpec, } |  |  |  |  |

| `planner::{run_ff, run_planner}` | use | planner::{run_ff, run_planner} |  |  |  |  |

| `planning_runtime::{ solve_planning_type, Agent, Goal as UniversalGoal, Method as PlanningMethod, PlanStep, PlannerError, PlannerLimits, PlanningProblem, PolicyEntry as UniversalPolicyEntry, PolicyOutcome as UniversalPolicyOutcome, QueueState, RdfTriple, State as UniversalState, Task as PlanningTask, Tool, Transition as UniversalTransition, UniversalPlan, UniversalPlanningRequest, WorkflowEdge, }` | use | planning_runtime::{ solve_planning_type, Agent, Goal as UniversalGoal, Method as PlanningMethod, PlanStep, PlannerError, PlannerLimits, PlanningProblem, PolicyEntry as UniversalPolicyEntry, PolicyOutcome as UniversalPolicyOutcome, QueueState, RdfTriple, State as UniversalState, Task as PlanningTask, Tool, Transition as UniversalTransition, UniversalPlan, UniversalPlanningRequest, WorkflowEdge, } |  |  |  |  |

| `planning_types::{ route_planning_request, PlanningCapability, PlanningRail, PlanningRequest, PlanningRoute, PlanningRouteError, PlanningType, }` | use | planning_types::{ route_planning_request, PlanningCapability, PlanningRail, PlanningRequest, PlanningRoute, PlanningRouteError, PlanningType, } |  |  |  |  |

| `policy_validation::{ validate_fond_policy, PolicyGuarantee, PolicyIssue, PolicyValidationReport, }` | use | policy_validation::{ validate_fond_policy, PolicyGuarantee, PolicyIssue, PolicyValidationReport, } |  |  |  |  |

| `ppddl::{ parse_ppddl, simulate_ppddl, solve_ppddl, validate_ppddl_policy, InitialStateProbability, PolicyDecision, PolicyOutcome, PolicyValidation, PpddlError, PpddlParseReport, ProbabilisticObjective, ProbabilisticOptions, ProbabilisticSolution, ProbabilisticState, ProbabilisticStatistics, SimulationReport, }` | use | ppddl::{ parse_ppddl, simulate_ppddl, solve_ppddl, validate_ppddl_policy, InitialStateProbability, PolicyDecision, PolicyOutcome, PolicyValidation, PpddlError, PpddlParseReport, ProbabilisticObjective, ProbabilisticOptions, ProbabilisticSolution, ProbabilisticState, ProbabilisticStatistics, SimulationReport, } |  |  |  |  |

| `production::{ parse_production, solve_ppddl_production, trace_production, validate_plan_production, PlanValidationEvidence, ProductionSession, }` | use | production::{ parse_production, solve_ppddl_production, trace_production, validate_plan_production, PlanValidationEvidence, ProductionSession, } |  |  |  |  |

| `production_explain::{decompose_production, explain_production}` | use | production_explain::{decompose_production, explain_production} |  |  |  |  |

| `readiness::{ capability_manifest, evaluate_readiness, production_input_fingerprint, solve_production, AuthorityClass, BuildIdentity, CapabilityContract, CapabilityEvaluation, CapabilityManifest, CompatibilityClass, DeterminismClass, InterfaceKind, ManifestError, OperationEnvelope, OutcomeClass, ProductionLimits, PublicError, ReadinessReport, ReadinessState, ReplayClass, SecurityClass, ValidationStatus, CANDIDATE_AUTHORITY, CAPABILITY_MANIFEST_SCHEMA, OPERATION_ENVELOPE_SCHEMA, }` | use | readiness::{ capability_manifest, evaluate_readiness, production_input_fingerprint, solve_production, AuthorityClass, BuildIdentity, CapabilityContract, CapabilityEvaluation, CapabilityManifest, CompatibilityClass, DeterminismClass, InterfaceKind, ManifestError, OperationEnvelope, OutcomeClass, ProductionLimits, PublicError, ReadinessReport, ReadinessState, ReplayClass, SecurityClass, ValidationStatus, CANDIDATE_AUTHORITY, CAPABILITY_MANIFEST_SCHEMA, OPERATION_ENVELOPE_SCHEMA, } |  |  |  |  |

| `session::{Session, Think, ThinkBudget, ThinkVerdict}` | use | session::{Session, Think, ThinkBudget, ThinkVerdict} |  |  |  |  |

| `trace::{trace, StateSnapshot}` | use | trace::{trace, StateSnapshot} |  |  |  |  |

| `types::ParseError` | use | types::ParseError |  |  |  |  |


### crates/ferroplan/src/mem.rs

| `FF_MEM_BUDGET_GB` | env_key | std::env::var("FF_MEM_BUDGET_GB") |  |  |  |  |

| `FF_MEM_TRIP_FRAC` | env_key | std::env::var("FF_MEM_TRIP_FRAC") |  |  |  |  |

| `FF_NO_MEM_WALL` | env_key | std::env::var("FF_NO_MEM_WALL") |  |  |  |  |

| `arm` | function | arm() -> Self |  |  |  |  |

| `armed` | function | armed(&self) -> bool |  |  |  |  |

| `declared_budget_bytes` | function | declared_budget_bytes() -> Option<u64> |  |  |  |  |

| `hit` | function | hit(&self) -> bool |  |  |  |  |

| `latch` | function | latch() |  |  |  |  |

| `latched` | function | latched() -> bool |  |  |  |  |

| `peak_resident_bytes` | function | peak_resident_bytes() -> Option<u64> |  |  |  |  |

| `resident_bytes` | function | resident_bytes() -> Option<u64> |  |  |  |  |

| `unarmed` | function | unarmed() -> Self |  |  |  |  |

| `MemWall` | struct | MemWall { trip_at: Option<u64> } |  |  |  |  |


### crates/ferroplan/src/novelty.rs

| `FF_NOV_R_CAP` | env_key | std::env::var("FF_NOV_R_CAP") |  |  |  |  |

| `FF_NUMNOV` | env_key | std::env::var("FF_NUMNOV") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `from_env` | function | from_env() -> Self |  |  |  |  |

| `r_partition_facts` | function | r_partition_facts( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[crate::types::NumPre], r_cap: usize, ) -> Vec<u32> |  |  |  |  |

| `search` | function | search( task: &PackedTask, threads: usize, max_eval: usize, forbidden: &[bool], slice: Option<(crate::clock::Clock, f64)>, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `search_driver` | function | search_driver( task: &PackedTask, max_eval: usize, forbidden: &[bool], slice: Option<(crate::clock::Clock, f64)>, cfg: &DriverCfg, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `search_light` | function | search_light( task: &PackedTask, max_eval: usize, forbidden: &[bool], ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `search_subgoal` | function | search_subgoal( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[crate::types::NumPre], threads: usize, max_eval: usize, forbidden: &[bool], slice: Option<(crate::clock::Clock, f64)>, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `DriverCfg` | struct | DriverCfg { pub partition: bool, pub width2: bool, pub r_cap: usize } |  |  |  |  |


### crates/ferroplan/src/operator_compiler.rs

| `OperatorCompileError` | enum | OperatorCompileError { EmptyName, ConflictingEffect(String), NoApplicableState, MissingTargetState { from: String, facts: BTreeSet<String>, }, AmbiguousTargetState { from: String, candidates: Vec<String>, } } |  |  |  |  |

| `compile_operator` | function | compile_operator( spec: &OperatorSpec, states: &[State], ) -> Result<CompiledOperator, OperatorCompileError> |  |  |  |  |

| `CompiledOperator` | struct | CompiledOperator { pub task: Task, pub transitions: Vec<Transition> } |  |  |  |  |

| `OperatorEffects` | struct | OperatorEffects { pub add: BTreeSet<String>, pub delete: BTreeSet<String> } |  |  |  |  |

| `OperatorSpec` | struct | OperatorSpec { pub name: String, pub preconditions: BTreeSet<String>, pub effects: OperatorEffects, pub cost: u64 } |  |  |  |  |


### crates/ferroplan/src/optimal.rs

| `FF_NO_HMAX_SPRINT` | env_key | std::env::var("FF_NO_HMAX_SPRINT") |  |  |  |  |

| `FF_NO_INC_LMCUT` | env_key | std::env::var("FF_NO_INC_LMCUT") |  |  |  |  |

| `FF_NO_LMCUT` | env_key | std::env::var("FF_NO_LMCUT") |  |  |  |  |

| `FF_NO_NODECAP_REFILL` | env_key | std::env::var("FF_NO_NODECAP_REFILL") |  |  |  |  |

| `FF_OPT_GATE_MARGIN` | env_key | std::env::var("FF_OPT_GATE_MARGIN") |  |  |  |  |

| `FF_OPT_NO_NUMFOLD` | env_key | std::env::var("FF_OPT_NO_NUMFOLD") |  |  |  |  |

| `FF_OPT_NO_NUMH` | env_key | std::env::var("FF_OPT_NO_NUMH") |  |  |  |  |

| `FF_OPT_NO_RESUME` | env_key | std::env::var("FF_OPT_NO_RESUME") |  |  |  |  |

| `FF_OPT_NO_ROOTGATE` | env_key | std::env::var("FF_OPT_NO_ROOTGATE") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `solve` | function | solve( task: &PackedTask, cf: Option<usize>, max_nodes: usize, orbit: Option<&crate::orbits::OrbitMap>, ) -> OptOutcome |  |  |  |  |

| `OptOutcome` | struct | OptOutcome { pub ops: Option<Vec<usize>>, pub cost: f64, pub expanded: usize, pub evaluated: usize, pub proven: bool, pub reject: Option<String>, pub heuristic: &'static str, pub clock_tripped: bool } |  |  |  |  |


### crates/ferroplan/src/orbits.rs

| `FF_NO_ORBIT` | env_key | std::env::var("FF_NO_ORBIT") |  |  |  |  |

| `FF_NO_ORBIT_CLASSICAL` | env_key | std::env::var("FF_NO_ORBIT_CLASSICAL") |  |  |  |  |

| `FF_ORBIT_DEBUG` | env_key | std::env::var("FF_ORBIT_DEBUG") |  |  |  |  |

| `FF_ORBIT_ISO` | env_key | std::env::var("FF_ORBIT_ISO") |  |  |  |  |

| `canonical_key` | function | canonical_key( &self, task: &PackedTask, state: &State, agenda: &[(i64, usize)], ) -> (crate::packed::StateKey, Vec<(i64, usize)>) |  |  |  |  |

| `canonical_skey` | function | canonical_skey( &self, task: &PackedTask, state: &State, cost_fluent: Option<usize>, ) -> crate::packed::StateKey |  |  |  |  |

| `canonical_skey_hash` | function | canonical_skey_hash( &self, task: &PackedTask, state: &State, cost_fluent: Option<usize>, ) -> u64 |  |  |  |  |

| `detect` | function | detect(domain: &Domain, problem: &Problem, task: &PackedTask) -> Option<OrbitMap> |  |  |  |  |

| `detect_classical` | function | detect_classical(domain: &Domain, problem: &Problem, task: &PackedTask) -> Option<OrbitMap> |  |  |  |  |

| `detect_classical_iso` | function | detect_classical_iso( domain: &Domain, problem: &Problem, task: &PackedTask, ) -> Option<OrbitMap> |  |  |  |  |

| `detect_iso` | function | detect_iso(domain: &Domain, problem: &Problem, task: &PackedTask) -> Option<OrbitMap> |  |  |  |  |

| `gen_key` | function | gen_key(&self, op: usize, classes: &[Vec<u16>]) -> Option<(u32, Vec<u16>)> |  |  |  |  |

| `goal_free_view` | function | goal_free_view(&self) -> Option<OrbitMap> |  |  |  |  |

| `iso_active` | function | iso_active(&self) -> bool |  |  |  |  |

| `iso_goal_witness` | function | iso_goal_witness( &self, task: &PackedTask, state: &State, goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<Vec<Vec<u16>>> |  |  |  |  |

| `iso_remap_op` | function | iso_remap_op(&self, sigma: &[Vec<u16>], op: usize) -> usize |  |  |  |  |

| `iso_untouched_goal` | function | iso_untouched_goal(&self) -> Option<&[u32]> |  |  |  |  |

| `stabilizer_classes` | function | stabilizer_classes(&self, state: &State, agenda: &[(f64, usize)]) -> Vec<Vec<u16>> |  |  |  |  |

| `IsoGoal` | struct | IsoGoal { desig: Vec<(u32, Vec<u16>)>, untouched: Vec<u32>, goal: Vec<u32> } |  |  |  |  |

| `Orbit` | struct | Orbit { pub facts: Vec<Vec<u32>>, pub fluent_slots: Vec<Vec<usize>>, pub ops: Vec<Vec<usize>> } |  |  |  |  |

| `OrbitMap` | struct | OrbitMap { pub orbits: Vec<Orbit>, pub iso: Option<IsoGoal>, pub op_owner: FxHashMap<usize, (usize, usize, usize)>, pub goal_bound: Vec<bool>, frozen: Vec<bool>, fact_fams: Vec<Family>, fact_touch: Vec<(u32, u32, Vec<u16>)>, op_fams: Vec<Family>, op_touch: FxHashMap<usize, (u32, Vec<u16>)>, flu_fams: Vec<Family>, flu_touch: Vec<(u32, u32, Vec<u16>)> } |  |  |  |  |


### crates/ferroplan/src/output.rs

| `preamble` | function | preamble(threads: usize) -> String |  |  |  |  |

| `render` | function | render(task: &PackedTask, result: &PlanResult, threads: usize) -> (String, i32) |  |  |  |  |


### crates/ferroplan/src/packed.rs

| `applicable_ops` | function | applicable_ops(&self, s: &State, out: &mut Vec<u32>) |  |  |  |  |

| `apply` | function | apply(&self, oi: usize, s: &State) -> State |  |  |  |  |

| `build_succ` | function | build_succ(pre_pos: &Csr<u32>, n_facts: usize, n_ops: usize) -> (Csr<u32>, Vec<u32>) |  |  |  |  |

| `cond_effs` | function | cond_effs(&self, oi: usize) -> impl Iterator<Item = &CondEff> + Clone |  |  |  |  |

| `fact_id` | function | fact_id(&self, disp: &str) -> Option<usize> |  |  |  |  |

| `finish` | function | finish(self) -> Csr<T> |  |  |  |  |

| `fluent_id` | function | fluent_id(&self, disp: &str) -> Option<usize> |  |  |  |  |

| `goal_met` | function | goal_met(&self, s: &State) -> bool |  |  |  |  |

| `goal_met_with` | function | goal_met_with(&self, s: &State, goal_pos: &[u32], goal_num: &[NumPre]) -> bool |  |  |  |  |

| `initial` | function | initial(&self) -> State |  |  |  |  |

| `n_cond_effs` | function | n_cond_effs(&self, oi: usize) -> usize |  |  |  |  |

| `new` | function | new() -> Self |  |  |  |  |

| `op_applicable` | function | op_applicable(&self, oi: usize, s: &State) -> bool |  |  |  |  |

| `push_row` | function | push_row(&mut self, items: impl IntoIterator<Item = T>) |  |  |  |  |

| `slice` | function | slice(&self, i: usize) -> &[T] |  |  |  |  |

| `state_key` | function | state_key(&self, s: &State) -> StateKey |  |  |  |  |

| `state_key_eq` | function | state_key_eq(&self, a: &State, b: &State, cost_fluent: Option<usize>) -> bool |  |  |  |  |

| `state_key_hash` | function | state_key_hash(&self, s: &State, cost_fluent: Option<usize>) -> u64 |  |  |  |  |

| `state_key_with_cost` | function | state_key_with_cost(&self, s: &State, cost_fluent: Option<usize>) -> StateKey |  |  |  |  |

| `static_fluent` | function | static_fluent(&self, disp: &str) -> Option<f64> |  |  |  |  |

| `CondEff` | struct | CondEff { pub cond_pos: Vec<u32>, pub cond_neg: Vec<u32>, pub cond_num: Vec<NumPre>, pub add: Vec<u32>, pub del: Vec<u32>, pub num: Vec<NumEff> } |  |  |  |  |

| `Csr` | struct | Csr { pub flat: Arc<[T]>, pub off: Arc<[u32]> } |  |  |  |  |

| `CsrBuilder` | struct | CsrBuilder { pub flat: Vec<T>, pub off: Vec<u32> } |  |  |  |  |

| `PackedTask` | struct | PackedTask { pub n_facts: usize, pub words: usize, pub n_ops: usize, pub op_display: Arc<[String]>, pub pre_pos: Csr<u32>, pub succ_by_fact: Csr<u32>, pub succ_always: Arc<[u32]>, pub add: Csr<u32>, pub del: Csr<u32>, pub pre_num: Csr<NumPre>, pub num_eff: Csr<NumEff>, pub cond: Csr<CondEff>, pub shared_cond: Arc<[CondEff]>, pub monitored: Arc<[bool]>, pub add_by_fact: Csr<u32>, pub neff_by_fluent: Csr<u32>, pub relevant_fluent: Vec<bool>, pub rel_fluents: Vec<u32>, pub init_bits: Vec<u64>, pub fv0: Vec<f64>, pub fdef0: Vec<bool>, pub goal_pos: Vec<u32>, pub goal_num: Vec<NumPre>, pub charge_pre_num: bool, pub pair_end: Option<Vec<u32>>, pub trpg: Option<Arc<crate::heuristic::TrpgInfo>>, pub fact_names: Arc<[String]>, pub fluent_names: Arc<[String]>, pub static_fluents: Arc<[(String, f64)]>, pub n_easy: usize, pub n_hard: usize, pub n_reach_facts: usize, pub n_reach_actions: usize, pub n_relevant_fluents: usize } |  |  |  |  |

| `State` | struct | State { pub bits: Vec<u64>, pub fv: Vec<f64>, pub fdef: Vec<bool> } |  |  |  |  |

| `StateKey` | struct | StateKey { pub bits: Vec<u64>, pub vals: Vec<i64> } |  |  |  |  |


### crates/ferroplan/src/par.rs

| `MIN_PAR` | const | MIN_PAR: usize |  |  |  |  |

| `FFDP_THREADS` | env_key | std::env::var("FFDP_THREADS") |  |  |  |  |

| `num_threads` | function | num_threads() -> usize |  |  |  |  |

| `par_map` | function | par_map(items: &[T], threads: usize, f: F) -> Vec<R> |  |  |  |  |

| `par_map_with` | function | par_map_with(items: &[T], threads: usize, init: I, f: F) -> Vec<R> |  |  |  |  |


### crates/ferroplan/src/parser.rs

| `parse_domain` | function | parse_domain(src: &str) -> Result<Domain, ParseError> |  |  |  |  |

| `parse_problem` | function | parse_problem(src: &str) -> Result<Problem, ParseError> |  |  |  |  |

| `:STRIPS` | str_key | SUPPORTED = ":STRIPS" |  |  |  |  |


### crates/ferroplan/src/partition.rs

| `interaction_partition` | function | interaction_partition(task: &PackedTask, groups: &[Vec<u32>]) -> Vec<Subgoal> |  |  |  |  |

| `interaction_partition_of` | function | interaction_partition_of( task: &PackedTask, groups: &[Vec<u32>], goals: &[u32], excluded_vars: &FxHashSet<usize>, ) -> Vec<Subgoal> |  |  |  |  |

| `is_empty` | function | is_empty(&self) -> bool |  |  |  |  |

| `merge_at` | function | merge_at(groups: &mut Vec<Subgoal>, i: usize, j: usize) -> usize |  |  |  |  |

| `merge_with_neighbor` | function | merge_with_neighbor(groups: &mut Vec<Subgoal>, i: usize) -> usize |  |  |  |  |

| `partition` | function | partition(task: &PackedTask) -> Vec<Subgoal> |  |  |  |  |

| `
    (define (domain t) (:requirements :strips)
      (:predicates (done1) (tok-a) (tok-b))
      (:action grab :precondition (tok-a)
        :effect (and (done1) (not (tok-a)) (tok-b)))
      (:action swap :precondition (tok-b)
        :effect (and (not (tok-b)) (tok-a))))` | str_key | DOM = "
    (define (domain t) (:requirements :strips)
      (:predicates (done1) (tok-a) (tok-b))
      (:action grab :precondition (tok-a)
        :effect (and (done1) (not (tok-a)) (tok-b)))
      (:action swap :precondition (tok-b)
        :effect (and (not (tok-b)) (tok-a))))" |  |  |  |  |

| `(define (problem p) (:domain t)
      (:init (tok-a)) (:goal (and (done1) (tok-b))))` | str_key | PRB = "(define (problem p) (:domain t)
      (:init (tok-a)) (:goal (and (done1) (tok-b))))" |  |  |  |  |

| `Subgoal` | struct | Subgoal { pub pos: Vec<u32>, pub num: Vec<NumPre> } |  |  |  |  |


### crates/ferroplan/src/pddl3.rs

| `COST` | const | COST: &str |  |  |  |  |

| `COST_DISP` | const | COST_DISP: &str |  |  |  |  |

| `FF_DEADLINE_WEIGHT` | env_key | std::env::var("FF_DEADLINE_WEIGHT") |  |  |  |  |

| `FF_ESPC_TRAJ_PAIRS` | env_key | std::env::var("FF_ESPC_TRAJ_PAIRS") |  |  |  |  |

| `FF_PREF_COMPILED` | env_key | std::env::var("FF_PREF_COMPILED") |  |  |  |  |

| `FF_PREF_COST_WEIGHT` | env_key | std::env::var("FF_PREF_COST_WEIGHT") |  |  |  |  |

| `FF_PREF_EVAL_BUDGET` | env_key | std::env::var("FF_PREF_EVAL_BUDGET") |  |  |  |  |

| `FF_PREF_GREEDY` | env_key | std::env::var("FF_PREF_GREEDY") |  |  |  |  |

| `FF_PREF_NO_BARRIER` | env_key | std::env::var("FF_PREF_NO_BARRIER") |  |  |  |  |

| `FF_PREF_NO_ESCALATE` | env_key | std::env::var("FF_PREF_NO_ESCALATE") |  |  |  |  |

| `FF_PREF_NO_RESTARTS` | env_key | std::env::var("FF_PREF_NO_RESTARTS") |  |  |  |  |

| `FF_PREF_NO_SEED` | env_key | std::env::var("FF_PREF_NO_SEED") |  |  |  |  |

| `FF_PREF_NO_SELECT` | env_key | std::env::var("FF_PREF_NO_SELECT") |  |  |  |  |

| `FF_PREF_NO_STATIC` | env_key | std::env::var("FF_PREF_NO_STATIC") |  |  |  |  |

| `FF_PREF_NUMLEGACY` | env_key | std::env::var("FF_PREF_NUMLEGACY") |  |  |  |  |

| `FF_PREF_SEED` | env_key | std::env::var("FF_PREF_SEED") |  |  |  |  |

| `FF_PREF_SEED3` | env_key | std::env::var("FF_PREF_SEED3") |  |  |  |  |

| `FF_PREF_SEED_BOUND` | env_key | std::env::var("FF_PREF_SEED_BOUND") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_RES_THRESH` | env_key | std::env::var("FF_RES_THRESH") |  |  |  |  |

| `FF_RES_WEIGHT` | env_key | std::env::var("FF_RES_WEIGHT") |  |  |  |  |

| `close_seed` | function | close_seed( task: &PackedTask, cost_fluent: usize, forgos: &[(usize, f64)], prefix: &[usize], ) -> Option<(Vec<usize>, f64)> |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Compiled |  |  |  |  |

| `display_metric` | function | display_metric(&self, optimized: f64) -> f64 |  |  |  |  |

| `hard_goal_plan` | function | hard_goal_plan( domain: &Domain, problem: &Problem, threads: usize, cfg: SearchCfg, ) -> Option<Vec<String>> |  |  |  |  |

| `hard_goal_seed` | function | hard_goal_seed( domain: &Domain, problem: &Problem, compiled: &PackedTask, threads: usize, cfg: SearchCfg, ) -> Option<Vec<usize>> |  |  |  |  |

| `has_preferences` | function | has_preferences(problem: &Problem) -> bool |  |  |  |  |

| `is_pddl3` | function | is_pddl3(problem: &Problem) -> bool |  |  |  |  |

| `lift_seed` | function | lift_seed(compiled: &PackedTask, names: &[String]) -> Option<Vec<usize>> |  |  |  |  |

| `metric_optimize` | function | metric_optimize( task: &PackedTask, cost_fluent: usize, forgos: &[(usize, f64)], groups: &[Vec<u32>], folded_metric: bool, threads: usize, ) -> Option<MetricResult> |  |  |  |  |

| `metric_optimize_seeded` | function | metric_optimize_seeded( task: &PackedTask, cost_fluent: usize, forgos: &[(usize, f64)], groups: &[Vec<u32>], folded_metric: bool, threads: usize, seed: Option<&[usize]>, ) -> Option<SeededResult> |  |  |  |  |

| `pref_weights` | function | pref_weights(domain: &Domain, problem: &Problem) -> HashMap<String, f64> |  |  |  |  |

| `preferences` | function | preferences(goal: &Formula, objs: &HashMap<Sym, Vec<Sym>>) -> Vec<(String, Formula)> |  |  |  |  |

| `(TOTAL-COST)` | str_key | COST_DISP = "(TOTAL-COST)" |  |  |  |  |

| `P3ENDED` | str_key | ENDED = "P3ENDED" |  |  |  |  |

| `P3PLANNING` | str_key | PLANNING = "P3PLANNING" |  |  |  |  |

| `TOTAL-COST` | str_key | COST = "TOTAL-COST" |  |  |  |  |

| `Compiled` | struct | Compiled { pub domain: Domain, pub problem: Problem, pub minimize: bool, pub maximized: bool, pub metric_konst: f64, pub n_prefs: usize, pub warn_other: bool, pub unsupported: Option<String>, pub synthetic: HashSet<String>, pub forgos: Vec<(String, f64)>, pub folded_metric: bool } |  |  |  |  |

| `MetricResult` | struct | MetricResult { pub ops: Vec<usize>, pub cost: f64, pub iterations: usize, pub proven: bool } |  |  |  |  |

| `PhaseTail` | struct | PhaseTail { pub end_op: usize, pub prefs: Vec<(Vec<usize>, usize)> } |  |  |  |  |

| `SeededResult` | struct | SeededResult { pub result: MetricResult, pub from_seed: bool } |  |  |  |  |


### crates/ferroplan/src/plan.rs

| `Validity` | enum | Validity { Valid, Invalid(String) } |  |  |  |  |

| `parse_classical` | function | parse_classical(src: &str) -> Vec<(String, Vec<String>)> |  |  |  |  |

| `parse_timed` | function | parse_timed(src: &str) -> Result<TimedPlan, String> |  |  |  |  |

| `validate_plan` | function | validate_plan( domain_src: &str, problem_src: &str, plan_src: &str, ) -> Result<Validity, String> |  |  |  |  |

| `(define (domain c)` | str_key | CLASSICAL_DOMAIN = "(define (domain c)" |  |  |  |  |

| `(define (domain ct)` | str_key | TEMPORAL_DOMAIN = "(define (domain ct)" |  |  |  |  |

| `(define (problem p) (:domain c)` | str_key | CLASSICAL_PROBLEM = "(define (problem p) (:domain c)" |  |  |  |  |


### crates/ferroplan/src/planner.rs

| `FF_SAT_CLASSICAL` | env_key | std::env::var("FF_SAT_CLASSICAL") |  |  |  |  |

| `run_ff` | function | run_ff(domain_src: &str, problem_src: &str, opts: &crate::Options) -> (String, i32) |  |  |  |  |

| `run_planner` | function | run_planner( domain_src: &str, problem_src: &str, opts: &crate::Options, ipc: bool, ) -> (String, i32) |  |  |  |  |

| `grounding budget reached! no plan found within budget (grounding NOT finished).` | str_key | GROUNDING_WALL_LINE = "grounding budget reached! no plan found within budget (grounding NOT finished)." |  |  |  |  |


### crates/ferroplan/src/planning_runtime.rs

| `PlannerError` | enum | PlannerError { EmptyInitialState, UnknownState { state: String, }, InvalidProbabilityMass { state: String, action: String, mass: u64, }, ResourceBound { resource: String, limit: usize, }, NoPlan, HierarchyCycle { task: String, }, UnknownTask { task: String, }, NoMethod { task: String, }, WorkflowCycle, WipBoundExceeded { queue: String, current: u64, max: u64, }, CapabilityUncovered { item: String, missing: BTreeSet<String>, }, AuthorityUnbound { tool: String, }, VerifierUnbound { tool: String, }, ReceiptUnbound { tool: String, }, InvalidRdfProjection { reason: String, }, Timeout { elapsed_ms: u128, limit_ms: u128, } } |  |  |  |  |

| `solve_planning_type` | function | solve_planning_type( request: &UniversalPlanningRequest, ) -> Result<UniversalPlan, PlannerError> |  |  |  |  |

| `Agent` | struct | Agent { pub id: String, pub capabilities: BTreeSet<String>, pub capacity: u64, pub current_wip: u64 } |  |  |  |  |

| `Goal` | struct | Goal { pub facts: BTreeSet<String>, pub numeric_min: BTreeMap<String, i64>, pub numeric_max: BTreeMap<String, i64> } |  |  |  |  |

| `Method` | struct | Method { pub id: String, pub task: String, pub subtasks: Vec<String> } |  |  |  |  |

| `PlanStep` | struct | PlanStep { pub action: String, pub from: Option<String>, pub to: Option<String>, pub start: u64, pub duration: u64, pub agent: Option<String>, pub tool: Option<String> } |  |  |  |  |

| `PlannerLimits` | struct | PlannerLimits { pub max_depth: usize, pub max_states: usize, pub max_iterations: usize, pub max_wall_ms: u64 } |  |  |  |  |

| `PlanningProblem` | struct | PlanningProblem { pub states: Vec<State>, pub initial_states: Vec<String>, pub goal: Goal, pub unsafe_states: BTreeSet<String>, pub soft_goal_facts: BTreeMap<String, u64>, pub transitions: Vec<Transition>, pub tasks: Vec<Task>, pub root_tasks: Vec<String>, pub methods: Vec<Method>, pub workflow_edges: Vec<WorkflowEdge>, pub queues: Vec<QueueState>, pub agents: Vec<Agent>, pub tools: Vec<Tool>, pub rdf: Vec<RdfTriple> } |  |  |  |  |

| `PolicyEntry` | struct | PolicyEntry { pub state: String, pub action: String, pub outcomes: Vec<PolicyOutcome> } |  |  |  |  |

| `PolicyOutcome` | struct | PolicyOutcome { pub state: String, pub probability_ppm: u32, pub observation: Option<String> } |  |  |  |  |

| `QueueState` | struct | QueueState { pub id: String, pub current_wip: u64, pub max_wip: u64 } |  |  |  |  |

| `RdfTriple` | struct | RdfTriple { pub subject: String, pub predicate: String, pub object: String } |  |  |  |  |

| `State` | struct | State { pub id: String, pub facts: BTreeSet<String>, pub fluents: BTreeMap<String, i64> } |  |  |  |  |

| `Task` | struct | Task { pub id: String, pub primitive_action: Option<String>, pub requires: BTreeSet<String> } |  |  |  |  |

| `Tool` | struct | Tool { pub id: String, pub capabilities: BTreeSet<String>, pub authority_bound: bool, pub verifier_bound: bool, pub receipt_bound: bool } |  |  |  |  |

| `Transition` | struct | Transition { pub action: String, pub from: String, pub to: String, pub cost: u64, pub duration: u64, pub reward: i64, pub probability_ppm: u32, pub observation: Option<String>, pub requires: BTreeSet<String> } |  |  |  |  |

| `UniversalPlan` | struct | UniversalPlan { pub planning_type: Option<PlanningType>, pub solved: bool, pub steps: Vec<PlanStep>, pub policy: Vec<PolicyEntry>, pub decomposition: Vec<String>, pub notes: Vec<String> } |  |  |  |  |

| `UniversalPlanningRequest` | struct | UniversalPlanningRequest { pub planning_type: PlanningType, pub problem: PlanningProblem, pub limits: PlannerLimits } |  |  |  |  |

| `WorkflowEdge` | struct | WorkflowEdge { pub before: String, pub after: String } |  |  |  |  |


### crates/ferroplan/src/planning_types.rs

| `ALL` | const | ALL: [Self; 18] |  |  |  |  |

| `PlanningCapability` | enum | PlanningCapability { DeterministicState, SequentialPlan, ActionCosts, OptimalityProof, NumericFluents, DurativeActions, TemporalValidation, SoftGoals, StochasticTransitions, NondeterministicTransitions, Policy, PolicyValidation, StrongCyclicValidation, BeliefState, OpenLoopPlan, ObservationBranching, CompoundTasks, Methods, PartialOrder, ReceiptJoin, QueueState, WipBounds, ResolutionObligations, AgentCapabilities, CoordinationPolicy, AdmittedGraph, DeterministicProjection, DelegationEnvelope, ToolCapabilities, AuthorityBinding, PrimitiveClosure } |  |  |  |  |

| `PlanningRail` | enum | PlanningRail { NativeDeterministic, NativeProbabilistic, NativeNondeterministic, NativeBeliefState, NativeHierarchical, NativeWorkflow, NativeFlowConstrained, NativeMultiAgent, GraphProjection, Delegation, CapabilityBinding } |  |  |  |  |

| `PlanningRouteError` | enum | PlanningRouteError { EmptySubject, MissingCapabilities { missing: BTreeSet<PlanningCapability>, }, AuthorityUnbound, VerifierUnbound, ReceiptUnbound } |  |  |  |  |

| `PlanningType` | enum | PlanningType { Classical, CostOptimal, Numeric, Temporal, Preferences, Probabilistic, Fond, Conformant, Contingent, Hierarchical, PartialOrder, Workflow, FlowConstrained, ResolutionAdaptive, MultiAgent, RdfDerived, A2aDelegated, McpBound } |  |  |  |  |

| `rail` | function | rail(self) -> PlanningRail |  |  |  |  |

| `required_capabilities` | function | required_capabilities(self) -> BTreeSet<PlanningCapability> |  |  |  |  |

| `route_planning_request` | function | route_planning_request( request: &PlanningRequest, ) -> Result<PlanningRoute, PlanningRouteError> |  |  |  |  |

| `token` | function | token(self) -> &'static str |  |  |  |  |

| `PlanningRequest` | struct | PlanningRequest { pub subject: String, pub planning_type: PlanningType, pub available_capabilities: BTreeSet<PlanningCapability>, pub authority_bound: bool, pub verifier_bound: bool, pub receipt_bound: bool } |  |  |  |  |

| `PlanningRoute` | struct | PlanningRoute { pub subject: String, pub planning_type: PlanningType, pub rail: PlanningRail, pub required_capabilities: BTreeSet<PlanningCapability> } |  |  |  |  |


### crates/ferroplan/src/policy_validation.rs

| `PolicyGuarantee` | enum | PolicyGuarantee { Strong, StrongCyclic, Invalid } |  |  |  |  |

| `PolicyIssue` | enum | PolicyIssue { UnknownInitialState { state: String, }, DuplicatePolicyState { state: String, }, UnknownPolicyState { state: String, }, PolicyOnGoalState { state: String, }, MissingPolicyEntry { state: String, }, UnknownAction { state: String, action: String, }, UnknownTransitionTarget { state: String, action: String, target: String, }, InvalidProbabilityMass { state: String, action: String, mass: u64, }, OutcomeMismatch { state: String, action: String, }, UnsafeReachableState { state: String, }, NoGoalProgress { state: String, } } |  |  |  |  |

| `validate_fond_policy` | function | validate_fond_policy( problem: &PlanningProblem, plan: &UniversalPlan, ) -> PolicyValidationReport |  |  |  |  |

| `PolicyValidationReport` | struct | PolicyValidationReport { pub valid: bool, pub guarantee: PolicyGuarantee, pub reachable_states: Vec<String>, pub reachable_goals: Vec<String>, pub issues: Vec<PolicyIssue> } |  |  |  |  |


### crates/ferroplan/src/portfolio.rs

| `FF_PORTFOLIO_SLICED` | env_key | std::env::var("FF_PORTFOLIO_SLICED") |  |  |  |  |

| `solve` | function | solve(task: &PackedTask, threads: usize, cfg: SearchCfg) -> Outcome |  |  |  |  |

| `Outcome` | struct | Outcome { pub ops: Option<Vec<usize>>, pub evaluated: usize, pub winner: Option<&'static str> } |  |  |  |  |


### crates/ferroplan/src/ppddl.rs

| `PpddlError` | enum | PpddlError { Syntax(String), DomainParse(ParseError), ProblemParse(ParseError), Derived(String), Unsupported(String), InvalidProbability(String), InvalidOptions(String), OutcomeLimit { action: String, limit: usize }, StateLimit { limit: usize }, TransitionLimit { limit: usize }, GroundingFailed, GroundingDivergence { action: String, expected: usize, observed: usize, }, InitialOutcomeLimit { limit: usize }, RewardViolation(String), PolicyLimit { limit: usize }, ValueTableLimit { limit: usize } } |  |  |  |  |

| `ProbabilisticObjective` | enum | ProbabilisticObjective { Auto, MaximizeGoalProbability, MinimizeGoalProbability, MaximizeExpectedReward, MinimizeExpectedReward, MaximizeExpectedMetric, MinimizeExpectedMetric } |  |  |  |  |

| `:PROBABILISTIC-EFFECTS` | str_key | PROB_REQ = ":PROBABILISTIC-EFFECTS" |  |  |  |  |

| `:REWARDS` | str_key | REWARD_REQ = ":REWARDS" |  |  |  |  |

| `PPDDL-A` | str_key | VARIANT_PREFIX = "PPDDL-A" |  |  |  |  |

| `PPDDL-INIT-PENDING` | str_key | INIT_PENDING = "PPDDL-INIT-PENDING" |  |  |  |  |

| `PPDDL-INITIALIZE` | str_key | INIT_ACTION = "PPDDL-INITIALIZE" |  |  |  |  |

| `PPDDL-MARKER-A` | str_key | MARKER_PREFIX = "PPDDL-MARKER-A" |  |  |  |  |

| `InitialStateProbability` | struct | InitialStateProbability { pub state: usize, pub probability: f64, pub goal: bool } |  |  |  |  |

| `PolicyDecision` | struct | PolicyDecision { pub state: usize, pub remaining: Option<usize>, pub action: String, pub args: Vec<String>, pub value: f64, pub outcomes: Vec<PolicyOutcome> } |  |  |  |  |

| `PolicyOutcome` | struct | PolicyOutcome { pub probability: f64, pub next_state: usize, pub reward: f64, pub goal: bool } |  |  |  |  |

| `PolicyValidation` | struct | PolicyValidation { pub valid: bool, pub checked_decisions: usize, pub max_probability_error: f64, pub errors: Vec<String> } |  |  |  |  |

| `PpddlParseReport` | struct | PpddlParseReport { pub ok: bool, pub domain: Option<String>, pub problem: Option<String>, pub probabilistic_actions: usize, pub normalized_outcomes: usize, pub initial_outcomes: usize, pub uses_rewards: bool, pub goal_reward: Option<String>, pub error: Option<String> } |  |  |  |  |

| `ProbabilisticOptions` | struct | ProbabilisticOptions { pub objective: ProbabilisticObjective, pub horizon: Option<usize>, pub discount: f64, pub epsilon: f64, pub max_iterations: usize, pub max_states: usize, pub max_transitions: usize, pub max_outcomes_per_action: usize, pub max_policy_entries: usize, pub max_value_cells: usize, pub max_initial_outcomes: usize, pub simulation_max_steps: usize, pub threads: usize } |  |  |  |  |

| `ProbabilisticSolution` | struct | ProbabilisticSolution { pub solved: bool, pub objective: ProbabilisticObjective, pub initial_value: f64, pub initial_distribution: Vec<InitialStateProbability>, pub states: Vec<ProbabilisticState>, pub initial_action: Option<String>, pub horizon: Option<usize>, pub discount: f64, pub declared_metric: Option<String>, pub policy: Vec<PolicyDecision>, pub statistics: ProbabilisticStatistics, pub notes: Vec<String> } |  |  |  |  |

| `ProbabilisticState` | struct | ProbabilisticState { pub id: usize, pub facts: Vec<String>, pub fluents: BTreeMap<String, f64>, pub goal: bool, pub initial_probability: f64 } |  |  |  |  |

| `ProbabilisticStatistics` | struct | ProbabilisticStatistics { pub grounded_facts: usize, pub grounded_outcome_operators: usize, pub grounded_actions: usize, pub initial_states: usize, pub reachable_states: usize, pub transitions: usize, pub iterations: usize, pub converged: bool, pub threads: usize } |  |  |  |  |

| `SimulationReport` | struct | SimulationReport { pub episodes: usize, pub reached_goal: usize, pub goal_rate: f64, pub average_reward: f64, pub average_discounted_reward: f64, pub average_steps: f64, pub seed: u64 } |  |  |  |  |


### crates/ferroplan/src/ppddl/compile/part07.rs

| `parse_ppddl` | function | parse_ppddl(domain_src: &str, problem_src: &str) -> PpddlParseReport |  |  |  |  |


### crates/ferroplan/src/ppddl/solver/part03.rs

| `solve_ppddl` | function | solve_ppddl( domain_src: &str, problem_src: &str, options: &ProbabilisticOptions, ) -> Result<ProbabilisticSolution, PpddlError> |  |  |  |  |


### crates/ferroplan/src/ppddl/solver/part04.rs

| `validate_ppddl_policy` | function | validate_ppddl_policy( domain_src: &str, problem_src: &str, options: &ProbabilisticOptions, solution: &ProbabilisticSolution, ) -> Result<PolicyValidation, PpddlError> |  |  |  |  |


### crates/ferroplan/src/ppddl/solver/part05.rs

| `simulate_ppddl` | function | simulate_ppddl( domain_src: &str, problem_src: &str, options: &ProbabilisticOptions, episodes: usize, seed: u64, ) -> Result<SimulationReport, PpddlError> |  |  |  |  |


### crates/ferroplan/src/production.rs

| `decompose_production` | function | decompose_production( domain: &str, problem: &str, options: &Options, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Decomposition> |  |  |  |  |

| `goal_met` | function | goal_met(&self) -> bool |  |  |  |  |

| `mind_bytes` | function | mind_bytes(&self) -> usize |  |  |  |  |

| `new` | function | new( domain: &str, problem: &str, options: &Options, limits: ProductionLimits, ) -> Result<Self, PublicError> |  |  |  |  |

| `parse_production` | function | parse_production( source: &str, max_input_bytes: usize, request_id: Option<&str>, ) -> OperationEnvelope<ParseReport> |  |  |  |  |

| `replan` | function | replan( &self, max_evaluated: usize, memory_mb: Option<usize>, request_id: Option<&str>, ) -> OperationEnvelope<Solution> |  |  |  |  |

| `solve_ppddl_production` | function | solve_ppddl_production( domain: &str, problem: &str, options: &ProbabilisticOptions, max_input_bytes: usize, max_output_bytes: usize, request_id: Option<&str>, ) -> OperationEnvelope<ProbabilisticSolution> |  |  |  |  |

| `trace_production` | function | trace_production( domain: &str, problem: &str, plan: &[(String, Vec<String>)], limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Vec<StateSnapshot>> |  |  |  |  |

| `validate_plan_production` | function | validate_plan_production( domain: &str, problem: &str, plan: &str, max_input_bytes: usize, max_plan_bytes: usize, request_id: Option<&str>, ) -> OperationEnvelope<PlanValidationEvidence> |  |  |  |  |

| `world_bytes` | function | world_bytes(&self) -> usize |  |  |  |  |

| `(define (domain smoke) (:requirements :strips) ` | str_key | DOMAIN = "(define (domain smoke) (:requirements :strips) " |  |  |  |  |

| `(define (problem smoke-p) (:domain smoke) ` | str_key | PROBLEM = "(define (problem smoke-p) (:domain smoke) " |  |  |  |  |

| `ferroplan.production-surface.v1` | str_key | PRODUCTION_SURFACE_HASH_DOMAIN = "ferroplan.production-surface.v1" |  |  |  |  |

| `PlanValidationEvidence` | struct | PlanValidationEvidence { pub valid: bool, pub reason: Option<String> } |  |  |  |  |

| `ProductionSession` | struct | ProductionSession { inner: Session, domain: String, problem: String, limits: ProductionLimits, input_fingerprint: String } |  |  |  |  |


### crates/ferroplan/src/production_explain.rs

| `decompose_production` | function | decompose_production( domain: &str, problem: &str, options: &Options, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Decomposition> |  |  |  |  |

| `explain_production` | function | explain_production( domain: &str, problem: &str, plan: &Plan, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Explanation> |  |  |  |  |

| `(define (domain smoke) (:requirements :strips) ` | str_key | DOMAIN = "(define (domain smoke) (:requirements :strips) " |  |  |  |  |

| `(define (problem smoke-p) (:domain smoke) ` | str_key | PROBLEM = "(define (problem smoke-p) (:domain smoke) " |  |  |  |  |

| `ferroplan.production-decompose.v1` | str_key | DECOMPOSE_HASH_DOMAIN = "ferroplan.production-decompose.v1" |  |  |  |  |

| `ferroplan.production-explain.v1` | str_key | EXPLAIN_HASH_DOMAIN = "ferroplan.production-explain.v1" |  |  |  |  |


### crates/ferroplan/src/reachability.rs

| `depth_reached` | function | depth_reached(&self) -> u32 |  |  |  |  |

| `from_predecessors` | function | from_predecessors( predecessors: &[Vec<u32>], prohibited: &[u32], max_depth: u32, ) -> Self |  |  |  |  |

| `from_successors` | function | from_successors( successors: &[Vec<u32>], prohibited: &[u32], max_depth: u32, ) -> Self |  |  |  |  |

| `is_safe` | function | is_safe(&self, state: u32) -> bool |  |  |  |  |

| `saturated` | function | saturated(&self) -> bool |  |  |  |  |

| `unsafe_count` | function | unsafe_count(&self) -> usize |  |  |  |  |

| `BackwardSafeSet` | struct | BackwardSafeSet { n_states: usize, unsafe_words: Vec<u64>, depth_reached: u32, saturated: bool } |  |  |  |  |


### crates/ferroplan/src/readiness.rs

| `CANDIDATE_AUTHORITY` | const | CANDIDATE_AUTHORITY: &str |  |  |  |  |

| `CAPABILITY_MANIFEST_SCHEMA` | const | CAPABILITY_MANIFEST_SCHEMA: &str |  |  |  |  |

| `OPERATION_ENVELOPE_SCHEMA` | const | OPERATION_ENVELOPE_SCHEMA: &str |  |  |  |  |

| `AuthorityClass` | enum | AuthorityClass { CandidateOnly, EvidenceOnly, PresentationOnly } |  |  |  |  |

| `CompatibilityClass` | enum | CompatibilityClass { Semver, VersionedSchema } |  |  |  |  |

| `DeterminismClass` | enum | DeterminismClass { Exact, OutcomeEquivalent, NotApplicable } |  |  |  |  |

| `InterfaceKind` | enum | InterfaceKind { RustLibrary, NativeCli, PythonAbi3, BrowserWasm, BevyGui, McpPlus, Plugin, Documentation, ReleasePipeline } |  |  |  |  |

| `ManifestError` | enum | ManifestError { Schema(String), NonCanonicalOrder, DuplicateId(String), MissingField { id: String, field: String }, MissingEvidence(String), NonCanonicalEvidence(String), MissingSourceIdentity, Serialization } |  |  |  |  |

| `OutcomeClass` | enum | OutcomeClass { Solved, NoPlan, LimitExceeded, Refused, Failed } |  |  |  |  |

| `ReadinessState` | enum | ReadinessState { Unknown, Declared, Partial, Admitted, Blocked, Unsupported, Refused } |  |  |  |  |

| `ReplayClass` | enum | ReplayClass { Exact, Outcome, BuildReproducible, NotApplicable } |  |  |  |  |

| `SecurityClass` | enum | SecurityClass { UntrustedInput, LocalPresentation, BuildControl } |  |  |  |  |

| `ValidationStatus` | enum | ValidationStatus { Valid, NotApplicable, Failed } |  |  |  |  |

| `capability_manifest` | function | capability_manifest() -> CapabilityManifest |  |  |  |  |

| `evaluate_readiness` | function | evaluate_readiness( source_identity: impl Into<String>, evidence: I, ) -> Result<ReadinessReport, ManifestError> |  |  |  |  |

| `fingerprint` | function | fingerprint(&self) -> Result<String, ManifestError> |  |  |  |  |

| `new` | function | new(code: impl Into<String>, message: impl Into<String>, retryable: bool) -> Self |  |  |  |  |

| `production_input_fingerprint` | function | production_input_fingerprint(domain: &str, problem: &str, options: &Options) -> String |  |  |  |  |

| `solve_production` | function | solve_production( domain: &str, problem: &str, options: &Options, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Solution> |  |  |  |  |

| `validate` | function | validate(&self) -> Result<(), ManifestError> |  |  |  |  |

| `(define (domain smoke) (:requirements :strips) ` | str_key | DOMAIN = "(define (domain smoke) (:requirements :strips) " |  |  |  |  |

| `(define (problem smoke-p) (:domain smoke) ` | str_key | PROBLEM = "(define (problem smoke-p) (:domain smoke) " |  |  |  |  |

| `candidate_only` | str_key | CANDIDATE_AUTHORITY = "candidate_only" |  |  |  |  |

| `ferroplan.capabilities.v1` | str_key | CAPABILITY_MANIFEST_SCHEMA = "ferroplan.capabilities.v1" |  |  |  |  |

| `ferroplan.capability-manifest.v1` | str_key | MANIFEST_HASH_DOMAIN = "ferroplan.capability-manifest.v1" |  |  |  |  |

| `ferroplan.operation.v1` | str_key | OPERATION_ENVELOPE_SCHEMA = "ferroplan.operation.v1" |  |  |  |  |

| `ferroplan.production-input.v1` | str_key | INPUT_HASH_DOMAIN = "ferroplan.production-input.v1" |  |  |  |  |

| `BuildIdentity` | struct | BuildIdentity { pub product_version: String, pub source_revision: Option<String>, pub manifest_fingerprint: Option<String> } |  |  |  |  |

| `CapabilityContract` | struct | CapabilityContract { pub id: String, pub version: String, pub owner: String, pub component: String, pub interface: InterfaceKind, pub authority: AuthorityClass, pub determinism: DeterminismClass, pub replay: ReplayClass, pub input_schema: String, pub output_schema: String, pub resource_profile: String, pub failure_contract: String, pub telemetry_contract: String, pub compatibility: CompatibilityClass, pub security: SecurityClass, pub shipped: bool, pub required_evidence: Vec<String> } |  |  |  |  |

| `CapabilityEvaluation` | struct | CapabilityEvaluation { pub capability_id: String, pub state: ReadinessState, pub satisfied_evidence: Vec<String>, pub missing_evidence: Vec<String> } |  |  |  |  |

| `CapabilityManifest` | struct | CapabilityManifest { pub schema_version: String, pub product_version: String, pub authority_notice: String, pub capabilities: Vec<CapabilityContract> } |  |  |  |  |

| `OperationEnvelope` | struct | OperationEnvelope { pub schema_version: String, pub request_id: String, pub capability_id: String, pub capability_version: String, pub build_identity: BuildIdentity, pub input_fingerprint: String, pub authority: String, pub outcome: OutcomeClass, pub validation: ValidationStatus, pub elapsed_micros: u64, pub counters: BTreeMap<String, u64>, pub warnings: Vec<String>, pub payload: Option<T>, pub error: Option<PublicError> } |  |  |  |  |

| `ProductionLimits` | struct | ProductionLimits { pub max_domain_bytes: usize, pub max_problem_bytes: usize, pub max_evaluated: usize, pub max_plan_steps: usize, pub max_output_bytes: usize, pub max_workers: usize } |  |  |  |  |

| `PublicError` | struct | PublicError { pub code: String, pub message: String, pub retryable: bool } |  |  |  |  |

| `ReadinessReport` | struct | ReadinessReport { pub schema_version: String, pub product_version: String, pub source_identity: String, pub manifest_fingerprint: String, pub evaluator_version: String, pub overall_state: ReadinessState, pub capabilities: Vec<CapabilityEvaluation> } |  |  |  |  |


### crates/ferroplan/src/report.rs

| `ff_plan` | function | ff_plan(task: &PackedTask, ops: &[usize]) -> String |  |  |  |  |

| `ipc_plan` | function | ipc_plan(task: &PackedTask, ops: &[usize], metric: Option<f64>) -> String |  |  |  |  |

| `metric_footer` | function | metric_footer( cost: f64, iterations: usize, n_prefs: usize, threads: usize, warn_other: bool, ) -> String |  |  |  |  |

| `preamble` | function | preamble(threads: usize) -> String |  |  |  |  |

| `timing` | function | timing(stats: &Stats, threads: usize) -> String |  |  |  |  |


### crates/ferroplan/src/resolve.rs

| `Solved` | enum | Solved { Plan(Vec<usize>, Stats), Unsolvable { capped: bool, } } |  |  |  |  |

| `FF_NO_LAMA` | env_key | std::env::var("FF_NO_LAMA") |  |  |  |  |

| `FF_RESLM` | env_key | std::env::var("FF_RESLM") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `solve` | function | solve( task: &PackedTask, threads: usize, cfg: crate::search::SearchCfg, mutex_groups: &[Vec<u32>], orbit: Option<&crate::orbits::OrbitMap>, ) -> Solved |  |  |  |  |

| `Stats` | struct | Stats { pub init_groups: usize, pub final_groups: usize, pub merges: usize, pub fallback: bool } |  |  |  |  |


### crates/ferroplan/src/resource.rs

| `detect_resources` | function | detect_resources(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Vec<ResourceVar> |  |  |  |  |

| `occupancy` | function | occupancy(&self, bits: &[u64]) -> u32 |  |  |  |  |

| `trip_bound` | function | trip_bound(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Option<TripBound> |  |  |  |  |

| `trips` | function | trips(&self, bits: &[u64]) -> i64 |  |  |  |  |

| `(define (domain ctr) (:requirements :typing)
      (:types count)
      (:predicates (avail ?s - count) (nxt ?lo ?hi - count))
      (:action consume :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?b ?a))
        :effect (and (not (avail ?a)) (avail ?b)))
      (:action restore :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?a ?b))
        :effect (and (not (avail ?a)) (avail ?b))))` | str_key | DOM = "(define (domain ctr) (:requirements :typing)
      (:types count)
      (:predicates (avail ?s - count) (nxt ?lo ?hi - count))
      (:action consume :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?b ?a))
        :effect (and (not (avail ?a)) (avail ?b)))
      (:action restore :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?a ?b))
        :effect (and (not (avail ?a)) (avail ?b))))" |  |  |  |  |

| `(define (domain tinytrans)
      (:requirements :strips :typing)
      (:types loc pkg cap)
      (:predicates (tat ?l - loc) (pat ?p - pkg ?l - loc) (pin ?p - pkg)
                   (cap ?c - cap) (nxt ?a ?b - cap))
      (:action mv :parameters (?a ?b - loc)
        :precondition (tat ?a) :effect (and (not (tat ?a)) (tat ?b)))
      (:action pick :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pat ?p ?l) (nxt ?a ?b) (cap ?b))
        :effect (and (not (pat ?p ?l)) (pin ?p) (cap ?a) (not (cap ?b))))
      (:action drop :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pin ?p) (nxt ?a ?b) (cap ?a))
        :effect (and (not (pin ?p)) (pat ?p ?l) (cap ?b) (not (cap ?a)))))` | str_key | TDOM = "(define (domain tinytrans)
      (:requirements :strips :typing)
      (:types loc pkg cap)
      (:predicates (tat ?l - loc) (pat ?p - pkg ?l - loc) (pin ?p - pkg)
                   (cap ?c - cap) (nxt ?a ?b - cap))
      (:action mv :parameters (?a ?b - loc)
        :precondition (tat ?a) :effect (and (not (tat ?a)) (tat ?b)))
      (:action pick :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pat ?p ?l) (nxt ?a ?b) (cap ?b))
        :effect (and (not (pat ?p ?l)) (pin ?p) (cap ?a) (not (cap ?b))))
      (:action drop :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pin ?p) (nxt ?a ?b) (cap ?a))
        :effect (and (not (pin ?p)) (pat ?p ?l) (cap ?b) (not (cap ?a)))))" |  |  |  |  |

| `(define (problem ctr1) (:domain ctr)
      (:objects c0 c1 c2 c3 - count)
      (:init (avail c3) (nxt c0 c1) (nxt c1 c2) (nxt c2 c3))
      (:goal (avail c0)))` | str_key | PROB = "(define (problem ctr1) (:domain ctr)
      (:objects c0 c1 c2 c3 - count)
      (:init (avail c3) (nxt c0 c1) (nxt c1 c2) (nxt c2 c3))
      (:goal (avail c0)))" |  |  |  |  |

| `(define (problem tt1) (:domain tinytrans)
      (:objects l1 l2 l3 - loc p1 p2 p3 - pkg c0 c1 c2 - cap)
      (:init (tat l1) (pat p1 l1) (pat p2 l1) (pat p3 l1)
             (cap c2) (nxt c0 c1) (nxt c1 c2))
      (:goal (and (pat p1 l2) (pat p2 l2) (pat p3 l3))))` | str_key | TPROB = "(define (problem tt1) (:domain tinytrans)
      (:objects l1 l2 l3 - loc p1 p2 p3 - pkg c0 c1 c2 - cap)
      (:init (tat l1) (pat p1 l1) (pat p2 l1) (pat p3 l1)
             (cap c2) (nxt c0 c1) (nxt c1 c2))
      (:goal (and (pat p1 l2) (pat p2 l2) (pat p3 l3))))" |  |  |  |  |

| `ResourceVar` | struct | ResourceVar { pub members: Vec<(u32, u32)> } |  |  |  |  |

| `TripBound` | struct | TripBound { pub goals: Vec<u32>, pub pool: i64 } |  |  |  |  |


### crates/ferroplan/src/sat.rs

| `FF_NO_SAT_LAYERGEN` | env_key | std::env::var("FF_NO_SAT_LAYERGEN") |  |  |  |  |

| `FF_NO_SAT_RATEBAIL` | env_key | std::env::var("FF_NO_SAT_RATEBAIL") |  |  |  |  |

| `FF_SAT_BRANCH` | env_key | std::env::var("FF_SAT_BRANCH") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `from_env` | function | from_env() -> Self |  |  |  |  |

| `requires_concurrency` | function | requires_concurrency(domain: &Domain, problem: &Problem) -> bool |  |  |  |  |

| `solve_classical` | function | solve_classical( task: &PackedTask, groups: &[Vec<u32>], cfg: &SatCfg, ) -> SatOutcome<Vec<usize>> |  |  |  |  |

| `solve_temporal` | function | solve_temporal( domain: &Domain, problem: &Problem, threads: usize, cfg: &SatCfg, ) -> SatOutcome<TimedPlan> |  |  |  |  |

| `solve_temporal_within` | function | solve_temporal_within( domain: &Domain, problem: &Problem, threads: usize, cfg: &SatCfg, budget_secs: Option<f64>, ) -> SatOutcome<TimedPlan> |  |  |  |  |

| `SatCfg` | struct | SatCfg { pub max_horizon: usize, pub conflicts_per_horizon: u64, pub cap_lits: u64 } |  |  |  |  |

| `SatOutcome` | struct | SatOutcome { pub plan: Option<P>, pub notes: Vec<String>, pub proven_at_every_horizon: bool, pub grounded_facts: usize, pub grounded_actions: usize } |  |  |  |  |


### crates/ferroplan/src/search.rs

| `DEFAULT_MAX_EVAL` | const | DEFAULT_MAX_EVAL: usize |  |  |  |  |

| `PlanResult` | enum | PlanResult { Plan { ops: Vec<usize>, advance: Vec<i32>, evaluated: usize, max_g: usize, }, Unsolvable { evaluated: usize, capped: bool, } } |  |  |  |  |

| `FF_CLM` | env_key | std::env::var("FF_CLM") |  |  |  |  |

| `FF_HTRACE` | env_key | std::env::var("FF_HTRACE") |  |  |  |  |

| `FF_LEN_ANYTIME` | env_key | std::env::var("FF_LEN_ANYTIME") |  |  |  |  |

| `FF_MEM_BUDGET_GB` | env_key | std::env::var("FF_MEM_BUDGET_GB") |  |  |  |  |

| `FF_NOVDRIVER_ONLY` | env_key | std::env::var("FF_NOVDRIVER_ONLY") |  |  |  |  |

| `FF_NOVELTY` | env_key | std::env::var("FF_NOVELTY") |  |  |  |  |

| `FF_NOVELTY_ONLY` | env_key | std::env::var("FF_NOVELTY_ONLY") |  |  |  |  |

| `FF_NOVLIGHT` | env_key | std::env::var("FF_NOVLIGHT") |  |  |  |  |

| `FF_NOVLIGHT_ONLY` | env_key | std::env::var("FF_NOVLIGHT_ONLY") |  |  |  |  |

| `FF_NOV_OLD` | env_key | std::env::var("FF_NOV_OLD") |  |  |  |  |

| `FF_NO_EHC_WALLCAP` | env_key | std::env::var("FF_NO_EHC_WALLCAP") |  |  |  |  |

| `FF_NO_ENRICH` | env_key | std::env::var("FF_NO_ENRICH") |  |  |  |  |

| `FF_NO_LAMA` | env_key | std::env::var("FF_NO_LAMA") |  |  |  |  |

| `FF_NO_NODECAP_REFILL` | env_key | std::env::var("FF_NO_NODECAP_REFILL") |  |  |  |  |

| `FF_NO_NOVELTY` | env_key | std::env::var("FF_NO_NOVELTY") |  |  |  |  |

| `FF_NO_NOVLIGHT` | env_key | std::env::var("FF_NO_NOVLIGHT") |  |  |  |  |

| `FF_NO_REFILL` | env_key | std::env::var("FF_NO_REFILL") |  |  |  |  |

| `FF_NO_RUNG_WALLCAP` | env_key | std::env::var("FF_NO_RUNG_WALLCAP") |  |  |  |  |

| `FF_REPORT_RESERVE_SECS` | env_key | std::env::var("FF_REPORT_RESERVE_SECS") |  |  |  |  |

| `FF_RESLM` | env_key | std::env::var("FF_RESLM") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_SEARCH_NODE_CAP` | env_key | std::env::var("FF_SEARCH_NODE_CAP") |  |  |  |  |

| `FF_TIME_LIMIT` | env_key | std::env::var("FF_TIME_LIMIT") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `arm_wall_limit` | function | arm_wall_limit() |  |  |  |  |

| `cancelled` | function | cancelled(&self) -> bool |  |  |  |  |

| `cost` | function | cost(&self, s: &State) -> f64 |  |  |  |  |

| `from_weights` | function | from_weights(weight_g: f64, weight_h: f64, max_eval: Option<usize>) -> Self |  |  |  |  |

| `holds` | function | holds(&self, s: &State) -> bool |  |  |  |  |

| `plan` | function | plan( task: &PackedTask, threads: usize, cfg: SearchCfg, ehc_first: bool, orbit: Option<&crate::orbits::OrbitMap>, ) -> PlanOutcome |  |  |  |  |

| `plan_avoiding` | function | plan_avoiding( task: &PackedTask, threads: usize, cfg: SearchCfg, ehc_first: bool, forbidden: &[bool], orbit: Option<&crate::orbits::OrbitMap>, ) -> PlanOutcome |  |  |  |  |

| `search` | function | search(task: &PackedTask, threads: usize, cfg: SearchCfg) -> PlanResult |  |  |  |  |

| `search_from` | function | search_from( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: Option<usize>, cost_bound: f64, threads: usize, cfg: SearchCfg, forbidden: &[bool], sat: Option<&SatGuidance>, closure: Option<&ClosureCost>, orbit: Option<&crate::orbits::OrbitMap>, ) -> PlanResult |  |  |  |  |

| `solve_closure_bounded` | function | solve_closure_bounded( task: &PackedTask, goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, bound: f64, closure: &ClosureCost, forbidden: &[bool], threads: usize, cfg: SearchCfg, sat: Option<&SatGuidance>, ) -> (Option<Vec<usize>>, usize, bool) |  |  |  |  |

| `solve_subgoal` | function | solve_subgoal( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], threads: usize, cfg: SearchCfg, orbit: Option<&crate::orbits::OrbitMap>, ) -> Option<Vec<usize>> |  |  |  |  |

| `solve_subgoal_avoiding` | function | solve_subgoal_avoiding( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], forbidden: &[bool], threads: usize, cfg: SearchCfg, ) -> Option<Vec<usize>> |  |  |  |  |

| `solve_subgoal_bounded` | function | solve_subgoal_bounded( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, bound: f64, threads: usize, cfg: SearchCfg, sat: Option<&SatGuidance>, ) -> (Option<Vec<usize>>, usize, bool) |  |  |  |  |

| `solve_subgoal_guided` | function | solve_subgoal_guided( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], forbidden: &[bool], threads: usize, cfg: SearchCfg, sat: Option<&SatGuidance>, ) -> (Option<Vec<usize>>, usize) |  |  |  |  |

| `with_cost_h` | function | with_cost_h(mut self, cost_fluent: usize) -> Self |  |  |  |  |

| `with_cost_weight` | function | with_cost_weight(mut self, w_c: f64) -> Self |  |  |  |  |

| `ClosureCost` | struct | ClosureCost { pub prefs: Vec<(f64, PrefPhi)> } |  |  |  |  |

| `PlanOutcome` | struct | PlanOutcome { pub ops: Option<Vec<usize>>, pub evaluated: usize, pub ehc_fell_back: bool, pub capped: bool } |  |  |  |  |

| `PrefPhi` | struct | PrefPhi { pub disjuncts: Vec<(Vec<u32>, Vec<NumPre>)> } |  |  |  |  |

| `SatGuidance` | struct | SatGuidance { pub prefs: Vec<(PrefPhi, i64)>, pub res: Vec<crate::resource::ResourceVar>, pub res_weight: i64, pub res_thresh: i64, pub deadline: Vec<(u32, u32, i64)>, pub deadline_weight: i64 } |  |  |  |  |

| `SearchCfg` | struct | SearchCfg { pub w_g: i64, pub w_h: i64, pub max_eval: usize, pub w_c: f64, pub h_cost: Option<usize>, pub anytime: bool, pub g_bound: usize, pub len_anytime: bool, pub w_lm: i64, pub w_res: i64, pub pref_ops: bool, pub node_bytes_target: Option<usize>, pub deadline: Option<(crate::clock::Clock, f64)>, pub ehc_wall_frac: Option<f64> } |  |  |  |  |


### crates/ferroplan/src/selection.rs

| `select` | function | select( task: &PackedTask, groups: &[Vec<u32>], weights: &[f64], dnf: &FxHashMap<usize, Vec<Vec<u32>>>, banned: &crate::hash::FxHashSet<u32>, ) -> Option<Selection> |  |  |  |  |

| `Selection` | struct | Selection { pub chosen: Vec<(usize, Vec<u32>)>, pub bound: f64, pub capped: bool } |  |  |  |  |


### crates/ferroplan/src/session.rs

| `ThinkVerdict` | enum | ThinkVerdict { Solved, Capped, Exhausted } |  |  |  |  |

| `apply_start` | function | apply_start(&mut self, name: &str) -> Result<(), String> |  |  |  |  |

| `elapse` | function | elapse(&mut self, dt: f64) -> Result<Vec<String>, String> |  |  |  |  |

| `fact` | function | fact(&self, name: &str) -> Option<bool> |  |  |  |  |

| `fluent` | function | fluent(&self, name: &str) -> Option<f64> |  |  |  |  |

| `fork` | function | fork(&self) -> Session |  |  |  |  |

| `goal_met` | function | goal_met(&self) -> bool |  |  |  |  |

| `mind_bytes` | function | mind_bytes(&self) -> usize |  |  |  |  |

| `new` | function | new(domain_src: &str, problem_src: &str, opts: &Options) -> Result<Session, String> |  |  |  |  |

| `observe` | function | observe(&mut self, sight: &[(&str, bool)]) -> Result<Vec<String>, String> |  |  |  |  |

| `plan_still_valid` | function | plan_still_valid(&self, plan: &Plan, from_step: usize) -> bool |  |  |  |  |

| `replan` | function | replan(&self) -> Solution |  |  |  |  |

| `replan_budgeted` | function | replan_budgeted(&self, max_evaluated: usize, memory_mb: Option<usize>) -> Solution |  |  |  |  |

| `replan_following` | function | replan_following( &self, prior: &Plan, from_step: usize, max_evaluated: usize, memory_mb: Option<usize>, ) -> Solution |  |  |  |  |

| `restrict_ops` | function | restrict_ops(&mut self, mut keep: impl FnMut(&str) -> bool) |  |  |  |  |

| `set_fact` | function | set_fact(&mut self, name: &str, value: bool) -> Result<(), String> |  |  |  |  |

| `set_fluent` | function | set_fluent(&mut self, name: &str, value: f64) -> Result<(), String> |  |  |  |  |

| `set_goal` | function | set_goal(&mut self, goal: &str) -> Result<(), String> |  |  |  |  |

| `set_timed_fact` | function | set_timed_fact(&mut self, dt: f64, name: &str, value: bool) -> Result<(), String> |  |  |  |  |

| `state_fingerprint` | function | state_fingerprint(&self) -> String |  |  |  |  |

| `think` | function | think(&self, budget: &ThinkBudget) -> Think |  |  |  |  |

| `think_following` | function | think_following(&self, prior: &Plan, from_step: usize, budget: &ThinkBudget) -> Think |  |  |  |  |

| `world_bytes` | function | world_bytes(&self) -> usize |  |  |  |  |

| `
        (define (domain gate) (:requirements :strips :numeric-fluents)
          (:predicates (done))
          (:functions (permit))
          (:action act :precondition (>= (permit) 1) :effect (done)))` | str_key | GDOM = "
        (define (domain gate) (:requirements :strips :numeric-fluents)
          (:predicates (done))
          (:functions (permit))
          (:action act :precondition (>= (permit) 1) :effect (done)))" |  |  |  |  |

| `
        (define (domain shop)
          (:requirements :strips :typing :durative-actions :numeric-fluents)
          (:types worker)
          (:predicates (idle ?w - worker) (built ?w - worker))
          (:functions (build-time ?w - worker))
          (:durative-action build
            :parameters (?w - worker)
            :duration (= ?duration (build-time ?w))
            :condition (at start (idle ?w))
            :effect (and (at start (not (idle ?w))) (at end (built ?w)))))` | str_key | DDOM = "
        (define (domain shop)
          (:requirements :strips :typing :durative-actions :numeric-fluents)
          (:types worker)
          (:predicates (idle ?w - worker) (built ?w - worker))
          (:functions (build-time ?w - worker))
          (:durative-action build
            :parameters (?w - worker)
            :duration (= ?duration (build-time ?w))
            :condition (at start (idle ?w))
            :effect (and (at start (not (idle ?w))) (at end (built ?w)))))" |  |  |  |  |

| `
        (define (problem g) (:domain gate)
          (:init (= (permit) 0))
          (:goal (done)))` | str_key | GPRB = "
        (define (problem g) (:domain gate)
          (:init (= (permit) 0))
          (:goal (done)))" |  |  |  |  |

| `
        (define (problem job) (:domain shop)
          (:objects w1 - worker)
          (:init (idle w1) (= (build-time w1) 5))
          (:goal (built w1)))` | str_key | DPRB = "
        (define (problem job) (:domain shop)
          (:objects w1 - worker)
          (:init (idle w1) (= (build-time w1) 5))
          (:goal (built w1)))" |  |  |  |  |

| `
    (define (domain farm) (:requirements :strips :typing :numeric-fluents)
      (:types agent place)
      (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
      (:functions (grain))
      (:action walk :parameters (?a - agent ?from ?to - place)
        :precondition (and (at ?a ?from) (road ?from ?to))
        :effect (and (not (at ?a ?from)) (at ?a ?to)))
      (:action harvest :parameters (?a - agent ?p - place)
        :precondition (and (at ?a ?p) (fertile ?p))
        :effect (increase (grain) 1)))` | str_key | DOM = "
    (define (domain farm) (:requirements :strips :typing :numeric-fluents)
      (:types agent place)
      (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
      (:functions (grain))
      (:action walk :parameters (?a - agent ?from ?to - place)
        :precondition (and (at ?a ?from) (road ?from ?to))
        :effect (and (not (at ?a ?from)) (at ?a ?to)))
      (:action harvest :parameters (?a - agent ?p - place)
        :precondition (and (at ?a ?p) (fertile ?p))
        :effect (increase (grain) 1)))" |  |  |  |  |

| `
    (define (domain lamp) (:requirements :strips :negative-preconditions)
      (:predicates (on) (broken))
      (:action switch-on :precondition (and (not (on)) (not (broken))) :effect (on))
      (:action switch-off :precondition (on) :effect (not (on))))` | str_key | NEG_DOM = "
    (define (domain lamp) (:requirements :strips :negative-preconditions)
      (:predicates (on) (broken))
      (:action switch-on :precondition (and (not (on)) (not (broken))) :effect (on))
      (:action switch-off :precondition (on) :effect (not (on))))" |  |  |  |  |

| `
    (define (domain rollers) (:requirements :strips :typing)
      (:types ball room)
      (:predicates (at ?b - ball ?r - room) (link ?x ?y - room)
                   (goal-room ?r - room) (home ?b - ball))
      (:action roll :parameters (?b - ball ?from ?to - room)
        :precondition (and (at ?b ?from) (link ?from ?to))
        :effect (and (not (at ?b ?from)) (at ?b ?to)))
      (:action park :parameters (?b - ball ?r - room)
        :precondition (and (at ?b ?r) (goal-room ?r))
        :effect (home ?b)))` | str_key | ORB_DOM = "
    (define (domain rollers) (:requirements :strips :typing)
      (:types ball room)
      (:predicates (at ?b - ball ?r - room) (link ?x ?y - room)
                   (goal-room ?r - room) (home ?b - ball))
      (:action roll :parameters (?b - ball ?from ?to - room)
        :precondition (and (at ?b ?from) (link ?from ?to))
        :effect (and (not (at ?b ?from)) (at ?b ?to)))
      (:action park :parameters (?b - ball ?r - room)
        :precondition (and (at ?b ?r) (goal-room ?r))
        :effect (home ?b)))" |  |  |  |  |

| `
    (define (domain seqshop) (:requirements :strips :typing :durative-actions)
      (:types w)
      (:predicates (idle ?x - w) (staged ?x - w) (built ?x - w) (power))
      (:durative-action stage1 :parameters (?x - w) :duration (= ?duration 5)
        :condition (at start (idle ?x))
        :effect (and (at start (not (idle ?x))) (at end (staged ?x))))
      (:durative-action stage2 :parameters (?x - w) :duration (= ?duration 5)
        :condition (and (at start (staged ?x)) (at start (power)))
        :effect (at end (built ?x)))
      (:durative-action grid :parameters () :duration (= ?duration 1)
        :condition (at start (power))
        :effect (and (at start (not (power))) (at end (power)))))` | str_key | SEQ_DOM = "
    (define (domain seqshop) (:requirements :strips :typing :durative-actions)
      (:types w)
      (:predicates (idle ?x - w) (staged ?x - w) (built ?x - w) (power))
      (:durative-action stage1 :parameters (?x - w) :duration (= ?duration 5)
        :condition (at start (idle ?x))
        :effect (and (at start (not (idle ?x))) (at end (staged ?x))))
      (:durative-action stage2 :parameters (?x - w) :duration (= ?duration 5)
        :condition (and (at start (staged ?x)) (at start (power)))
        :effect (at end (built ?x)))
      (:durative-action grid :parameters () :duration (= ?duration 1)
        :condition (at start (power))
        :effect (and (at start (not (power))) (at end (power)))))" |  |  |  |  |

| `
    (define (domain shop) (:requirements :strips :typing :durative-actions)
      (:types job machine)
      (:predicates (todo ?j - job) (done ?j - job) (up ?m - machine) (fast ?m - machine)
                   (slow ?m - machine))
      (:durative-action run-fast :parameters (?j - job ?m - machine)
        :duration (= ?duration 2)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (fast ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action run-slow :parameters (?j - job ?m - machine)
        :duration (= ?duration 8)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (slow ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action maintain :parameters (?m - machine)
        :duration (= ?duration 1)
        :condition (at start (up ?m))
        :effect (and (at start (not (up ?m))) (at end (up ?m)))))` | str_key | SHOP_DOM = "
    (define (domain shop) (:requirements :strips :typing :durative-actions)
      (:types job machine)
      (:predicates (todo ?j - job) (done ?j - job) (up ?m - machine) (fast ?m - machine)
                   (slow ?m - machine))
      (:durative-action run-fast :parameters (?j - job ?m - machine)
        :duration (= ?duration 2)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (fast ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action run-slow :parameters (?j - job ?m - machine)
        :duration (= ?duration 8)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (slow ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action maintain :parameters (?m - machine)
        :duration (= ?duration 1)
        :condition (at start (up ?m))
        :effect (and (at start (not (up ?m))) (at end (up ?m)))))" |  |  |  |  |

| `
    (define (domain workshop) (:requirements :strips :typing :durative-actions)
      (:types worker)
      (:predicates (idle ?w - worker) (built ?w - worker))
      (:durative-action build
        :parameters (?w - worker)
        :duration (= ?duration 5)
        :condition (at start (idle ?w))
        :effect (and (at start (not (idle ?w))) (at end (built ?w)))))` | str_key | TDOM = "
    (define (domain workshop) (:requirements :strips :typing :durative-actions)
      (:types worker)
      (:predicates (idle ?w - worker) (built ?w - worker))
      (:durative-action build
        :parameters (?w - worker)
        :duration (= ?duration 5)
        :condition (at start (idle ?w))
        :effect (and (at start (not (idle ?w))) (at end (built ?w)))))" |  |  |  |  |

| `
    (define (problem p) (:domain farm)
      (:objects v1 - agent hut field - place)
      (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
      (:goal (>= (grain) 2)))` | str_key | PRB = "
    (define (problem p) (:domain farm)
      (:objects v1 - agent hut field - place)
      (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
      (:goal (>= (grain) 2)))" |  |  |  |  |

| `
    (define (problem p) (:domain lamp)
      (:init) (:goal (on)))` | str_key | NEG_PRB = "
    (define (problem p) (:domain lamp)
      (:init) (:goal (on)))" |  |  |  |  |

| `
    (define (problem p) (:domain rollers)
      (:objects b1 b2 - ball ra rb - room)
      (:init (at b1 ra) (at b2 ra) (link ra rb) (link rb ra) (goal-room rb))
      (:goal (and (home b1) (home b2))))` | str_key | ORB_PRB = "
    (define (problem p) (:domain rollers)
      (:objects b1 b2 - ball ra rb - room)
      (:init (at b1 ra) (at b2 ra) (link ra rb) (link rb ra) (goal-room rb))
      (:goal (and (home b1) (home b2))))" |  |  |  |  |

| `
    (define (problem p) (:domain seqshop)
      (:objects w1 - w)
      (:init (idle w1) (power))
      (:goal (built w1)))` | str_key | SEQ_PRB = "
    (define (problem p) (:domain seqshop)
      (:objects w1 - w)
      (:init (idle w1) (power))
      (:goal (built w1)))" |  |  |  |  |

| `
    (define (problem p) (:domain shop)
      (:objects j1 j2 - job f s - machine)
      (:init (todo j1) (todo j2) (up f) (up s) (fast f) (slow s))
      (:goal (and (done j1) (done j2))))` | str_key | SHOP_PRB = "
    (define (problem p) (:domain shop)
      (:objects j1 j2 - job f s - machine)
      (:init (todo j1) (todo j2) (up f) (up s) (fast f) (slow s))
      (:goal (and (done j1) (done j2))))" |  |  |  |  |

| `
    (define (problem shift) (:domain workshop)
      (:objects w1 w2 - worker)
      (:init (idle w1) (idle w2))
      (:goal (and (built w1) (built w2))))` | str_key | TPRB = "
    (define (problem shift) (:domain workshop)
      (:objects w1 w2 - worker)
      (:init (idle w1) (idle w2))
      (:goal (and (built w1) (built w2))))" |  |  |  |  |

| `urn:ferroplan:session-state:v1` | str_key | DOMAIN = "urn:ferroplan:session-state:v1" |  |  |  |  |

| `Session` | struct | Session { task: PackedTask, threads: usize, weight_g: f64, weight_h: f64, max_evaluated: Option<usize>, ehc_first: bool, fact_ids: Arc<FxHashMap<String, u32>>, dynamic: Arc<[bool]>, fluent_ids: Arc<FxHashMap<String, u32>>, temporal: Option<Arc<crate::temporal::TemporalCompiled>>, tier: crate::features::DemandMode, running_preds: Vec<String>, op_ids: Arc<FxHashMap<String, usize>>, mirror: Arc<FxHashMap<u32, u32>>, forbidden: Vec<bool>, timed: Vec<(f64, u32, bool)>, til_setters: Arc<FxHashMap<(u32, bool), usize>>, running: Vec<(f64, usize)>, lifted: Option<Arc<(crate::types::Domain, crate::types::Problem)>>, goal_formula: Formula } |  |  |  |  |

| `Think` | struct | Think { pub solution: Solution, pub capped: bool, pub spent_ms: u64, pub spent_evals: usize, pub verdict: ThinkVerdict } |  |  |  |  |

| `ThinkBudget` | struct | ThinkBudget { pub max_evaluated: Option<usize>, pub wall_ms: Option<u64>, pub memory_mb: Option<usize> } |  |  |  |  |


### crates/ferroplan/src/tcompress.rs

| `UNWALLED_EVALS` | const | UNWALLED_EVALS: usize |  |  |  |  |

| `WALL_FRAC` | const | WALL_FRAC: f64 |  |  |  |  |

| `Bet` | enum | Bet { First, Rest } |  |  |  |  |

| `FF_NO_TCOMPRESS` | env_key | std::env::var("FF_NO_TCOMPRESS") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> (Domain, Problem) |  |  |  |  |

| `declines` | function | declines(domain: &Domain, problem: &Problem) -> Option<&'static str> |  |  |  |  |

| `lay_out` | function | lay_out( domain: &Domain, task: &PackedTask, ops: &[usize], shift: bool, ) -> Option<TimedPlan> |  |  |  |  |

| `solve` | function | solve(domain: &Domain, problem: &Problem, threads: usize, bet: Bet) -> Option<TimedPlan> |  |  |  |  |


### crates/ferroplan/src/temporal.rs

| `FF_H_ENDGATE` | env_key | std::env::var("FF_H_ENDGATE") |  |  |  |  |

| `FF_LAX_HELPFUL` | env_key | std::env::var("FF_LAX_HELPFUL") |  |  |  |  |

| `FF_NOREL` | env_key | std::env::var("FF_NOREL") |  |  |  |  |

| `FF_NO_LADDER_DEDUP` | env_key | std::env::var("FF_NO_LADDER_DEDUP") |  |  |  |  |

| `FF_NO_SAT` | env_key | std::env::var("FF_NO_SAT") |  |  |  |  |

| `FF_NO_TSUCC` | env_key | std::env::var("FF_NO_TSUCC") |  |  |  |  |

| `FF_NO_TSYMM` | env_key | std::env::var("FF_NO_TSYMM") |  |  |  |  |

| `FF_ORBIT_DEBUG` | env_key | std::env::var("FF_ORBIT_DEBUG") |  |  |  |  |

| `FF_ORBIT_GEN` | env_key | std::env::var("FF_ORBIT_GEN") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_TAGENDA_W` | env_key | std::env::var("FF_TAGENDA_W") |  |  |  |  |

| `FF_TAGENDA_W_PRUNE` | env_key | std::env::var("FF_TAGENDA_W_PRUNE") |  |  |  |  |

| `FF_TB_FREE_G` | env_key | std::env::var("FF_TB_FREE_G") |  |  |  |  |

| `FF_TDEMAND_W` | env_key | std::env::var("FF_TDEMAND_W") |  |  |  |  |

| `FF_TEMPORAL_ABS_KEY` | env_key | std::env::var("FF_TEMPORAL_ABS_KEY") |  |  |  |  |

| `FF_TEMPORAL_NODE_CAP` | env_key | std::env::var("FF_TEMPORAL_NODE_CAP") |  |  |  |  |

| `FF_TEVAL_BUDGET` | env_key | std::env::var("FF_TEVAL_BUDGET") |  |  |  |  |

| `FF_TLAMA` | env_key | std::env::var("FF_TLAMA") |  |  |  |  |

| `FF_TLIFO` | env_key | std::env::var("FF_TLIFO") |  |  |  |  |

| `FF_TRPG` | env_key | std::env::var("FF_TRPG") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> TemporalCompiled |  |  |  |  |

| `is_temporal` | function | is_temporal(domain: &Domain) -> bool |  |  |  |  |

| `prepare` | function | prepare(domain: &'a Domain, problem: &'a Problem) -> Option<Self> |  |  |  |  |

| `score` | function | score(&self, plan: &TimedPlan) -> Option<SoftScore> |  |  |  |  |

| `score_soft` | function | score_soft(domain: &Domain, problem: &Problem, plan: &TimedPlan) -> Option<SoftScore> |  |  |  |  |

| `solve` | function | solve(domain: &Domain, problem: &Problem, threads: usize) -> Option<TimedPlan> |  |  |  |  |

| `solve_scored` | function | solve_scored(domain: &Domain, problem: &Problem, threads: usize) -> Option<ScoredPlan> |  |  |  |  |

| `to_ipc` | function | to_ipc(&self) -> String |  |  |  |  |

| `validate` | function | validate(domain: &Domain, problem: &Problem, plan: &TimedPlan) -> Result<(), String> |  |  |  |  |

| `ScoredPlan` | struct | ScoredPlan { pub plan: TimedPlan, pub score: Option<SoftScore>, pub unscored: bool } |  |  |  |  |

| `SnapInfo` | struct | SnapInfo { pub start_action: Sym, pub end_action: Sym, pub running_pred: Sym, pub duration: Duration, pub invariant: Formula, pub params: Vec<(Sym, Sym)> } |  |  |  |  |

| `SoftScore` | struct | SoftScore { pub metric: Option<f64>, pub violated: Vec<String>, pub satisfied: usize } |  |  |  |  |

| `SoftScorer` | struct | SoftScorer { domain: &'a Domain, problem: &'a Problem, objs: HashMap<Sym, Vec<Sym>>, goal_prefs: Vec<(String, Formula)>, exp: crate::constraints::Expanded, c: TemporalCompiled, task: PackedTask } |  |  |  |  |

| `TemporalCompiled` | struct | TemporalCompiled { pub domain: Domain, pub problem: Problem, pub snaps: Vec<SnapInfo>, pub til_ops: Vec<(f64, Sym)> } |  |  |  |  |

| `TimedPlan` | struct | TimedPlan { pub steps: Vec<TimedStep>, pub makespan: f64 } |  |  |  |  |

| `TimedStep` | struct | TimedStep { pub time: f64, pub action: String, pub duration: Option<f64> } |  |  |  |  |


### crates/ferroplan/src/trace.rs

| `trace` | function | trace( domain_src: &str, problem_src: &str, plan: &[(String, Vec<String>)], ) -> Result<Vec<StateSnapshot>, String> |  |  |  |  |

| `
    (define (domain logi) (:requirements :typing)
      (:types location truck)
      (:predicates (at ?t - truck ?l - location) (road ?a ?b - location))
      (:action drive :parameters (?t - truck ?from ?to - location)
        :precondition (and (at ?t ?from) (road ?from ?to))
        :effect (and (not (at ?t ?from)) (at ?t ?to))))` | str_key | DOM = "
    (define (domain logi) (:requirements :typing)
      (:types location truck)
      (:predicates (at ?t - truck ?l - location) (road ?a ?b - location))
      (:action drive :parameters (?t - truck ?from ?to - location)
        :precondition (and (at ?t ?from) (road ?from ?to))
        :effect (and (not (at ?t ?from)) (at ?t ?to))))" |  |  |  |  |

| `
    (define (problem p) (:domain logi)
      (:objects a b - location  t1 - truck)
      (:init (at t1 a) (road a b))
      (:goal (at t1 b)))` | str_key | PRB = "
    (define (problem p) (:domain logi)
      (:objects a b - location  t1 - truck)
      (:init (at t1 a) (road a b))
      (:goal (at t1 b)))" |  |  |  |  |

| `StateSnapshot` | struct | StateSnapshot { pub facts: Vec<String>, pub fluents: Vec<(String, f64)> } |  |  |  |  |


### crates/ferroplan/src/tresolve.rs

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `solve` | function | solve(domain: &Domain, problem: &Problem, threads: usize) -> Option<TimedPlan> |  |  |  |  |


### crates/ferroplan/src/tsched.rs

| `n_actors` | function | n_actors(domain: &Domain, problem: &Problem) -> usize |  |  |  |  |

| `reschedule` | function | reschedule(domain: &Domain, problem: &Problem, plan: &TimedPlan) -> Option<TimedPlan> |  |  |  |  |

| `single_actor_problem` | function | single_actor_problem(domain: &Domain, problem: &Problem) -> Problem |  |  |  |  |


### crates/ferroplan/src/types.rs

| `DURATION_PSEUDO` | const | DURATION_PSEUDO: &str |  |  |  |  |

| `AssignOp` | enum | AssignOp { Assign, Increase, Decrease, ScaleUp, ScaleDown } |  |  |  |  |

| `CompOp` | enum | CompOp { Lt, Le, Eq, Ge, Gt } |  |  |  |  |

| `Constraint` | enum | Constraint { And(Vec<Constraint>), Forall(Vec<(Sym, Sym)>, Box<Constraint>), Pref(Option<Sym>, Box<Constraint>), Always(Formula), Sometime(Formula), AtMostOnce(Formula), SometimeAfter(Formula, Formula), SometimeBefore(Formula, Formula), AtEnd(Formula), Within(f64, Formula), AlwaysWithin(f64, Formula, Formula), HoldDuring(f64, f64, Formula), HoldAfter(f64, Formula) } |  |  |  |  |

| `Effect` | enum | Effect { Add(Sym, Vec<Term>), Del(Sym, Vec<Term>), Num(AssignOp, Sym, Vec<Term>, Expr), And(Vec<Effect>), When(Formula, Box<Effect>), Forall(Vec<(Sym, Sym)>, Box<Effect>) } |  |  |  |  |

| `Expr` | enum | Expr { Num(f64), Fluent(Sym, Vec<Term>), Add(Box<Expr>, Box<Expr>), Sub(Box<Expr>, Box<Expr>), Mul(Box<Expr>, Box<Expr>), Div(Box<Expr>, Box<Expr>), Neg(Box<Expr>) } |  |  |  |  |

| `Formula` | enum | Formula { And(Vec<Formula>), Or(Vec<Formula>), Not(Box<Formula>), Atom(Sym, Vec<Term>), Comp(CompOp, Expr, Expr), Forall(Vec<(Sym, Sym)>, Box<Formula>), Exists(Vec<(Sym, Sym)>, Box<Formula>), Eq(Term, Term), Pref(Option<Sym>, Box<Formula>), True, False } |  |  |  |  |

| `MetricDir` | enum | MetricDir { Minimize, Maximize } |  |  |  |  |

| `NExpr` | enum | NExpr { Num(f64), Fluent(u32), Add(Box<NExpr>, Box<NExpr>), Sub(Box<NExpr>, Box<NExpr>), Mul(Box<NExpr>, Box<NExpr>), Div(Box<NExpr>, Box<NExpr>), Neg(Box<NExpr>) } |  |  |  |  |

| `Term` | enum | Term { Var(Sym), Const(Sym) } |  |  |  |  |

| `TimeSpec` | enum | TimeSpec { Start, End, All } |  |  |  |  |

| `chosen` | function | chosen(&self) -> Option<&Expr> |  |  |  |  |

| `collect_fluents` | function | collect_fluents(&self, out: &mut Vec<u32>) |  |  |  |  |

| `eval` | function | eval(&self, fv: &[f64], def: &[bool]) -> Option<f64> |  |  |  |  |

| `eval_numpre` | function | eval_numpre(np: &NumPre, fv: &[f64], def: &[bool]) -> Option<bool> |  |  |  |  |

| `fixed` | function | fixed(e: Expr) -> Self |  |  |  |  |

| `new` | function | new(line: u32, message: impl Into<String>) -> Self |  |  |  |  |

| `?DURATION` | str_key | DURATION_PSEUDO = "?DURATION" |  |  |  |  |

| `Action` | struct | Action { pub name: Sym, pub params: Vec<(Sym, Sym)>, pub precond: Formula, pub effect: Effect, pub monitored: bool } |  |  |  |  |

| `DerivedRule` | struct | DerivedRule { pub head: Sym, pub params: Vec<(Sym, Sym)>, pub body: Formula } |  |  |  |  |

| `Domain` | struct | Domain { pub name: Sym, pub requirements: Vec<Sym>, pub types: Vec<Sym>, pub type_parent: Vec<(Sym, Sym)>, pub constants: Vec<(Sym, Sym)>, pub predicates: Vec<(Sym, Vec<Sym>)>, pub functions: Vec<(Sym, Vec<Sym>)>, pub actions: Vec<Action>, pub durative_actions: Vec<DurativeAction>, pub constraints: Vec<Constraint>, pub derived: Vec<DerivedRule>, pub monitors: Vec<Effect> } |  |  |  |  |

| `Duration` | struct | Duration { pub min: Option<Expr>, pub max: Option<Expr> } |  |  |  |  |

| `DurativeAction` | struct | DurativeAction { pub name: Sym, pub params: Vec<(Sym, Sym)>, pub duration: Duration, pub conditions: Vec<(TimeSpec, Formula)>, pub effects: Vec<(TimeSpec, Effect)> } |  |  |  |  |

| `NumEff` | struct | NumEff { pub op: AssignOp, pub target: u32, pub value: NExpr } |  |  |  |  |

| `NumPre` | struct | NumPre { pub op: CompOp, pub lhs: NExpr, pub rhs: NExpr } |  |  |  |  |

| `ParseError` | struct | ParseError { pub line: u32, pub message: String } |  |  |  |  |

| `Problem` | struct | Problem { pub name: Sym, pub domain_name: Sym, pub objects: Vec<(Sym, Sym)>, pub init_atoms: Vec<(Sym, Vec<Sym>)>, pub init_fluents: Vec<((Sym, Vec<Sym>), f64)>, pub til: Vec<TimedLiteral>, pub goal: Formula, pub constraints: Vec<Constraint>, pub metric: Option<(MetricDir, Expr)> } |  |  |  |  |

| `TimedLiteral` | struct | TimedLiteral { pub time: f64, pub add: bool, pub pred: Sym, pub args: Vec<Sym> } |  |  |  |  |


### crates/ferroplan/src/verify.rs

| `verify` | function | verify( domain_src: &str, problem_src: &str, plan: &[(String, Vec<String>)], ) -> Result<Verified, String> |  |  |  |  |

| `Verified` | struct | Verified { pub metric: f64, pub hard_goal_met: bool, pub satisfied: usize, pub violated: usize, pub constraints_met: bool, pub constraint_failures: Vec<String>, pub constraint_prefs: Vec<(String, bool)> } |  |  |  |  |


### crates/ferroplan/src/viz.rs

| `PredKind` | enum | PredKind { Edge, Position, Property } |  |  |  |  |

| `build` | function | build(domain: &Domain, problem: &Problem) -> Self |  |  |  |  |

| `domain_to_pddl` | function | domain_to_pddl( name: &str, requirements: &str, types: &[(String, String)], predicates: &[(String, Vec<String>)], actions_raw: &[String], ) -> String |  |  |  |  |

| `dynamic_predicates` | function | dynamic_predicates(domain: &Domain) -> BTreeSet<String> |  |  |  |  |

| `goal_facts` | function | goal_facts(problem: &Problem) -> Vec<(String, Vec<String>)> |  |  |  |  |

| `positions_at` | function | positions_at(&self, facts: &[String]) -> HashMap<String, Option<String>> |  |  |  |  |

| `to_pddl` | function | to_pddl( name: &str, domain_name: &str, objects: &[(String, String)], init: &[(String, Vec<String>)], goal: &[(String, Vec<String>)], ) -> String |  |  |  |  |

| `
    (define (domain logi) (:requirements :typing)
      (:types location truck package)
      (:predicates (at ?x - truck ?l - location) (road ?a ?b - location)
                   (in ?p - package ?t - truck) (delivered ?p - package))
      (:action drive :parameters (?t - truck ?from ?to - location)
        :precondition (and (at ?t ?from) (road ?from ?to))
        :effect (and (not (at ?t ?from)) (at ?t ?to))))` | str_key | DOM = "
    (define (domain logi) (:requirements :typing)
      (:types location truck package)
      (:predicates (at ?x - truck ?l - location) (road ?a ?b - location)
                   (in ?p - package ?t - truck) (delivered ?p - package))
      (:action drive :parameters (?t - truck ?from ?to - location)
        :precondition (and (at ?t ?from) (road ?from ?to))
        :effect (and (not (at ?t ?from)) (at ?t ?to))))" |  |  |  |  |

| `
    (define (problem p) (:domain logi)
      (:objects a b c - location  t1 - truck  p1 - package)
      (:init (at t1 a) (road a b) (road b c) (in p1 t1))
      (:goal (delivered p1)))` | str_key | PRB = "
    (define (problem p) (:domain logi)
      (:objects a b c - location  t1 - truck  p1 - package)
      (:init (at t1 a) (road a b) (road b c) (in p1 t1))
      (:goal (delivered p1)))" |  |  |  |  |

| `AT` | str_key | POSITION_NAMES = "AT" |  |  |  |  |

| `ROAD` | str_key | EDGE_NAMES = "ROAD" |  |  |  |  |

| `VizEdge` | struct | VizEdge { pub a: String, pub b: String, pub pred: String } |  |  |  |  |

| `VizGraph` | struct | VizGraph { pub nodes: Vec<VizNode>, pub edges: Vec<VizEdge>, pub mobiles: Vec<VizMobile>, pub props_by_object: BTreeMap<String, Vec<String>>, pub goal_by_object: BTreeMap<String, Vec<String>>, pub pred_kind: BTreeMap<String, PredKind>, pub location_types: BTreeSet<String> } |  |  |  |  |

| `VizMobile` | struct | VizMobile { pub object: String, pub ty: String, pub at: Option<String>, pub at_raw: Option<String> } |  |  |  |  |

| `VizNode` | struct | VizNode { pub object: String, pub ty: String } |  |  |  |  |


### crates/ferroplan/tests/action_costs.rs

| `
(define (domain roads)
  (:requirements :strips :typing :action-costs)
  (:types loc)
  (:constants a b c - loc)
  (:predicates (at ?l - loc))
  (:functions (total-cost) - number)
  (:action hop
    :parameters ()
    :precondition (at a)
    :effect (and (not (at a)) (at c) (increase (total-cost) 10)))
  (:action step1
    :parameters ()
    :precondition (at a)
    :effect (and (not (at a)) (at b) (increase (total-cost) 1)))
  (:action step2
    :parameters ()
    :precondition (at b)
    :effect (and (not (at b)) (at c) (increase (total-cost) 1))))
` | str_key | ROADS_DOMAIN = "
(define (domain roads)
  (:requirements :strips :typing :action-costs)
  (:types loc)
  (:constants a b c - loc)
  (:predicates (at ?l - loc))
  (:functions (total-cost) - number)
  (:action hop
    :parameters ()
    :precondition (at a)
    :effect (and (not (at a)) (at c) (increase (total-cost) 10)))
  (:action step1
    :parameters ()
    :precondition (at a)
    :effect (and (not (at a)) (at b) (increase (total-cost) 1)))
  (:action step2
    :parameters ()
    :precondition (at b)
    :effect (and (not (at b)) (at c) (increase (total-cost) 1))))
" |  |  |  |  |

| `
(define (problem roads-1) (:domain roads)
  (:init (at a) (= (total-cost) 0))
  (:goal (at c))
  (:metric minimize (total-cost)))
` | str_key | ROADS_PROBLEM = "
(define (problem roads-1) (:domain roads)
  (:init (at a) (= (total-cost) 0))
  (:goal (at c))
  (:metric minimize (total-cost)))
" |  |  |  |  |


### crates/ferroplan/tests/adl.rs

| `(define (domain adl1)
 (:requirements :typing :adl :negative-preconditions)
 (:types item)
 (:predicates (tagged ?x - item) (done) (linked ?a - item ?b - item))
 (:action tag :parameters (?x - item) :precondition (not (tagged ?x)) :effect (tagged ?x))
 (:action link :parameters (?a - item ?b - item)
   :precondition (and (not (= ?a ?b)) (tagged ?a) (tagged ?b))
   :effect (linked ?a ?b))
 (:action finish :parameters ()
   :precondition (and (forall (?x - item) (tagged ?x))
                      (exists (?a - item ?b - item) (linked ?a ?b)))
   :effect (done)))` | str_key | DOM = "(define (domain adl1)
 (:requirements :typing :adl :negative-preconditions)
 (:types item)
 (:predicates (tagged ?x - item) (done) (linked ?a - item ?b - item))
 (:action tag :parameters (?x - item) :precondition (not (tagged ?x)) :effect (tagged ?x))
 (:action link :parameters (?a - item ?b - item)
   :precondition (and (not (= ?a ?b)) (tagged ?a) (tagged ?b))
   :effect (linked ?a ?b))
 (:action finish :parameters ()
   :precondition (and (forall (?x - item) (tagged ?x))
                      (exists (?a - item ?b - item) (linked ?a ?b)))
   :effect (done)))" |  |  |  |  |

| `(define (domain briefcase)
 (:requirements :typing :adl)
 (:types obj loc)
 (:predicates (at-bc ?l - loc) (inbc ?o - obj) (at ?o - obj ?l - loc))
 (:action move :parameters (?from ?to - loc)
   :precondition (at-bc ?from)
   :effect (and (at-bc ?to) (not (at-bc ?from))
                (forall (?o - obj)
                  (when (inbc ?o) (and (at ?o ?to) (not (at ?o ?from)))))))
 (:action putin :parameters (?o - obj ?l - loc)
   :precondition (and (at-bc ?l) (at ?o ?l))
   :effect (inbc ?o))
 (:action takeout :parameters (?o - obj)
   :precondition (inbc ?o)
   :effect (not (inbc ?o))))` | str_key | BRIEFCASE = "(define (domain briefcase)
 (:requirements :typing :adl)
 (:types obj loc)
 (:predicates (at-bc ?l - loc) (inbc ?o - obj) (at ?o - obj ?l - loc))
 (:action move :parameters (?from ?to - loc)
   :precondition (at-bc ?from)
   :effect (and (at-bc ?to) (not (at-bc ?from))
                (forall (?o - obj)
                  (when (inbc ?o) (and (at ?o ?to) (not (at ?o ?from)))))))
 (:action putin :parameters (?o - obj ?l - loc)
   :precondition (and (at-bc ?l) (at ?o ?l))
   :effect (inbc ?o))
 (:action takeout :parameters (?o - obj)
   :precondition (inbc ?o)
   :effect (not (inbc ?o))))" |  |  |  |  |

| `(define (domain toggle)
 (:requirements :adl)
 (:predicates (on) (marker))
 (:action flip :parameters ()
   :precondition (marker)
   :effect (and (when (on) (not (on))) (when (not (on)) (on)))))` | str_key | TOGGLE = "(define (domain toggle)
 (:requirements :adl)
 (:predicates (on) (marker))
 (:action flip :parameters ()
   :precondition (marker)
   :effect (and (when (on) (not (on))) (when (not (on)) (on)))))" |  |  |  |  |


### crates/ferroplan/tests/api.rs

| `(define (domain g)
 (:requirements :strips :typing)
 (:types loc)
 (:predicates (at ?l - loc) (link ?a - loc ?b - loc))
 (:action move :parameters (?a ?b - loc)
   :precondition (and (at ?a) (link ?a ?b)) :effect (and (at ?b) (not (at ?a)))))` | str_key | GRID = "(define (domain g)
 (:requirements :strips :typing)
 (:types loc)
 (:predicates (at ?l - loc) (link ?a - loc ?b - loc))
 (:action move :parameters (?a ?b - loc)
   :precondition (and (at ?a) (link ?a ?b)) :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (domain t)
 (:requirements :strips :typing)
 (:types loc pkg)
 (:predicates (truck-at ?l - loc) (pkg-at ?p - pkg ?l - loc) (in ?p - pkg) (road ?a ?b - loc))
 (:action drive :parameters (?a ?b - loc)
   :precondition (and (truck-at ?a) (road ?a ?b)) :effect (and (truck-at ?b) (not (truck-at ?a))))
 (:action load :parameters (?p - pkg ?l - loc)
   :precondition (and (pkg-at ?p ?l) (truck-at ?l)) :effect (and (in ?p) (not (pkg-at ?p ?l))))
 (:action unload :parameters (?p - pkg ?l - loc)
   :precondition (and (in ?p) (truck-at ?l)) :effect (and (pkg-at ?p ?l) (not (in ?p)))))` | str_key | TRANSPORT = "(define (domain t)
 (:requirements :strips :typing)
 (:types loc pkg)
 (:predicates (truck-at ?l - loc) (pkg-at ?p - pkg ?l - loc) (in ?p - pkg) (road ?a ?b - loc))
 (:action drive :parameters (?a ?b - loc)
   :precondition (and (truck-at ?a) (road ?a ?b)) :effect (and (truck-at ?b) (not (truck-at ?a))))
 (:action load :parameters (?p - pkg ?l - loc)
   :precondition (and (pkg-at ?p ?l) (truck-at ?l)) :effect (and (in ?p) (not (pkg-at ?p ?l))))
 (:action unload :parameters (?p - pkg ?l - loc)
   :precondition (and (in ?p) (truck-at ?l)) :effect (and (pkg-at ?p ?l) (not (in ?p)))))" |  |  |  |  |


### crates/ferroplan/tests/api_panic_hunt.rs

| `parse_domain` | str_key | TARGETS_PARSER = "parse_domain" |  |  |  |  |

| `preprocess` | str_key | TARGETS_PREPROCESS = "preprocess" |  |  |  |  |


### crates/ferroplan/tests/call_budget.rs

| `CALL_BUDGET_CHILD` | env_key | std::env::var("CALL_BUDGET_CHILD") |  |  |  |  |


### crates/ferroplan/tests/common/external.rs

| `FERROPLAN_CORPUS_DIR` | env_key | std::env::var_os("FERROPLAN_CORPUS_DIR") |  |  |  |  |

| `FERROPLAN_ORACLE_DIR` | env_key | std::env::var_os("FERROPLAN_ORACLE_DIR") |  |  |  |  |

| `FERROPLAN_RUN_DIR` | env_key | std::env::var_os("FERROPLAN_RUN_DIR") |  |  |  |  |

| `HOME` | env_key | std::env::var_os("HOME") |  |  |  |  |

| `corpus_dir` | function | corpus_dir() -> PathBuf |  |  |  |  |

| `corpus_ipc_dir` | function | corpus_ipc_dir() -> PathBuf |  |  |  |  |

| `differential_run_dir` | function | differential_run_dir() -> PathBuf |  |  |  |  |

| `harness_present` | function | harness_present(what: &str, path: &Path) -> bool |  |  |  |  |

| `oracle_dir` | function | oracle_dir() -> PathBuf |  |  |  |  |

| `oracle_runner` | function | oracle_runner() -> PathBuf |  |  |  |  |


### crates/ferroplan/tests/common/mod.rs

| `PROVENANCE_DIFF_T61` | const | PROVENANCE_DIFF_T61: &[&str] |  |  |  |  |

| `PROVENANCE_FUZZ_T31` | const | PROVENANCE_FUZZ_T31: &[&str] |  |  |  |  |

| `base_sizes` | function | base_sizes(rng: &mut Rng) -> Sizes |  |  |  |  |

| `below` | function | below(&mut self, n: u64) -> u64 |  |  |  |  |

| `chance` | function | chance(&mut self, percent: u64) -> bool |  |  |  |  |

| `draw_for` | function | draw_for(seed: u64, sizes: Sizes) -> (String, String) |  |  |  |  |

| `draw_valid` | function | draw_valid(seed: u64, sizes: Sizes) -> (String, String) |  |  |  |  |

| `generate` | function | generate(seed: u64, sizes: Sizes) -> Model |  |  |  |  |

| `generate_valid` | function | generate_valid(seed: u64, sizes: Sizes) -> Model |  |  |  |  |

| `halve_sizes` | function | halve_sizes(s: Sizes) -> Sizes |  |  |  |  |

| `mutation_of` | function | mutation_of(seed: u64) -> bool |  |  |  |  |

| `new` | function | new(seed: u64) -> Self |  |  |  |  |

| `next_u64` | function | next_u64(&mut self) -> u64 |  |  |  |  |

| `pick_idx` | function | pick_idx(&mut self, len: usize) -> usize |  |  |  |  |

| `range` | function | range(&mut self, lo: u64, hi: u64) -> u64 |  |  |  |  |

| `range_usize` | function | range_usize(&mut self, lo: usize, hi: usize) -> usize |  |  |  |  |

| `render` | function | render(model: &Model, problem_name: &str) -> (String, String) |  |  |  |  |

| `render_with_provenance` | function | render_with_provenance( model: &Model, problem_name: &str, header: &[&str], ) -> (String, String) |  |  |  |  |

| `sizes_for` | function | sizes_for(seed: u64) -> Sizes |  |  |  |  |

| `;; differential-fuzz: self-authored seeded VALID draw (ticket fond-htn-61,` | str_key | PROVENANCE_DIFF_T61 = ";; differential-fuzz: self-authored seeded VALID draw (ticket fond-htn-61," |  |  |  |  |

| `;; fuzz-found: self-authored seeded draw (ticket fond-htn-31,` | str_key | PROVENANCE_FUZZ_T31 = ";; fuzz-found: self-authored seeded draw (ticket fond-htn-31," |  |  |  |  |

| `Model` | struct | Model { types: Vec<String>, preds: Vec<(String, Vec<usize>)>, actions: Vec<ActionM>, tasks: Vec<(String, Vec<(String, usize)>)>, methods: Vec<MethodM>, objects: Vec<(String, usize)>, init: Vec<LitO>, goal: Vec<LitO>, root: Vec<(String, CallM)> } |  |  |  |  |

| `Rng` | struct | Rng { u64 } |  |  |  |  |

| `Sizes` | struct | Sizes { pub types: usize, pub preds: usize, pub tasks: usize, pub actions: usize, pub objects: usize, pub subs: usize, pub root_subs: usize } |  |  |  |  |


### crates/ferroplan/tests/complex_prefs.rs

| `
(define (domain cond-pref)
  (:requirements :typing :durative-actions :preferences)
  (:types thing)
  (:predicates (clean ?x - thing) (ready ?x - thing) (moved ?x - thing) (quiet))
  (:durative-action move
    :parameters (?x - thing)
    :duration (= ?duration 1)
    :condition (and (preference pc (at start (clean ?x)))
                    (preference pall (at start (forall (?y - thing) (clean ?y))))
                    (at start (ready ?x))
                    (at start (preference pq (quiet)))
                    (preference po (over all (quiet))))
    :effect (and (at start (not (ready ?x))) (at end (moved ?x)))))
` | str_key | COND_DOMAIN = "
(define (domain cond-pref)
  (:requirements :typing :durative-actions :preferences)
  (:types thing)
  (:predicates (clean ?x - thing) (ready ?x - thing) (moved ?x - thing) (quiet))
  (:durative-action move
    :parameters (?x - thing)
    :duration (= ?duration 1)
    :condition (and (preference pc (at start (clean ?x)))
                    (preference pall (at start (forall (?y - thing) (clean ?y))))
                    (at start (ready ?x))
                    (at start (preference pq (quiet)))
                    (preference po (over all (quiet))))
    :effect (and (at start (not (ready ?x))) (at end (moved ?x)))))
" |  |  |  |  |

| `
(define (domain cp-mini)
  (:requirements :strips :durative-actions :constraints :preferences)
  (:predicates (fresh-work) (fresh-wave) (done) (waved) (never-obtainable))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 5)
    :condition (at start (fresh-work))
    :effect (and (at start (not (fresh-work))) (at end (done))))
  (:durative-action wave
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-wave))
    :effect (and (at start (not (fresh-wave))) (at end (waved)))))
` | str_key | DOMAIN = "
(define (domain cp-mini)
  (:requirements :strips :durative-actions :constraints :preferences)
  (:predicates (fresh-work) (fresh-wave) (done) (waved) (never-obtainable))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 5)
    :condition (at start (fresh-work))
    :effect (and (at start (not (fresh-work))) (at end (done))))
  (:durative-action wave
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-wave))
    :effect (and (at start (not (fresh-wave))) (at end (waved)))))
" |  |  |  |  |

| `
(define (problem cond-pref-1) (:domain cond-pref)
  (:objects a b - thing)
  (:init (clean a) (ready a) (ready b) (quiet))
  (:goal (and (moved a) (moved b)))
  (:metric minimize (+ (* 1 (is-violated pc)) (* 10 (is-violated pall))
                       (* 100 (is-violated pq)) (* 1000 (is-violated po)))))
` | str_key | COND_PROBLEM = "
(define (problem cond-pref-1) (:domain cond-pref)
  (:objects a b - thing)
  (:init (clean a) (ready a) (ready b) (quiet))
  (:goal (and (moved a) (moved b)))
  (:metric minimize (+ (* 1 (is-violated pc)) (* 10 (is-violated pall))
                       (* 100 (is-violated pq)) (* 1000 (is-violated po)))))
" |  |  |  |  |

| `
(define (problem cp-mixed) (:domain cp-mini)
  (:init (fresh-work) (fresh-wave))
  (:goal (and (done) (preference gp (waved))))
  (:constraints (and (preference cp (sometime (waved)))
                     (preference cq (sometime (never-obtainable)))))
  (:metric minimize (+ (* 2 (is-violated gp))
                       (* 3 (is-violated cp))
                       (* 5 (is-violated cq)))))
` | str_key | P_MIXED = "
(define (problem cp-mixed) (:domain cp-mini)
  (:init (fresh-work) (fresh-wave))
  (:goal (and (done) (preference gp (waved))))
  (:constraints (and (preference cp (sometime (waved)))
                     (preference cq (sometime (never-obtainable)))))
  (:metric minimize (+ (* 2 (is-violated gp))
                       (* 3 (is-violated cp))
                       (* 5 (is-violated cq)))))
" |  |  |  |  |

| `
(define (problem cp-sat) (:domain cp-mini)
  (:init (fresh-work) (fresh-wave))
  (:goal (and (done) (preference gp (waved))))
  (:constraints (preference cp (sometime (waved))))
  (:metric minimize (+ (* 2 (is-violated gp)) (* 3 (is-violated cp)))))
` | str_key | P_SATISFIABLE = "
(define (problem cp-sat) (:domain cp-mini)
  (:init (fresh-work) (fresh-wave))
  (:goal (and (done) (preference gp (waved))))
  (:constraints (preference cp (sometime (waved))))
  (:metric minimize (+ (* 2 (is-violated gp)) (* 3 (is-violated cp)))))
" |  |  |  |  |


### crates/ferroplan/tests/constraints.rs

| `(define (domain sw)
  (:requirements :strips :constraints)
  (:predicates (on) (off) (lamp) (used))
  (:action flip-on :precondition (off) :effect (and (not (off)) (on)))
  (:action flip-off :precondition (on) :effect (and (not (on)) (off)))
  (:action light :precondition (on) :effect (and (lamp) (used))))` | str_key | DOM = "(define (domain sw)
  (:requirements :strips :constraints)
  (:predicates (on) (off) (lamp) (used))
  (:action flip-on :precondition (off) :effect (and (not (off)) (on)))
  (:action flip-off :precondition (on) :effect (and (not (on)) (off)))
  (:action light :precondition (on) :effect (and (lamp) (used))))" |  |  |  |  |

| `(define (domain sws)
  (:requirements :strips :constraints)
  (:predicates (on) (off) (lamp) (used) (linked))
  (:action flip-on :precondition (off) :effect (and (not (off)) (on)))
  (:action flip-off :precondition (on) :effect (and (not (on)) (off)))
  (:action light :precondition (on) :effect (and (lamp) (used))))` | str_key | DOMS = "(define (domain sws)
  (:requirements :strips :constraints)
  (:predicates (on) (off) (lamp) (used) (linked))
  (:action flip-on :precondition (off) :effect (and (not (off)) (on)))
  (:action flip-off :precondition (on) :effect (and (not (on)) (off)))
  (:action light :precondition (on) :effect (and (lamp) (used))))" |  |  |  |  |


### crates/ferroplan/tests/daily_agent_methods_ppddl.rs

| `../../../examples/daily_agent_methods/domain.ppddl` | str_key | DOMAIN = "../../../examples/daily_agent_methods/domain.ppddl" |  |  |  |  |

| `../../../examples/daily_agent_methods/method-catalog.json` | str_key | CATALOG = "../../../examples/daily_agent_methods/method-catalog.json" |  |  |  |  |

| `../../../examples/daily_agent_methods/problem-2026-07-31.ppddl` | str_key | PROBLEM = "../../../examples/daily_agent_methods/problem-2026-07-31.ppddl" |  |  |  |  |

| `../../../examples/daily_agent_methods/receipt-2026-07-31.json` | str_key | RECEIPT = "../../../examples/daily_agent_methods/receipt-2026-07-31.json" |  |  |  |  |


### crates/ferroplan/tests/decompose.rs

| `
(define (domain acc)
  (:requirements :durative-actions :numeric-fluents)
  (:functions (x))
  (:durative-action step :parameters () :duration (= ?duration 1)
    :condition () :effect (at end (increase (x) 1))))
` | str_key | SINGLE = "
(define (domain acc)
  (:requirements :durative-actions :numeric-fluents)
  (:functions (x))
  (:durative-action step :parameters () :duration (= ?duration 1)
    :condition () :effect (at end (increase (x) 1))))
" |  |  |  |  |

| `
(define (domain mk)
  (:requirements :durative-actions :numeric-fluents)
  (:functions (a) (b))
  (:durative-action make-a :parameters () :duration (= ?duration 2)
    :condition () :effect (at end (increase (a) 1)))
  (:durative-action make-b :parameters () :duration (= ?duration 3)
    :condition () :effect (at end (increase (b) 1))))
` | str_key | TWO_DELIVERABLES = "
(define (domain mk)
  (:requirements :durative-actions :numeric-fluents)
  (:functions (a) (b))
  (:durative-action make-a :parameters () :duration (= ?duration 2)
    :condition () :effect (at end (increase (a) 1)))
  (:durative-action make-b :parameters () :duration (= ?duration 3)
    :condition () :effect (at end (increase (b) 1))))
" |  |  |  |  |

| `(define (problem p) (:domain acc) (:init (= (x) 0)) (:goal (>= (x) 3)))` | str_key | SINGLE_PROB = "(define (problem p) (:domain acc) (:init (= (x) 0)) (:goal (>= (x) 3)))" |  |  |  |  |

| `(define (problem p) (:domain mk)
  (:init (= (a) 0) (= (b) 0))
  (:goal (and (>= (a) 1) (>= (b) 1))))` | str_key | TWO_PROB = "(define (problem p) (:domain mk)
  (:init (= (a) 0) (= (b) 0))
  (:goal (and (>= (a) 1) (>= (b) 1))))" |  |  |  |  |


### crates/ferroplan/tests/differential_fuzz.rs

| `DIFFERENTIAL_FUZZ_FRESH` | env_key | std::env::var("DIFFERENTIAL_FUZZ_FRESH") |  |  |  |  |

| `CARGO_MANIFEST_DIR` | str_key | LEDGER_PATH = "CARGO_MANIFEST_DIR" |  |  |  |  |

| `flexible` | str_key | ORACLE_MODE = "flexible" |  |  |  |  |

| `redecomposition` | str_key | KNOWN_DIVERGENCES = "redecomposition" |  |  |  |  |


### crates/ferroplan/tests/endgate.rs

| `FF_H_ENDGATE` | env_key | std::env::var("FF_H_ENDGATE") |  |  |  |  |

| `
(define (domain endgate)
  (:predicates (ga) (gb))
  (:action snap-start :parameters () :effect (ga))
  (:action snap-end   :parameters () :effect (gb)))` | str_key | SNAP_DOM = "
(define (domain endgate)
  (:predicates (ga) (gb))
  (:action snap-start :parameters () :effect (ga))
  (:action snap-end   :parameters () :effect (gb)))" |  |  |  |  |


### crates/ferroplan/tests/enrich.rs

| `ENRICH_CAP` | env_key | std::env::var("ENRICH_CAP") |  |  |  |  |

| `ENRICH_CHILD` | env_key | std::env::var("ENRICH_CHILD") |  |  |  |  |

| `ENRICH_K` | env_key | std::env::var("ENRICH_K") |  |  |  |  |


### crates/ferroplan/tests/fluent_fold.rs

| `
(define (domain adlfold)
  (:requirements :adl :typing :numeric-fluents)
  (:types box)
  (:predicates (open ?b - box) (heavy ?b - box) (moved ?b - box))
  (:functions (weight ?b - box) (carried))
  (:action move
    :parameters (?b - box)
    :precondition (not (moved ?b))
    :effect (and (moved ?b)
                 (when (heavy ?b) (increase (carried) (weight ?b))))))` | str_key | ADL_DOM = "
(define (domain adlfold)
  (:requirements :adl :typing :numeric-fluents)
  (:types box)
  (:predicates (open ?b - box) (heavy ?b - box) (moved ?b - box))
  (:functions (weight ?b - box) (carried))
  (:action move
    :parameters (?b - box)
    :precondition (not (moved ?b))
    :effect (and (moved ?b)
                 (when (heavy ?b) (increase (carried) (weight ?b))))))" |  |  |  |  |

| `
(define (domain costfold)
  (:requirements :strips :typing :action-costs)
  (:types loc)
  (:predicates (at ?l - loc) (road ?a ?b - loc) (visited ?l - loc))
  (:functions (total-cost) (toll ?a ?b - loc))
  (:action go
    :parameters (?a ?b - loc)
    :precondition (and (at ?a) (road ?a ?b))
    :effect (and (not (at ?a)) (at ?b) (visited ?b)
                 (increase (total-cost) (toll ?a ?b)))))` | str_key | COSTS_DOM = "
(define (domain costfold)
  (:requirements :strips :typing :action-costs)
  (:types loc)
  (:predicates (at ?l - loc) (road ?a ?b - loc) (visited ?l - loc))
  (:functions (total-cost) (toll ?a ?b - loc))
  (:action go
    :parameters (?a ?b - loc)
    :precondition (and (at ?a) (road ?a ?b))
    :effect (and (not (at ?a)) (at ?b) (visited ?b)
                 (increase (total-cost) (toll ?a ?b)))))" |  |  |  |  |

| `
(define (domain durexpr)
  (:requirements :strips :durative-actions :numeric-fluents)
  (:predicates (ready) (boosted) (done))
  (:functions (speed) (progress))
  (:durative-action boost
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (ready))
    :effect (and (at start (not (ready)))
                 (at start (increase (speed) 2))
                 (at end (boosted))))
  (:durative-action work
    :parameters ()
    :duration (= ?duration (speed))
    :condition (at start (boosted))
    :effect (and (at start (increase (progress) ?duration))
                 (at end (done)))))` | str_key | DUREXPR_DOM = "
(define (domain durexpr)
  (:requirements :strips :durative-actions :numeric-fluents)
  (:predicates (ready) (boosted) (done))
  (:functions (speed) (progress))
  (:durative-action boost
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (ready))
    :effect (and (at start (not (ready)))
                 (at start (increase (speed) 2))
                 (at end (boosted))))
  (:durative-action work
    :parameters ()
    :duration (= ?duration (speed))
    :condition (at start (boosted))
    :effect (and (at start (increase (progress) ?duration))
                 (at end (done)))))" |  |  |  |  |

| `
(define (domain minitpp)
  (:requirements :strips :typing :numeric-fluents)
  (:types place goods)
  (:predicates (at ?p - place) (link ?a ?b - place))
  (:functions (price ?g - goods ?p - place) (bought ?g - goods)
              (request ?g - goods) (spent))
  (:action drive
    :parameters (?a ?b - place)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (not (at ?a)) (at ?b)))
  (:action buy
    :parameters (?g - goods ?p - place)
    :precondition (and (at ?p) (< (bought ?g) (request ?g)))
    :effect (and (increase (bought ?g) 1)
                 (increase (spent) (price ?g ?p)))))` | str_key | MINITPP_DOM = "
(define (domain minitpp)
  (:requirements :strips :typing :numeric-fluents)
  (:types place goods)
  (:predicates (at ?p - place) (link ?a ?b - place))
  (:functions (price ?g - goods ?p - place) (bought ?g - goods)
              (request ?g - goods) (spent))
  (:action drive
    :parameters (?a ?b - place)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (not (at ?a)) (at ?b)))
  (:action buy
    :parameters (?g - goods ?p - place)
    :precondition (and (at ?p) (< (bought ?g) (request ?g)))
    :effect (and (increase (bought ?g) 1)
                 (increase (spent) (price ?g ?p)))))" |  |  |  |  |

| `
(define (problem adlfold-1) (:domain adlfold)
  (:objects b1 b2 b3 - box)
  (:init (heavy b1) (heavy b3)
         (= (weight b1) 10) (= (weight b2) 1) (= (weight b3) 3)
         (= (carried) 0))
  (:goal (and (moved b1) (moved b2) (moved b3))))` | str_key | ADL_PRB = "
(define (problem adlfold-1) (:domain adlfold)
  (:objects b1 b2 b3 - box)
  (:init (heavy b1) (heavy b3)
         (= (weight b1) 10) (= (weight b2) 1) (= (weight b3) 3)
         (= (carried) 0))
  (:goal (and (moved b1) (moved b2) (moved b3))))" |  |  |  |  |

| `
(define (problem costfold-1) (:domain costfold)
  (:objects a b c d - loc)
  (:init (at a) (road a b) (road b c) (road a c) (road c d)
         (= (toll a b) 1) (= (toll b c) 1) (= (toll a c) 5) (= (toll c d) 2)
         (= (total-cost) 0))
  (:goal (visited d))
  (:metric minimize (total-cost)))` | str_key | COSTS_PRB = "
(define (problem costfold-1) (:domain costfold)
  (:objects a b c d - loc)
  (:init (at a) (road a b) (road b c) (road a c) (road c d)
         (= (toll a b) 1) (= (toll b c) 1) (= (toll a c) 5) (= (toll c d) 2)
         (= (total-cost) 0))
  (:goal (visited d))
  (:metric minimize (total-cost)))" |  |  |  |  |

| `
(define (problem durexpr-1) (:domain durexpr)
  (:init (ready) (= (speed) 1) (= (progress) 0))
  (:goal (and (done) (>= (progress) 3))))` | str_key | DUREXPR_PRB = "
(define (problem durexpr-1) (:domain durexpr)
  (:init (ready) (= (speed) 1) (= (progress) 0))
  (:goal (and (done) (>= (progress) 3))))" |  |  |  |  |

| `
(define (problem minitpp-1) (:domain minitpp)
  (:objects p1 p2 p3 - place g1 g2 - goods)
  (:init (at p1) (link p1 p2) (link p2 p3) (link p2 p1) (link p3 p2)
         (= (price g1 p1) 4) (= (price g1 p2) 2) (= (price g1 p3) 7)
         (= (price g2 p1) 5) (= (price g2 p2) 9) (= (price g2 p3) 1)
         (= (bought g1) 0) (= (bought g2) 0)
         (= (request g1) 2) (= (request g2) 1)
         (= (spent) 0))
  (:goal (and (>= (bought g1) (request g1)) (>= (bought g2) (request g2)))))` | str_key | MINITPP_PRB = "
(define (problem minitpp-1) (:domain minitpp)
  (:objects p1 p2 p3 - place g1 g2 - goods)
  (:init (at p1) (link p1 p2) (link p2 p3) (link p2 p1) (link p3 p2)
         (= (price g1 p1) 4) (= (price g1 p2) 2) (= (price g1 p3) 7)
         (= (price g2 p1) 5) (= (price g2 p2) 9) (= (price g2 p3) 1)
         (= (bought g1) 0) (= (bought g2) 0)
         (= (request g1) 2) (= (request g2) 1)
         (= (spent) 0))
  (:goal (and (>= (bought g1) (request g1)) (>= (bought g2) (request g2)))))" |  |  |  |  |


### crates/ferroplan/tests/fond_flat_oracle.rs

| `strong FOND fixed point` | str_key | STRONG_NOTE = "strong FOND fixed point" |  |  |  |  |

| `strong-cyclic FOND fixpoint` | str_key | STRONG_CYCLIC_NOTE = "strong-cyclic FOND fixpoint" |  |  |  |  |

| `tireworld` | str_key | DOMAINS = "tireworld" |  |  |  |  |


### crates/ferroplan/tests/fond_htn_micro.rs

| `drop-retry` | str_key | ALL = "drop-retry" |  |  |  |  |

| `fixtures/fond-htn-micro/both-branches-deadend/domain.hddl` | str_key | BOTH_BRANCHES_DEADEND = "fixtures/fond-htn-micro/both-branches-deadend/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/drop-retry/domain.hddl` | str_key | DROP_RETRY = "fixtures/fond-htn-micro/drop-retry/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/grow-loop/domain.hddl` | str_key | GROW_LOOP = "fixtures/fond-htn-micro/grow-loop/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/plain-chain/domain.hddl` | str_key | PLAIN_CHAIN = "fixtures/fond-htn-micro/plain-chain/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/sense-then-branch/domain.hddl` | str_key | SENSE_THEN_BRANCH = "fixtures/fond-htn-micro/sense-then-branch/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/supervisor-fail/domain.hddl` | str_key | SUPERVISOR_FAIL = "fixtures/fond-htn-micro/supervisor-fail/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/tray-dirty/domain.hddl` | str_key | TRAY_DIRTY = "fixtures/fond-htn-micro/tray-dirty/domain.hddl" |  |  |  |  |


### crates/ferroplan/tests/fond_htn_oracle.rs

| `CARGO_MANIFEST_DIR` | str_key | FIXTURE_DIR = "CARGO_MANIFEST_DIR" |  |  |  |  |

| `fixtures/fond-htn/oracle-goldens.json` | str_key | GOLDENS_RAW = "fixtures/fond-htn/oracle-goldens.json" |  |  |  |  |


### crates/ferroplan/tests/fond_property.rs

| `ticket floor is 300+ instances` | str_key | _ = "ticket floor is 300+ instances" |  |  |  |  |


### crates/ferroplan/tests/fond_threshold.rs

| `drop-retry` | str_key | CASES = "drop-retry" |  |  |  |  |


### crates/ferroplan/tests/fond_unsafe_hddl.rs

| `fixtures/fond-unsafe/abyss-avoid-problem.hddl` | str_key | ABYSS_AVOID_PROBLEM = "fixtures/fond-unsafe/abyss-avoid-problem.hddl" |  |  |  |  |

| `fixtures/fond-unsafe/abyss-avoid.hddl` | str_key | ABYSS_AVOID_DOMAIN = "fixtures/fond-unsafe/abyss-avoid.hddl" |  |  |  |  |

| `fixtures/fond-unsafe/abyss-both-problem.hddl` | str_key | ABYSS_BOTH_PROBLEM = "fixtures/fond-unsafe/abyss-both-problem.hddl" |  |  |  |  |

| `fixtures/fond-unsafe/abyss-both.hddl` | str_key | ABYSS_BOTH_DOMAIN = "fixtures/fond-unsafe/abyss-both.hddl" |  |  |  |  |


### crates/ferroplan/tests/ground_caps_ipc_addendum.rs

| `freecell_learned_ecai_16` | str_key | CASES = "freecell_learned_ecai_16" |  |  |  |  |


### crates/ferroplan/tests/ground_wall.rs

| `GROUND_WALL_CHILD` | env_key | std::env::var("GROUND_WALL_CHILD") |  |  |  |  |


### crates/ferroplan/tests/grounding_prune_ipc_rerun.rs

| `freecell_learned_ecai_16` | str_key | CASES = "freecell_learned_ecai_16" |  |  |  |  |


### crates/ferroplan/tests/hddl_adversarial.rs

| `abstract-task-without-decomposition-domain.hddl` | str_key | CORPUS_ACCEPTED_SCOPE_GAPS = "abstract-task-without-decomposition-domain.hddl" |  |  |  |  |


### crates/ferroplan/tests/hddl_fuzz_roundtrip.rs

| `TIMEOUT(normalized)` | str_key | TIMEOUT_TAG = "TIMEOUT(normalized)" |  |  |  |  |

| `solve:TIMEOUT(normalized)` | str_key | TIMEOUT_SOLVE = "solve:TIMEOUT(normalized)" |  |  |  |  |


### crates/ferroplan/tests/htn_ipc2023.rs

| `blocksworld_gtohp` | str_key | INSTANCES = "blocksworld_gtohp" |  |  |  |  |


### crates/ferroplan/tests/htn_oracle.rs

| `PCP_1` | str_key | KNOWN_MISMATCHES = "PCP_1" |  |  |  |  |


### crates/ferroplan/tests/inc_lmcut.rs

| `INC_LMCUT_CHILD` | env_key | std::env::var("INC_LMCUT_CHILD") |  |  |  |  |


### crates/ferroplan/tests/ipc_sweep.rs

| `assemblyhierarchical` | str_key | DOMAINS = "assemblyhierarchical" |  |  |  |  |

| `lamps` | str_key | SAMPLED = "lamps" |  |  |  |  |


### crates/ferroplan/tests/ladder_dedup.rs

| `LADDER_DEDUP_CHILD` | env_key | std::env::var("LADDER_DEDUP_CHILD") |  |  |  |  |


### crates/ferroplan/tests/ladder_wall.rs

| `LADDER_WALL_CHILD` | env_key | std::env::var("LADDER_WALL_CHILD") |  |  |  |  |


### crates/ferroplan/tests/landmarks.rs

| `
(define (domain chain)
  (:requirements :strips)
  (:predicates (a) (b) (c))
  (:action ab :parameters () :precondition (a) :effect (b))
  (:action bc :parameters () :precondition (b) :effect (c)))
` | str_key | CHAIN = "
(define (domain chain)
  (:requirements :strips)
  (:predicates (a) (b) (c))
  (:action ab :parameters () :precondition (a) :effect (b))
  (:action bc :parameters () :precondition (b) :effect (c)))
" |  |  |  |  |


### crates/ferroplan/tests/mcv_ground.rs

| `MCV_ROUTE_CHILD` | env_key | std::env::var("MCV_ROUTE_CHILD") |  |  |  |  |

| `MCV_STRAT_CHILD` | env_key | std::env::var("MCV_STRAT_CHILD") |  |  |  |  |

| `(define (domain fixroute) (:requirements :typing)
  (:types item key - object)
  (:predicates (tok ?a ?b - item ?c - key) (done ?a - item) (seeded))
  (:action seed :parameters ()
    :precondition (and) :effect (and (seeded) (tok i1 i2 k1)))
  (:action reap :parameters (?a ?b - item ?c - key)
    :precondition (and (seeded) (tok ?a ?b ?c))
    :effect (done ?a)))` | str_key | ROUTE_DOM = "(define (domain fixroute) (:requirements :typing)
  (:types item key - object)
  (:predicates (tok ?a ?b - item ?c - key) (done ?a - item) (seeded))
  (:action seed :parameters ()
    :precondition (and) :effect (and (seeded) (tok i1 i2 k1)))
  (:action reap :parameters (?a ?b - item ?c - key)
    :precondition (and (seeded) (tok ?a ?b ?c))
    :effect (done ?a)))" |  |  |  |  |


### crates/ferroplan/tests/mem_wall.rs

| `MEM_WALL_CHILD` | env_key | std::env::var("MEM_WALL_CHILD") |  |  |  |  |

| `0.25` | str_key | BUDGET_GB = "0.25" |  |  |  |  |


### crates/ferroplan/tests/memory_stress.rs

| `FERROPLAN_MEMORY_STRESS_CHILD` | str_key | CHILD_ENV = "FERROPLAN_MEMORY_STRESS_CHILD" |  |  |  |  |

| `predicates-500` | str_key | SAMPLED_CASES = "predicates-500" |  |  |  |  |


### crates/ferroplan/tests/mfw_python_oracle.rs

| `FERROPLAN_REQUIRE_MFW_ORACLE` | env_key | std::env::var_os("FERROPLAN_REQUIRE_MFW_ORACLE") |  |  |  |  |

| `MFW_PLANNER_ORACLE_PYTHONPATH` | env_key | std::env::var("MFW_PLANNER_ORACLE_PYTHONPATH") |  |  |  |  |


### crates/ferroplan/tests/netben.rs

| `
(define (domain nb)
  (:requirements :strips :action-costs)
  (:predicates (have-a) (have-b) (blocked))
  (:functions (total-cost) - number)
  (:action get-a
    :parameters ()
    :precondition ()
    :effect (and (have-a) (increase (total-cost) 3)))
  (:action get-b
    :parameters ()
    :precondition (blocked)
    :effect (and (have-b) (increase (total-cost) 1))))
` | str_key | NB_DOMAIN = "
(define (domain nb)
  (:requirements :strips :action-costs)
  (:predicates (have-a) (have-b) (blocked))
  (:functions (total-cost) - number)
  (:action get-a
    :parameters ()
    :precondition ()
    :effect (and (have-a) (increase (total-cost) 3)))
  (:action get-b
    :parameters ()
    :precondition (blocked)
    :effect (and (have-b) (increase (total-cost) 1))))
" |  |  |  |  |

| `
(define (problem nb-1) (:domain nb)
  (:init (= (total-cost) 0))
  (:goal (and (preference pa (have-a)) (preference pb (have-b))))
  (:metric maximize (- 15 (+ (total-cost)
                             (* (is-violated pa) 10)
                             (* (is-violated pb) 5)))))
` | str_key | NB_PROBLEM = "
(define (problem nb-1) (:domain nb)
  (:init (= (total-cost) 0))
  (:goal (and (preference pa (have-a)) (preference pb (have-b))))
  (:metric maximize (- 15 (+ (total-cost)
                             (* (is-violated pa) 10)
                             (* (is-violated pb) 5)))))
" |  |  |  |  |


### crates/ferroplan/tests/node_cap.rs

| `NODE_CAP_RLIMIT_CHILD` | env_key | std::env::var("NODE_CAP_RLIMIT_CHILD") |  |  |  |  |


### crates/ferroplan/tests/novdriver.rs

| `NOVDRIVER_CHILD` | env_key | std::env::var("NOVDRIVER_CHILD") |  |  |  |  |


### crates/ferroplan/tests/numfold.rs

| `NUMFOLD_CHILD` | env_key | std::env::var("NUMFOLD_CHILD") |  |  |  |  |


### crates/ferroplan/tests/numopt_arm.rs

| `NUMOPT_ARM_CHILD` | env_key | std::env::var("NUMOPT_ARM_CHILD") |  |  |  |  |


### crates/ferroplan/tests/numpre.rs

| `../../../benchmarks/bench/chained-band-domain.pddl` | str_key | CHAIN_DOM = "../../../benchmarks/bench/chained-band-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/chained-band-i1.pddl` | str_key | CHAIN_PRB = "../../../benchmarks/bench/chained-band-i1.pddl" |  |  |  |  |

| `../../../benchmarks/bench/fo-sailing-domain.pddl` | str_key | FOSAIL_DOM = "../../../benchmarks/bench/fo-sailing-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/fo-sailing-i8.pddl` | str_key | FOSAIL_I8 = "../../../benchmarks/bench/fo-sailing-i8.pddl" |  |  |  |  |

| `../../../benchmarks/bench/sailing-band-domain.pddl` | str_key | SAIL_DOM = "../../../benchmarks/bench/sailing-band-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/sailing-band-i1.pddl` | str_key | SAIL_PRB = "../../../benchmarks/bench/sailing-band-i1.pddl" |  |  |  |  |

| `../../../benchmarks/bench/trader-cycle-domain.pddl` | str_key | TRADE_DOM = "../../../benchmarks/bench/trader-cycle-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/trader-cycle-i1.pddl` | str_key | TRADE_PRB = "../../../benchmarks/bench/trader-cycle-i1.pddl" |  |  |  |  |

| `../../../benchmarks/bench/watering-line-domain.pddl` | str_key | WATER_DOM = "../../../benchmarks/bench/watering-line-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/watering-line-i1.pddl` | str_key | WATER_PRB = "../../../benchmarks/bench/watering-line-i1.pddl" |  |  |  |  |


### crates/ferroplan/tests/opt_wall.rs

| `OPT_WALL_CHILD` | env_key | std::env::var("OPT_WALL_CHILD") |  |  |  |  |


### crates/ferroplan/tests/orbit_classical.rs

| `
(define (domain fees)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget) (alldone))
  (:functions (total-cost) (fee ?g - gadget))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (fee ?g))))
  (:action finish
    :parameters (?g - gadget)
    :precondition (done ?g)
    :effect (alldone)))
` | str_key | FEE_DOM = "
(define (domain fees)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget) (alldone))
  (:functions (total-cost) (fee ?g - gadget))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (fee ?g))))
  (:action finish
    :parameters (?g - gadget)
    :precondition (done ?g)
    :effect (alldone)))
" |  |  |  |  |

| `
(define (domain gadgets)
  (:requirements :strips :typing :action-costs)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) 1))))
` | str_key | GADGET_DOM_CONST = "
(define (domain gadgets)
  (:requirements :strips :typing :action-costs)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) 1))))
" |  |  |  |  |

| `
(define (domain gadgets-dyn)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost) (surcharge))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (surcharge))))
  (:action bump
    :parameters ()
    :precondition (and)
    :effect (increase (surcharge) 1)))
` | str_key | GADGET_DOM_DYN = "
(define (domain gadgets-dyn)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost) (surcharge))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (surcharge))))
  (:action bump
    :parameters ()
    :precondition (and)
    :effect (increase (surcharge) 1)))
" |  |  |  |  |

| `
(define (domain gadgets-static)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost) (surcharge))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (surcharge)))))
` | str_key | GADGET_DOM_STATIC = "
(define (domain gadgets-static)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost) (surcharge))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (surcharge)))))
" |  |  |  |  |

| `
(define (domain mini-snack)
  (:requirements :strips :typing)
  (:types child bread sandwich tray)
  (:predicates (at-kitchen-bread ?b - bread) (at-kitchen-sandwich ?s - sandwich)
               (notexist ?s - sandwich) (ontray ?s - sandwich ?t - tray)
               (served ?c - child) (waiting ?c - child))
  (:action make
    :parameters (?s - sandwich ?b - bread)
    :precondition (and (notexist ?s) (at-kitchen-bread ?b))
    :effect (and (not (notexist ?s)) (not (at-kitchen-bread ?b))
                 (at-kitchen-sandwich ?s)))
  (:action put
    :parameters (?s - sandwich ?t - tray)
    :precondition (at-kitchen-sandwich ?s)
    :effect (and (not (at-kitchen-sandwich ?s)) (ontray ?s ?t)))
  (:action serve
    :parameters (?s - sandwich ?t - tray ?c - child)
    :precondition (and (ontray ?s ?t) (waiting ?c))
    :effect (and (not (ontray ?s ?t)) (not (waiting ?c)) (served ?c))))
` | str_key | SNACK_DOM = "
(define (domain mini-snack)
  (:requirements :strips :typing)
  (:types child bread sandwich tray)
  (:predicates (at-kitchen-bread ?b - bread) (at-kitchen-sandwich ?s - sandwich)
               (notexist ?s - sandwich) (ontray ?s - sandwich ?t - tray)
               (served ?c - child) (waiting ?c - child))
  (:action make
    :parameters (?s - sandwich ?b - bread)
    :precondition (and (notexist ?s) (at-kitchen-bread ?b))
    :effect (and (not (notexist ?s)) (not (at-kitchen-bread ?b))
                 (at-kitchen-sandwich ?s)))
  (:action put
    :parameters (?s - sandwich ?t - tray)
    :precondition (at-kitchen-sandwich ?s)
    :effect (and (not (at-kitchen-sandwich ?s)) (ontray ?s ?t)))
  (:action serve
    :parameters (?s - sandwich ?t - tray ?c - child)
    :precondition (and (ontray ?s ?t) (waiting ?c))
    :effect (and (not (ontray ?s ?t)) (not (waiting ?c)) (served ?c))))
" |  |  |  |  |

| `
(define (problem fees-1)
  (:domain fees)
  (:objects g1 g2 - gadget)
  (:init (fresh g1) (fresh g2) (= (total-cost) 0)
         (= (fee g1) 1) (= (fee g2) 9))
  (:goal (alldone))
  (:metric minimize (total-cost)))
` | str_key | FEE_PRB = "
(define (problem fees-1)
  (:domain fees)
  (:objects g1 g2 - gadget)
  (:init (fresh g1) (fresh g2) (= (total-cost) 0)
         (= (fee g1) 1) (= (fee g2) 9))
  (:goal (alldone))
  (:metric minimize (total-cost)))
" |  |  |  |  |

| `
(define (problem mini-snack-1)
  (:domain mini-snack)
  (:objects c1 c2 c3 - child b1 b2 b3 - bread s1 s2 s3 s4 - sandwich t1 - tray)
  (:init (waiting c1) (waiting c2) (waiting c3)
         (at-kitchen-bread b1) (at-kitchen-bread b2) (at-kitchen-bread b3)
         (notexist s1) (notexist s2) (notexist s3) (notexist s4))
  (:goal (and (served c1) (served c2) (served c3))))
` | str_key | SNACK_PRB = "
(define (problem mini-snack-1)
  (:domain mini-snack)
  (:objects c1 c2 c3 - child b1 b2 b3 - bread s1 s2 s3 s4 - sandwich t1 - tray)
  (:init (waiting c1) (waiting c2) (waiting c3)
         (at-kitchen-bread b1) (at-kitchen-bread b2) (at-kitchen-bread b3)
         (notexist s1) (notexist s2) (notexist s3) (notexist s4))
  (:goal (and (served c1) (served c2) (served c3))))
" |  |  |  |  |


### crates/ferroplan/tests/orbit_iso.rs

| `
    (define (problem iso-paint-uniform)
      (:domain iso-paint)
      (:objects b1 b2 b3 b4 - block)
      (:init (clean b1) (clean b2) (clean b3) (clean b4))
      (:goal (and (red b1) (red b2) (red b3) (red b4))))
    ` | str_key | UNIFORM_PRB = "
    (define (problem iso-paint-uniform)
      (:domain iso-paint)
      (:objects b1 b2 b3 b4 - block)
      (:init (clean b1) (clean b2) (clean b3) (clean b4))
      (:goal (and (red b1) (red b2) (red b3) (red b4))))
    " |  |  |  |  |

| `
(define (domain iso-bake)
  (:requirements :strips :typing :durative-actions)
  (:types piece)
  (:predicates (raw ?p - piece) (made ?p - piece)
               (fancy ?p - piece) (plain ?p - piece) (free))
  (:durative-action make
    :parameters (?p - piece)
    :duration (= ?duration 2)
    :condition (and (at start (raw ?p)) (over all (free)))
    :effect (and (at start (not (raw ?p))) (at end (made ?p))))
  (:durative-action finish-fancy
    :parameters (?p - piece)
    :duration (= ?duration 1)
    :condition (at start (made ?p))
    :effect (at end (fancy ?p)))
  (:durative-action finish-plain
    :parameters (?p - piece)
    :duration (= ?duration 1)
    :condition (at start (made ?p))
    :effect (at end (plain ?p))))
` | str_key | BAKE_DOM = "
(define (domain iso-bake)
  (:requirements :strips :typing :durative-actions)
  (:types piece)
  (:predicates (raw ?p - piece) (made ?p - piece)
               (fancy ?p - piece) (plain ?p - piece) (free))
  (:durative-action make
    :parameters (?p - piece)
    :duration (= ?duration 2)
    :condition (and (at start (raw ?p)) (over all (free)))
    :effect (and (at start (not (raw ?p))) (at end (made ?p))))
  (:durative-action finish-fancy
    :parameters (?p - piece)
    :duration (= ?duration 1)
    :condition (at start (made ?p))
    :effect (at end (fancy ?p)))
  (:durative-action finish-plain
    :parameters (?p - piece)
    :duration (= ?duration 1)
    :condition (at start (made ?p))
    :effect (at end (plain ?p))))
" |  |  |  |  |

| `
(define (domain iso-paint)
  (:requirements :strips :typing)
  (:types block)
  (:predicates (clean ?b - block) (red ?b - block) (blue ?b - block))
  (:action paint-red
    :parameters (?b - block)
    :precondition (clean ?b)
    :effect (and (not (clean ?b)) (red ?b)))
  (:action paint-blue
    :parameters (?b - block)
    :precondition (clean ?b)
    :effect (and (not (clean ?b)) (blue ?b))))
` | str_key | PAINT_DOM = "
(define (domain iso-paint)
  (:requirements :strips :typing)
  (:types block)
  (:predicates (clean ?b - block) (red ?b - block) (blue ?b - block))
  (:action paint-red
    :parameters (?b - block)
    :precondition (clean ?b)
    :effect (and (not (clean ?b)) (red ?b)))
  (:action paint-blue
    :parameters (?b - block)
    :precondition (clean ?b)
    :effect (and (not (clean ?b)) (blue ?b))))
" |  |  |  |  |

| `
(define (problem iso-bake-1)
  (:domain iso-bake)
  (:objects a b c - piece)
  (:init (raw a) (raw b) (raw c) (free))
  (:goal (and (fancy a) (plain b))))
` | str_key | BAKE_PRB = "
(define (problem iso-bake-1)
  (:domain iso-bake)
  (:objects a b c - piece)
  (:init (raw a) (raw b) (raw c) (free))
  (:goal (and (fancy a) (plain b))))
" |  |  |  |  |

| `
(define (problem iso-paint-1)
  (:domain iso-paint)
  (:objects b1 b2 b3 b4 - block)
  (:init (clean b1) (clean b2) (clean b3) (clean b4))
  (:goal (and (red b1) (blue b2))))
` | str_key | PAINT_PRB = "
(define (problem iso-paint-1)
  (:domain iso-paint)
  (:objects b1 b2 b3 b4 - block)
  (:init (clean b1) (clean b2) (clean b3) (clean b4))
  (:goal (and (red b1) (blue b2))))
" |  |  |  |  |

| `
(define (problem iso-paint-2)
  (:domain iso-paint)
  (:objects b1 b2 b3 b4 - block)
  (:init (clean b1) (clean b2) (clean b3) (clean b4))
  (:goal (and (red b1) (blue b3))))
` | str_key | PAINT_PRB_B3 = "
(define (problem iso-paint-2)
  (:domain iso-paint)
  (:objects b1 b2 b3 b4 - block)
  (:init (clean b1) (clean b2) (clean b3) (clean b4))
  (:goal (and (red b1) (blue b3))))
" |  |  |  |  |


### crates/ferroplan/tests/orbits.rs

| `
(define (domain fuel-gap)
  (:requirements :typing :durative-actions :numeric-fluents)
  (:types rig)
  (:predicates (idle) (hot) (done) (dipped) (refilled))
  (:functions (level))
  (:durative-action run
    :parameters (?r - rig)
    :duration (= ?duration 10)
    :condition (and (at start (idle)) (over all (>= (level) 1)))
    :effect (and (at start (not (idle))) (at start (hot))
                 (at end (not (hot))) (at end (done))))
  (:durative-action topup
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (idle))
    :effect (at start (increase (level) 2)))
  (:durative-action dip
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (hot))
    :effect (and (at start (dipped)) (at start (decrease (level) 2))))
  (:durative-action refill
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (dipped))
    :effect (and (at start (refilled)) (at start (increase (level) 2)))))
` | str_key | FUEL_DOM = "
(define (domain fuel-gap)
  (:requirements :typing :durative-actions :numeric-fluents)
  (:types rig)
  (:predicates (idle) (hot) (done) (dipped) (refilled))
  (:functions (level))
  (:durative-action run
    :parameters (?r - rig)
    :duration (= ?duration 10)
    :condition (and (at start (idle)) (over all (>= (level) 1)))
    :effect (and (at start (not (idle))) (at start (hot))
                 (at end (not (hot))) (at end (done))))
  (:durative-action topup
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (idle))
    :effect (at start (increase (level) 2)))
  (:durative-action dip
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (hot))
    :effect (and (at start (dipped)) (at start (decrease (level) 2))))
  (:durative-action refill
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (dipped))
    :effect (and (at start (refilled)) (at start (increase (level) 2)))))
" |  |  |  |  |

| `
(define (domain kiln-gap)
  (:requirements :typing :durative-actions :timed-initial-literals)
  (:types piece)
  (:predicates (ready) (raw ?p - piece) (prepped ?p - piece) (baked ?p - piece))
  (:durative-action prep
    :parameters (?p - piece)
    :duration (= ?duration 6)
    :condition (at start (raw ?p))
    :effect (and (at start (not (raw ?p))) (at end (prepped ?p))))
  (:durative-action bake
    :parameters (?p - piece)
    :duration (= ?duration 3)
    :condition (and (at start (prepped ?p)) (over all (ready)))
    :effect (at end (baked ?p))))
` | str_key | KILN_DOM = "
(define (domain kiln-gap)
  (:requirements :typing :durative-actions :timed-initial-literals)
  (:types piece)
  (:predicates (ready) (raw ?p - piece) (prepped ?p - piece) (baked ?p - piece))
  (:durative-action prep
    :parameters (?p - piece)
    :duration (= ?duration 6)
    :condition (at start (raw ?p))
    :effect (and (at start (not (raw ?p))) (at end (prepped ?p))))
  (:durative-action bake
    :parameters (?p - piece)
    :duration (= ?duration 3)
    :condition (and (at start (prepped ?p)) (over all (ready)))
    :effect (at end (baked ?p))))
" |  |  |  |  |

| `
(define (domain mini-tms)
  (:requirements :strips :typing :durative-actions)
  (:types piece)
  (:predicates (raw ?p - piece) (made ?p - piece) (glued ?a ?b - piece) (free))
  (:durative-action make
    :parameters (?p - piece)
    :duration (= ?duration 2)
    :condition (and (at start (raw ?p)) (over all (free)))
    :effect (and (at start (not (raw ?p))) (at end (made ?p))))
  (:durative-action glue
    :parameters (?a ?b - piece)
    :duration (= ?duration 3)
    :condition (and (at start (made ?a)) (at start (made ?b)))
    :effect (at end (glued ?a ?b))))
` | str_key | MINI_DOM = "
(define (domain mini-tms)
  (:requirements :strips :typing :durative-actions)
  (:types piece)
  (:predicates (raw ?p - piece) (made ?p - piece) (glued ?a ?b - piece) (free))
  (:durative-action make
    :parameters (?p - piece)
    :duration (= ?duration 2)
    :condition (and (at start (raw ?p)) (over all (free)))
    :effect (and (at start (not (raw ?p))) (at end (made ?p))))
  (:durative-action glue
    :parameters (?a ?b - piece)
    :duration (= ?duration 3)
    :condition (and (at start (made ?a)) (at start (made ?b)))
    :effect (at end (glued ?a ?b))))
" |  |  |  |  |

| `
(define (problem fuel-gap-1)
  (:domain fuel-gap)
  (:objects r1 - rig)
  (:init (idle) (= (level) 2))
  (:goal (and (done) (dipped) (refilled)))
  (:metric minimize (total-time)))
` | str_key | FUEL_PROB = "
(define (problem fuel-gap-1)
  (:domain fuel-gap)
  (:objects r1 - rig)
  (:init (idle) (= (level) 2))
  (:goal (and (done) (dipped) (refilled)))
  (:metric minimize (total-time)))
" |  |  |  |  |

| `
(define (problem kiln-gap-1)
  (:domain kiln-gap)
  (:objects p1 - piece)
  (:init (raw p1) (ready)
         (at 8 (not (ready)))
         (at 8.001 (ready)))
  (:goal (baked p1))
  (:metric minimize (total-time)))
` | str_key | KILN_PROB = "
(define (problem kiln-gap-1)
  (:domain kiln-gap)
  (:objects p1 - piece)
  (:init (raw p1) (ready)
         (at 8 (not (ready)))
         (at 8.001 (ready)))
  (:goal (baked p1))
  (:metric minimize (total-time)))
" |  |  |  |  |

| `
(define (problem mini-tms-1)
  (:domain mini-tms)
  (:objects a1 b1 a2 b2 - piece)
  (:init (raw a1) (raw b1) (raw a2) (raw b2) (free))
  (:goal (and (glued a1 b1) (glued a2 b2))))
` | str_key | MINI_PROB = "
(define (problem mini-tms-1)
  (:domain mini-tms)
  (:objects a1 b1 a2 b2 - piece)
  (:init (raw a1) (raw b1) (raw a2) (raw b2) (free))
  (:goal (and (glued a1 b1) (glued a2 b2))))
" |  |  |  |  |


### crates/ferroplan/tests/parse.rs

| `(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room) (free ?g - gripper))
  (:functions (cost))
  (:action move :parameters (?from ?to - room)
    :precondition (at-robby ?from) :effect (and (not (at-robby ?from)) (at-robby ?to)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r)) :effect (not (at ?b ?r))))` | str_key | DOM = "(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room) (free ?g - gripper))
  (:functions (cost))
  (:action move :parameters (?from ?to - room)
    :precondition (at-robby ?from) :effect (and (not (at-robby ?from)) (at-robby ?to)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r)) :effect (not (at ?b ?r))))" |  |  |  |  |

| `(define (problem gripper-1) (:domain gripper)
  (:objects rooma roomb - room b1 b2 - ball left right - gripper)
  (:init (at-robby rooma) (at b1 rooma) (= (cost) 0))
  (:goal (at b1 roomb))
  (:metric minimize (cost)))` | str_key | PROB = "(define (problem gripper-1) (:domain gripper)
  (:objects rooma roomb - room b1 b2 - ball left right - gripper)
  (:init (at-robby rooma) (at b1 rooma) (= (cost) 0))
  (:goal (at b1 roomb))
  (:metric minimize (cost)))" |  |  |  |  |


### crates/ferroplan/tests/pddl3.rs

| `(define (domain mk)
 (:requirements :strips :typing :adl :fluents)
 (:types item)
 (:predicates (special ?x - item) (can ?x - item))
 (:functions (total-cost))
 (:action make :parameters (?x - item) :precondition (can ?x) :effect (special ?x)))` | str_key | MARK = "(define (domain mk)
 (:requirements :strips :typing :adl :fluents)
 (:types item)
 (:predicates (special ?x - item) (can ?x - item))
 (:functions (total-cost))
 (:action make :parameters (?x - item) :precondition (can ?x) :effect (special ?x)))" |  |  |  |  |

| `(define (domain sp)
 (:requirements :strips :adl :fluents)
 (:predicates (ready) (done))
 (:functions (total-cost))
 (:action go :parameters ()
   :precondition (preference want (ready))
   :effect (done)))` | str_key | SOFTPRE = "(define (domain sp)
 (:requirements :strips :adl :fluents)
 (:predicates (ready) (done))
 (:functions (total-cost))
 (:action go :parameters ()
   :precondition (preference want (ready))
   :effect (done)))" |  |  |  |  |


### crates/ferroplan/tests/portfolio.rs

| `(define (domain chain)
  (:predicates (p0) (p1) (p2) (p3))
  (:action s1 :precondition (p0) :effect (p1))
  (:action s2 :precondition (p1) :effect (p2))
  (:action s3 :precondition (p2) :effect (p3)))` | str_key | DOM = "(define (domain chain)
  (:predicates (p0) (p1) (p2) (p3))
  (:action s1 :precondition (p0) :effect (p1))
  (:action s2 :precondition (p1) :effect (p2))
  (:action s3 :precondition (p2) :effect (p3)))" |  |  |  |  |


### crates/ferroplan/tests/pref_chase_wall.rs

| `PREF_CHASE_WALL_CHILD` | env_key | std::env::var("PREF_CHASE_WALL_CHILD") |  |  |  |  |


### crates/ferroplan/tests/pref_seed.rs

| `(define (domain corridor)
 (:requirements :strips :typing :preferences)
 (:types cell)
 (:predicates (at ?c - cell) (adj ?a ?b - cell) (lit ?c - cell) (visited ?c - cell))
 (:action move :parameters (?a ?b - cell)
   :precondition (and (at ?a) (adj ?a ?b) (preference darkstep (lit ?a)))
   :effect (and (not (at ?a)) (at ?b) (visited ?b)))
 (:action light :parameters (?a - cell)
   :precondition (at ?a)
   :effect (lit ?a)))` | str_key | CORRIDOR = "(define (domain corridor)
 (:requirements :strips :typing :preferences)
 (:types cell)
 (:predicates (at ?c - cell) (adj ?a ?b - cell) (lit ?c - cell) (visited ?c - cell))
 (:action move :parameters (?a ?b - cell)
   :precondition (and (at ?a) (adj ?a ?b) (preference darkstep (lit ?a)))
   :effect (and (not (at ?a)) (at ?b) (visited ?b)))
 (:action light :parameters (?a - cell)
   :precondition (at ?a)
   :effect (lit ?a)))" |  |  |  |  |

| `(define (problem walk) (:domain corridor)
 (:objects c0 c1 c2 c3 s1 - cell)
 (:init (at c0)
        (adj c0 c1) (adj c1 c2) (adj c2 c3) (adj c1 s1) (adj s1 c1))
 (:goal (and (at c3) (preference sidetrip (visited s1))))
 (:metric minimize (+ (is-violated darkstep) (* 5 (is-violated sidetrip)))))` | str_key | WALK = "(define (problem walk) (:domain corridor)
 (:objects c0 c1 c2 c3 s1 - cell)
 (:init (at c0)
        (adj c0 c1) (adj c1 c2) (adj c2 c3) (adj c1 s1) (adj s1 c1))
 (:goal (and (at c3) (preference sidetrip (visited s1))))
 (:metric minimize (+ (is-violated darkstep) (* 5 (is-violated sidetrip)))))" |  |  |  |  |


### crates/ferroplan/tests/refill.rs

| `REFILL_CHILD` | env_key | std::env::var("REFILL_CHILD") |  |  |  |  |


### crates/ferroplan/tests/robustness.rs

| `(define (domain d) (:requirements :strips)
  (:predicates (a) (b))
  (:action go :parameters () :precondition (a) :effect (and (not (a)) (b))))` | str_key | GOOD_DOM = "(define (domain d) (:requirements :strips)
  (:predicates (a) (b))
  (:action go :parameters () :precondition (a) :effect (and (not (a)) (b))))" |  |  |  |  |

| `(define (problem p) (:domain d) (:init (a)) (:goal (b)))` | str_key | GOOD_PROB = "(define (problem p) (:domain d) (:init (a)) (:goal (b)))" |  |  |  |  |


### crates/ferroplan/tests/sa2a_goal_set_ruling.rs

| `fixtures/sa2a-v26.9.17/sa2a-v26.9.17-domain.hddl` | str_key | DOMAIN = "fixtures/sa2a-v26.9.17/sa2a-v26.9.17-domain.hddl" |  |  |  |  |

| `fixtures/sa2a-v26.9.17/sa2a-v26.9.17-problem.hddl` | str_key | PROBLEM = "fixtures/sa2a-v26.9.17/sa2a-v26.9.17-problem.hddl" |  |  |  |  |


### crates/ferroplan/tests/sat_promo_wall.rs

| `SAT_PROMO_CHILD` | env_key | std::env::var("SAT_PROMO_CHILD") |  |  |  |  |

| `
(define (domain sat-rc)
  (:requirements :strips :durative-actions)
  (:predicates (light) (open) (fresh-shine) (fresh-mend) (fresh-deliver)
               (fresh-door) (mended) (delivered))
  (:durative-action shine
    :parameters ()
    :duration (= ?duration 20)
    :condition (at start (fresh-shine))
    :effect (and (at start (not (fresh-shine)))
                 (at start (light))
                 (at end (not (light)))))
  (:durative-action mend
    :parameters ()
    :duration (= ?duration 9)
    :condition (and (at start (fresh-mend)) (over all (light)))
    :effect (and (at start (not (fresh-mend))) (at end (mended))))
  (:durative-action deliver
    :parameters ()
    :duration (= ?duration 6)
    :condition (and (at start (fresh-deliver)) (at end (open)))
    :effect (and (at start (not (fresh-deliver))) (at end (delivered))))
  (:durative-action door
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-door))
    :effect (and (at start (not (fresh-door)))
                 (at start (open))
                 (at end (not (open))))))
` | str_key | RC_DOMAIN = "
(define (domain sat-rc)
  (:requirements :strips :durative-actions)
  (:predicates (light) (open) (fresh-shine) (fresh-mend) (fresh-deliver)
               (fresh-door) (mended) (delivered))
  (:durative-action shine
    :parameters ()
    :duration (= ?duration 20)
    :condition (at start (fresh-shine))
    :effect (and (at start (not (fresh-shine)))
                 (at start (light))
                 (at end (not (light)))))
  (:durative-action mend
    :parameters ()
    :duration (= ?duration 9)
    :condition (and (at start (fresh-mend)) (over all (light)))
    :effect (and (at start (not (fresh-mend))) (at end (mended))))
  (:durative-action deliver
    :parameters ()
    :duration (= ?duration 6)
    :condition (and (at start (fresh-deliver)) (at end (open)))
    :effect (and (at start (not (fresh-deliver))) (at end (delivered))))
  (:durative-action door
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-door))
    :effect (and (at start (not (fresh-door)))
                 (at start (open))
                 (at end (not (open))))))
" |  |  |  |  |

| `
(define (problem sat-rc-1) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend) (fresh-deliver) (fresh-door))
  (:goal (and (mended) (delivered))))
` | str_key | RC_PROBLEM = "
(define (problem sat-rc-1) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend) (fresh-deliver) (fresh-door))
  (:goal (and (mended) (delivered))))
" |  |  |  |  |

| `
(define (problem sat-rc-env) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend))
  (:goal (mended)))
` | str_key | ENVELOPE_PROBLEM = "
(define (problem sat-rc-env) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend))
  (:goal (mended)))
" |  |  |  |  |


### crates/ferroplan/tests/sat_wing.rs

| `FERROPLAN_VAL` | env_key | std::env::var("FERROPLAN_VAL") |  |  |  |  |

| `
(define (domain sat-micro)
  (:requirements :strips :typing)
  (:types loc)
  (:predicates (at ?l - loc) (adj ?a ?b - loc))
  (:action move
    :parameters (?a ?b - loc)
    :precondition (and (at ?a) (adj ?a ?b))
    :effect (and (not (at ?a)) (at ?b))))
` | str_key | MICRO_DOMAIN = "
(define (domain sat-micro)
  (:requirements :strips :typing)
  (:types loc)
  (:predicates (at ?l - loc) (adj ?a ?b - loc))
  (:action move
    :parameters (?a ?b - loc)
    :precondition (and (at ?a) (adj ?a ?b))
    :effect (and (not (at ?a)) (at ?b))))
" |  |  |  |  |

| `
(define (domain sat-rc)
  (:requirements :strips :durative-actions)
  (:predicates (light) (open) (fresh-shine) (fresh-mend) (fresh-deliver)
               (fresh-door) (mended) (delivered))
  (:durative-action shine
    :parameters ()
    :duration (= ?duration 20)
    :condition (at start (fresh-shine))
    :effect (and (at start (not (fresh-shine)))
                 (at start (light))
                 (at end (not (light)))))
  (:durative-action mend
    :parameters ()
    :duration (= ?duration 9)
    :condition (and (at start (fresh-mend)) (over all (light)))
    :effect (and (at start (not (fresh-mend))) (at end (mended))))
  (:durative-action deliver
    :parameters ()
    :duration (= ?duration 6)
    :condition (and (at start (fresh-deliver)) (at end (open)))
    :effect (and (at start (not (fresh-deliver))) (at end (delivered))))
  (:durative-action door
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-door))
    :effect (and (at start (not (fresh-door)))
                 (at start (open))
                 (at end (not (open))))))
` | str_key | RC_DOMAIN = "
(define (domain sat-rc)
  (:requirements :strips :durative-actions)
  (:predicates (light) (open) (fresh-shine) (fresh-mend) (fresh-deliver)
               (fresh-door) (mended) (delivered))
  (:durative-action shine
    :parameters ()
    :duration (= ?duration 20)
    :condition (at start (fresh-shine))
    :effect (and (at start (not (fresh-shine)))
                 (at start (light))
                 (at end (not (light)))))
  (:durative-action mend
    :parameters ()
    :duration (= ?duration 9)
    :condition (and (at start (fresh-mend)) (over all (light)))
    :effect (and (at start (not (fresh-mend))) (at end (mended))))
  (:durative-action deliver
    :parameters ()
    :duration (= ?duration 6)
    :condition (and (at start (fresh-deliver)) (at end (open)))
    :effect (and (at start (not (fresh-deliver))) (at end (delivered))))
  (:durative-action door
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-door))
    :effect (and (at start (not (fresh-door)))
                 (at start (open))
                 (at end (not (open))))))
" |  |  |  |  |

| `
(define (problem sat-micro-1) (:domain sat-micro)
  (:objects l1 l2 l3 - loc)
  (:init (at l1) (adj l1 l2) (adj l2 l3))
  (:goal (at l3)))
` | str_key | MICRO_PROBLEM = "
(define (problem sat-micro-1) (:domain sat-micro)
  (:objects l1 l2 l3 - loc)
  (:init (at l1) (adj l1 l2) (adj l2 l3))
  (:goal (at l3)))
" |  |  |  |  |

| `
(define (problem sat-rc-1) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend) (fresh-deliver) (fresh-door))
  (:goal (and (mended) (delivered))))
` | str_key | RC_PROBLEM = "
(define (problem sat-rc-1) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend) (fresh-deliver) (fresh-door))
  (:goal (and (mended) (delivered))))
" |  |  |  |  |


### crates/ferroplan/tests/scaling_ladder.rs

| `SCALING_LADDER_RESULTS` | env_key | std::env::var("SCALING_LADDER_RESULTS") |  |  |  |  |

| `SCALING_LADDER_RSS_KB` | env_key | std::env::var("SCALING_LADDER_RSS_KB") |  |  |  |  |

| `| family | n | m | ground_actions | ground_methods | states | transitions | parse_ms | ground_ms | translate_ms | solve_ms | outcome |` | str_key | SWEEP_HEADER = "| family | n | m | ground_actions | ground_methods | states | transitions | parse_ms | ground_ms | translate_ms | solve_ms | outcome |" |  |  |  |  |


### crates/ferroplan/tests/tcompress.rs

| `TCOMPRESS_TCONC_CHILD` | env_key | std::env::var("TCOMPRESS_TCONC_CHILD") |  |  |  |  |

| `(define (domain shop)
 (:requirements :typing :durative-actions)
 (:types job machine)
 (:predicates (todo ?j - job) (done ?j - job) (fits ?j - job ?m - machine)
              (free ?m - machine) (busy ?m - machine))
 (:durative-action run
   :parameters (?j - job ?m - machine)
   :duration (= ?duration 3)
   :condition (and (at start (todo ?j)) (at start (free ?m))
                   (over all (fits ?j ?m)) (at end (busy ?m)))
   :effect (and (at start (not (todo ?j))) (at start (not (free ?m))) (at start (busy ?m))
                (at end (not (busy ?m))) (at end (free ?m)) (at end (done ?j)))))` | str_key | SHOP = "(define (domain shop)
 (:requirements :typing :durative-actions)
 (:types job machine)
 (:predicates (todo ?j - job) (done ?j - job) (fits ?j - job ?m - machine)
              (free ?m - machine) (busy ?m - machine))
 (:durative-action run
   :parameters (?j - job ?m - machine)
   :duration (= ?duration 3)
   :condition (and (at start (todo ?j)) (at start (free ?m))
                   (over all (fits ?j ?m)) (at end (busy ?m)))
   :effect (and (at start (not (todo ?j))) (at start (not (free ?m))) (at start (busy ?m))
                (at end (not (busy ?m))) (at end (free ?m)) (at end (done ?j)))))" |  |  |  |  |


### crates/ferroplan/tests/tdemand.rs

| `(define (domain mr)
  (:requirements :durative-actions :numeric-fluents)
  (:predicates (ready))
  (:functions (raw) (mid) (top))
  (:durative-action gather :parameters ()
    :duration (= ?duration 1)
    :condition (at start (ready))
    :effect (at end (increase (raw) 1)))
  (:durative-action refine :parameters ()
    :duration (= ?duration 1)
    :condition (at start (>= (raw) 1))
    :effect (and (at start (decrease (raw) 1)) (at end (increase (mid) 1))))
  (:durative-action assemble :parameters ()
    :duration (= ?duration 1)
    :condition (at start (>= (mid) 1))
    :effect (and (at start (decrease (mid) 1)) (at end (increase (top) 1)))))` | str_key | DOM = "(define (domain mr)
  (:requirements :durative-actions :numeric-fluents)
  (:predicates (ready))
  (:functions (raw) (mid) (top))
  (:durative-action gather :parameters ()
    :duration (= ?duration 1)
    :condition (at start (ready))
    :effect (at end (increase (raw) 1)))
  (:durative-action refine :parameters ()
    :duration (= ?duration 1)
    :condition (at start (>= (raw) 1))
    :effect (and (at start (decrease (raw) 1)) (at end (increase (mid) 1))))
  (:durative-action assemble :parameters ()
    :duration (= ?duration 1)
    :condition (at start (>= (mid) 1))
    :effect (and (at start (decrease (mid) 1)) (at end (increase (top) 1)))))" |  |  |  |  |

| `(define (problem mr3) (:domain mr)
  (:init (ready) (= (raw) 0) (= (mid) 0) (= (top) 0))
  (:goal (>= (top) 3)))` | str_key | PROB = "(define (problem mr3) (:domain mr)
  (:init (ready) (= (raw) 0) (= (mid) 0) (= (top) 0))
  (:goal (>= (top) 3)))" |  |  |  |  |


### crates/ferroplan/tests/temporal.rs

| `ESCALATION_CHILD` | env_key | std::env::var("ESCALATION_CHILD") |  |  |  |  |

| `
(define (domain crew)
  (:requirements :typing :durative-actions :numeric-fluents)
  (:types task)
  (:predicates (done ?t - task))
  (:functions (avail))
  (:durative-action do
    :parameters (?t - task)
    :duration (= ?duration 5)
    :condition (at start (>= (avail) 1))
    :effect (and (at start (decrease (avail) 1))
                 (at end (increase (avail) 1))
                 (at end (done ?t)))))
` | str_key | RESOURCE_DOM = "
(define (domain crew)
  (:requirements :typing :durative-actions :numeric-fluents)
  (:types task)
  (:predicates (done ?t - task))
  (:functions (avail))
  (:durative-action do
    :parameters (?t - task)
    :duration (= ?duration 5)
    :condition (at start (>= (avail) 1))
    :effect (and (at start (decrease (avail) 1))
                 (at end (increase (avail) 1))
                 (at end (done ?t)))))
" |  |  |  |  |

| `
(define (domain gate)
  (:requirements :durative-actions)
  (:predicates (open) (through))
  (:durative-action pass
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (open))
    :effect (at end (through))))
` | str_key | TIL_DOM = "
(define (domain gate)
  (:requirements :durative-actions)
  (:predicates (open) (through))
  (:durative-action pass
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (open))
    :effect (at end (through))))
" |  |  |  |  |

| `
(define (domain ineq)
  (:requirements :durative-actions)
  (:predicates (done))
  (:durative-action work
    :parameters ()
    :duration (and (>= ?duration 2) (<= ?duration 5))
    :condition ()
    :effect (at end (done))))
` | str_key | INEQ_DOM = "
(define (domain ineq)
  (:requirements :durative-actions)
  (:predicates (done))
  (:durative-action work
    :parameters ()
    :duration (and (>= ?duration 2) (<= ?duration 5))
    :condition ()
    :effect (at end (done))))
" |  |  |  |  |

| `
(define (domain t)
  (:requirements :strips :durative-actions :numeric-fluents)
  (:predicates (at) (goal) (light))
  (:durative-action act
    :parameters ()
    :duration (= ?duration 3)
    :condition (and (at start (at)) (over all (light)))
    :effect (and (at start (not (at))) (at end (goal)))))` | str_key | DUR_DOM = "
(define (domain t)
  (:requirements :strips :durative-actions :numeric-fluents)
  (:predicates (at) (goal) (light))
  (:durative-action act
    :parameters ()
    :duration (= ?duration 3)
    :condition (and (at start (at)) (over all (light)))
    :effect (and (at start (not (at))) (at end (goal)))))" |  |  |  |  |

| `
(define (domain temporal-test)
  (:requirements :strips :typing :durative-actions :numeric-fluents)
  (:types location)
  (:predicates (at ?l - location) (connected ?a ?b - location) (free))
  (:functions (dist ?a ?b - location))
  (:durative-action move
    :parameters (?from ?to - location)
    :duration (= ?duration (dist ?from ?to))
    :condition (and (at start (at ?from))
                    (at start (connected ?from ?to))
                    (over all (free)))
    :effect (and (at start (not (at ?from)))
                 (at end (at ?to)))))
` | str_key | DOM = "
(define (domain temporal-test)
  (:requirements :strips :typing :durative-actions :numeric-fluents)
  (:types location)
  (:predicates (at ?l - location) (connected ?a ?b - location) (free))
  (:functions (dist ?a ?b - location))
  (:durative-action move
    :parameters (?from ?to - location)
    :duration (= ?duration (dist ?from ?to))
    :condition (and (at start (at ?from))
                    (at start (connected ?from ?to))
                    (over all (free)))
    :effect (and (at start (not (at ?from)))
                 (at end (at ?to)))))
" |  |  |  |  |

| `(define (problem g) (:domain gate)
  (:init (at 5 (open)))
  (:goal (through)))` | str_key | TIL_PROB = "(define (problem g) (:domain gate)
  (:init (at 5 (open)))
  (:goal (through)))" |  |  |  |  |

| `(define (problem p) (:domain t) (:init (at) (light)) (:goal (goal)))` | str_key | DUR_PROB = "(define (problem p) (:domain t) (:init (at) (light)) (:goal (goal)))" |  |  |  |  |

| `(define (problem w) (:domain ineq) (:init) (:goal (done)))` | str_key | INEQ_PROB = "(define (problem w) (:domain ineq) (:init) (:goal (done)))" |  |  |  |  |


### crates/ferroplan/tests/temporal_constraints.rs

| `(define (domain tconstr)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (home) (done) (flag))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (home))
    :effect (at end (done)))
  (:durative-action raise
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (home))
    :effect (at end (flag))))` | str_key | ATEND_DOM = "(define (domain tconstr)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (home) (done) (flag))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (home))
    :effect (at end (done)))
  (:durative-action raise
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (home))
    :effect (at end (flag))))" |  |  |  |  |

| `(define (domain tresp)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (idle) (alarm) (handled) (done))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (idle))
    :effect (and (at start (not (idle))) (at start (alarm)) (at end (done))))
  (:durative-action quiet-work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (idle))
    :effect (and (at start (not (idle))) (at end (done))))
  (:durative-action respond
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (alarm))
    :effect (at end (handled))))` | str_key | RESPOND_DOM = "(define (domain tresp)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (idle) (alarm) (handled) (done))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (idle))
    :effect (and (at start (not (idle))) (at start (alarm)) (at end (done))))
  (:durative-action quiet-work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (idle))
    :effect (and (at start (not (idle))) (at end (done))))
  (:durative-action respond
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (alarm))
    :effect (at end (handled))))" |  |  |  |  |

| `(define (domain twin)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (home) (done) (flag))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (home))
    :effect (at end (done)))
  (:durative-action quick-flag
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (home))
    :effect (at end (flag)))
  (:durative-action slow-flag
    :parameters ()
    :duration (= ?duration 6)
    :condition (at start (home))
    :effect (at end (flag))))` | str_key | WITHIN_DOM = "(define (domain twin)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (home) (done) (flag))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (home))
    :effect (at end (done)))
  (:durative-action quick-flag
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (home))
    :effect (at end (flag)))
  (:durative-action slow-flag
    :parameters ()
    :duration (= ?duration 6)
    :condition (at start (home))
    :effect (at end (flag))))" |  |  |  |  |


### crates/ferroplan/tests/tground_wall.rs

| `TGROUND_WALL_CHILD` | env_key | std::env::var("TGROUND_WALL_CHILD") |  |  |  |  |


### crates/ferroplan/tests/think_following.rs

| `(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move
    :parameters (?from - room ?to - room)
    :precondition (at-robby ?from)
    :effect (and (at-robby ?to) (not (at-robby ?from))))
  (:action pick
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))` | str_key | GRIPPER_DOMAIN = "(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move
    :parameters (?from - room ?to - room)
    :precondition (at-robby ?from)
    :effect (and (at-robby ?to) (not (at-robby ?from))))
  (:action pick
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))" |  |  |  |  |

| `(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))` | str_key | CORRIDOR_DOMAIN = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (problem corridor)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c d))
  (:goal (at d)))` | str_key | CORRIDOR_PROBLEM = "(define (problem corridor)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c d))
  (:goal (at d)))" |  |  |  |  |

| `(define (problem dead-end)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c))
  (:goal (at d)))` | str_key | DEAD_END_PROBLEM = "(define (problem dead-end)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c))
  (:goal (at d)))" |  |  |  |  |

| `(define (problem gripper-4)
  (:domain gripper)
  (:objects rooma roomb - room b1 b2 b3 b4 - ball left right - gripper)
  (:init (at-robby rooma) (free left) (free right)
         (at b1 rooma) (at b2 rooma) (at b3 rooma) (at b4 rooma))
  (:goal (and (at b1 roomb) (at b2 roomb) (at b3 roomb) (at b4 roomb))))` | str_key | GRIPPER_PROBLEM = "(define (problem gripper-4)
  (:domain gripper)
  (:objects rooma roomb - room b1 b2 b3 b4 - ball left right - gripper)
  (:init (at-robby rooma) (free left) (free right)
         (at b1 rooma) (at b2 rooma) (at b3 rooma) (at b4 rooma))
  (:goal (and (at b1 roomb) (at b2 roomb) (at b3 roomb) (at b4 roomb))))" |  |  |  |  |


### crates/ferroplan/tests/translate_wall_ipc_addendum.rs

| `pcp_1` | str_key | CASES = "pcp_1" |  |  |  |  |


### crates/ferroplan/tests/tsearch_wall.rs

| `TSEARCH_WALL_CHILD` | env_key | std::env::var("TSEARCH_WALL_CHILD") |  |  |  |  |


### crates/ferroplan/tests/zero_duration.rs

| `
(define (domain z0)
  (:requirements :strips :durative-actions)
  (:predicates (a) (g) (h2))
  (:durative-action zap
    :parameters ()
    :duration (= ?duration 0)
    :condition (at start (a))
    :effect (at start (g)))
  (:durative-action chain
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (g))
    :effect (at end (h2))))
` | str_key | DOMAIN = "
(define (domain z0)
  (:requirements :strips :durative-actions)
  (:predicates (a) (g) (h2))
  (:durative-action zap
    :parameters ()
    :duration (= ?duration 0)
    :condition (at start (a))
    :effect (at start (g)))
  (:durative-action chain
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (g))
    :effect (at end (h2))))
" |  |  |  |  |



<!-- AGENT-FORBIDDEN-END -->

## Signature/type/default/errors table

<!-- RIGID table: header order is fixed; rows come only from the query. -->

| Item | Type | Signature | Params | Defaults | Errors | Invariants |
|------|------|-----------|--------|----------|--------|------------|

| `advance` | function | advance(time: Res<Time>, mut plan: ResMut<Plan>) |  |  |  |  |

| `animate` | function | animate( plan: Res<Plan>, scene: Res<Scene>, nodes: Query<(&NodeObj, &Transform)>, mut mobiles: Query<(&MobileObj, &FanOffset, &mut Transform), Without<NodeObj>>, ) |  |  |  |  |

| `controls` | function | controls( keys: Res<ButtonInput<KeyCode>>, scene: Res<Scene>, editor: Res<crate::blocks::Editor>, mut plan: ResMut<Plan>, mut job: ResMut<SolveJob>, ) |  |  |  |  |

| `frac` | function | frac(&self) -> f32 |  |  |  |  |

| `poll_solve` | function | poll_solve(mut job: ResMut<SolveJob>, mut plan: ResMut<Plan>) |  |  |  |  |

| `span` | function | span(&self) -> f32 |  |  |  |  |

| `start_frac` | function | start_frac(&self, step: &Step, idx: usize) -> f32 |  |  |  |  |

| `Plan` | struct | Plan { pub steps: Vec<Step>, pub snapshots: Vec<StateSnapshot>, pub t: f32, pub playing: bool, pub status: String, pub temporal: bool, pub makespan: f32 } |  |  |  |  |

| `SolveJob` | struct | SolveJob { Option<Task<SolveResult>> } |  |  |  |  |

| `Act` | enum | Act { AddObject, RemoveObject(usize), CycleType(usize), AddFact(bool), RemoveFact(bool, usize), CyclePred(bool, usize), CycleArg(bool, usize, usize), AddType, RemoveType(usize), CycleSuper(usize), AddPred, RemovePred(usize), AddArg(usize), RemoveArg(usize), CycleArgType(usize, usize), AddAction, RemoveAction(usize), AddParam(usize), RemoveParam(usize), CycleParamType(usize, usize), TogglePreKind(usize), AddWhen(usize), RemoveWhen(usize, usize), AddLit(usize, LitLoc), RemoveLit(usize, LitLoc, usize), CycleLitPred(usize, LitLoc, usize), CycleLitArg(usize, LitLoc, usize, usize), ToggleNeg(usize, LitLoc, usize), SetFocus(Focus), ToggleMode, Apply, Export, Close } |  |  |  |  |

| `DragKind` | enum | DragKind { Init(usize), Goal(usize) } |  |  |  |  |

| `Focus` | enum | Focus { DomainName, TypeName(usize), PredName(usize), ActionName(usize) } |  |  |  |  |

| `LitLoc` | enum | LitLoc { Pre, Eff, WhenCond(usize), WhenEff(usize) } |  |  |  |  |

| `Mode` | enum | Mode { Problem, Domain } |  |  |  |  |

| `Zone` | enum | Zone { Init, Goal } |  |  |  |  |

| `editor_drag` | function | editor_drag( mouse: Res<ButtonInput<MouseButton>>, windows: Query<&Window>, mut drag: ResMut<Drag>, mut editor: ResMut<Editor>, grips: Query<(&DragKind, &RelativeCursorPosition)>, zones: Query<(&Zone, &RelativeCursorPosition)>, mut ghosts: Query<&mut Node, With<Ghost>>, mut commands: Commands, ) |  |  |  |  |

| `handle_clicks` | function | handle_clicks( interactions: Query<(&Interaction, &Act), (Changed<Interaction>, With<Button>)>, mut editor: ResMut<Editor>, mut scene: ResMut<Scene>, ) |  |  |  |  |

| `rebuild` | function | rebuild( mut commands: Commands, mut editor: ResMut<Editor>, roots: Query<Entity, With<EditorRoot>>, ) |  |  |  |  |

| `scroll_editor` | function | scroll_editor( mut wheel: MessageReader<MouseWheel>, keys: Res<ButtonInput<KeyCode>>, editor: Res<Editor>, mut q: Query<&mut ScrollPosition, With<EditorRoot>>, ) |  |  |  |  |

| `text_input` | function | text_input(mut evr: MessageReader<KeyboardInput>, mut editor: ResMut<Editor>) |  |  |  |  |

| `toggle_editor` | function | toggle_editor( keys: Res<ButtonInput<KeyCode>>, scene: Res<Scene>, mut editor: ResMut<Editor>, ) |  |  |  |  |

| `../demo/domain.pddl` | str_key | DOMAIN = "../demo/domain.pddl" |  |  |  |  |

| `../demo/problem.pddl` | str_key | PROBLEM = "../demo/problem.pddl" |  |  |  |  |

| `Drag` | struct | Drag { held: Option<DragKind>, ghost: Option<Entity> } |  |  |  |  |

| `Editor` | struct | Editor { pub open: bool, pub focus: Option<Focus>, mode: Mode, dirty: bool, status: String, problem_name: String, objects: Vec<(String, String)>, init: Vec<(String, Vec<String>)>, goal: Vec<(String, Vec<String>)>, counters: HashMap<String, u32>, seeded: bool, dname: String, requirements: String, types: Vec<(String, String)>, dpreds: Vec<(String, Vec<String>)>, actions: Vec<EdAction>, dseeded: bool } |  |  |  |  |

| `EditorRoot` | struct |  |  |  |  |  |

| `Ghost` | struct |  |  |  |  |  |

| `gantt_now` | function | gantt_now(plan: Res<Plan>, mut now: Query<&mut Node, With<GanttNow>>) |  |  |  |  |

| `gantt_visibility` | function | gantt_visibility( plan: Res<Plan>, state: Res<GanttState>, editor: Res<crate::blocks::Editor>, mut panel: Query<&mut Visibility, With<GanttPanel>>, ) |  |  |  |  |

| `rebuild_gantt` | function | rebuild_gantt( mut commands: Commands, plan: Res<Plan>, mut state: ResMut<GanttState>, track: Query<Entity, With<GanttTrack>>, bars: Query<Entity, With<GanttBar>>, ) |  |  |  |  |

| `setup_gantt` | function | setup_gantt(mut commands: Commands) |  |  |  |  |

| `toggle_gantt` | function | toggle_gantt( keys: Res<ButtonInput<KeyCode>>, editor: Res<crate::blocks::Editor>, mut state: ResMut<GanttState>, ) |  |  |  |  |

| `GanttBar` | struct |  |  |  |  |  |

| `GanttNow` | struct |  |  |  |  |  |

| `GanttPanel` | struct |  |  |  |  |  |

| `GanttState` | struct | GanttState { pub open: bool, built_for: usize, built_span: f32 } |  |  |  |  |

| `GanttTrack` | struct |  |  |  |  |  |

| `IconShape` | enum | IconShape { Circle, Truck, Box, Person, Robot, Machine, Diamond } |  |  |  |  |

| `color_for` | function | color_for(ty: &str) -> Color |  |  |  |  |

| `mat_handle` | function | mat_handle( materials: &mut Assets<ColorMaterial>, cache: &mut MatCache, color: Color, ) -> Handle<ColorMaterial> |  |  |  |  |

| `mesh_handle` | function | mesh_handle( meshes: &mut Assets<Mesh>, cache: &mut MeshCache, shape: IconShape, size: f32, ) -> Handle<Mesh> |  |  |  |  |

| `shape_for` | function | shape_for(ty: &str) -> IconShape |  |  |  |  |

| `draw_selection` | function | draw_selection( mut gizmos: Gizmos, selected: Res<Selected>, nodes: Query<(&NodeObj, &Transform)>, mobiles: Query<(&MobileObj, &Transform), Without<NodeObj>>, ) |  |  |  |  |

| `interact` | function | interact( mouse: Res<ButtonInput<MouseButton>>, windows: Query<&Window>, editor: Res<crate::blocks::Editor>, cam_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>, mut nodes: Query<(Entity, &NodeObj, &mut Transform)>, mobiles: Query<(&MobileObj, &Transform), Without<NodeObj>>, transport: Res<crate::transport::Transport>, mut selected: ResMut<Selected>, mut drag: ResMut<DragState>, ) |  |  |  |  |

| `DragState` | struct | DragState { node: Option<Entity> } |  |  |  |  |

| `Selected` | struct | Selected { pub Option<String> } |  |  |  |  |

| `ACC` | const | ACC: Color |  |  |  |  |

| `BG` | const | BG: Color |  |  |  |  |

| `BG2` | const | BG2: Color |  |  |  |  |

| `CRATE_AMBER` | const | CRATE_AMBER: Color |  |  |  |  |

| `CY` | const | CY: Color |  |  |  |  |

| `EDGE` | const | EDGE: Color |  |  |  |  |

| `EDGE2` | const | EDGE2: Color |  |  |  |  |

| `FAINT` | const | FAINT: Color |  |  |  |  |

| `GREY_NODE` | const | GREY_NODE: Color |  |  |  |  |

| `INK` | const | INK: Color |  |  |  |  |

| `MUT` | const | MUT: Color |  |  |  |  |

| `NODE_PURPLE` | const | NODE_PURPLE: Color |  |  |  |  |

| `PANEL` | const | PANEL: Color |  |  |  |  |

| `PANEL2` | const | PANEL2: Color |  |  |  |  |

| `PANEL_BLUR` | const | PANEL_BLUR: Color |  |  |  |  |

| `RIG_GREEN` | const | RIG_GREEN: Color |  |  |  |  |

| `ZONE` | const | ZONE: Color |  |  |  |  |

| `MOBILE_SIZE` | const | MOBILE_SIZE: f32 |  |  |  |  |

| `NODE_SIZE` | const | NODE_SIZE: f32 |  |  |  |  |

| `camera_nav` | function | camera_nav( mouse: Res<ButtonInput<MouseButton>>, editor: Res<crate::blocks::Editor>, mut motion: MessageReader<bevy::input::mouse::MouseMotion>, mut wheel: MessageReader<MouseWheel>, mut cam: Query<(&mut Transform, &mut Projection), With<MainCamera>>, ) |  |  |  |  |

| `draw_edges` | function | draw_edges( mut gizmos: Gizmos, scene: Res<Scene>, plan: Res<crate::anim::Plan>, nodes: Query<(&NodeObj, &Transform)>, ) |  |  |  |  |

| `handle_drops` | function | handle_drops(mut drops: MessageReader<FileDragAndDrop>, mut scene: ResMut<Scene>) |  |  |  |  |

| `load_src` | function | load_src(&mut self, src: &str) |  |  |  |  |

| `respawn_graph` | function | respawn_graph( mut commands: Commands, mut scene: ResMut<Scene>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, existing: Query<Entity, With<GraphItem>>, ) |  |  |  |  |

| `setup` | function | setup(mut commands: Commands) |  |  |  |  |

| `FanOffset` | struct | FanOffset { pub Vec2 } |  |  |  |  |

| `GraphItem` | struct |  |  |  |  |  |

| `MainCamera` | struct |  |  |  |  |  |

| `MobileObj` | struct | MobileObj { pub String } |  |  |  |  |

| `NodeObj` | struct | NodeObj { pub String } |  |  |  |  |

| `Scene` | struct | Scene { pub domain: Option<Domain>, pub problem: Option<Problem>, pub domain_src: String, pub problem_src: String, pub graph: VizGraph, pub dirty: bool, pub status: String } |  |  |  |  |

| `rebuild_notches` | function | rebuild_notches( mut commands: Commands, plan: Res<Plan>, mut state: ResMut<Transport>, track: Query<Entity, With<ScrubTrack>>, notches: Query<Entity, With<StepNotch>>, ) |  |  |  |  |

| `setup_transport` | function | setup_transport(mut commands: Commands) |  |  |  |  |

| `transport_input` | function | transport_input( mouse: Res<ButtonInput<MouseButton>>, mut transport: ResMut<Transport>, mut plan: ResMut<Plan>, play_btn: Query<&Interaction, (With<PlayButton>, Changed<Interaction>)>, track: Query<(&Interaction, &RelativeCursorPosition), With<ScrubTrack>>, ) |  |  |  |  |

| `transport_sync` | function | transport_sync( plan: Res<Plan>, mut fill: Query<&mut Node, (With<ScrubFill>, Without<Playhead>)>, mut head: Query<&mut Node, (With<Playhead>, Without<ScrubFill>)>, mut icon: Query<&mut Text, (With<PlayIcon>, Without<TransportLabel>)>, mut label: Query<&mut Text, (With<TransportLabel>, Without<PlayIcon>)>, ) |  |  |  |  |

| `transport_visibility` | function | transport_visibility(plan: Res<Plan>, mut bar: Query<&mut Visibility, With<TransportBar>>) |  |  |  |  |

| `PlayButton` | struct |  |  |  |  |  |

| `PlayIcon` | struct |  |  |  |  |  |

| `Playhead` | struct |  |  |  |  |  |

| `ScrubFill` | struct |  |  |  |  |  |

| `ScrubTrack` | struct |  |  |  |  |  |

| `StepNotch` | struct |  |  |  |  |  |

| `Transport` | struct | Transport { pub hovering: bool, built_for: usize } |  |  |  |  |

| `TransportBar` | struct |  |  |  |  |  |

| `TransportLabel` | struct |  |  |  |  |  |

| `setup_ui` | function | setup_ui(mut commands: Commands) |  |  |  |  |

| `update_info` | function | update_info( scene: Res<Scene>, selected: Res<Selected>, plan: Res<Plan>, mut q: Query<&mut Text, With<InfoText>>, ) |  |  |  |  |

| `InfoText` | struct |  |  |  |  |  |

| `ferroplan.handoff` | str_key | KEY = "ferroplan.handoff" |  |  |  |  |

| `FF_THREADS` | const | FF_THREADS: usize |  |  |  |  |

| `FF_WEIGHT_G` | const | FF_WEIGHT_G: f64 |  |  |  |  |

| `FF_WEIGHT_H` | const | FF_WEIGHT_H: f64 |  |  |  |  |

| `PPDDL_DISCOUNT` | const | PPDDL_DISCOUNT: f64 |  |  |  |  |

| `PPDDL_EPISODES` | const | PPDDL_EPISODES: usize |  |  |  |  |

| `PPDDL_EPSILON` | const | PPDDL_EPSILON: f64 |  |  |  |  |

| `PPDDL_HORIZON` | const | PPDDL_HORIZON: usize |  |  |  |  |

| `PPDDL_MAX_INITIAL_OUTCOMES` | const | PPDDL_MAX_INITIAL_OUTCOMES: usize |  |  |  |  |

| `PPDDL_MAX_ITERATIONS` | const | PPDDL_MAX_ITERATIONS: usize |  |  |  |  |

| `PPDDL_MAX_OUTCOMES_PER_ACTION` | const | PPDDL_MAX_OUTCOMES_PER_ACTION: usize |  |  |  |  |

| `PPDDL_MAX_POLICY_ENTRIES` | const | PPDDL_MAX_POLICY_ENTRIES: usize |  |  |  |  |

| `PPDDL_MAX_STATES` | const | PPDDL_MAX_STATES: usize |  |  |  |  |

| `PPDDL_MAX_TRANSITIONS` | const | PPDDL_MAX_TRANSITIONS: usize |  |  |  |  |

| `PPDDL_MAX_VALUE_CELLS` | const | PPDDL_MAX_VALUE_CELLS: usize |  |  |  |  |

| `PPDDL_SEED` | const | PPDDL_SEED: u64 |  |  |  |  |

| `PPDDL_SIMULATION_MAX_STEPS` | const | PPDDL_SIMULATION_MAX_STEPS: usize |  |  |  |  |

| `PPDDL_THREADS` | const | PPDDL_THREADS: usize |  |  |  |  |

| `ModeArg` | enum | ModeArg { Auto, Ff, Partition, Pddl3, Temporal, Portfolio, Optimal, Sat } |  |  |  |  |

| `ObjectiveArg` | enum | ObjectiveArg { Auto, MaximizeGoalProbability, MinimizeGoalProbability, MaximizeExpectedReward, MinimizeExpectedReward, MaximizeExpectedMetric, MinimizeExpectedMetric } |  |  |  |  |

| `SearchArg` | enum | SearchArg { Auto, Ehc, BestFirst, EhcThenBestFirst } |  |  |  |  |

| `compile_pack` | function | compile_pack(pack: &ObservationPack, output_dir: &Path) -> Result<HarvestReceipt> |  |  |  |  |

| `replay_pack` | function | replay_pack( pack: &ObservationPack, expected: &HarvestReceipt, output_dir: &Path, ) -> Result<HarvestReceipt> |  |  |  |  |

| `extract_operators` | function | extract_operators(report: &AdmissionReport) -> Vec<PlanningOperator> |  |  |  |  |

| `collect_with_gh` | function | collect_with_gh( repositories: &[String], window: ObservationWindow, max_pages: usize, ) -> Result<ObservationPack> |  |  |  |  |

| `admit` | function | admit(pack: &ObservationPack) -> AdmissionReport |  |  |  |  |

| `digest_bytes` | function | digest_bytes(bytes: &[u8]) -> String |  |  |  |  |

| `load_observation_pack` | function | load_observation_pack(path: &Path) -> Result<ObservationPack> |  |  |  |  |

| `load_receipt` | function | load_receipt(path: &Path) -> Result<HarvestReceipt> |  |  |  |  |

| `receipt_exit_code` | function | receipt_exit_code(receipt: &HarvestReceipt) -> i32 |  |  |  |  |

| `save_observation_pack` | function | save_observation_pack(path: &Path, pack: &ObservationPack) -> Result<()> |  |  |  |  |

| `validate_pack` | function | validate_pack(pack: &ObservationPack) -> Result<()> |  |  |  |  |

| `validate_window` | function | validate_window(window: &ObservationWindow) -> Result<()> |  |  |  |  |

| `compile::{compile_pack, replay_pack}` | use | compile::{compile_pack, replay_pack} |  |  |  |  |

| `extract::extract_operators` | use | extract::extract_operators |  |  |  |  |

| `gh::collect_with_gh` | use | gh::collect_with_gh |  |  |  |  |

| model::* (glob re-export; see prose note) | use | model::* |  |  |  |  |

| `ADMISSION_SCHEMA` | const | ADMISSION_SCHEMA: &str |  |  |  |  |

| `CATALOG_SCHEMA` | const | CATALOG_SCHEMA: &str |  |  |  |  |

| `OBSERVATION_SCHEMA` | const | OBSERVATION_SCHEMA: &str |  |  |  |  |

| `RECEIPT_SCHEMA` | const | RECEIPT_SCHEMA: &str |  |  |  |  |

| `ActuationClass` | enum | ActuationClass { Select, Construct, Do, HookIntent } |  |  |  |  |

| `AdmissionLevel` | enum | AdmissionLevel { Observed, IdentityResolved, ExecutionObserved, ResultCorroborated, ReceiptVerified, ReplayVerified } |  |  |  |  |

| `ExecutionResult` | enum | ExecutionResult { Pass, Fail, Cancelled, Pending, Unknown } |  |  |  |  |

| `FinalState` | enum | FinalState { PartialAlive, Alive, Blocked, BuildBroken, Unknown, Unsupported } |  |  |  |  |

| `GallCheckpoint` | enum | GallCheckpoint { G0Orient, G1Fence, G2Observe, G3Admit, G4Plan, G5Manufacture, G6Verify, G7Replay, G8ReleaseAdmission, G9SunsetAdmission } |  |  |  |  |

| `RefusalCode` | enum | RefusalCode { MissingExactSourceIdentity, OutsideObservationWindow, MissingChangedPaths, ExecutionNotObserved, WorkflowRunNotBoundToHead, ProbabilityEvidenceMissing, InvalidProbability, ProbabilityMassExceeded, OperatorBoundExceeded } |  |  |  |  |

| `ReplayState` | enum | ReplayState { NotExecuted, ReplayMatch, ReplayMismatch } |  |  |  |  |

| `ferroplan-harvest-admission/v1` | str_key | ADMISSION_SCHEMA = "ferroplan-harvest-admission/v1" |  |  |  |  |

| `ferroplan-harvest-receipt/v1` | str_key | RECEIPT_SCHEMA = "ferroplan-harvest-receipt/v1" |  |  |  |  |

| `ferroplan-method-catalog/v1` | str_key | CATALOG_SCHEMA = "ferroplan-method-catalog/v1" |  |  |  |  |

| `ferroplan-observation-pack/v1` | str_key | OBSERVATION_SCHEMA = "ferroplan-observation-pack/v1" |  |  |  |  |

| `AdmissionReport` | struct | AdmissionReport { pub schema: String, pub admitted: Vec<AdmittedWork>, pub excluded: Vec<ExcludedWork>, pub unresolved_transport_failures: Vec<TransportFailure> } |  |  |  |  |

| `AdmittedWork` | struct | AdmittedWork { pub identity: String, pub level: AdmissionLevel, pub work: ObservedWorkItem, pub evidence: Vec<EvidenceRef> } |  |  |  |  |

| `ArtifactEvidence` | struct | ArtifactEvidence { pub name: String, pub source_sha: String, pub evidence_url: String, pub digest: Option<String>, pub size_bytes: Option<u64> } |  |  |  |  |

| `EvidenceRef` | struct | EvidenceRef { pub kind: String, pub identity: String, pub location: String } |  |  |  |  |

| `ExcludedWork` | struct | ExcludedWork { pub identity: String, pub code: RefusalCode, pub detail: String } |  |  |  |  |

| `ExecutionEvidence` | struct | ExecutionEvidence { pub surface: String, pub command: String, pub source_sha: String, pub result: ExecutionResult, pub exit_code: Option<i32>, pub observed_at_utc: String, pub evidence_url: String } |  |  |  |  |

| `HarvestReceipt` | struct | HarvestReceipt { pub schema: String, pub run_id: String, pub receipt_digest: String, pub source_pack_digest: String, pub catalog_digest: String, pub source_revisions: Vec<SourceRevision>, pub source_work: Vec<String>, pub admitted_work: Vec<String>, pub excluded_work: Vec<ExcludedWork>, pub operators_added: Vec<String>, pub operators_deduplicated: usize, pub probabilistic_operators: usize, pub outputs: Vec<OutputArtifact>, pub validation: ValidationSummary, pub replay: ReplayState, pub generated_outputs_hand_edited: bool, pub transport_failures: Vec<TransportFailure>, pub failures: Vec<String>, pub exclusions: Vec<String>, pub final_state: FinalState } |  |  |  |  |

| `MethodCatalog` | struct | MethodCatalog { pub schema: String, pub run_id: String, pub source_pack_digest: String, pub raw_operator_count: usize, pub operator_count: usize, pub operators: Vec<PlanningOperator> } |  |  |  |  |

| `ObservationPack` | struct | ObservationPack { pub schema: String, pub run_id: String, pub window: ObservationWindow, pub repositories: Vec<String>, pub work_items: Vec<ObservedWorkItem>, pub transport_failures: Vec<TransportFailure> } |  |  |  |  |

| `ObservationWindow` | struct | ObservationWindow { pub start_utc: String, pub end_exclusive_utc: String, pub timezone: String } |  |  |  |  |

| `ObservedOutcome` | struct | ObservedOutcome { pub label: String, pub probability: f64, pub success: bool, pub evidence: Vec<EvidenceRef> } |  |  |  |  |

| `ObservedWorkItem` | struct | ObservedWorkItem { pub repository: String, pub sha: String, pub parent_sha: Option<String>, pub message: String, pub committed_at_utc: String, pub source_url: String, pub changed_paths: Vec<String>, pub executions: Vec<ExecutionEvidence>, pub artifacts: Vec<ArtifactEvidence>, pub probabilistic_outcomes: Vec<ObservedOutcome> } |  |  |  |  |

| `OperatorOutcome` | struct | OperatorOutcome { pub label: String, pub probability: f64, pub success: bool, pub evidence: Vec<EvidenceRef> } |  |  |  |  |

| `OutputArtifact` | struct | OutputArtifact { pub path: String, pub bytes: usize, pub blake3: String } |  |  |  |  |

| `PlanningOperator` | struct | PlanningOperator { pub id: String, pub name: String, pub signature: String, pub checkpoint: GallCheckpoint, pub actuation_class: ActuationClass, pub preconditions: Vec<String>, pub effects: Vec<String>, pub invariants: Vec<String>, pub failures: Vec<String>, pub refusals: Vec<String>, pub receipt_hook: bool, pub replay_hook: bool, pub probabilistic_outcomes: Vec<OperatorOutcome>, pub evidence: Vec<EvidenceRef>, pub source_work: Vec<String> } |  |  |  |  |

| `SourceRevision` | struct | SourceRevision { pub repository: String, pub base_sha: Option<String>, pub head_sha: String } |  |  |  |  |

| `TransportFailure` | struct | TransportFailure { pub repository: String, pub operation: String, pub state: String, pub detail: String } |  |  |  |  |

| `ValidationRecord` | struct | ValidationRecord { pub command: String, pub result: String, pub detail: Option<String> } |  |  |  |  |

| `ValidationSummary` | struct | ValidationSummary { pub parse_ok: bool, pub parse_error: Option<String>, pub solve_attempted: bool, pub solved: Option<bool>, pub initial_value: Option<f64>, pub policy_valid: Option<bool>, pub policy_errors: Vec<String>, pub records: Vec<ValidationRecord> } |  |  |  |  |

| `FIXTURE_F_WALL_SECS` | env_key | std::env::var("FIXTURE_F_WALL_SECS") |  |  |  |  |

| `validate_pair` | function | validate_pair(domain_src: &str, problem_src: &str) -> Result<Summary, String> |  |  |  |  |

| `validate_paths` | function | validate_paths(domain_path: &str, problem_path: &str) -> Result<Summary, String> |  |  |  |  |

| `Summary` | struct | Summary { pub domain: String, pub problem: String, pub actions: usize, pub tasks: usize, pub methods: usize, pub ground_methods: usize } |  |  |  |  |

| `Effect` | enum | Effect { Empty, Literal(Literal), And(Vec<Effect>), When(GoalDesc, Box<Effect>), Oneof(Vec<Effect>), Increase(AtomicFormula, NumericValue), Decrease(AtomicFormula, NumericValue) } |  |  |  |  |

| `GoalDesc` | enum | GoalDesc { Empty, Atom(AtomicFormula), Not(Box<GoalDesc>), And(Vec<GoalDesc>), Or(Vec<GoalDesc>), Imply(Box<GoalDesc>, Box<GoalDesc>), Forall(Vec<TypedParam>, Box<GoalDesc>), Exists(Vec<TypedParam>, Box<GoalDesc>) } |  |  |  |  |

| `Literal` | enum | Literal { Pos(AtomicFormula), Neg(AtomicFormula) } |  |  |  |  |

| `NumericValue` | enum | NumericValue { Number(String), Fluent(AtomicFormula) } |  |  |  |  |

| `Term` | enum | Term { Var(VarName), Const(Name) } |  |  |  |  |

| `ActionDef` | struct | ActionDef { pub name: Name, pub params: Vec<TypedParam>, pub precondition: GoalDesc, pub effect: Effect, pub probability_weights: Option<Vec<String>> } |  |  |  |  |

| `AtomicFormula` | struct | AtomicFormula { pub predicate: Name, pub args: Vec<Term> } |  |  |  |  |

| `ConstraintDef` | struct | ConstraintDef { pub kind: Name, pub raw: String } |  |  |  |  |

| `Domain` | struct | Domain { pub name: Name, pub types: TypeDef, pub constants: Vec<TypedObject>, pub predicates: Vec<PredicateDef>, pub numeric_fluents: Vec<NumericFluentDecl>, pub tasks: Vec<TaskDef>, pub actions: Vec<ActionDef>, pub methods: Vec<MethodDef>, pub constraints: Vec<ConstraintDef> } |  |  |  |  |

| `MethodDef` | struct | MethodDef { pub name: Name, pub params: Vec<TypedParam>, pub task: TaskCall, pub precondition: GoalDesc, pub effect: Effect, pub network: TaskNetwork } |  |  |  |  |

| `NumericFluentDecl` | struct | NumericFluentDecl { pub name: Name, pub params: Vec<TypedParam>, pub value: NumericValue } |  |  |  |  |

| `OrderEdge` | struct | OrderEdge { pub before: Name, pub after: Name } |  |  |  |  |

| `PredicateDef` | struct | PredicateDef { pub name: Name, pub params: Vec<TypedParam> } |  |  |  |  |

| `Problem` | struct | Problem { pub name: Name, pub domain_name: Name, pub objects: Vec<TypedObject>, pub init: Vec<AtomicFormula>, pub goal: GoalDesc, pub htn: TaskNetwork, pub constraints: Vec<ConstraintDef> } |  |  |  |  |

| `Subtask` | struct | Subtask { pub id: Name, pub task: TaskCall } |  |  |  |  |

| `TaskCall` | struct | TaskCall { pub name: Name, pub args: Vec<Term> } |  |  |  |  |

| `TaskDef` | struct | TaskDef { pub name: Name, pub params: Vec<TypedParam> } |  |  |  |  |

| `TaskNetwork` | struct | TaskNetwork { pub params: Vec<TypedParam>, pub subtasks: Vec<Subtask>, pub order: Vec<OrderEdge> } |  |  |  |  |

| `TypeDef` | struct | TypeDef { pub parents: BTreeMap<Name, BTreeSet<Name>>, pub declared: Vec<Name> } |  |  |  |  |

| `TypedObject` | struct | TypedObject { pub name: Name, pub type_name: Name } |  |  |  |  |

| `TypedParam` | struct | TypedParam { pub var: VarName, pub type_name: Name } |  |  |  |  |

| `DEFAULT_MAX_GOAL_DEPTH` | const | DEFAULT_MAX_GOAL_DEPTH: usize |  |  |  |  |

| `GroundError` | enum | GroundError { Validation(validate::ValidationError), TypeCycle(String), UnboundVariable(String), UnsupportedPrecondition(String), LimitExceeded(String), UnsupportedNumericFluent(String), UnsupportedConstraint(String), GoalTooDeep { depth: usize, budget: usize }, Timeout { elapsed_ms: u128, limit_ms: u128 } } |  |  |  |  |

| `GroundGoal` | enum | GroundGoal { Empty, Atom(String), Eq(String, String), Not(Box<GroundGoal>), And(Vec<GroundGoal>), Or(Vec<GroundGoal>) } |  |  |  |  |

| `action_applicable` | function | action_applicable( action: &GroundAction, facts: &BTreeSet<String>, ) -> Result<bool, GroundError> |  |  |  |  |

| `build_type_closure` | function | build_type_closure( domain: &Domain, ) -> Result<BTreeMap<String, BTreeSet<String>>, GroundError> |  |  |  |  |

| `compute_reachability` | function | compute_reachability( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, initial_facts: &BTreeSet<String>, limits: &GroundingLimits, ) -> Result<ReachabilityInfo, GroundError> |  |  |  |  |

| `compute_task_relevance` | function | compute_task_relevance( domain: &Domain, problem: &Problem, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, ) -> Result<TaskRelevanceInfo, GroundError> |  |  |  |  |

| `evaluate_ground_goal` | function | evaluate_ground_goal( goal: &GroundGoal, facts: &BTreeSet<String>, ) -> Result<bool, GroundError> |  |  |  |  |

| `evaluate_ground_goal_with_budget` | function | evaluate_ground_goal_with_budget( goal: &GroundGoal, facts: &BTreeSet<String>, budget: usize, ) -> Result<bool, GroundError> |  |  |  |  |

| `ground` | function | ground( domain: &Domain, problem: &Problem, limits: &GroundingLimits, ) -> Result<GroundedIR, GroundError> |  |  |  |  |

| `ground_actions` | function | ground_actions( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, ) -> Result<Vec<GroundAction>, GroundError> |  |  |  |  |

| `ground_actions_reachable` | function | ground_actions_reachable( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, reachability: &ReachabilityInfo, ) -> Result<Vec<GroundAction>, GroundError> |  |  |  |  |

| `ground_actions_relevant` | function | ground_actions_relevant( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, relevance: &TaskRelevanceInfo, reachability: Option<&ReachabilityInfo>, ) -> Result<Vec<GroundAction>, GroundError> |  |  |  |  |

| `ground_initial_facts` | function | ground_initial_facts(problem: &Problem) -> Result<BTreeSet<String>, GroundError> |  |  |  |  |

| `ground_methods` | function | ground_methods( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, ) -> Result<Vec<GroundMethod>, GroundError> |  |  |  |  |

| `ground_methods_reachable` | function | ground_methods_reachable( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, reachability: &ReachabilityInfo, ) -> Result<Vec<GroundMethod>, GroundError> |  |  |  |  |

| `ground_methods_relevant` | function | ground_methods_relevant( domain: &Domain, objects_by_type: &BTreeMap<String, Vec<String>>, limits: &GroundingLimits, relevance: &TaskRelevanceInfo, reachability: Option<&ReachabilityInfo>, ) -> Result<Vec<GroundMethod>, GroundError> |  |  |  |  |

| `ground_root_network` | function | ground_root_network( problem: &Problem, objects_by_type: &BTreeMap<String, Vec<String>>, ) -> Result<Vec<GroundRootNetwork>, GroundError> |  |  |  |  |

| `index_objects_by_type` | function | index_objects_by_type( domain: &Domain, problem: &Problem, closure: &BTreeMap<String, BTreeSet<String>>, ) -> BTreeMap<String, Vec<String>> |  |  |  |  |

| `../fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../fixtures/a/domain.hddl" |  |  |  |  |

| `../fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../fixtures/a/problem.hddl" |  |  |  |  |

| `../fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/c/problem.hddl` | str_key | FIXTURE_C_PROBLEM = "../fixtures/c/problem.hddl" |  |  |  |  |

| `../fixtures/d/domain.hddl` | str_key | FIXTURE_D_DOMAIN = "../fixtures/d/domain.hddl" |  |  |  |  |

| `../fixtures/d/problem.hddl` | str_key | FIXTURE_D_PROBLEM = "../fixtures/d/problem.hddl" |  |  |  |  |

| `../fixtures/e/domain.hddl` | str_key | FIXTURE_E_DOMAIN = "../fixtures/e/domain.hddl" |  |  |  |  |

| `../fixtures/e/problem.hddl` | str_key | FIXTURE_E_PROBLEM = "../fixtures/e/problem.hddl" |  |  |  |  |

| `GroundAction` | struct | GroundAction { pub name: String, pub precondition: GroundGoal, pub outcomes: Vec<GroundEffectBranch> } |  |  |  |  |

| `GroundConditional` | struct | GroundConditional { pub pos_cond: BTreeSet<String>, pub neg_cond: BTreeSet<String>, pub add: BTreeSet<String>, pub del: BTreeSet<String> } |  |  |  |  |

| `GroundEffectBranch` | struct | GroundEffectBranch { pub add: BTreeSet<String>, pub del: BTreeSet<String>, pub conditional: Vec<GroundConditional>, pub probability_weight: Option<String> } |  |  |  |  |

| `GroundMethod` | struct | GroundMethod { pub name: String, pub task_name: String, pub precondition: GroundGoal, pub effect: GroundEffectBranch, pub subtasks: Vec<GroundSubtask>, pub order: Vec<(String, String)> } |  |  |  |  |

| `GroundRootNetwork` | struct | GroundRootNetwork { pub subtasks: Vec<GroundSubtask>, pub order: Vec<(String, String)> } |  |  |  |  |

| `GroundSubtask` | struct | GroundSubtask { pub id: String, pub task_name: String } |  |  |  |  |

| `GroundedIR` | struct | GroundedIR { pub actions: Vec<GroundAction>, pub methods: Vec<GroundMethod>, pub root_networks: Vec<GroundRootNetwork>, pub initial_facts: BTreeSet<String>, pub goal: GoalDesc } |  |  |  |  |

| `GroundingLimits` | struct | GroundingLimits { pub max_ground_actions: usize, pub max_ground_methods: usize, pub prune_unreachable: bool, pub prune_irrelevant: bool, pub max_wall: Option<Duration> } |  |  |  |  |

| `ReachabilityInfo` | struct | ReachabilityInfo { pub facts: BTreeSet<String>, pub reachable_actions: BTreeSet<String> } |  |  |  |  |

| `TaskRelevanceInfo` | struct | TaskRelevanceInfo { pub relevant_tasks: BTreeSet<String> } |  |  |  |  |

| `DEFAULT_MAX_PARSE_DEPTH` | const | DEFAULT_MAX_PARSE_DEPTH: usize |  |  |  |  |

| `ParseError` | enum | ParseError { Syntax(String), UnsupportedConstruct(String), MalformedOneof(String), NestedProbabilisticBlock(String), NestingTooDeep { line: usize, column: usize, budget: usize, } } |  |  |  |  |

| `parse_domain` | function | parse_domain(src: &str) -> Result<Domain, ParseError> |  |  |  |  |

| `parse_domain_with_budget` | function | parse_domain_with_budget(src: &str, max_depth: usize) -> Result<Domain, ParseError> |  |  |  |  |

| `parse_problem` | function | parse_problem(src: &str) -> Result<Problem, ParseError> |  |  |  |  |

| `parse_problem_with_budget` | function | parse_problem_with_budget(src: &str, max_depth: usize) -> Result<Problem, ParseError> |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (and (p) (oneof (q) (r)))))` | str_key | DOMAIN = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (and (p) (oneof (q) (r)))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (and (when (r) (p))) (q))))` | str_key | UNDER_AND = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (and (when (r) (p))) (q))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (when (r) (p)) (q))))` | str_key | DIRECT = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof (when (r) (p)) (q))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (when (r) (oneof (p) (q)))))` | str_key | DOMAIN = "(define (domain bad)
          (:predicates (p) (q) (r))
          (:action a
            :parameters ()
            :precondition ()
            :effect (when (r) (oneof (p) (q)))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition (oneof (p) (q))
            :effect (and (p))))` | str_key | DOMAIN = "(define (domain bad)
          (:predicates (p) (q))
          (:action a
            :parameters ()
            :precondition (oneof (p) (q))
            :effect (and (p))))" |  |  |  |  |

| `(define (domain bad)
          (:predicates (p))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof)))` | str_key | DOMAIN = "(define (domain bad)
          (:predicates (p))
          (:action a
            :parameters ()
            :precondition ()
            :effect (oneof)))" |  |  |  |  |

| `(define (domain childsnack-style)
          (:types child)
          (:predicates (served ?c - child) (dirty ?c - child))
          (:action putdown
            :parameters (?c - child)
            :precondition ()
            :effect (oneof
              (and (served ?c))
              (and (served ?c) (not (dirty ?c))))))` | str_key | DOMAIN = "(define (domain childsnack-style)
          (:types child)
          (:predicates (served ?c - child) (dirty ?c - child))
          (:action putdown
            :parameters (?c - child)
            :precondition ()
            :effect (oneof
              (and (served ?c))
              (and (served ?c) (not (dirty ?c))))))" |  |  |  |  |

| `(define (domain drop-d)
          (:types loc truck)
          (:predicates (at ?t - truck ?l - loc))
          (:action drop
            :parameters (?t - truck ?from - loc ?to - loc)
            :precondition (at ?t ?from)
            :effect (oneof
              (and (not (at ?t ?from)) (at ?t ?to))
              ())))` | str_key | DOMAIN = "(define (domain drop-d)
          (:types loc truck)
          (:predicates (at ?t - truck ?l - loc))
          (:action drop
            :parameters (?t - truck ?from - loc ?to - loc)
            :precondition (at ?t ?from)
            :effect (oneof
              (and (not (at ?t ?from)) (at ?t ?to))
              ())))" |  |  |  |  |

| `(define (domain one-way)
          (:predicates (p) (q))
          (:action once
            :parameters ()
            :precondition ()
            :effect (oneof (and (p) (q)))))` | str_key | DOMAIN = "(define (domain one-way)
          (:predicates (p) (q))
          (:action once
            :parameters ()
            :precondition ()
            :effect (oneof (and (p) (q)))))" |  |  |  |  |

| `(define (domain ordering-operator)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task deliver :parameters (?l - loc))
          (:method m-deliver
            :parameters (?l - loc)
            :task (deliver ?l)
            :subtasks (and
              (t1 (deliver ?l))
              (t2 (deliver ?l))
              (t3 (deliver ?l)))
            :ordering (and
              (< t1 t2)
              (t2 < t3))))` | str_key | DOMAIN = "(define (domain ordering-operator)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task deliver :parameters (?l - loc))
          (:method m-deliver
            :parameters (?l - loc)
            :task (deliver ?l)
            :subtasks (and
              (t1 (deliver ?l))
              (t2 (deliver ?l))
              (t3 (deliver ?l)))
            :ordering (and
              (< t1 t2)
              (t2 < t3))))" |  |  |  |  |

| `(define (domain temporal-d)
  (:durative-action fly
    :parameters (?a ?b)
    :duration (= ?duration 10)
    :condition (at start (at ?a))
    :effect (at end (at ?b))))` | str_key | DOMAIN = "(define (domain temporal-d)
  (:durative-action fly
    :parameters (?a ?b)
    :duration (= ?duration 10)
    :condition (at start (at ?a))
    :effect (at end (at ?b))))" |  |  |  |  |

| `(define (domain three-way)
          (:predicates (p) (q) (r))
          (:action tri
            :parameters ()
            :precondition ()
            :effect (oneof (p) (q) (r))))` | str_key | DOMAIN = "(define (domain three-way)
          (:predicates (p) (q) (r))
          (:action tri
            :parameters ()
            :precondition ()
            :effect (oneof (p) (q) (r))))" |  |  |  |  |

| `../fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../fixtures/a/domain.hddl" |  |  |  |  |

| `../fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../fixtures/a/problem.hddl" |  |  |  |  |

| `../fixtures/b/domain.hddl` | str_key | FIXTURE_B_DOMAIN = "../fixtures/b/domain.hddl" |  |  |  |  |

| `../fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/e/domain.hddl` | str_key | FIXTURE_E_DOMAIN = "../fixtures/e/domain.hddl" |  |  |  |  |

| `../fixtures/f/domain.hddl` | str_key | FIXTURE_F_DOMAIN = "../fixtures/f/domain.hddl" |  |  |  |  |

| `../fixtures/f/problem.hddl` | str_key | FIXTURE_F_PROBLEM = "../fixtures/f/problem.hddl" |  |  |  |  |

| `has_probabilistic` | function | has_probabilistic(src: &str) -> bool |  |  |  |  |

| `preprocess` | function | preprocess(src: &str) -> Result<(String, BTreeMap<String, Vec<String>>), ParseError> |  |  |  |  |

| `TranslateError` | enum | TranslateError { UnsupportedNegativeGoal, UnsupportedGoalConnective(String), MalformedTermEquality { found: usize, }, UnboundVariable(String), TaskNetworkDepthExceeded { addr: String, limit: usize, }, Timeout { elapsed_ms: u128, limit_ms: u128, }, MemoryLimitExceeded { states: usize, limit: usize, }, Ground(GroundError) } |  |  |  |  |

| `translate` | function | translate( ir: &GroundedIR, limits: &TranslateLimits, ) -> Result<PlanningProblem, TranslateError> |  |  |  |  |

| `(define (domain coin-empty)
  (:predicates (heads))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof () (heads)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))` | str_key | DOMAIN = "(define (domain coin-empty)
  (:predicates (heads))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof () (heads)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))" |  |  |  |  |

| `(define (domain eq-gate)
  (:requirements :typing :equality :method-preconditions)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p) (q))
  (:task go :parameters (?x - loc))
  (:action mark-p :parameters () :precondition () :effect (p))
  (:action mark-q :parameters () :precondition () :effect (q))
  (:method m-eq
    :parameters (?x - loc)
    :task (go ?x)
    :precondition (= ?x a)
    :ordered-subtasks (and (t1 (mark-p))))
  (:method m-neq
    :parameters (?x - loc)
    :task (go ?x)
    :precondition (not (= ?x a))
    :ordered-subtasks (and (t1 (mark-q)))))` | str_key | DOMAIN = "(define (domain eq-gate)
  (:requirements :typing :equality :method-preconditions)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p) (q))
  (:task go :parameters (?x - loc))
  (:action mark-p :parameters () :precondition () :effect (p))
  (:action mark-q :parameters () :precondition () :effect (q))
  (:method m-eq
    :parameters (?x - loc)
    :task (go ?x)
    :precondition (= ?x a)
    :ordered-subtasks (and (t1 (mark-p))))
  (:method m-neq
    :parameters (?x - loc)
    :task (go ?x)
    :precondition (not (= ?x a))
    :ordered-subtasks (and (t1 (mark-q)))))" |  |  |  |  |

| `(define (domain eq-goal)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p))
  (:task go :parameters ())
  (:method m-go
    :task (go)
    :ordered-subtasks ()))` | str_key | DOMAIN = "(define (domain eq-goal)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p))
  (:task go :parameters ())
  (:method m-go
    :task (go)
    :ordered-subtasks ()))" |  |  |  |  |

| `(define (domain eq-when)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p) (q))
  (:task go :parameters (?x - loc))
  (:action probe :parameters (?x - loc)
    :precondition ()
    :effect (and
      (when (= ?x a) (and (not (q)) (p)))
      (when (not (= ?x a)) (and (not (p)) (q)))))
  (:method m-go
    :parameters (?x - loc)
    :task (go ?x)
    :ordered-subtasks (and (t1 (probe ?x)))))` | str_key | DOMAIN = "(define (domain eq-when)
  (:types loc)
  (:constants a b - loc)
  (:predicates (p) (q))
  (:task go :parameters (?x - loc))
  (:action probe :parameters (?x - loc)
    :precondition ()
    :effect (and
      (when (= ?x a) (and (not (q)) (p)))
      (when (not (= ?x a)) (and (not (p)) (q)))))
  (:method m-go
    :parameters (?x - loc)
    :task (go ?x)
    :ordered-subtasks (and (t1 (probe ?x)))))" |  |  |  |  |

| `(define (domain gated-tri)
  (:predicates (ready) (p) (q) (r))
  (:task go :parameters ())
  (:action tri
    :parameters ()
    :precondition (ready)
    :effect (oneof (p) (q) (r)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (tri)))))` | str_key | DOMAIN = "(define (domain gated-tri)
  (:predicates (ready) (p) (q) (r))
  (:task go :parameters ())
  (:action tri
    :parameters ()
    :precondition (ready)
    :effect (oneof (p) (q) (r)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (tri)))))" |  |  |  |  |

| `(define (domain method-precond-g)
  (:predicates (ready) (done-a) (done-b))
  (:task run :parameters ())
  (:method use-a
    :parameters ()
    :task (run)
    :precondition (ready)
    :ordered-subtasks (mark-a))
  (:method use-b
    :parameters ()
    :task (run)
    :precondition (not (ready))
    :ordered-subtasks (mark-b))
  (:action mark-a
    :effect (done-a))
  (:action mark-b
    :effect (done-b)))` | str_key | DOMAIN = "(define (domain method-precond-g)
  (:predicates (ready) (done-a) (done-b))
  (:task run :parameters ())
  (:method use-a
    :parameters ()
    :task (run)
    :precondition (ready)
    :ordered-subtasks (mark-a))
  (:method use-b
    :parameters ()
    :task (run)
    :precondition (not (ready))
    :ordered-subtasks (mark-b))
  (:action mark-a
    :effect (done-a))
  (:action mark-b
    :effect (done-b)))" |  |  |  |  |

| `(define (domain noop-then-move)
  (:predicates (at-a) (at-b))
  (:task go :parameters ())
  (:action pause
    :parameters ()
    :precondition ()
    :effect (and))
  (:action move
    :parameters ()
    :precondition (at-a)
    :effect (and (not (at-a)) (at-b)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (pause)) (t2 (move)))))` | str_key | DOMAIN = "(define (domain noop-then-move)
  (:predicates (at-a) (at-b))
  (:task go :parameters ())
  (:action pause
    :parameters ()
    :precondition ()
    :effect (and))
  (:action move
    :parameters ()
    :precondition (at-a)
    :effect (and (not (at-a)) (at-b)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (pause)) (t2 (move)))))" |  |  |  |  |

| `(define (domain shortcut-g)
  (:types loc)
  (:predicates (at ?l - loc) (cheated))
  (:task run :parameters ())
  (:action drive
    :parameters (?a - loc ?b - loc)
    :precondition (at ?a)
    :effect (and (not (at ?a)) (at ?b)))
  (:action special-action
    :parameters (?a - loc)
    :precondition (at ?a)
    :effect (and (cheated)))
  (:method m-run
    :parameters ()
    :task (run)
    :ordered-subtasks (and (t1 (drive l1 l2)))))` | str_key | DOMAIN = "(define (domain shortcut-g)
  (:types loc)
  (:predicates (at ?l - loc) (cheated))
  (:task run :parameters ())
  (:action drive
    :parameters (?a - loc ?b - loc)
    :precondition (at ?a)
    :effect (and (not (at ?a)) (at ?b)))
  (:action special-action
    :parameters (?a - loc)
    :precondition (at ?a)
    :effect (and (cheated)))
  (:method m-run
    :parameters ()
    :task (run)
    :ordered-subtasks (and (t1 (drive l1 l2)))))" |  |  |  |  |

| `(define (domain three-way)
  (:predicates (p) (q) (r))
  (:task go :parameters ())
  (:action tri
    :parameters ()
    :precondition ()
    :effect (oneof (p) (q) (r)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (tri)))))` | str_key | DOMAIN = "(define (domain three-way)
  (:predicates (p) (q) (r))
  (:task go :parameters ())
  (:action tri
    :parameters ()
    :precondition ()
    :effect (oneof (p) (q) (r)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (tri)))))" |  |  |  |  |

| `(define (problem coin-empty-p1)
  (:domain coin-empty)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem coin-empty-p1)
  (:domain coin-empty)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem eq-when-p)
  (:domain eq-when)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go a)) (g2 (go b))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem eq-when-p)
  (:domain eq-when)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go a)) (g2 (go b))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem gated-tri-p1)
  (:domain gated-tri)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem gated-tri-p1)
  (:domain gated-tri)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem method-precond-g-p1)
  (:domain method-precond-g)
  (:objects)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (ready))
  (:goal (and (done-a))))` | str_key | PROBLEM = "(define (problem method-precond-g-p1)
  (:domain method-precond-g)
  (:objects)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (ready))
  (:goal (and (done-a))))" |  |  |  |  |

| `(define (problem noop-then-move-p1)
  (:domain noop-then-move)
  (:objects)
  (:init (at-a))
  (:goal (at-b))
  (:htn :ordered-subtasks (and (g1 (go)))))` | str_key | PROBLEM = "(define (problem noop-then-move-p1)
  (:domain noop-then-move)
  (:objects)
  (:init (at-a))
  (:goal (at-b))
  (:htn :ordered-subtasks (and (g1 (go)))))" |  |  |  |  |

| `(define (problem shortcut-g-p1)
  (:domain shortcut-g)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (at l1))
  (:goal (and (cheated))))` | str_key | PROBLEM = "(define (problem shortcut-g-p1)
  (:domain shortcut-g)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (at l1))
  (:goal (and (cheated))))" |  |  |  |  |

| `(define (problem three-way-p1)
  (:domain three-way)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem three-way-p1)
  (:domain three-way)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem transport-a-p1-neg)
  (:domain transport-a)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (and (at l2) (not (has-package)))))` | str_key | PROBLEM = "(define (problem transport-a-p1-neg)
  (:domain transport-a)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (and (at l2) (not (has-package)))))" |  |  |  |  |

| `(define (problem transport-a-p1-or)
  (:domain transport-a)
  (:objects l1 l2 l3 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (or (at l2) (at l3))))` | str_key | PROBLEM = "(define (problem transport-a-p1-or)
  (:domain transport-a)
  (:objects l1 l2 l3 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (or (at l2) (at l3))))" |  |  |  |  |

| `../fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../fixtures/a/domain.hddl" |  |  |  |  |

| `../fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../fixtures/a/problem.hddl" |  |  |  |  |

| `../fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/c/problem.hddl` | str_key | FIXTURE_C_PROBLEM = "../fixtures/c/problem.hddl" |  |  |  |  |

| `../fixtures/e/domain.hddl` | str_key | FIXTURE_E_DOMAIN = "../fixtures/e/domain.hddl" |  |  |  |  |

| `../fixtures/e/problem.hddl` | str_key | FIXTURE_E_PROBLEM = "../fixtures/e/problem.hddl" |  |  |  |  |

| `../fixtures/f/domain.hddl` | str_key | FIXTURE_F_DOMAIN = "../fixtures/f/domain.hddl" |  |  |  |  |

| `../fixtures/f/problem.hddl` | str_key | FIXTURE_F_PROBLEM = "../fixtures/f/problem.hddl" |  |  |  |  |

| `Goal` | struct | Goal { pub facts: BTreeSet<String> } |  |  |  |  |

| `Method` | struct | Method { pub id: String, pub task: String, pub subtasks: Vec<String> } |  |  |  |  |

| `PlanningProblem` | struct | PlanningProblem { pub states: Vec<State>, pub initial_states: Vec<String>, pub goal: Goal, pub transitions: Vec<Transition>, pub tasks: Vec<Task>, pub root_tasks: Vec<String>, pub methods: Vec<Method> } |  |  |  |  |

| `State` | struct | State { pub id: String, pub facts: BTreeSet<String> } |  |  |  |  |

| `Task` | struct | Task { pub id: String, pub primitive_action: Option<String> } |  |  |  |  |

| `Transition` | struct | Transition { pub action: String, pub from: String, pub to: String, pub probability_ppm: u32 } |  |  |  |  |

| `TranslateLimits` | struct | TranslateLimits { pub max_task_network_depth: usize, pub max_wall: Option<std::time::Duration>, pub max_states: Option<usize> } |  |  |  |  |

| `DuplicateKind` | enum | DuplicateKind { Task, Predicate, Action, Method, Object } |  |  |  |  |

| `ValidationError` | enum | ValidationError { UnknownTaskOrAction(String), ArityMismatch { task: String, expected: usize, found: usize, }, UndefinedPredicate(String), UndefinedType(String), DuplicateDefinition { kind: DuplicateKind, name: String }, CyclicTypeHierarchy { type_name: String, }, PredicateArityMismatch { predicate: String, expected: usize, found: usize, }, MethodHeadNotCompoundTask { method: String, name: String, }, UnknownMethodVariable { method: String, var: String, }, UndefinedOrderRef { in_method: Option<String>, id: String, }, CyclicOrdering { in_method: Option<String>, cycle: Vec<String>, }, MissingTaskNetwork, UnknownConstant { name: String, }, ArgumentTypeMismatch { callee: String, position: usize, expected: String, found: String, }, NonGroundInitAtom { predicate: String, }, NonGroundRootSubtaskArg { id: String, } } |  |  |  |  |

| `ValidationWarning` | enum | ValidationWarning { UnrefinableCompoundTask { task: String, }, DuplicateTypeDeclaration { name: String, } } |  |  |  |  |

| `validate_domain` | function | validate_domain(domain: &Domain) -> Result<(), ValidationError> |  |  |  |  |

| `validate_domain_with_warnings` | function | validate_domain_with_warnings( domain: &Domain, ) -> Result<Vec<ValidationWarning>, ValidationError> |  |  |  |  |

| `validate_problem` | function | validate_problem(domain: &Domain, problem: &Problem) -> Result<(), ValidationError> |  |  |  |  |

| `validate_problem_with_warnings` | function | validate_problem_with_warnings( domain: &Domain, problem: &Problem, ) -> Result<Vec<ValidationWarning>, ValidationError> |  |  |  |  |

| `../fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../fixtures/a/domain.hddl" |  |  |  |  |

| `../fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../fixtures/a/problem.hddl" |  |  |  |  |

| `../fixtures/b/domain.hddl` | str_key | FIXTURE_B_DOMAIN = "../fixtures/b/domain.hddl" |  |  |  |  |

| `../fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/d/domain.hddl` | str_key | FIXTURE_D_DOMAIN = "../fixtures/d/domain.hddl" |  |  |  |  |

| `../fixtures/f/domain.hddl` | str_key | FIXTURE_F_DOMAIN = "../fixtures/f/domain.hddl" |  |  |  |  |

| `../fixtures/g/domain.hddl` | str_key | FIXTURE_G_DOMAIN = "../fixtures/g/domain.hddl" |  |  |  |  |

| `../fixtures/g/problem.hddl` | str_key | FIXTURE_G_PROBLEM = "../fixtures/g/problem.hddl" |  |  |  |  |

| `CARGO_MANIFEST_DIR` | str_key | DOMAIN_PATH = "CARGO_MANIFEST_DIR" |  |  |  |  |

| `preserve` | str_key | ORDER = "preserve" |  |  |  |  |

| `../fixtures/c/domain.hddl` | str_key | C_DOMAIN = "../fixtures/c/domain.hddl" |  |  |  |  |

| `../fixtures/c/problem.hddl` | str_key | C_PROBLEM = "../fixtures/c/problem.hddl" |  |  |  |  |

| `canonical_digest` | str_key | RESOURCE_TOOLS = "canonical_digest" |  |  |  |  |

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |

| `urn:chatman:claude-code-admission:v1` | str_key | RECEIPT_DOMAIN = "urn:chatman:claude-code-admission:v1" |  |  |  |  |

| `dx_manifest` | str_key | RESOURCE_TOOLS = "dx_manifest" |  |  |  |  |

| `plugins/chatman-ecosystem/ontology/ferroplan-experience.ttl` | str_key | ONTOLOGY_SOURCE = "plugins/chatman-ecosystem/ontology/ferroplan-experience.ttl" |  |  |  |  |

| `solve` | str_key | CAPABILITIES = "solve" |  |  |  |  |

| `Applies fact and/or fluent observations to a session's belief (via Session::observe for facts, Session::set_fluent per changed fluent), bumping epoch if anything surprised, and reports whether the currently-stored plan is still valid from the current cursor. Fluent comparison against the prior value is an exact to_bits() equality check, not epsilon-tolerant.` | str_key | OBSERVE_ONTOLOGY = "Applies fact and/or fluent observations to a session's belief (via Session::observe for facts, Session::set_fluent per changed fluent), bumping epoch if anything surprised, and reports whether the currently-stored plan is still valid from the current cursor. Fluent comparison against the prior value is an exact to_bits() equality check, not epsilon-tolerant." |  |  |  |  |

| `Apply a bounded heterogeneous session transaction on a staged fork and commit exactly once, or refuse without partial mutation.` | str_key | QOL_BATCH_ONTOLOGY = "Apply a bounded heterogeneous session transaction on a staged fork and commit exactly once, or refuse without partial mutation." |  |  |  |  |

| `Atomically manufacture a ready persistent planning mind from domain, problem, goal, authority scope, and bounded search settings.` | str_key | WIZARD_BOOTSTRAP_ONTOLOGY = "Atomically manufacture a ready persistent planning mind from domain, problem, goal, authority scope, and bounded search settings." |  |  |  |  |

| `Classify a tool or protocol failure into a typed cause with bounded confidence, corrective actions, and refusal-preserving recovery guidance.` | str_key | DOCTOR_EXPLAIN_ONTOLOGY = "Classify a tool or protocol failure into a typed cause with bounded confidence, corrective actions, and refusal-preserving recovery guidance." |  |  |  |  |

| `Compile a high-level operator intent into an ordered, inspectable Ferroplan tool recipe with preflight, rollback, and receipt checkpoints.` | str_key | WIZARD_RECIPE_ONTOLOGY = "Compile a high-level operator intent into an ordered, inspectable Ferroplan tool recipe with preflight, rollback, and receipt checkpoints." |  |  |  |  |

| `Diagnose global or per-session health, assign typed findings, calculate standing, and emit executable remediation hints without mutating state.` | str_key | DOCTOR_SCAN_ONTOLOGY = "Diagnose global or per-session health, assign typed findings, calculate standing, and emit executable remediation hints without mutating state." |  |  |  |  |

| `Enumerate a bounded combinatorial capability lattice, minimal reachability depths, dependency edges, blocked frontiers, and theoretical composition capacity.` | str_key | VISION_LATTICE_ONTOLOGY = "Enumerate a bounded combinatorial capability lattice, minimal reachability depths, dependency edges, blocked frontiers, and theoretical composition capacity." |  |  |  |  |

| `Gall Checkpoint 9 (Recursive Multifractal Allocation): runs cmca_allocate's exact admission law at a root frontier, then descends into zero or more selected admitted nodes, each with a fresh local N=8/F=10 frontier, chaining every depth's payload_digest into the next depth's envelope server-side (never caller-supplied/trusted). Refuses the whole call -- no partial chain -- on: selected_parent_node naming an id that was not an admitted candidate at the immediately preceding depth (ParentNodeUnknown-shaped refusal); selected_parent_node repeating an id already used to enter an earlier depth on the same chain (cyclic-ancestry refusal); or any depth's own admission failing cmca_allocate's underlying candidate/forest/factor law. Same-input calls are byte-identical (deterministic replay) since every depth is a pure function of its own input plus the previous depth's real digest.` | str_key | CMCA_RECURSIVE_ONTOLOGY = "Gall Checkpoint 9 (Recursive Multifractal Allocation): runs cmca_allocate's exact admission law at a root frontier, then descends into zero or more selected admitted nodes, each with a fresh local N=8/F=10 frontier, chaining every depth's payload_digest into the next depth's envelope server-side (never caller-supplied/trusted). Refuses the whole call -- no partial chain -- on: selected_parent_node naming an id that was not an admitted candidate at the immediately preceding depth (ParentNodeUnknown-shaped refusal); selected_parent_node repeating an id already used to enter an earlier depth on the same chain (cyclic-ancestry refusal); or any depth's own admission failing cmca_allocate's underlying candidate/forest/factor law. Same-input calls are byte-identical (deterministic replay) since every depth is a pure function of its own input plus the previous depth's real digest." |  |  |  |  |

| `Inferred: a validation tool beyond plain syntax parsing — most plausibly checking a domain+problem pair grounds successfully (i.e. exercising ferroplan::api's grounding path without necessarily searching for a full plan), used as a pre-flight check before an expensive solve/decompose call. No struct named Validate*/ValidationReport was present in the extracted api.rs public-surface listing, so this tool's exact backing type and field schema is UNVERIFIED against the source excerpts available to this ontology; modeled at the tool-name/purpose level only.` | str_key | VALIDATE_ONTOLOGY = "Inferred: a validation tool beyond plain syntax parsing — most plausibly checking a domain+problem pair grounds successfully (i.e. exercising ferroplan::api's grounding path without necessarily searching for a full plan), used as a pre-flight check before an expensive solve/decompose call. No struct named Validate*/ValidationReport was present in the extracted api.rs public-surface listing, so this tool's exact backing type and field schema is UNVERIFIED against the source excerpts available to this ontology; modeled at the tool-name/purpose level only." |  |  |  |  |

| `Inferred: binds a CMCA/BCINR allocation payload (such as cmca_allocate's output) into a chained receipt, analogous in spirit to the session tools' chain_receipt over SESSION_RECEIPT_DOMAIN, but scoped to allocation payloads rather than session events. Exact input/output field schema not captured at fine grain in the extraction available.` | str_key | BIND_ALLOC_ONTOLOGY = "Inferred: binds a CMCA/BCINR allocation payload (such as cmca_allocate's output) into a chained receipt, analogous in spirit to the session tools' chain_receipt over SESSION_RECEIPT_DOMAIN, but scoped to allocation payloads rather than session events. Exact input/output field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Inferred: binds a ferroplan Plan/Solution payload into a chained receipt, so a downstream actuation broker (per chatman-ecosystem.ttl's ce:BRCE) can require ce:requiresReceipt before treating a candidate plan as admissible. Exact input/output field schema not captured at fine grain in the extraction available.` | str_key | BIND_PLAN_ONTOLOGY = "Inferred: binds a ferroplan Plan/Solution payload into a chained receipt, so a downstream actuation broker (per chatman-ecosystem.ttl's ce:BRCE) can require ce:requiresReceipt before treating a candidate plan as admissible. Exact input/output field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Inferred: computes a canonical (deterministic-serialization) blake3 digest over an arbitrary JSON payload, for use as a stable content-identity input to receipt chaining elsewhere. Exact input/output field schema not captured at fine grain in the extraction available; documented at the tool-name/purpose level only.` | str_key | DIGEST_ONTOLOGY = "Inferred: computes a canonical (deterministic-serialization) blake3 digest over an arbitrary JSON payload, for use as a stable content-identity input to receipt chaining elsewhere. Exact input/output field schema not captured at fine grain in the extraction available; documented at the tool-name/purpose level only." |  |  |  |  |

| `Inferred: verifies a previously-bound receipt (allocation or plan) against its claimed chain head / digest, returning whether the chain is intact. Exact input/output field schema not captured at fine grain in the extraction available.` | str_key | VERIFY_ONTOLOGY = "Inferred: verifies a previously-bound receipt (allocation or plan) against its claimed chain head / digest, returning whether the chain is intact. Exact input/output field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Manufacture a deterministic transport-neutral BLAKE3 integrity envelope with correlation, causation, idempotency, predecessor, and expiry fields; it performs no network operation.` | str_key | TELCO_ENVELOPE_ONTOLOGY = "Manufacture a deterministic transport-neutral BLAKE3 integrity envelope with correlation, causation, idempotency, predecessor, and expiry fields; it performs no network operation." |  |  |  |  |

| `Moves the session's cursor forward by completed_steps against the stored last_plan's length; errors if that would run past the plan's end. Advancing the cursor does not itself apply any world effects — effects still enter belief only via session_observe (or set_fact/elapse on the underlying Session), per the source comment at line 5 and the tool description at line 288.` | str_key | ADVANCE_ONTOLOGY = "Moves the session's cursor forward by completed_steps against the stored last_plan's length; errors if that would run past the plan's end. Advancing the cursor does not itself apply any world effects — effects still enter belief only via session_observe (or set_fact/elapse on the underlying Session), per the source comment at line 5 and the tool description at line 288." |  |  |  |  |

| `Opens (grounds) a new Session under session_id and stores it server-side, chaining an 'opened' receipt. Rejects if the session_id already exists unless replace is true. Rejects non-canonical session_ids (must be alphanumeric/-/_/./:).` | str_key | OPEN_ONTOLOGY = "Opens (grounds) a new Session under session_id and stores it server-side, chaining an 'opened' receipt. Rejects if the session_id already exists unless replace is true. Rejects non-canonical session_ids (must be alphanumeric/-/_/./:)." |  |  |  |  |

| `Read session state, selected facts and fluents, plan standing, diagnostics, memory, lineage, and recent history in one round trip.` | str_key | QOL_SNAPSHOT_ONTOLOGY = "Read session state, selected facts and fluents, plan standing, diagnostics, memory, lineage, and recent history in one round trip." |  |  |  |  |

| `Read-only status snapshot of a session — no receipt is chained (nothing mutated).` | str_key | STATUS_ONTOLOGY = "Read-only status snapshot of a session — no receipt is chained (nothing mutated)." |  |  |  |  |

| `Removes the session from server state (frees its grounded world if no other session shares it). No receipt chaining; no error if the session_id doesn't exist.` | str_key | CLOSE_ONTOLOGY = "Removes the session from server state (frees its grounded world if no other session shares it). No receipt chaining; no error if the session_id doesn't exist." |  |  |  |  |

| `Retargets a session's goal via Session::set_goal, resets cursor to 0, and bumps epoch. remaining_plan_valid in the response checks plan_still_valid(last_plan, 0) — against the just-reset cursor, not whatever cursor held before the call.` | str_key | SET_GOAL_ONTOLOGY = "Retargets a session's goal via Session::set_goal, resets cursor to 0, and bumps epoch. remaining_plan_valid in the response checks plan_still_valid(last_plan, 0) — against the just-reset cursor, not whatever cursor held before the call." |  |  |  |  |

| `Return the complete self-describing Ferroplan capability manifest, including authority categories, contracts, effects, reversibility, receipt behavior, and composition examples.` | str_key | DX_MANIFEST_ONTOLOGY = "Return the complete self-describing Ferroplan capability manifest, including authority categories, contracts, effects, reversibility, receipt behavior, and composition examples." |  |  |  |  |

| `Runs the bcinr_cmca (Chatman Multifractal Cascade Allocator) fixed-size allocation over exactly N=8 candidates, validating each candidate's id (non-empty, unique), parent (forest structure: exactly one root, no cycles), fixed-point factors (F entries, finite, in [0, u32::MAX/65536]), and cost. Note: this tool is CMCA/BCINR allocation surface bundled onto the session server, not itself a Session operation — it is the only tool here with no session_id and no receipt chaining on the ManagedSession scheme (it returns a self-contained payload_digest instead).` | str_key | CMCA_ONTOLOGY = "Runs the bcinr_cmca (Chatman Multifractal Cascade Allocator) fixed-size allocation over exactly N=8 candidates, validating each candidate's id (non-empty, unique), parent (forest structure: exactly one root, no cycles), fixed-point factors (F entries, finite, in [0, u32::MAX/65536]), and cost. Note: this tool is CMCA/BCINR allocation surface bundled onto the session server, not itself a Session operation — it is the only tool here with no session_id and no receipt chaining on the ManagedSession scheme (it returns a self-contained payload_digest instead)." |  |  |  |  |

| `Search the bounded capability graph for a minimal deterministic tool sequence from admitted starting atoms to requested outcome atoms.` | str_key | DX_COMPOSE_ONTOLOGY = "Search the bounded capability graph for a minimal deterministic tool sequence from admitted starting atoms to requested outcome atoms." |  |  |  |  |

| `The think step: if the stored last_plan is still valid from the current cursor, short-circuits to a 'follow' decision (searched:false) without invoking any planner, regardless of prefer_follow. Otherwise searches: prefer_follow only changes behavior when a (now-invalid) prior plan exists — then it chooses between Session::replan_following (bias toward the prior plan's structure) and Session::replan_budgeted (from-scratch bounded search); with no prior plan at all it always uses replan_budgeted regardless of prefer_follow. Cursor is always reset to 0 after any search path (both branches). decision is 'replan' if solved else 'bounded-refusal'.` | str_key | THINK_ONTOLOGY = "The think step: if the stored last_plan is still valid from the current cursor, short-circuits to a 'follow' decision (searched:false) without invoking any planner, regardless of prefer_follow. Otherwise searches: prefer_follow only changes behavior when a (now-invalid) prior plan exists — then it chooses between Session::replan_following (bias toward the prior plan's structure) and Session::replan_budgeted (from-scratch bounded search); with no prior plan at all it always uses replan_budgeted regardless of prefer_follow. Cursor is always reset to 0 after any search path (both branches). decision is 'replan' if solved else 'bounded-refusal'." |  |  |  |  |

| `Verify a transport envelope's schema, payload identity, envelope identity, routing expectations, predecessor, and expiry without treating integrity as authentication.` | str_key | TELCO_VERIFY_ONTOLOGY = "Verify a transport envelope's schema, payload identity, envelope identity, routing expectations, predecessor, and expiry without treating integrity as authentication." |  |  |  |  |

| `Wraps ferroplan::api::decompose(domain_src, problem_src, opts) -> Result<Decomposition, SolveError>: decomposes a temporal goal into solvable contracts, solves and stitches them, and returns the inspectable fp:Decomposition. Inferred: MCP wrapper's exact JSON field schema not captured at fine grain in the extraction available.` | str_key | DECOMPOSE_ONTOLOGY = "Wraps ferroplan::api::decompose(domain_src, problem_src, opts) -> Result<Decomposition, SolveError>: decomposes a temporal goal into solvable contracts, solves and stitches them, and returns the inspectable fp:Decomposition. Inferred: MCP wrapper's exact JSON field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Wraps ferroplan::api::parse(src) -> ParseReport: validates PDDL syntax and returns a structure summary without grounding or solving, auto-detecting domain vs problem. Inferred: MCP wrapper's exact JSON field schema not captured at fine grain in the extraction available.` | str_key | PARSE_ONTOLOGY = "Wraps ferroplan::api::parse(src) -> ParseReport: validates PDDL syntax and returns a structure summary without grounding or solving, auto-detecting domain vs problem. Inferred: MCP wrapper's exact JSON field schema not captured at fine grain in the extraction available." |  |  |  |  |

| `Wraps ferroplan::api::solve(domain_src, problem_src, opts) -> Result<Solution, SolveError>: parses domain+problem PDDL, grounds, and searches for a plan under the given fp:Options, returning the fp:Solution shape. Inferred: field-by-field JSON request/response schema for the MCP wrapper itself (as opposed to the underlying api::solve signature, which is documented in api.rs) was not captured at fine grain in the extraction available.` | str_key | SOLVE_ONTOLOGY = "Wraps ferroplan::api::solve(domain_src, problem_src, opts) -> Result<Solution, SolveError>: parses domain+problem PDDL, grounds, and searches for a plan under the given fp:Options, returning the fp:Solution shape. Inferred: field-by-field JSON request/response schema for the MCP wrapper itself (as opposed to the underlying api::solve signature, which is documented in api.rs) was not captured at fine grain in the extraction available." |  |  |  |  |

| `solve` | str_key | MAIN_RESOURCE_TOOLS = "solve" |  |  |  |  |

| `ferroplan-mcp-plus/1.0` | str_key | PROFILE = "ferroplan-mcp-plus/1.0" |  |  |  |  |

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |

| `session_open` | str_key | RESOURCE_TOOLS = "session_open" |  |  |  |  |

| `urn:chatman:ferroplan-session-chain:v1` | str_key | SESSION_RECEIPT_DOMAIN = "urn:chatman:ferroplan-session-chain:v1" |  |  |  |  |

| `domain_digest` | str_key | KEYS = "domain_digest" |  |  |  |  |

| `session_list` | str_key | RESOURCE_TOOLS = "session_list" |  |  |  |  |

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |

| `DOM` | const | DOM: &str |  |  |  |  |

| `PROB` | const | PROB: &str |  |  |  |  |

| `call` | function | call(&mut self, tool: &str, args: Value) -> Value |  |  |  |  |

| `call_json` | function | call_json(&mut self, tool: &str, args: Value) -> Value |  |  |  |  |

| `call_text` | function | call_text(&mut self, tool: &str, args: Value) -> (String, bool) |  |  |  |  |

| `finish` | function | finish(mut self) |  |  |  |  |

| `notify` | function | notify(&mut self, method: &str) |  |  |  |  |

| `request` | function | request(&mut self, method: &str, params: Value) -> Value |  |  |  |  |

| `start` | function | start() -> Client |  |  |  |  |

| `(define (domain d) (:requirements :strips) (:predicates (p) (q) (r)) ` | str_key | DOM = "(define (domain d) (:requirements :strips) (:predicates (p) (q) (r)) " |  |  |  |  |

| `(define (problem pr) (:domain d) (:init (p)) (:goal (r)))` | str_key | PROB = "(define (problem pr) (:domain d) (:init (p)) (:goal (r)))" |  |  |  |  |

| `Client` | struct | Client { child: Child, stdin: Option<ChildStdin>, stdout: BufReader<ChildStdout>, next_id: i64 } |  |  |  |  |

| `(define (domain loc) (:requirements :strips) ` | str_key | DOM = "(define (domain loc) (:requirements :strips) " |  |  |  |  |

| `(define (problem locp) (:domain loc) (:init (at-a)) (:goal (at-c)))` | str_key | PROB = "(define (problem locp) (:domain loc) (:init (at-a)) (:goal (at-c)))" |  |  |  |  |

| `../../plugins/chatman-ecosystem/ontology/ferroplan-domain.ttl` | str_key | ONTOLOGIES = "../../plugins/chatman-ecosystem/ontology/ferroplan-domain.ttl" |  |  |  |  |

| `solve` | str_key | MAIN_STRUCTS = "solve" |  |  |  |  |

| `src/generated/tool_ontology.rs` | str_key | GENERATED_REL = "src/generated/tool_ontology.rs" |  |  |  |  |

| `src/main.rs` | str_key | ROUTER_SOURCES = "src/main.rs" |  |  |  |  |

| `solve` | str_key | ALL_42_TOOLS = "solve" |  |  |  |  |

| `fb9321d27882169acc83aaca0639b319cd3b7900` | str_key | BCINR_REVISION = "fb9321d27882169acc83aaca0639b319cd3b7900" |  |  |  |  |

| `
(define (domain farm) (:requirements :strips :typing :numeric-fluents)
  (:types agent place)
  (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
  (:functions (grain))
  (:action walk :parameters (?a - agent ?from ?to - place)
    :precondition (and (at ?a ?from) (road ?from ?to))
    :effect (and (not (at ?a ?from)) (at ?a ?to)))
  (:action harvest :parameters (?a - agent ?p - place)
    :precondition (and (at ?a ?p) (fertile ?p))
    :effect (increase (grain) 1)))` | str_key | FARM_DOM = "
(define (domain farm) (:requirements :strips :typing :numeric-fluents)
  (:types agent place)
  (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
  (:functions (grain))
  (:action walk :parameters (?a - agent ?from ?to - place)
    :precondition (and (at ?a ?from) (road ?from ?to))
    :effect (and (not (at ?a ?from)) (at ?a ?to)))
  (:action harvest :parameters (?a - agent ?p - place)
    :precondition (and (at ?a ?p) (fertile ?p))
    :effect (increase (grain) 1)))" |  |  |  |  |

| `
(define (domain rollers) (:requirements :strips :typing)
  (:types ball room)
  (:predicates (at ?b - ball ?r - room) (link ?x ?y - room)
               (goal-room ?r - room) (home ?b - ball))
  (:action roll :parameters (?b - ball ?from ?to - room)
    :precondition (and (at ?b ?from) (link ?from ?to))
    :effect (and (not (at ?b ?from)) (at ?b ?to)))
  (:action park :parameters (?b - ball ?r - room)
    :precondition (and (at ?b ?r) (goal-room ?r))
    :effect (home ?b)))` | str_key | ORB_DOM = "
(define (domain rollers) (:requirements :strips :typing)
  (:types ball room)
  (:predicates (at ?b - ball ?r - room) (link ?x ?y - room)
               (goal-room ?r - room) (home ?b - ball))
  (:action roll :parameters (?b - ball ?from ?to - room)
    :precondition (and (at ?b ?from) (link ?from ?to))
    :effect (and (not (at ?b ?from)) (at ?b ?to)))
  (:action park :parameters (?b - ball ?r - room)
    :precondition (and (at ?b ?r) (goal-room ?r))
    :effect (home ?b)))" |  |  |  |  |

| `
(define (problem p) (:domain farm)
  (:objects v1 - agent hut field - place)
  (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
  (:goal (>= (grain) 2)))` | str_key | FARM_PRB = "
(define (problem p) (:domain farm)
  (:objects v1 - agent hut field - place)
  (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
  (:goal (>= (grain) 2)))" |  |  |  |  |

| `
(define (problem p) (:domain rollers)
  (:objects b1 b2 - ball ra rb - room)
  (:init (at b1 ra) (at b2 ra) (link ra rb) (link rb ra) (goal-room rb))
  (:goal (and (home b1) (home b2))))` | str_key | ORB_PRB = "
(define (problem p) (:domain rollers)
  (:objects b1 b2 - ball ra rb - room)
  (:init (at b1 ra) (at b2 ra) (link ra rb) (link rb ra) (goal-room rb))
  (:goal (and (home b1) (home b2))))" |  |  |  |  |

| `(define (domain d3) (:requirements :strips) ` | str_key | DOM = "(define (domain d3) (:requirements :strips) " |  |  |  |  |

| `(define (problem pr3) (:domain d3) (:init (p)) (:goal (s)))` | str_key | PROB = "(define (problem pr3) (:domain d3) (:init (p)) (:goal (s)))" |  |  |  |  |

| `(define (domain d) (:requirements :strips) (:predicates (p) (q)) ` | str_key | DOM = "(define (domain d) (:requirements :strips) (:predicates (p) (q)) " |  |  |  |  |

| `(define (problem pr) (:domain d) (:init (p)) (:goal (q)))` | str_key | PROB = "(define (problem pr) (:domain d) (:init (p)) (:goal (q)))" |  |  |  |  |

| `(define (domain d) (:requirements :strips) (:predicates (p) (q)) ` | str_key | DOM = "(define (domain d) (:requirements :strips) (:predicates (p) (q)) " |  |  |  |  |

| `(define (problem pr) (:domain d) (:init (p)) (:goal (q)))` | str_key | PROB = "(define (problem pr) (:domain d) (:init (p)) (:goal (q)))" |  |  |  |  |

| `Authority` | enum | Authority { Observe, Select, Construct, Do } |  |  |  |  |

| `permits` | function | permits(g: Authority, n: Authority) -> bool |  |  |  |  |

| `exponential` | function | exponential(base: u64, attempt: u32, cap: u64) -> u64 |  |  |  |  |

| `new` | function | new(n: u32) -> Self |  |  |  |  |

| `remaining` | function | remaining(&self) -> u32 |  |  |  |  |

| `take` | function | take(&mut self) -> bool |  |  |  |  |

| `Budget` | struct | Budget { remaining: u32 } |  |  |  |  |

| `compatible` | function | compatible(available: &[Capability], required: &str) -> bool |  |  |  |  |

| `Capability` | struct | Capability { pub &'static str } |  |  |  |  |

| `open` | function | open(&self) -> bool |  |  |  |  |

| `record_failure` | function | record_failure(&mut self) |  |  |  |  |

| `Circuit` | struct | Circuit { pub failures: u32, pub threshold: u32 } |  |  |  |  |

| `bounded` | function | bounded(x: &[T], n: usize) -> Vec<T> |  |  |  |  |

| `fail` | function | fail(&mut self, id: &str) |  |  |  |  |

| `Coordinator` | struct | Coordinator { pub graph: Graph, pub excluded: Vec<String> } |  |  |  |  |

| `after` | function | after(d: Duration) -> Self |  |  |  |  |

| `expired` | function | expired(&self) -> bool |  |  |  |  |

| `Deadline` | struct | Deadline { pub Instant } |  |  |  |  |

| `dispatch` | function | dispatch(p: &P, s: &str) -> Outcome<Vec<String>> |  |  |  |  |

| `exclude` | function | exclude(&mut self) |  |  |  |  |

| `Edge` | struct | Edge { pub id: String, pub provider: String, pub enabled: bool } |  |  |  |  |

| `next` | function | next(self) -> Self |  |  |  |  |

| `Epoch` | struct | Epoch { pub u64 } |  |  |  |  |

| `admitted` | function | admitted(xs: &[Observation]) -> bool |  |  |  |  |

| `admitted` | function | admitted(&self) -> bool |  |  |  |  |

| `ExactSubject` | struct | ExactSubject { pub repo: String, pub sha: String } |  |  |  |  |

| `FailureClass` | enum | FailureClass { Local, Edge, Authority, Unknown } |  |  |  |  |

| `classify` | function | classify(o: &Outcome<T>) -> Option<FailureClass> |  |  |  |  |

| `reselect` | function | reselect(g: &'a Graph, x: &[String]) -> Option<&'a str> |  |  |  |  |

| `exclude` | function | exclude(&mut self, id: &str) |  |  |  |  |

| `lawful` | function | lawful(&self) -> impl Iterator<Item = &Edge> |  |  |  |  |

| `Graph` | struct | Graph { pub edges: Vec<Edge> } |  |  |  |  |

| `leaves` | function | leaves(&'a self, out: &mut Vec<&'a str>) |  |  |  |  |

| `Task` | struct | Task { pub id: String, pub children: Vec<Task> } |  |  |  |  |

| `Health` | enum | Health { Healthy, Degraded, Open } |  |  |  |  |

| `eligible` | function | eligible(h: Health) -> bool |  |  |  |  |

| `key` | function | key(subject: &str, epoch: u64, provider: &str) -> u64 |  |  |  |  |

| `valid_for` | function | valid_for(&self, o: &str, e: u64) -> bool |  |  |  |  |

| `Lease` | struct | Lease { pub owner: String, pub epoch: u64 } |  |  |  |  |

| `Observation` | struct | Observation { pub key: String, pub value: String, pub source: String } |  |  |  |  |

| `bound` | function | bound(&self) -> bool |  |  |  |  |

| `Event` | struct | Event { pub id: String, pub activity: String, pub objects: Vec<String> } |  |  |  |  |

| `Outcome` | enum | Outcome { Success(T), Retryable(String), Permanent(String), Refused(String), Unknown(String) } |  |  |  |  |

| `is_success` | function | is_success(&self) -> bool |  |  |  |  |

| `valid` | function | valid(&self) -> bool |  |  |  |  |

| `Plan` | struct | Plan { pub provider: String, pub steps: Vec<String>, pub cost: u64 } |  |  |  |  |

| `select` | function | select(e: &'a [Edge], x: &[String]) -> Option<&'a Edge> |  |  |  |  |

| `exact` | function | exact(&self) -> bool |  |  |  |  |

| `EvidenceSet` | struct | EvidenceSet { pub subject: String, pub sources: Vec<String> } |  |  |  |  |

| `ranked` | function | ranked(mut ps: Vec<Plan>) -> Vec<Plan> |  |  |  |  |

| `acyclic` | function | acyclic(e: &[Order]) -> bool |  |  |  |  |

| `Order` | struct | Order { pub before: String, pub after: String } |  |  |  |  |

| `Provider` | trait |  |  |  |  |  |

| `changed` | function | changed(a: &EpochArtifact, b: &EpochArtifact) -> bool |  |  |  |  |

| `EpochArtifact` | struct | EpochArtifact { pub epoch: Epoch, pub digest: u64 } |  |  |  |  |

| `exact` | function | exact(&self) -> bool |  |  |  |  |

| `Receipt` | struct | Receipt { pub subject_sha: String, pub epoch: u64, pub provider: String, pub edge: String, pub outcome: String } |  |  |  |  |

| `reconcile` | function | reconcile(g: &mut Graph, s: &[(String, Health)]) |  |  |  |  |

| `exclude_failed` | function | exclude_failed(g: &mut Graph, edge: &str) -> usize |  |  |  |  |

| `contains` | function | contains(&self, id: &str) -> bool |  |  |  |  |

| `register` | function | register(&mut self, id: impl Into<String>) |  |  |  |  |

| `Registry` | struct | Registry { ids: BTreeSet<String> } |  |  |  |  |

| `same_decision` | function | same_decision(a: &Receipt, b: &Receipt) -> bool |  |  |  |  |

| `next_edge` | function | next_edge(&self) -> Option<&str> |  |  |  |  |

| `is_empty` | function | is_empty(&self) -> bool |  |  |  |  |

| `len` | function | len(&self) -> usize |  |  |  |  |

| `pop_next` | function | pop_next(&mut self) -> Option<T> |  |  |  |  |

| `push` | function | push(&mut self, x: T) |  |  |  |  |

| `Scheduler` | struct | Scheduler { q: VecDeque<T> } |  |  |  |  |

| `State` | enum | State { Bound, Planning, Executing, Recovering, Succeeded, Failed } |  |  |  |  |

| `allowed` | function | allowed(a: State, b: State) -> bool |  |  |  |  |

| `Restart` | enum | Restart { Permanent, Transient, Temporary } |  |  |  |  |

| `should_restart` | function | should_restart(r: Restart, abnormal: bool) -> bool |  |  |  |  |

| `next` | function | next(ps: &'a [Plan], failed: &str) -> Option<&'a Plan> |  |  |  |  |

| `success_rate` | function | success_rate(&self) -> f64 |  |  |  |  |

| `Counters` | struct | Counters { pub attempts: u64, pub recoveries: u64, pub successes: u64 } |  |  |  |  |

| `actionable` | function | actionable(&self) -> bool |  |  |  |  |

| `Counterexample` | struct | Counterexample { pub invariant: String, pub trace: Vec<String> } |  |  |  |  |

| `ModelRole` | struct | ModelRole { pub context: bool, pub select: bool, pub construct: bool, pub do_act: bool } |  |  |  |  |

| `add` | function | add(&mut self, level: usize) |  |  |  |  |

| `analyze_conflict` | function | analyze_conflict(ctx: &mut Context, conflict: Conflict) -> usize |  |  |  |  |

| `clause` | function | clause(&self) -> &[Lit] |  |  |  |  |

| `involved` | function | involved(&self) -> &[ClauseRef] |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `test` | function | test(&self, level: usize) -> bool |  |  |  |  |

| `AnalyzeConflict` | struct | AnalyzeConflict { clause: Vec<Lit>, current_level_count: usize, var_flags: Vec<bool>, to_clean: Vec<Var>, involved: Vec<ClauseRef>, stack: Vec<Lit> } |  |  |  |  |

| `EnqueueAssumption` | enum | EnqueueAssumption { Done, Enqueued, Conflict } |  |  |  |  |

| `assumption_levels` | function | assumption_levels(&self) -> usize |  |  |  |  |

| `enqueue_assumption` | function | enqueue_assumption(ctx: &mut Context) -> EnqueueAssumption |  |  |  |  |

| `full_restart` | function | full_restart(&mut self) |  |  |  |  |

| `set_assumptions` | function | set_assumptions(ctx: &mut Context, user_assumptions: &[Lit]) |  |  |  |  |

| `user_failed_core` | function | user_failed_core(&self) -> &[Lit] |  |  |  |  |

| `Assumptions` | struct | Assumptions { assumptions: Vec<Lit>, failed_core: Vec<Lit>, user_failed_core: Vec<Lit>, assumption_levels: usize } |  |  |  |  |

| `add_binary_clause` | function | add_binary_clause(&mut self, lits: [Lit; 2]) |  |  |  |  |

| `count` | function | count(&self) -> usize |  |  |  |  |

| `implied` | function | implied(&self, lit: Lit) -> &[Lit] |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `simplify_binary` | function | simplify_binary(ctx: &mut Context) |  |  |  |  |

| `BinaryClauses` | struct | BinaryClauses { by_lit: Vec<Vec<Lit>>, count: usize } |  |  |  |  |

| `conflict_step` | function | conflict_step(ctx: &mut Context) |  |  |  |  |

| `header` | function | header(&self) -> &ClauseHeader |  |  |  |  |

| `header_mut` | function | header_mut(&mut self) -> &mut ClauseHeader |  |  |  |  |

| `lits` | function | lits(&self) -> &[Lit] |  |  |  |  |

| `lits_mut` | function | lits_mut(&mut self) -> &mut [Lit] |  |  |  |  |

| `Clause` | struct | Clause { data: [LitIdx] } |  |  |  |  |

| `activity::{decay_clause_activities, ClauseActivity}` | use | activity::{decay_clause_activities, ClauseActivity} |  |  |  |  |

| `alloc::{ClauseAlloc, ClauseRef}` | use | alloc::{ClauseAlloc, ClauseRef} |  |  |  |  |

| `db::{ClauseDb, Tier}` | use | db::{ClauseDb, Tier} |  |  |  |  |

| `gc::collect_garbage` | use | gc::collect_garbage |  |  |  |  |

| `header::ClauseHeader` | use | header::ClauseHeader |  |  |  |  |

| `bump_clause_activity` | function | bump_clause_activity(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| `decay_clause_activities` | function | decay_clause_activities(ctx: &mut Context) |  |  |  |  |

| `ClauseActivity` | struct | ClauseActivity { bump: f32, inv_decay: f32 } |  |  |  |  |

| `add_clause` | function | add_clause(&mut self, mut header: ClauseHeader, lits: &[Lit]) -> ClauseRef |  |  |  |  |

| `buffer_size` | function | buffer_size(&self) -> usize |  |  |  |  |

| `check_bounds` | function | check_bounds(&self, cref: ClauseRef, len: usize) |  |  |  |  |

| `clause` | function | clause(&self, cref: ClauseRef) -> &Clause |  |  |  |  |

| `clause_mut` | function | clause_mut(&mut self, cref: ClauseRef) -> &mut Clause |  |  |  |  |

| `header` | function | header(&self, cref: ClauseRef) -> &ClauseHeader |  |  |  |  |

| `header_mut` | function | header_mut(&mut self, cref: ClauseRef) -> &mut ClauseHeader |  |  |  |  |

| `header_unchecked_mut` | function | header_unchecked_mut(&mut self, cref: ClauseRef) -> &mut ClauseHeader |  |  |  |  |

| `lits_ptr_mut_unchecked` | function | lits_ptr_mut_unchecked(&mut self, cref: ClauseRef) -> *mut Lit |  |  |  |  |

| `with_capacity` | function | with_capacity(capacity: usize) -> ClauseAlloc |  |  |  |  |

| `ClauseAlloc` | struct | ClauseAlloc { buffer: Vec<LitIdx> } |  |  |  |  |

| `ClauseRef` | struct | ClauseRef { offset: ClauseOffset } |  |  |  |  |

| `assess_learned_clause` | function | assess_learned_clause(ctx: &mut Context, lits: &[Lit]) -> ClauseHeader |  |  |  |  |

| `bump_clause` | function | bump_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| `Tier` | enum | Tier { Irred = 0, Core = 1, Mid = 2, Local = 3 } |  |  |  |  |

| `add_clause` | function | add_clause(ctx: &mut Context, header: ClauseHeader, lits: &[Lit]) -> ClauseRef |  |  |  |  |

| `clauses_iter` | function | clauses_iter( db: &'a ClauseDb, alloc: &'a ClauseAlloc, ) -> impl Iterator<Item = ClauseRef> + 'a |  |  |  |  |

| `count` | function | count() -> usize |  |  |  |  |

| `count_by_tier` | function | count_by_tier(&self, tier: Tier) -> usize |  |  |  |  |

| `delete_clause` | function | delete_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| `filter_clauses` | function | filter_clauses( alloc: &mut ClauseAlloc, db: &mut ClauseDb, watchlists: &mut crate::prop::Watchlists, mut filter: F, ) |  |  |  |  |

| `from_index` | function | from_index(index: usize) -> Tier |  |  |  |  |

| `set_clause_tier` | function | set_clause_tier(ctx: &mut Context, cref: ClauseRef, tier: Tier) |  |  |  |  |

| `try_delete_clause` | function | try_delete_clause(ctx: &mut Context, cref: ClauseRef) -> bool |  |  |  |  |

| `ClauseDb` | struct | ClauseDb { pub(crate) clauses: Vec<ClauseRef>, pub(super) by_tier: [Vec<ClauseRef>; Tier::count()], pub(super) count_by_tier: [usize; Tier::count()], pub(super) garbage_size: usize } |  |  |  |  |

| `collect_garbage` | function | collect_garbage(ctx: &mut Context) |  |  |  |  |

| `active` | function | active(&self) -> bool |  |  |  |  |

| `activity` | function | activity(&self) -> f32 |  |  |  |  |

| `deleted` | function | deleted(&self) -> bool |  |  |  |  |

| `glue` | function | glue(&self) -> usize |  |  |  |  |

| `len` | function | len(&self) -> usize |  |  |  |  |

| `mark` | function | mark(&self) -> bool |  |  |  |  |

| `new` | function | new() -> ClauseHeader |  |  |  |  |

| `set_active` | function | set_active(&mut self, active: bool) |  |  |  |  |

| `set_activity` | function | set_activity(&mut self, activity: f32) |  |  |  |  |

| `set_deleted` | function | set_deleted(&mut self, deleted: bool) |  |  |  |  |

| `set_glue` | function | set_glue(&mut self, glue: usize) |  |  |  |  |

| `set_len` | function | set_len(&mut self, length: usize) |  |  |  |  |

| `set_mark` | function | set_mark(&mut self, mark: bool) |  |  |  |  |

| `set_tier` | function | set_tier(&mut self, tier: Tier) |  |  |  |  |

| `tier` | function | tier(&self) -> Tier |  |  |  |  |

| `ClauseHeader` | struct | ClauseHeader { pub(super) data: [LitIdx; HEADER_LEN] } |  |  |  |  |

| `dedup_and_mark_by_tier` | function | dedup_and_mark_by_tier(ctx: &mut Context, tier: Tier) |  |  |  |  |

| `reduce_locals` | function | reduce_locals(ctx: &mut Context) |  |  |  |  |

| `reduce_mids` | function | reduce_mids(ctx: &mut Context) |  |  |  |  |

| `is_empty` | function | is_empty(&self) -> bool |  |  |  |  |

| `iter` | function | iter(&self) -> impl Iterator<Item = &[Lit]> |  |  |  |  |

| `len` | function | len(&self) -> usize |  |  |  |  |

| `new` | function | new() -> CnfFormula |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `var_count` | function | var_count(&self) -> usize |  |  |  |  |

| `CnfFormula` | struct | CnfFormula { var_count: usize, literals: Vec<Lit>, clause_ranges: Vec<Range<usize>> } |  |  |  |  |

| `ExtendFormula` | trait |  |  |  |  |  |

| `SolverConfig` | struct | SolverConfig { pub vsids_decay: f32, pub clause_activity_decay: f32, pub reduce_locals_interval: u64, pub reduce_mids_interval: u64, pub luby_restart_interval_scale: u64 } |  |  |  |  |

| `set_var_count` | function | set_var_count(ctx: &mut Context, count: usize) |  |  |  |  |

| `Context` | struct | Context { pub analyze_conflict: AnalyzeConflict, pub assignment: Assignment, pub assumptions: Assumptions, pub binary_clauses: BinaryClauses, pub clause_activity: ClauseActivity, pub clause_alloc: ClauseAlloc, pub clause_db: ClauseDb, pub impl_graph: ImplGraph, pub model: Model, pub schedule: Schedule, pub solver_config: SolverConfig, pub solver_state: SolverState, pub tmp_data: TmpData, pub tmp_flags: TmpFlags, pub trail: Trail, pub variables: Variables, pub vsids: Vsids, pub watchlists: Watchlists } |  |  |  |  |

| `make_decision` | function | make_decision(ctx: &mut Context) -> bool |  |  |  |  |

| `bump` | function | bump(&mut self, var: Var) |  |  |  |  |

| `decay` | function | decay(&mut self) |  |  |  |  |

| `make_available` | function | make_available(&mut self, var: Var) |  |  |  |  |

| `make_unavailable` | function | make_unavailable(&mut self, var: Var) |  |  |  |  |

| `reset` | function | reset(&mut self, var: Var) |  |  |  |  |

| `seed` | function | seed(&mut self, var: Var, activity: f32) |  |  |  |  |

| `set_decay` | function | set_decay(&mut self, decay: f32) |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `var_count` | function | var_count(&self) -> usize |  |  |  |  |

| `Vsids` | struct | Vsids { activity: Vec<f32>, heap: Vec<Var>, position: Vec<Option<usize>>, bump: f32, inv_decay: f32 } |  |  |  |  |

| `DimacsError` | enum | DimacsError { Io(io::Error), Parse { line: usize, msg: String } } |  |  |  |  |

| `parse_dimacs` | function | parse_dimacs(input: impl io::BufRead) -> Result<CnfFormula, DimacsError> |  |  |  |  |

| `parse_dimacs_str` | function | parse_dimacs_str(input: &str) -> Result<CnfFormula, DimacsError> |  |  |  |  |

| `write_dimacs` | function | write_dimacs(target: &mut impl io::Write, formula: &CnfFormula) -> io::Result<()> |  |  |  |  |

| `compute_glue` | function | compute_glue(tmp_flags: &mut TmpFlags, impl_graph: &ImplGraph, lits: &[Lit]) -> usize |  |  |  |  |

| `cnf::{CnfFormula, ExtendFormula}` | use | cnf::{CnfFormula, ExtendFormula} |  |  |  |  |

| `lit::{Lit, Var}` | use | lit::{Lit, Var} |  |  |  |  |

| `solver::{Solver, SolverError}` | use | solver::{Solver, SolverError} |  |  |  |  |

| `code` | function | code(self) -> usize |  |  |  |  |

| `from_code` | function | from_code(code: usize) -> Lit |  |  |  |  |

| `from_dimacs` | function | from_dimacs(number: isize) -> Var |  |  |  |  |

| `from_index` | function | from_index(index: usize) -> Var |  |  |  |  |

| `from_var` | function | from_var(var: Var, polarity: bool) -> Lit |  |  |  |  |

| `index` | function | index(self) -> usize |  |  |  |  |

| `is_negative` | function | is_negative(self) -> bool |  |  |  |  |

| `is_positive` | function | is_positive(self) -> bool |  |  |  |  |

| `lit` | function | lit(self, polarity: bool) -> Lit |  |  |  |  |

| `map_var` | function | map_var(self, f: impl FnOnce(Var) -> Var) -> Lit |  |  |  |  |

| `max_count` | function | max_count() -> usize |  |  |  |  |

| `max_var` | function | max_var() -> Var |  |  |  |  |

| `negative` | function | negative(self) -> Lit |  |  |  |  |

| `positive` | function | positive(self) -> Lit |  |  |  |  |

| `to_dimacs` | function | to_dimacs(self) -> isize |  |  |  |  |

| `var` | function | var(self) -> Var |  |  |  |  |

| `Lit` | struct | Lit { code: LitIdx } |  |  |  |  |

| `Var` | struct | Var { index: LitIdx } |  |  |  |  |

| `load_clause` | function | load_clause(ctx: &mut Context, user_lits: &[Lit]) |  |  |  |  |

| `assignment` | function | assignment(&self) -> &[Option<bool>] |  |  |  |  |

| `reconstruct_global_model` | function | reconstruct_global_model(ctx: &mut Context) |  |  |  |  |

| `Model` | struct | Model { assignment: Vec<Option<bool>> } |  |  |  |  |

| `propagate` | function | propagate(ctx: &mut Context) -> Result<(), Conflict> |  |  |  |  |

| `assignment::{backtrack, enqueue_assignment, full_restart, restart, Assignment, Trail}` | use | assignment::{backtrack, enqueue_assignment, full_restart, restart, Assignment, Trail} |  |  |  |  |

| `graph::{Conflict, ImplGraph, Reason}` | use | graph::{Conflict, ImplGraph, Reason} |  |  |  |  |

| `watch::{enable_watchlists, Watch, Watchlists}` | use | watch::{enable_watchlists, Watch, Watchlists} |  |  |  |  |

| `assign_lit` | function | assign_lit(&mut self, lit: Lit) |  |  |  |  |

| `assignment` | function | assignment(&self) -> &[Option<bool>] |  |  |  |  |

| `backtrack` | function | backtrack(ctx: &mut Context, level: usize) |  |  |  |  |

| `clear` | function | clear(&mut self) |  |  |  |  |

| `current_level` | function | current_level(&self) -> usize |  |  |  |  |

| `enqueue_assignment` | function | enqueue_assignment( assignment: &mut Assignment, impl_graph: &mut ImplGraph, trail: &mut Trail, lit: Lit, reason: Reason, ) |  |  |  |  |

| `fast_option_eq` | function | fast_option_eq(a: Option<bool>, b: Option<bool>) -> bool |  |  |  |  |

| `full_restart` | function | full_restart(ctx: &mut Context) |  |  |  |  |

| `last_var_value` | function | last_var_value(&self, var: Var) -> bool |  |  |  |  |

| `lit_is_false` | function | lit_is_false(&self, lit: Lit) -> bool |  |  |  |  |

| `lit_is_true` | function | lit_is_true(&self, lit: Lit) -> bool |  |  |  |  |

| `lit_is_unk` | function | lit_is_unk(&self, lit: Lit) -> bool |  |  |  |  |

| `lit_value` | function | lit_value(&self, lit: Lit) -> Option<bool> |  |  |  |  |

| `new_decision_level` | function | new_decision_level(&mut self) |  |  |  |  |

| `pop_queue` | function | pop_queue(&mut self) -> Option<Lit> |  |  |  |  |

| `queue_head` | function | queue_head(&self) -> Option<Lit> |  |  |  |  |

| `restart` | function | restart(ctx: &mut Context) |  |  |  |  |

| `set_var` | function | set_var(&mut self, var: Var, assignment: Option<bool>) |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `trail` | function | trail(&self) -> &[Lit] |  |  |  |  |

| `unassign_var` | function | unassign_var(&mut self, var: Var) |  |  |  |  |

| `var_value` | function | var_value(&self, var: Var) -> Option<bool> |  |  |  |  |

| `Assignment` | struct | Assignment { assignment: Vec<Option<bool>>, last_value: Vec<bool> } |  |  |  |  |

| `Trail` | struct | Trail { trail: Vec<Lit>, queue_head_pos: usize, decisions: Vec<LitIdx>, units_removed: usize } |  |  |  |  |

| `propagate_binary` | function | propagate_binary(ctx: &mut Context, lit: Lit) -> Result<(), Conflict> |  |  |  |  |

| `Conflict` | enum | Conflict { Binary([Lit; 2]), Long(ClauseRef) } |  |  |  |  |

| `Reason` | enum | Reason { Unit, Binary([Lit; 1]), Long(ClauseRef) } |  |  |  |  |

| `is_removed_unit` | function | is_removed_unit(&self, var: Var) -> bool |  |  |  |  |

| `is_unit` | function | is_unit(&self) -> bool |  |  |  |  |

| `level` | function | level(&self, var: Var) -> usize |  |  |  |  |

| `lits` | function | lits(&'a self, alloc: &'a ClauseAlloc) -> &'a [Lit] |  |  |  |  |

| `reason` | function | reason(&self, var: Var) -> &Reason |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `update_reason` | function | update_reason(&mut self, var: Var, reason: Reason) |  |  |  |  |

| `update_removed_unit` | function | update_removed_unit(&mut self, var: Var) |  |  |  |  |

| `ImplGraph` | struct | ImplGraph { pub nodes: Vec<ImplNode> } |  |  |  |  |

| `ImplNode` | struct | ImplNode { pub reason: Reason, pub level: LitIdx, pub depth: LitIdx } |  |  |  |  |

| `propagate_long` | function | propagate_long(ctx: &mut Context, lit: Lit) -> Result<(), Conflict> |  |  |  |  |

| `add_watch` | function | add_watch(&mut self, lit: Lit, watch: Watch) |  |  |  |  |

| `disable` | function | disable(&mut self) |  |  |  |  |

| `enable_watchlists` | function | enable_watchlists(ctx: &mut Context) |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `watch_clause` | function | watch_clause(&mut self, cref: ClauseRef, lits: [Lit; 2]) |  |  |  |  |

| `watched_by_mut` | function | watched_by_mut(&mut self, lit: Lit) -> &mut Vec<Watch> |  |  |  |  |

| `Watch` | struct | Watch { pub cref: ClauseRef, pub blocking: Lit } |  |  |  |  |

| `Watchlists` | struct | Watchlists { watches: Vec<Vec<Watch>>, enabled: bool } |  |  |  |  |

| `schedule_step` | function | schedule_step(ctx: &mut Context) -> bool |  |  |  |  |

| `Schedule` | struct | Schedule { conflicts: u64, next_restart: u64, restarts: u64, luby: LubySequence, pub conflict_limit: Option<u64>, pub conflicts_this_solve: u64 } |  |  |  |  |

| `advance` | function | advance(&mut self) -> u64 |  |  |  |  |

| `LubySequence` | struct | LubySequence { u: u64, v: u64 } |  |  |  |  |

| `SolverError` | enum | SolverError { Interrupted } |  |  |  |  |

| `add_formula` | function | add_formula(&mut self, formula: &CnfFormula) |  |  |  |  |

| `assume` | function | assume(&mut self, assumptions: &[Lit]) |  |  |  |  |

| `failed_core` | function | failed_core(&self) -> Option<&[Lit]> |  |  |  |  |

| `is_recoverable` | function | is_recoverable(&self) -> bool |  |  |  |  |

| `model` | function | model(&self) -> Option<Vec<Lit>> |  |  |  |  |

| `new` | function | new() -> Solver |  |  |  |  |

| `seed_activity` | function | seed_activity(&mut self, seeds: &[(usize, f32)]) |  |  |  |  |

| `set_conflict_limit` | function | set_conflict_limit(&mut self, limit: Option<u64>) |  |  |  |  |

| `solve` | function | solve(&mut self) -> Result<bool, SolverError> |  |  |  |  |

| `Solver` | struct | Solver { ctx: Box<Context> } |  |  |  |  |

| `SatState` | enum | SatState { Unknown, Sat, Unsat, UnsatUnderAssumptions } |  |  |  |  |

| `SolverState` | struct | SolverState { pub sat_state: SatState } |  |  |  |  |

| `set_var_count` | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| `TmpData` | struct | TmpData { pub lits: Vec<Lit>, pub lits_2: Vec<Lit> } |  |  |  |  |

| `TmpFlags` | struct | TmpFlags { pub flags: Vec<bool> } |  |  |  |  |

| `prove_units` | function | prove_units(ctx: &mut Context) -> bool |  |  |  |  |

| `resurrect_unit` | function | resurrect_unit(ctx: &mut Context, lit: Lit) |  |  |  |  |

| `unit_simplify` | function | unit_simplify(ctx: &mut Context) |  |  |  |  |

| `existing_user_from_solver` | function | existing_user_from_solver(&self, solver: Var) -> Var |  |  |  |  |

| `global_from_solver` | function | global_from_solver(&self) -> &VarMap |  |  |  |  |

| `global_from_solver_mut` | function | global_from_solver_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `global_from_user` | function | global_from_user(&self) -> &VarMap |  |  |  |  |

| `global_from_user_mut` | function | global_from_user_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `global_var_iter` | function | global_var_iter(&self) -> impl Iterator<Item = Var> + '_ |  |  |  |  |

| `global_watermark` | function | global_watermark(&self) -> usize |  |  |  |  |

| `initialize_solver_var` | function | initialize_solver_var(ctx: &mut Context, solver: Var, global: Var) |  |  |  |  |

| `new_user_var` | function | new_user_var(ctx: &mut Context) -> Var |  |  |  |  |

| `next_unmapped_solver` | function | next_unmapped_solver(&self) -> Var |  |  |  |  |

| `next_unmapped_user` | function | next_unmapped_user(&self) -> Var |  |  |  |  |

| `remove_solver_var` | function | remove_solver_var(ctx: &mut Context, solver: Var) |  |  |  |  |

| `solver_from_global` | function | solver_from_global(&self) -> &VarMap |  |  |  |  |

| `solver_from_global_mut` | function | solver_from_global_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `solver_from_user` | function | solver_from_user(ctx: &mut Context, user: Var) -> Var |  |  |  |  |

| `solver_from_user_lits` | function | solver_from_user_lits(ctx: &mut Context, solver_lits: &mut Vec<Lit>, user_lits: &[Lit]) |  |  |  |  |

| `solver_var_present` | function | solver_var_present(&self, solver: Var) -> bool |  |  |  |  |

| `solver_watermark` | function | solver_watermark(&self) -> usize |  |  |  |  |

| `user_from_global` | function | user_from_global(&self) -> &VarMap |  |  |  |  |

| `user_var_iter` | function | user_var_iter(&self) -> impl Iterator<Item = Var> + '_ |  |  |  |  |

| `user_watermark` | function | user_watermark(&self) -> usize |  |  |  |  |

| `var_data_global` | function | var_data_global(&self, global: Var) -> &VarData |  |  |  |  |

| `var_data_global_mut` | function | var_data_global_mut(&mut self, global: Var) -> &mut VarData |  |  |  |  |

| `var_data_solver_mut` | function | var_data_solver_mut(&mut self, solver: Var) -> &mut VarData |  |  |  |  |

| `Variables` | struct | Variables { global_from_user: VarBiMap, solver_from_global: VarBiMap, solver_freelist: HashSet<Var>, var_data: Vec<VarData> } |  |  |  |  |

| `user_default` | function | user_default() -> VarData |  |  |  |  |

| `VarData` | struct | VarData { pub unit: Option<bool>, pub isolated: bool, pub assumed: bool, pub deleted: bool } |  |  |  |  |

| `bwd` | function | bwd(&self) -> &VarMap |  |  |  |  |

| `bwd_mut` | function | bwd_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `fwd` | function | fwd(&self) -> &VarMap |  |  |  |  |

| `fwd_mut` | function | fwd_mut(&mut self) -> VarBiMapMut<'_> |  |  |  |  |

| `get` | function | get(&self, from: Var) -> Option<Var> |  |  |  |  |

| `insert` | function | insert(&mut self, into: Var, from: Var) |  |  |  |  |

| `remove` | function | remove(&mut self, from: Var) |  |  |  |  |

| `watermark` | function | watermark(&self) -> usize |  |  |  |  |

| `VarBiMap` | struct | VarBiMap { fwd: VarMap, bwd: VarMap } |  |  |  |  |

| `VarBiMapMut` | struct | VarBiMapMut { fwd: &'a mut VarMap, bwd: &'a mut VarMap } |  |  |  |  |

| `VarMap` | struct | VarMap { mapping: Vec<LitIdx> } |  |  |  |  |

| `CORRIDOR_DOMAIN` | const | CORRIDOR_DOMAIN: &str |  |  |  |  |

| `MAX_PROBE_CANDIDATES` | const | MAX_PROBE_CANDIDATES: usize |  |  |  |  |

| `MAX_PROBE_FACT_BYTES` | const | MAX_PROBE_FACT_BYTES: usize |  |  |  |  |

| `MAX_PROBE_OBSERVATIONS` | const | MAX_PROBE_OBSERVATIONS: usize |  |  |  |  |

| `admit_candidates` | function | admit_candidates( candidates: &[ProbeCandidate], goal_limit: usize, surface: &str, ) -> Result<(), Refusal> |  |  |  |  |

| `corridor_problem` | function | corridor_problem(n: usize) -> String |  |  |  |  |

| `probe_all` | function | probe_all( parent: &Session, candidates: &[ProbeCandidate], evals: usize, mem_mb: usize, ) -> Vec<Value> |  |  |  |  |

| `probe_candidate` | function | probe_candidate( parent: &Session, candidate: &ProbeCandidate, evals: usize, mem_mb: usize, ) -> Value |  |  |  |  |

| `repair` | function | repair( inner: &Session, plan: &mut Option<Plan>, cursor: &mut usize, evals: usize, mem_mb: usize, ) -> Value |  |  |  |  |

| `ProbeCandidate` | struct | ProbeCandidate { pub id: String, pub goal: Option<String>, pub sight: Vec<(String, bool)>, pub restrict_contains: Option<String> } |  |  |  |  |

| `advance` | function | advance(&mut self) |  |  |  |  |

| `apply_start` | function | apply_start(&mut self, name: &str) -> Result<(), JsValue> |  |  |  |  |

| `drop_plan` | function | drop_plan(&mut self) |  |  |  |  |

| `elapse` | function | elapse(&mut self, dt: f64) -> Result<String, JsValue> |  |  |  |  |

| `explain` | function | explain(domain: &str, problem: &str, plan_json: &str) -> String |  |  |  |  |

| `fact` | function | fact(&self, name: &str) -> JsValue |  |  |  |  |

| `fluent` | function | fluent(&self, name: &str) -> JsValue |  |  |  |  |

| `fond_validate` | function | fond_validate(problem_json: &str, plan_json: &str) -> String |  |  |  |  |

| `fork` | function | fork(&self) -> WasmSession |  |  |  |  |

| `goal_met` | function | goal_met(&self) -> bool |  |  |  |  |

| `has_plan` | function | has_plan(&self) -> bool |  |  |  |  |

| `mind_bytes` | function | mind_bytes(&self) -> usize |  |  |  |  |

| `new` | function | new(domain: &str, problem: &str) -> Result<WasmSession, JsValue> |  |  |  |  |

| `observe` | function | observe(&mut self, sight_json: &str) -> Result<String, JsValue> |  |  |  |  |

| `plan` | function | plan( domain: &str, problem: &str, mode: Option<String>, flags: Option<String>, search: Option<String>, ) -> String |  |  |  |  |

| `plan_production` | function | plan_production( domain: &str, problem: &str, mode: Option<String>, search: Option<String>, max_evaluated: Option<usize>, max_plan_steps: Option<usize>, max_output_bytes: Option<usize>, request_id: Option<String>, ) -> String |  |  |  |  |

| `plan_valid_json` | function | plan_valid_json(&self, plan_json: &str, from: usize) -> bool |  |  |  |  |

| `probe_json` | function | probe_json(&self, candidates_json: &str, evals: usize, mem_mb: usize) -> String |  |  |  |  |

| `readiness` | function | readiness() -> String |  |  |  |  |

| `repair` | function | repair(&mut self, evals: usize, mem_mb: usize) -> String |  |  |  |  |

| `replan_following` | function | replan_following(&mut self, evals: usize, mem_mb: usize) -> String |  |  |  |  |

| `restrict_contains` | function | restrict_contains(&mut self, filter: String) |  |  |  |  |

| `restrict_prefix_claims` | function | restrict_prefix_claims(&mut self, prefix: String, claimed: String) |  |  |  |  |

| `set_fact` | function | set_fact(&mut self, name: &str, value: bool) -> Result<(), JsValue> |  |  |  |  |

| `set_fluent` | function | set_fluent(&mut self, name: &str, value: f64) -> Result<(), JsValue> |  |  |  |  |

| `set_goal` | function | set_goal(&mut self, goal: &str) -> Result<(), JsValue> |  |  |  |  |

| `set_timed_fact` | function | set_timed_fact(&mut self, dt: f64, name: &str, value: bool) -> Result<(), JsValue> |  |  |  |  |

| `step_json` | function | step_json(&self) -> String |  |  |  |  |

| `suffix_json` | function | suffix_json(&self) -> String |  |  |  |  |

| `think` | function | think(&mut self, evals: usize, mem_mb: usize) -> String |  |  |  |  |

| `valid` | function | valid(&self) -> bool |  |  |  |  |

| `version` | function | version() -> String |  |  |  |  |

| `world_bytes` | function | world_bytes(&self) -> usize |  |  |  |  |

| `WasmSession` | struct | WasmSession { inner: ferroplan::Session, plan: Option<ferroplan::api::Plan>, cursor: usize } |  |  |  |  |

| `browser_impl::{ explain, fond_validate, plan, plan_production, readiness, version, WasmSession, }` | use | browser_impl::{ explain, fond_validate, plan, plan_production, readiness, version, WasmSession, } |  |  |  |  |

| `MAX_PROBE_ID_BYTES` | const | MAX_PROBE_ID_BYTES: usize |  |  |  |  |

| `contradictory_fact` | function | contradictory_fact(sight: &[(String, bool)]) -> Option<&str> |  |  |  |  |

| `duplicate_id` | function | duplicate_id(ids: impl IntoIterator<Item = &'a str>) -> Option<&'a str> |  |  |  |  |

| `fact_key` | function | fact_key(fact: &str) -> String |  |  |  |  |

| `malformed_id` | function | malformed_id(ids: impl IntoIterator<Item = &'a str>) -> Option<&'a str> |  |  |  |  |

| `fp_alloc` | function | fp_alloc(len: usize) -> *mut u8 |  |  |  |  |

| `fp_call` | function | fp_call(ptr: *mut u8, len: usize) -> u64 |  |  |  |  |

| `fp_dealloc` | function | fp_dealloc(ptr: *mut u8, len: usize) |  |  |  |  |

| `(define (domain rooms)
      (:requirements :strips :typing)
      (:types room)
      (:predicates (at ?r - room) (link ?a - room ?b - room))
      (:action go
        :parameters (?a - room ?b - room)
        :precondition (and (at ?a) (link ?a ?b))
        :effect (and (at ?b) (not (at ?a)))))` | str_key | CORRIDOR_DOMAIN = "(define (domain rooms)
      (:requirements :strips :typing)
      (:types room)
      (:predicates (at ?r - room) (link ?a - room ?b - room))
      (:action go
        :parameters (?a - room ?b - room)
        :precondition (and (at ?a) (link ?a ?b))
        :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (problem corridor)
      (:domain rooms)
      (:objects a b c d - room)
      (:init (at a) (link a b) (link b c) (link c d))
      (:goal (at d)))` | str_key | CORRIDOR_PROBLEM = "(define (problem corridor)
      (:domain rooms)
      (:objects a b c d - room)
      (:init (at a) (link a b) (link b c) (link c d))
      (:goal (at d)))" |  |  |  |  |

| `(define (problem dead-end)
      (:domain rooms)
      (:objects a b c d - room)
      (:init (at a) (link a b) (link b c))
      (:goal (at d)))` | str_key | CORRIDOR_DEAD_END_PROBLEM = "(define (problem dead-end)
      (:domain rooms)
      (:objects a b c d - room)
      (:init (at a) (link a b) (link b c))
      (:goal (at d)))" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../../ferroplan-hddl/fixtures/c/domain.hddl" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/c/problem.hddl` | str_key | FIXTURE_C_PROBLEM = "../../ferroplan-hddl/fixtures/c/problem.hddl" |  |  |  |  |

| `../../ontology/ferroplan-wasm.ttl` | str_key | ONTOLOGY_REL = "../../ontology/ferroplan-wasm.ttl" |  |  |  |  |

| `fp_alloc` | str_key | EXPECTED_EXPORTS = "fp_alloc" |  |  |  |  |

| `fp_dealloc` | str_key | FREE_SYMBOL = "fp_dealloc" |  |  |  |  |

| `ontology/contract.ttl` | str_key | CONTRACTS_REL = "ontology/contract.ttl" |  |  |  |  |

| `registry/capability-registry.json` | str_key | REGISTRY_REL = "registry/capability-registry.json" |  |  |  |  |

| `src/wasi_abi.rs` | str_key | WASI_ABI_REL = "src/wasi_abi.rs" |  |  |  |  |

| `(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))` | str_key | DOMAIN = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (problem two-room)
  (:domain rooms)
  (:objects a b - room)
  (:init (at a) (link a b))
  (:goal (at b)))` | str_key | PROBLEM = "(define (problem two-room)
  (:domain rooms)
  (:objects a b - room)
  (:init (at a) (link a b))
  (:goal (at b)))" |  |  |  |  |

| `../../../ggen-marketplace/packs/qri-qualification-profile-pack` | str_key | PACK_REL = "../../../ggen-marketplace/packs/qri-qualification-profile-pack" |  |  |  |  |

| `(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))` | str_key | DOMAIN = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (problem repair)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c b))
  (:goal (at b)))` | str_key | PROBLEM = "(define (problem repair)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c b))
  (:goal (at b)))" |  |  |  |  |

| `# c` | str_key | REG = "# c" |  |  |  |  |

| `x` | str_key | ONT = "x" |  |  |  |  |

| `drop-retry` | str_key | DIRS = "drop-retry" |  |  |  |  |

| `(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))` | str_key | DOMAIN = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain-domain.pddl` | str_key | DOM = "../../../benchmarks/bench/bazaar-chain-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain-x2m.pddl` | str_key | PRB_X2M = "../../../benchmarks/bench/bazaar-chain-x2m.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain.pddl` | str_key | PRB = "../../../benchmarks/bench/bazaar-chain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain-domain.pddl` | str_key | DOM = "../../../benchmarks/bench/bazaar-chain-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain-x2.pddl` | str_key | PRB_X2 = "../../../benchmarks/bench/bazaar-chain-x2.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-chain.pddl` | str_key | PRB = "../../../benchmarks/bench/bazaar-chain.pddl" |  |  |  |  |

| `
(define (domain homestead) (:requirements :strips :typing :numeric-fluents)
  (:types agent place)
  (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
  (:functions (grain))
  (:action walk :parameters (?a - agent ?from ?to - place)
    :precondition (and (at ?a ?from) (road ?from ?to))
    :effect (and (not (at ?a ?from)) (at ?a ?to)))
  (:action harvest :parameters (?a - agent ?p - place)
    :precondition (and (at ?a ?p) (fertile ?p))
    :effect (increase (grain) 1)))` | str_key | DOM = "
(define (domain homestead) (:requirements :strips :typing :numeric-fluents)
  (:types agent place)
  (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
  (:functions (grain))
  (:action walk :parameters (?a - agent ?from ?to - place)
    :precondition (and (at ?a ?from) (road ?from ?to))
    :effect (and (not (at ?a ?from)) (at ?a ?to)))
  (:action harvest :parameters (?a - agent ?p - place)
    :precondition (and (at ?a ?p) (fertile ?p))
    :effect (increase (grain) 1)))" |  |  |  |  |

| `
(define (problem morning) (:domain homestead)
  (:objects vera - agent hut field barn - place)
  (:init (at vera hut) (road hut field) (road field hut)
         (road field barn) (road barn field) (fertile field) (= (grain) 0))
  (:goal (>= (grain) 3)))` | str_key | PRB = "
(define (problem morning) (:domain homestead)
  (:objects vera - agent hut field barn - place)
  (:init (at vera hut) (road hut field) (road field hut)
         (road field barn) (road barn field) (fertile field) (= (grain) 0))
  (:goal (>= (grain) 3)))" |  |  |  |  |

| `(define (domain gripper)
 (:requirements :strips :typing)
 (:types room ball gripper)
 (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
              (free ?g - gripper) (carry ?b - ball ?g - gripper))
 (:action move :parameters (?from ?to - room)
   :precondition (at-robby ?from) :effect (and (at-robby ?to) (not (at-robby ?from))))
 (:action pick :parameters (?b - ball ?r - room ?g - gripper)
   :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
   :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
 (:action drop :parameters (?b - ball ?r - room ?g - gripper)
   :precondition (and (carry ?b ?g) (at-robby ?r))
   :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))` | str_key | DOMAIN = "(define (domain gripper)
 (:requirements :strips :typing)
 (:types room ball gripper)
 (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
              (free ?g - gripper) (carry ?b - ball ?g - gripper))
 (:action move :parameters (?from ?to - room)
   :precondition (at-robby ?from) :effect (and (at-robby ?to) (not (at-robby ?from))))
 (:action pick :parameters (?b - ball ?r - room ?g - gripper)
   :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
   :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
 (:action drop :parameters (?b - ball ?r - room ?g - gripper)
   :precondition (and (carry ?b ?g) (at-robby ?r))
   :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))" |  |  |  |  |

| `(define (problem g1) (:domain gripper)
 (:objects rooma roomb - room  ball1 ball2 - ball  left right - gripper)
 (:init (at-robby rooma) (free left) (free right)
        (at ball1 rooma) (at ball2 rooma))
 (:goal (and (at ball1 roomb) (at ball2 roomb))))` | str_key | PROBLEM = "(define (problem g1) (:domain gripper)
 (:objects rooma roomb - room  ball1 ball2 - ball  left right - gripper)
 (:init (at-robby rooma) (free left) (free right)
        (at ball1 rooma) (at ball2 rooma))
 (:goal (and (at ball1 roomb) (at ball2 roomb))))" |  |  |  |  |

| `../../../benchmarks/bench/bazaar-redistribution.pddl` | str_key | PRB = "../../../benchmarks/bench/bazaar-redistribution.pddl" |  |  |  |  |

| `../../../benchmarks/bench/bazaar.pddl` | str_key | DOM = "../../../benchmarks/bench/bazaar.pddl" |  |  |  |  |

| `(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move :parameters (?from ?to - room)
    :precondition (at-robby ?from)
    :effect (and (not (at-robby ?from)) (at-robby ?to)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))` | str_key | DOMAIN = "(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move :parameters (?from ?to - room)
    :precondition (at-robby ?from)
    :effect (and (not (at-robby ?from)) (at-robby ?to)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))" |  |  |  |  |

| `(define (problem gripper-1) (:domain gripper)
  (:objects rooma roomb - room b1 - ball left - gripper)
  (:init (at-robby rooma) (at b1 rooma) (free left))
  (:goal (at b1 roomb)))` | str_key | PROBLEM = "(define (problem gripper-1) (:domain gripper)
  (:objects rooma roomb - room b1 - ball left - gripper)
  (:init (at-robby rooma) (at b1 rooma) (free left))
  (:goal (at b1 roomb)))" |  |  |  |  |

| `THINK_EVALS` | env_key | std::env::var("THINK_EVALS") |  |  |  |  |

| `../../../benchmarks/village/domain.pddl` | str_key | DOM = "../../../benchmarks/village/domain.pddl" |  |  |  |  |

| `../../../benchmarks/village/pair.pddl` | str_key | PRB = "../../../benchmarks/village/pair.pddl" |  |  |  |  |

| `../../../benchmarks/village/domain.pddl` | str_key | DOM = "../../../benchmarks/village/domain.pddl" |  |  |  |  |

| `../../../benchmarks/village/pair.pddl` | str_key | PRB = "../../../benchmarks/village/pair.pddl" |  |  |  |  |

| `Mode` | enum | Mode { Auto, Ff, Partition, Pddl3, Temporal, Portfolio, Optimal, Sat } |  |  |  |  |

| `Search` | enum | Search { Auto, Ehc, BestFirst, EhcThenBestFirst } |  |  |  |  |

| `SolveError` | enum | SolveError { DomainParse(crate::types::ParseError), ProblemParse(crate::types::ParseError), EmptyType { kind: String, pred: String, ty: String, }, Derived(String), Unsupported(String) } |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_SAT_CLASSICAL` | env_key | std::env::var("FF_SAT_CLASSICAL") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `decompose` | function | decompose( domain_src: &str, problem_src: &str, opts: &Options, ) -> Result<Decomposition, SolveError> |  |  |  |  |

| `parse` | function | parse(src: &str) -> ParseReport |  |  |  |  |

| `solve` | function | solve(domain_src: &str, problem_src: &str, opts: &Options) -> Result<Solution, SolveError> |  |  |  |  |

| `Contract` | struct | Contract { pub index: usize, pub goal: String, pub steps: Vec<Step>, pub makespan: f64, pub offset: f64 } |  |  |  |  |

| `Decomposition` | struct | Decomposition { pub solved: bool, pub contracts: Vec<Contract>, pub plan: Option<Plan>, pub monolithic: bool, pub notes: Vec<String> } |  |  |  |  |

| `DomainSummary` | struct | DomainSummary { pub types: Vec<String>, pub predicates: Vec<String>, pub functions: Vec<String>, pub actions: Vec<String>, pub durative_actions: Vec<String>, pub derived: usize } |  |  |  |  |

| `Options` | struct | Options { pub mode: Mode, pub search: Search, pub helpful_actions: bool, pub weight_g: f64, pub weight_h: f64, pub threads: usize, pub max_evaluated: Option<usize>, pub optimize: bool, pub wall_ms: Option<u64>, pub should_continue: Option<std::sync::Arc<std::sync::atomic::AtomicBool>> } |  |  |  |  |

| `ParseReport` | struct | ParseReport { pub ok: bool, pub kind: Option<String>, pub name: Option<String>, pub requirements: Vec<String>, pub error: Option<String>, pub domain: Option<DomainSummary>, pub problem: Option<ProblemSummary> } |  |  |  |  |

| `Plan` | struct | Plan { pub steps: Vec<Step>, pub length: usize, pub metric: Option<f64>, pub makespan: Option<f64> } |  |  |  |  |

| `ProblemSummary` | struct | ProblemSummary { pub domain: String, pub objects: usize, pub init_facts: usize, pub init_fluents: usize, pub timed_initial_literals: usize, pub has_goal: bool, pub has_metric: bool } |  |  |  |  |

| `Solution` | struct | Solution { pub solved: bool, pub mode: Mode, pub plan: Option<Plan>, pub statistics: Statistics, pub notes: Vec<String> } |  |  |  |  |

| `Statistics` | struct | Statistics { pub grounded_facts: usize, pub grounded_actions: usize, pub evaluated_states: usize, pub threads: usize } |  |  |  |  |

| `Step` | struct | Step { pub index: usize, pub action: String, pub args: Vec<String>, pub time: Option<f64>, pub duration: Option<f64> } |  |  |  |  |

| `clear` | function | clear(w: &mut [u64], i: usize) |  |  |  |  |

| `count` | function | count(w: &[u64]) -> usize |  |  |  |  |

| `set` | function | set(w: &mut [u64], i: usize) |  |  |  |  |

| `test` | function | test(w: &[u64], i: usize) -> bool |  |  |  |  |

| `words_for` | function | words_for(n_bits: usize) -> usize |  |  |  |  |

| `elapsed_ms` | function | elapsed_ms(&self) -> u128 |  |  |  |  |

| `elapsed_secs` | function | elapsed_secs(&self) -> f64 |  |  |  |  |

| `elapsed_us` | function | elapsed_us(&self) -> u128 |  |  |  |  |

| `now` | function | now() -> Self |  |  |  |  |

| `Clock` | struct | Clock { t0: std::time::Instant } |  |  |  |  |

| `END_ACTION` | const | END_ACTION: &str |  |  |  |  |

| `Traj` | enum | Traj { Always(Formula), Sometime(Formula), AtMostOnce(Formula), SometimeAfter(Formula, Formula), SometimeBefore(Formula, Formula), AtEnd(Formula), Within(f64, Formula), AlwaysWithin(f64, Formula, Formula) } |  |  |  |  |

| `FF_CONSTRAINTS_REJECT` | env_key | std::env::var("FF_CONSTRAINTS_REJECT") |  |  |  |  |

| `FF_NO_COND_SHARE` | env_key | std::env::var("FF_NO_COND_SHARE") |  |  |  |  |

| `FF_NO_TRAJ_END` | env_key | std::env::var("FF_NO_TRAJ_END") |  |  |  |  |

| `FF_PREF_NO_STATIC` | env_key | std::env::var("FF_PREF_NO_STATIC") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `accepted` | function | accepted(&self) -> bool |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> |  |  |  |  |

| `expand` | function | expand(domain: &Domain, problem: &Problem) -> Result<Expanded, String> |  |  |  |  |

| `gate` | function | gate(domain: &Domain, problem: &Problem) -> Result<Option<(Domain, Problem)>, String> |  |  |  |  |

| `hard_only_gated` | function | hard_only_gated( domain: &Domain, problem: &Problem, ) -> Result<Option<(Domain, Problem)>, String> |  |  |  |  |

| `new` | function | new(traj: &'a Traj) -> Self |  |  |  |  |

| `op_name` | function | op_name(&self) -> &'static str |  |  |  |  |

| `step` | function | step(&mut self, holds: &mut dyn FnMut(&Formula) -> bool) |  |  |  |  |

| `step_at` | function | step_at(&mut self, time: f64, holds: &mut dyn FnMut(&Formula) -> bool) |  |  |  |  |

| `TRAJ-CLOCK` | str_key | CLOCK_FLUENT = "TRAJ-CLOCK" |  |  |  |  |

| `TRAJ-END` | str_key | END_ACTION = "TRAJ-END" |  |  |  |  |

| `Expanded` | struct | Expanded { pub hard: Vec<Traj>, pub soft: Vec<(String, Vec<Traj>)> } |  |  |  |  |

| `Fold` | struct | Fold { traj: &'a Traj, ok: bool, seen: bool, holding: bool, pending: bool, safe: bool, last: bool, due: f64 } |  |  |  |  |

| `FF_COST_SWEEP_EVALS` | env_key | std::env::var("FF_COST_SWEEP_EVALS") |  |  |  |  |

| `FF_LEN_SWEEP_EVALS` | env_key | std::env::var("FF_LEN_SWEEP_EVALS") |  |  |  |  |

| `improve` | function | improve( task: &PackedTask, cf: usize, ops: Vec<usize>, first_cost: f64, threads: usize, base: SearchCfg, spent: usize, orbit: Option<&crate::orbits::OrbitMap>, ) -> CostOutcome |  |  |  |  |

| `improve_length` | function | improve_length( task: &PackedTask, ops: Vec<usize>, threads: usize, base: SearchCfg, spent: usize, ) -> (Vec<usize>, usize, bool) |  |  |  |  |

| `metric_fluent` | function | metric_fluent(problem: &Problem) -> Option<String> |  |  |  |  |

| `optimize_text` | function | optimize_text( problem: &Problem, task: &PackedTask, optimize: bool, threads: usize, cfg: SearchCfg, ops: &mut Vec<usize>, orbit: Option<&crate::orbits::OrbitMap>, ) -> Option<(f64, &'static str)> |  |  |  |  |

| `plan_cost` | function | plan_cost(task: &PackedTask, cf: usize, ops: &[usize]) -> Option<f64> |  |  |  |  |

| `CostOutcome` | struct | CostOutcome { pub ops: Vec<usize>, pub cost: f64, pub improved: bool, pub proven: bool, pub evaluated: usize } |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> |  |  |  |  |

| `(define (domain g) (:requirements :typing :adl)
      (:types node)
      (:predicates (link ?a ?b - node) (reachable ?a ?b - node) (at ?n - node) (visited ?n - node))
      (:derived (reachable ?a ?b - node)
        (or (link ?a ?b)
            (exists (?c - node) (and (link ?a ?c) (reachable ?c ?b)))))
      (:action go :parameters (?from ?to - node)
        :precondition (and (at ?from) (reachable ?from ?to))
        :effect (and (not (at ?from)) (at ?to) (visited ?to))))` | str_key | DOM = "(define (domain g) (:requirements :typing :adl)
      (:types node)
      (:predicates (link ?a ?b - node) (reachable ?a ?b - node) (at ?n - node) (visited ?n - node))
      (:derived (reachable ?a ?b - node)
        (or (link ?a ?b)
            (exists (?c - node) (and (link ?a ?c) (reachable ?c ?b)))))
      (:action go :parameters (?from ?to - node)
        :precondition (and (at ?from) (reachable ?from ?to))
        :effect (and (not (at ?from)) (at ?to) (visited ?to))))" |  |  |  |  |

| `FF_ESPC_MONO` | env_key | std::env::var("FF_ESPC_MONO") |  |  |  |  |

| `FF_ESPC_TIME_MS` | env_key | std::env::var("FF_ESPC_TIME_MS") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `espc_optimize` | function | espc_optimize( task: &PackedTask, cost_fluent: usize, sat: &mut SatGuidance, seed: Option<(Vec<usize>, f64)>, part: Option<EspcPartition>, threads: usize, cfg: SearchCfg, ) -> Option<EspcResult> |  |  |  |  |

| `EspcPartition` | struct | EspcPartition { pub comps: Vec<Subgoal>, pub tail: PhaseTail, pub assoc: FxHashMap<u32, Vec<u32>> } |  |  |  |  |

| `EspcResult` | struct | EspcResult { pub ops: Vec<usize>, pub cost: f64, pub iterations: usize } |  |  |  |  |

| `MAX_PRIMARY_ACTIVATORS` | const | MAX_PRIMARY_ACTIVATORS: usize |  |  |  |  |

| `EveError` | enum | EveError { Missing { field: String }, SplitRequired { directive: SplitDirective } } |  |  |  |  |

| `EveStage` | enum | EveStage { GroundHumanPurpose, ProjectGenesis, DecomposeHddl, GovernUncertaintyPpddl, ManufactureGgen, ExposeMcpPlus, ActuateBrce, ObserveOcel2, ConformTruexKernel, AdmitReceipt, ReplayTruex } |  |  |  |  |

| `PlanningRegime` | enum | PlanningRegime { Deterministic, Probabilistic } |  |  |  |  |

| `enter` | function | enter(request: EveRequest) -> Result<EveHandoff, EveError> |  |  |  |  |

| `ferroplan.eve-genesis.v1` | str_key | EVE_PROTOCOL = "ferroplan.eve-genesis.v1" |  |  |  |  |

| `Activator` | struct | Activator { pub name: String, pub value: String } |  |  |  |  |

| `CapabilityTarget` | struct | CapabilityTarget { pub capability: String, pub route: String, pub authority_scopes: Vec<String> } |  |  |  |  |

| `Eve` | struct |  |  |  |  |  |

| `EveHandoff` | struct | EveHandoff { pub protocol: String, pub closure_id: String, pub planning_regime: PlanningRegime, pub stages: Vec<EveStage>, pub goal: GroundedGoal, pub genesis: GenesisProjection, pub hddl: HddlDecompositionRequest, pub ppddl: Option<PpddlPolicyRequest>, pub ggen: GgenManufacturingRequest, pub mcp_plus: McpPlusHandoff, pub truex: TruexContinuation } |  |  |  |  |

| `EveRequest` | struct | EveRequest { pub purpose: HumanPurpose, pub genesis: GenesisWorld, pub manufacture: ManufactureTarget, pub capability: CapabilityTarget } |  |  |  |  |

| `GenesisProjection` | struct | GenesisProjection { pub ontology_rdf: String, pub construct_query: String } |  |  |  |  |

| `GenesisWorld` | struct | GenesisWorld { pub ontology_rdf: String, pub construct_query: String, pub hddl: HddlSurface, pub ppddl: Option<PpddlSurface> } |  |  |  |  |

| `GgenManufacturingRequest` | struct | GgenManufacturingRequest { pub target: ManufactureTarget, pub closure_id: String, pub candidate_only: bool } |  |  |  |  |

| `GroundedGoal` | struct | GroundedGoal { pub statement: String, pub desired_consequence: String, pub actor: Option<String>, pub root_task: String, pub activators: Vec<Activator> } |  |  |  |  |

| `HddlDecompositionRequest` | struct | HddlDecompositionRequest { pub domain: String, pub problem: String, pub root_task: String } |  |  |  |  |

| `HddlSurface` | struct | HddlSurface { pub domain: String, pub problem: String, pub root_task: String } |  |  |  |  |

| `HumanPurpose` | struct | HumanPurpose { pub statement: String, pub desired_consequence: String, pub actor: Option<String>, pub activators: Vec<Activator> } |  |  |  |  |

| `ManufactureTarget` | struct | ManufactureTarget { pub name: String, pub template: String, pub artifact_kind: String, pub output: String } |  |  |  |  |

| `McpPlusHandoff` | struct | McpPlusHandoff { pub target: CapabilityTarget, pub closure_id: String, pub ambient_authority: bool, pub brce_required: bool, pub receipt_obligations: Vec<String> } |  |  |  |  |

| `PpddlPolicyRequest` | struct | PpddlPolicyRequest { pub domain: String, pub problem: String } |  |  |  |  |

| `PpddlSurface` | struct | PpddlSurface { pub domain: String, pub problem: String } |  |  |  |  |

| `SplitDirective` | struct | SplitDirective { pub provided: usize, pub maximum: usize, pub groups: Vec<Vec<Activator>> } |  |  |  |  |

| `TruexContinuation` | struct | TruexContinuation { pub expected_process_geometry: String, pub observed_path_format: String, pub conformance_engine: String, pub terminal_authority: String, pub replay_required: bool } |  |  |  |  |

| `DemandMode` | enum | DemandMode { Off, Numeric, Full } |  |  |  |  |

| `FF_NO_ESCALATE` | env_key | std::env::var("FF_NO_ESCALATE") |  |  |  |  |

| `FF_NO_ESPC` | env_key | std::env::var("FF_NO_ESPC") |  |  |  |  |

| `FF_NO_TDEMAND` | env_key | std::env::var("FF_NO_TDEMAND") |  |  |  |  |

| `FF_TCONC` | env_key | std::env::var("FF_TCONC") |  |  |  |  |

| `FF_TDECOMP` | env_key | std::env::var("FF_TDECOMP") |  |  |  |  |

| `FF_TDEMAND` | env_key | std::env::var("FF_TDEMAND") |  |  |  |  |

| `clear_overrides` | function | clear_overrides() |  |  |  |  |

| `demand_mode` | function | demand_mode() -> DemandMode |  |  |  |  |

| `escalate` | function | escalate() -> bool |  |  |  |  |

| `espc` | function | espc() -> bool |  |  |  |  |

| `set_escalate_override` | function | set_escalate_override(on: bool) |  |  |  |  |

| `set_espc_override` | function | set_espc_override(on: bool) |  |  |  |  |

| `set_overrides` | function | set_overrides(tdemand: bool, tdecomp: bool, tconc: bool) |  |  |  |  |

| `tconc` | function | tconc() -> bool |  |  |  |  |

| `tdecomp` | function | tdecomp() -> bool |  |  |  |  |

| `tdemand` | function | tdemand() -> bool |  |  |  |  |

| `Outcome` | enum | Outcome { Task(PackedTask), GoalTrue, GoalFalse(String), GoalUndefinedFluent(String), EmptyType { kind: &'static str, pred: String, ty: String, }, WallExhausted(String) } |  |  |  |  |

| `FF_GROUND_PHASES` | env_key | std::env::var("FF_GROUND_PHASES") |  |  |  |  |

| `FF_NO_DNF_STATIC` | env_key | std::env::var("FF_NO_DNF_STATIC") |  |  |  |  |

| `FF_NO_FACT_COMPACT` | env_key | std::env::var("FF_NO_FACT_COMPACT") |  |  |  |  |

| `FF_NO_FIXPOINT_GROUND` | env_key | std::env::var("FF_NO_FIXPOINT_GROUND") |  |  |  |  |

| `FF_NO_FLUENT_COMPACT` | env_key | std::env::var("FF_NO_FLUENT_COMPACT") |  |  |  |  |

| `FF_NO_FLUENT_FOLD` | env_key | std::env::var("FF_NO_FLUENT_FOLD") |  |  |  |  |

| `FF_NO_GOAL_FACTOR` | env_key | std::env::var("FF_NO_GOAL_FACTOR") |  |  |  |  |

| `FF_NO_JOIN_INDEX` | env_key | std::env::var("FF_NO_JOIN_INDEX") |  |  |  |  |

| `FF_NO_MCV_JOIN` | env_key | std::env::var("FF_NO_MCV_JOIN") |  |  |  |  |

| `FF_NO_STRAT_GROUND` | env_key | std::env::var("FF_NO_STRAT_GROUND") |  |  |  |  |

| `FF_NUMPRE_TEMPORAL` | env_key | std::env::var("FF_NUMPRE_TEMPORAL") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `ground` | function | ground(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_fixpoint` | function | ground_fixpoint(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_stratified` | function | ground_stratified(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_stratified_walled` | function | ground_stratified_walled(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_task` | function | ground_task(domain: &Domain, problem: &Problem, threads: usize) -> Option<PackedTask> |  |  |  |  |

| `initial_state` | function | initial_state(t: &PackedTask) -> State |  |  |  |  |

| `objects_by_type` | function | objects_by_type(domain: &Domain, problem: &Problem) -> HashMap<Sym, Vec<Sym>> |  |  |  |  |

| crate::packed::PackedTask as Task (alias re-export; see prose note) | use | crate::packed::PackedTask as Task |  |  |  |  |

| `FxHasher` | struct | FxHasher { hash: u64 } |  |  |  |  |

| `HddlError` | enum | HddlError { Parse(String), Ground(String), Translate(String), Planner(PlannerError), RootTaskMismatch { root_task: String, problem_root_network: Vec<String>, }, Timeout { elapsed_ms: u128, limit_ms: u128, }, WorkerPanicked(String) } |  |  |  |  |

| `adapt_problem` | function | adapt_problem(p: ferroplan_hddl::translate::PlanningProblem) -> PlanningProblem |  |  |  |  |

| `solve_hddl` | function | solve_hddl( domain_src: &str, problem_src: &str, limits: &PlannerLimits, ) -> Result<UniversalPlan, HddlError> |  |  |  |  |

| `solve_hddl_from_eve` | function | solve_hddl_from_eve( handoff: &EveHandoff, limits: &PlannerLimits, ) -> Result<UniversalPlan, HddlError> |  |  |  |  |

| `(define (domain coin)
  (:predicates (heads) (tails))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof (heads) (tails)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))` | str_key | DOMAIN = "(define (domain coin)
  (:predicates (heads) (tails))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof (heads) (tails)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))" |  |  |  |  |

| `(define (domain coin-empty)
  (:predicates (heads))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof () (heads)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))` | str_key | DOMAIN = "(define (domain coin-empty)
  (:predicates (heads))
  (:task go :parameters ())
  (:action toss
    :parameters ()
    :precondition ()
    :effect (oneof () (heads)))
  (:method m-go
    :task (go)
    :ordered-subtasks (and (t1 (toss)))))" |  |  |  |  |

| `(define (problem coin-empty-p1)
  (:domain coin-empty)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))` | str_key | PROBLEM = "(define (problem coin-empty-p1)
  (:domain coin-empty)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal ()))" |  |  |  |  |

| `(define (problem coin-p1)
  (:domain coin)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal (or (heads) (tails))))` | str_key | PROBLEM = "(define (problem coin-p1)
  (:domain coin)
  (:objects)
  (:htn :parameters () :ordered-subtasks (and (g1 (go))))
  (:init)
  (:goal (or (heads) (tails))))" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/a/domain.hddl` | str_key | FIXTURE_A_DOMAIN = "../../ferroplan-hddl/fixtures/a/domain.hddl" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/a/problem.hddl` | str_key | FIXTURE_A_PROBLEM = "../../ferroplan-hddl/fixtures/a/problem.hddl" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/c/domain.hddl` | str_key | FIXTURE_C_DOMAIN = "../../ferroplan-hddl/fixtures/c/domain.hddl" |  |  |  |  |

| `../../ferroplan-hddl/fixtures/c/problem.hddl` | str_key | FIXTURE_C_PROBLEM = "../../ferroplan-hddl/fixtures/c/problem.hddl" |  |  |  |  |

| `T_BUILD` | const | T_BUILD: std::sync::atomic::AtomicU64 |  |  |  |  |

| `T_EXTRACT` | const | T_EXTRACT: std::sync::atomic::AtomicU64 |  |  |  |  |

| `T_RESET` | const | T_RESET: std::sync::atomic::AtomicU64 |  |  |  |  |

| `FF_NO_NEED_DIRS` | env_key | std::env::var("FF_NO_NEED_DIRS") |  |  |  |  |

| `FF_NO_NUMH` | env_key | std::env::var("FF_NO_NUMH") |  |  |  |  |

| `FF_NO_NUMPRE` | env_key | std::env::var("FF_NO_NUMPRE") |  |  |  |  |

| `FF_NO_NUMPRE_CHAIN` | env_key | std::env::var("FF_NO_NUMPRE_CHAIN") |  |  |  |  |

| `FF_NUMPRE_DEPTH` | env_key | std::env::var("FF_NUMPRE_DEPTH") |  |  |  |  |

| `FF_NUMPRE_NODAMP` | env_key | std::env::var("FF_NUMPRE_NODAMP") |  |  |  |  |

| `FF_NUMPRE_NOSKIP` | env_key | std::env::var("FF_NUMPRE_NOSKIP") |  |  |  |  |

| `FF_NUMPRE_NOSUM` | env_key | std::env::var("FF_NUMPRE_NOSUM") |  |  |  |  |

| `extraction_need_facts` | function | extraction_need_facts(sc: &Scratch) -> Vec<(u32, u32)> |  |  |  |  |

| `helpful_needed_adders` | function | helpful_needed_adders( task: &PackedTask, sc: &Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> Vec<u32> |  |  |  |  |

| `new` | function | new(task: &PackedTask) -> Self |  |  |  |  |

| `reachability_layers` | function | reachability_layers( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> (Vec<u32>, Vec<u32>) |  |  |  |  |

| `relaxed` | function | relaxed( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> Option<i32> |  |  |  |  |

| `relaxed_costed` | function | relaxed_costed( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, ) -> Option<i32> |  |  |  |  |

| `relaxed_helpful` | function | relaxed_helpful( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<(i32, Vec<u32>)> |  |  |  |  |

| `relaxed_plan_cost` | function | relaxed_plan_cost( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, ) -> Option<f64> |  |  |  |  |

| `relaxed_to` | function | relaxed_to( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<i32> |  |  |  |  |

| `
    (define (domain watering-mini)
      (:requirements :typing :numeric-fluents)
      (:types agent plant - object)
      (:functions (x ?o - object) (carrying ?a - agent)
                  (poured ?p - plant) (maxx))
      (:action move_right :parameters (?a - agent)
        :precondition (<= (+ (x ?a) 1) (maxx))
        :effect (increase (x ?a) 1))
      (:action move_left :parameters (?a - agent)
        :precondition (>= (- (x ?a) 1) 0)
        :effect (decrease (x ?a) 1))
      (:action pour :parameters (?a - agent ?p - plant)
        :precondition (and (= (x ?a) (x ?p)) (>= (carrying ?a) 1))
        :effect (and (decrease (carrying ?a) 1) (increase (poured ?p) 1))))` | str_key | WATER_DOM = "
    (define (domain watering-mini)
      (:requirements :typing :numeric-fluents)
      (:types agent plant - object)
      (:functions (x ?o - object) (carrying ?a - agent)
                  (poured ?p - plant) (maxx))
      (:action move_right :parameters (?a - agent)
        :precondition (<= (+ (x ?a) 1) (maxx))
        :effect (increase (x ?a) 1))
      (:action move_left :parameters (?a - agent)
        :precondition (>= (- (x ?a) 1) 0)
        :effect (decrease (x ?a) 1))
      (:action pour :parameters (?a - agent ?p - plant)
        :precondition (and (= (x ?a) (x ?p)) (>= (carrying ?a) 1))
        :effect (and (decrease (carrying ?a) 1) (increase (poured ?p) 1))))" |  |  |  |  |

| `
    (define (problem watering-mini-1) (:domain watering-mini)
      (:objects a1 - agent pa pb - plant)
      (:init (= (x a1) 0) (= (x pa) 1) (= (x pb) 9)
             (= (carrying a1) 5) (= (poured pa) 0) (= (poured pb) 0)
             (= (maxx) 12))
      (:goal (and (= (poured pa) 1) (= (poured pb) 1))))` | str_key | WATER_PRB = "
    (define (problem watering-mini-1) (:domain watering-mini)
      (:objects a1 - agent pa pb - plant)
      (:init (= (x a1) 0) (= (x pa) 1) (= (x pb) 9)
             (= (carrying a1) 5) (= (poured pa) 0) (= (poured pb) 0)
             (= (maxx) 12))
      (:goal (and (= (poured pa) 1) (= (poured pb) 1))))" |  |  |  |  |

| `(define (domain drain)
      (:requirements :fluents)
      (:predicates (there) (idle))
      (:functions (energy))
      (:action drive :parameters ()
        :precondition (>= (energy) 8)
        :effect (and (there) (decrease (energy) 8)))
      (:action wander :parameters ()
        :precondition (idle)
        :effect (decrease (energy) 1)))` | str_key | DRAIN_DOM = "(define (domain drain)
      (:requirements :fluents)
      (:predicates (there) (idle))
      (:functions (energy))
      (:action drive :parameters ()
        :precondition (>= (energy) 8)
        :effect (and (there) (decrease (energy) 8)))
      (:action wander :parameters ()
        :precondition (idle)
        :effect (decrease (energy) 1)))" |  |  |  |  |

| `(define (problem d1) (:domain drain)
      (:init (idle) (= (energy) 5)) (:goal (there)))` | str_key | DRAIN_PRB = "(define (problem d1) (:domain drain)
      (:init (idle) (= (energy) 5)) (:goal (there)))" |  |  |  |  |

| `Scratch` | struct | Scratch { reached: Vec<bool>, fact_layer: Vec<u32>, op_layer: Vec<u32>, gen: u32, op_stamp: Vec<u32>, applicable: Vec<u32>, lb: Vec<f64>, ub: Vec<f64>, selected: Vec<u32>, need_fact: Vec<u32>, queue: Vec<u32>, num_applied: Vec<u32>, cond_ops: Vec<u32>, helpful: Vec<u32>, fact_time: Vec<f64>, op_time: Vec<f64> } |  |  |  |  |

| `TrpgInfo` | struct | TrpgInfo { pub start_of: Vec<u32>, pub lag: Vec<f64>, pub floor: Vec<f64>, pub windows: Vec<Vec<TrpgWindow>> } |  |  |  |  |

| `TrpgWindow` | struct | TrpgWindow { pub fact: u32, pub providers: Vec<(u32, f64)>, pub close: f64 } |  |  |  |  |

| `explain` | function | explain(domain_src: &str, problem_src: &str, plan: &Plan) -> Result<Explanation, String> |  |  |  |  |

| `(define (domain chain)
      (:requirements :strips)
      (:predicates (a) (b) (c) (d))
      (:action MK-B :parameters () :precondition (a) :effect (b))
      (:action MK-C :parameters () :precondition (b) :effect (c)))` | str_key | CHAIN_DOM = "(define (domain chain)
      (:requirements :strips)
      (:predicates (a) (b) (c) (d))
      (:action MK-B :parameters () :precondition (a) :effect (b))
      (:action MK-C :parameters () :precondition (b) :effect (c)))" |  |  |  |  |

| `(define (problem chain-1) (:domain chain)
      (:init (a)) (:goal (c)))` | str_key | CHAIN_PRB = "(define (problem chain-1) (:domain chain)
      (:init (a)) (:goal (c)))" |  |  |  |  |

| `CausalLink` | struct | CausalLink { pub provider: Option<usize>, pub consumer: usize, pub fact: String } |  |  |  |  |

| `Explanation` | struct | Explanation { pub kind: String, pub causal_links: Vec<CausalLink>, pub invariant_spans: Vec<InvariantSpan>, pub preferences: Vec<PrefReport> } |  |  |  |  |

| `InvariantSpan` | struct | InvariantSpan { pub step: usize, pub action: String, pub start: f64, pub end: f64, pub conditions: Vec<String> } |  |  |  |  |

| `PrefReport` | struct | PrefReport { pub name: String, pub satisfied: bool, pub weight: f64 } |  |  |  |  |

| `synthesize` | function | synthesize(domain: &Domain, task: &PackedTask) -> Vec<Vec<u32>> |  |  |  |  |

| `(define (domain blocks)
      (:requirements :strips :typing)
      (:types block)
      (:predicates (on ?x ?y - block) (ontable ?x - block) (clear ?x - block)
                   (handempty) (holding ?x - block))
      (:action pickup :parameters (?x - block)
        :precondition (and (clear ?x) (ontable ?x) (handempty))
        :effect (and (not (ontable ?x)) (not (clear ?x)) (not (handempty)) (holding ?x)))
      (:action putdown :parameters (?x - block)
        :precondition (holding ?x)
        :effect (and (not (holding ?x)) (clear ?x) (handempty) (ontable ?x)))
      (:action stack :parameters (?x ?y - block)
        :precondition (and (holding ?x) (clear ?y))
        :effect (and (not (holding ?x)) (not (clear ?y)) (clear ?x) (handempty) (on ?x ?y)))
      (:action unstack :parameters (?x ?y - block)
        :precondition (and (on ?x ?y) (clear ?x) (handempty))
        :effect (and (holding ?x) (clear ?y) (not (clear ?x)) (not (on ?x ?y)) (not (handempty)))))` | str_key | BLOCKS = "(define (domain blocks)
      (:requirements :strips :typing)
      (:types block)
      (:predicates (on ?x ?y - block) (ontable ?x - block) (clear ?x - block)
                   (handempty) (holding ?x - block))
      (:action pickup :parameters (?x - block)
        :precondition (and (clear ?x) (ontable ?x) (handempty))
        :effect (and (not (ontable ?x)) (not (clear ?x)) (not (handempty)) (holding ?x)))
      (:action putdown :parameters (?x - block)
        :precondition (holding ?x)
        :effect (and (not (holding ?x)) (clear ?x) (handempty) (ontable ?x)))
      (:action stack :parameters (?x ?y - block)
        :precondition (and (holding ?x) (clear ?y))
        :effect (and (not (holding ?x)) (not (clear ?y)) (clear ?x) (handempty) (on ?x ?y)))
      (:action unstack :parameters (?x ?y - block)
        :precondition (and (on ?x ?y) (clear ?x) (handempty))
        :effect (and (holding ?x) (clear ?y) (not (clear ?x)) (not (on ?x ?y)) (not (handempty)))))" |  |  |  |  |

| `(define (domain gripper)
      (:requirements :strips :typing)
      (:types room ball)
      (:predicates (at-robby ?r - room) (ball-at ?b - ball ?r - room) (carry ?b - ball))
      (:action move :parameters (?from ?to - room)
        :precondition (at-robby ?from)
        :effect (and (not (at-robby ?from)) (at-robby ?to)))
      (:action pick :parameters (?b - ball ?r - room)
        :precondition (and (ball-at ?b ?r) (at-robby ?r))
        :effect (and (not (ball-at ?b ?r)) (carry ?b)))
      (:action drop :parameters (?b - ball ?r - room)
        :precondition (and (carry ?b) (at-robby ?r))
        :effect (and (not (carry ?b)) (ball-at ?b ?r))))` | str_key | GRIPPER = "(define (domain gripper)
      (:requirements :strips :typing)
      (:types room ball)
      (:predicates (at-robby ?r - room) (ball-at ?b - ball ?r - room) (carry ?b - ball))
      (:action move :parameters (?from ?to - room)
        :precondition (at-robby ?from)
        :effect (and (not (at-robby ?from)) (at-robby ?to)))
      (:action pick :parameters (?b - ball ?r - room)
        :precondition (and (ball-at ?b ?r) (at-robby ?r))
        :effect (and (not (ball-at ?b ?r)) (carry ?b)))
      (:action drop :parameters (?b - ball ?r - room)
        :precondition (and (carry ?b) (at-robby ?r))
        :effect (and (not (carry ?b)) (ball-at ?b ?r))))" |  |  |  |  |

| `(define (domain log)
      (:requirements :strips :typing)
      (:types vehicle package location)
      (:predicates (at ?x - object ?l - location) (in ?p - package ?v - vehicle)
                   (road ?a ?b - location))
      (:action drive :parameters (?v - vehicle ?from ?to - location)
        :precondition (and (at ?v ?from) (road ?from ?to))
        :effect (and (not (at ?v ?from)) (at ?v ?to)))
      (:action load :parameters (?p - package ?v - vehicle ?l - location)
        :precondition (and (at ?p ?l) (at ?v ?l))
        :effect (and (not (at ?p ?l)) (in ?p ?v)))
      (:action unload :parameters (?p - package ?v - vehicle ?l - location)
        :precondition (and (in ?p ?v) (at ?v ?l))
        :effect (and (not (in ?p ?v)) (at ?p ?l))))` | str_key | LOGISTICS = "(define (domain log)
      (:requirements :strips :typing)
      (:types vehicle package location)
      (:predicates (at ?x - object ?l - location) (in ?p - package ?v - vehicle)
                   (road ?a ?b - location))
      (:action drive :parameters (?v - vehicle ?from ?to - location)
        :precondition (and (at ?v ?from) (road ?from ?to))
        :effect (and (not (at ?v ?from)) (at ?v ?to)))
      (:action load :parameters (?p - package ?v - vehicle ?l - location)
        :precondition (and (at ?p ?l) (at ?v ?l))
        :effect (and (not (at ?p ?l)) (in ?p ?v)))
      (:action unload :parameters (?p - package ?v - vehicle ?l - location)
        :precondition (and (in ?p ?v) (at ?v ?l))
        :effect (and (not (in ?p ?v)) (at ?p ?l))))" |  |  |  |  |

| `(define (problem p) (:domain blocks)
      (:objects a b c - block)
      (:init (ontable a) (on b a) (on c b) (clear c) (handempty))
      (:goal (on a b)))` | str_key | BLOCKS_PROB = "(define (problem p) (:domain blocks)
      (:objects a b c - block)
      (:init (ontable a) (on b a) (on c b) (clear c) (handempty))
      (:goal (on a b)))" |  |  |  |  |

| `(define (problem p) (:domain gripper)
      (:objects ra rb - room b1 - ball)
      (:init (at-robby ra) (ball-at b1 ra))
      (:goal (at-robby rb)))` | str_key | GRIP_PROB = "(define (problem p) (:domain gripper)
      (:objects ra rb - room b1 - ball)
      (:init (at-robby ra) (ball-at b1 ra))
      (:goal (at-robby rb)))" |  |  |  |  |

| `(define (problem p) (:domain log)
      (:objects v1 - vehicle pk1 - package a b c - location)
      (:init (at v1 a) (at pk1 b) (road a b) (road b c) (road a c))
      (:goal (at pk1 c)))` | str_key | LOG_PROB = "(define (problem p) (:domain log)
      (:objects v1 - vehicle pk1 - package a b c - location)
      (:init (at v1 a) (at pk1 b) (road a b) (road b c) (road a c))
      (:goal (at pk1 c)))" |  |  |  |  |

| `FF_LAMA_EXT_ARRIVAL` | env_key | std::env::var("FF_LAMA_EXT_ARRIVAL") |  |  |  |  |

| `FF_LEN_ANYTIME` | env_key | std::env::var("FF_LEN_ANYTIME") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `search` | function | search( task: &PackedTask, threads: usize, max_eval: usize, forbidden: &[bool], slice: Option<(crate::clock::Clock, f64)>, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `search_subgoal` | function | search_subgoal( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[crate::types::NumPre], threads: usize, max_eval: usize, forbidden: &[bool], len_anytime: bool, slice: Option<(crate::clock::Clock, f64)>, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `goal_landmarks` | function | goal_landmarks(task: &PackedTask) -> Vec<u32> |  |  |  |  |

| `landmarks_for` | function | landmarks_for( task: &PackedTask, start: &crate::packed::State, goal_pos: &[u32], ) -> Vec<u32> |  |  |  |  |

| `Tok` | enum | Tok { LParen, RParen, Dash, Var(String), Name(String), Num(f64), Op(String) } |  |  |  |  |

| `lex` | function | lex(input: &str) -> Result<(Vec<Tok>, Vec<u32>), crate::types::ParseError> |  |  |  |  |

| `api::{ decompose, parse, solve, Contract, Decomposition, DomainSummary, Metric, Mode, Options, ParseReport, Plan, ProblemSummary, Search, Solution, SolveError, Statistics, Step, }` | use | api::{ decompose, parse, solve, Contract, Decomposition, DomainSummary, Metric, Mode, Options, ParseReport, Plan, ProblemSummary, Search, Solution, SolveError, Statistics, Step, } |  |  |  |  |

| `eve::{ Activator, CapabilityTarget, Eve, EveError, EveHandoff, EveRequest, EveStage, GenesisProjection, GenesisWorld, GgenManufacturingRequest, GroundedGoal, HddlDecompositionRequest, HddlSurface, HumanPurpose, ManufactureTarget, McpPlusHandoff, PlanningRegime, PpddlPolicyRequest, PpddlSurface, SplitDirective, TruexContinuation, MAX_PRIMARY_ACTIVATORS, }` | use | eve::{ Activator, CapabilityTarget, Eve, EveError, EveHandoff, EveRequest, EveStage, GenesisProjection, GenesisWorld, GgenManufacturingRequest, GroundedGoal, HddlDecompositionRequest, HddlSurface, HumanPurpose, ManufactureTarget, McpPlusHandoff, PlanningRegime, PpddlPolicyRequest, PpddlSurface, SplitDirective, TruexContinuation, MAX_PRIMARY_ACTIVATORS, } |  |  |  |  |

| `hddl::{solve_hddl, HddlError}` | use | hddl::{solve_hddl, HddlError} |  |  |  |  |

| `operator_compiler::{ compile_operator, CompiledOperator, OperatorCompileError, OperatorEffects, OperatorSpec, }` | use | operator_compiler::{ compile_operator, CompiledOperator, OperatorCompileError, OperatorEffects, OperatorSpec, } |  |  |  |  |

| `planner::{run_ff, run_planner}` | use | planner::{run_ff, run_planner} |  |  |  |  |

| `planning_runtime::{ solve_planning_type, Agent, Goal as UniversalGoal, Method as PlanningMethod, PlanStep, PlannerError, PlannerLimits, PlanningProblem, PolicyEntry as UniversalPolicyEntry, PolicyOutcome as UniversalPolicyOutcome, QueueState, RdfTriple, State as UniversalState, Task as PlanningTask, Tool, Transition as UniversalTransition, UniversalPlan, UniversalPlanningRequest, WorkflowEdge, }` | use | planning_runtime::{ solve_planning_type, Agent, Goal as UniversalGoal, Method as PlanningMethod, PlanStep, PlannerError, PlannerLimits, PlanningProblem, PolicyEntry as UniversalPolicyEntry, PolicyOutcome as UniversalPolicyOutcome, QueueState, RdfTriple, State as UniversalState, Task as PlanningTask, Tool, Transition as UniversalTransition, UniversalPlan, UniversalPlanningRequest, WorkflowEdge, } |  |  |  |  |

| `planning_types::{ route_planning_request, PlanningCapability, PlanningRail, PlanningRequest, PlanningRoute, PlanningRouteError, PlanningType, }` | use | planning_types::{ route_planning_request, PlanningCapability, PlanningRail, PlanningRequest, PlanningRoute, PlanningRouteError, PlanningType, } |  |  |  |  |

| `policy_validation::{ validate_fond_policy, PolicyGuarantee, PolicyIssue, PolicyValidationReport, }` | use | policy_validation::{ validate_fond_policy, PolicyGuarantee, PolicyIssue, PolicyValidationReport, } |  |  |  |  |

| `ppddl::{ parse_ppddl, simulate_ppddl, solve_ppddl, validate_ppddl_policy, InitialStateProbability, PolicyDecision, PolicyOutcome, PolicyValidation, PpddlError, PpddlParseReport, ProbabilisticObjective, ProbabilisticOptions, ProbabilisticSolution, ProbabilisticState, ProbabilisticStatistics, SimulationReport, }` | use | ppddl::{ parse_ppddl, simulate_ppddl, solve_ppddl, validate_ppddl_policy, InitialStateProbability, PolicyDecision, PolicyOutcome, PolicyValidation, PpddlError, PpddlParseReport, ProbabilisticObjective, ProbabilisticOptions, ProbabilisticSolution, ProbabilisticState, ProbabilisticStatistics, SimulationReport, } |  |  |  |  |

| `production::{ parse_production, solve_ppddl_production, trace_production, validate_plan_production, PlanValidationEvidence, ProductionSession, }` | use | production::{ parse_production, solve_ppddl_production, trace_production, validate_plan_production, PlanValidationEvidence, ProductionSession, } |  |  |  |  |

| `production_explain::{decompose_production, explain_production}` | use | production_explain::{decompose_production, explain_production} |  |  |  |  |

| `readiness::{ capability_manifest, evaluate_readiness, production_input_fingerprint, solve_production, AuthorityClass, BuildIdentity, CapabilityContract, CapabilityEvaluation, CapabilityManifest, CompatibilityClass, DeterminismClass, InterfaceKind, ManifestError, OperationEnvelope, OutcomeClass, ProductionLimits, PublicError, ReadinessReport, ReadinessState, ReplayClass, SecurityClass, ValidationStatus, CANDIDATE_AUTHORITY, CAPABILITY_MANIFEST_SCHEMA, OPERATION_ENVELOPE_SCHEMA, }` | use | readiness::{ capability_manifest, evaluate_readiness, production_input_fingerprint, solve_production, AuthorityClass, BuildIdentity, CapabilityContract, CapabilityEvaluation, CapabilityManifest, CompatibilityClass, DeterminismClass, InterfaceKind, ManifestError, OperationEnvelope, OutcomeClass, ProductionLimits, PublicError, ReadinessReport, ReadinessState, ReplayClass, SecurityClass, ValidationStatus, CANDIDATE_AUTHORITY, CAPABILITY_MANIFEST_SCHEMA, OPERATION_ENVELOPE_SCHEMA, } |  |  |  |  |

| `session::{Session, Think, ThinkBudget, ThinkVerdict}` | use | session::{Session, Think, ThinkBudget, ThinkVerdict} |  |  |  |  |

| `trace::{trace, StateSnapshot}` | use | trace::{trace, StateSnapshot} |  |  |  |  |

| `types::ParseError` | use | types::ParseError |  |  |  |  |

| `FF_MEM_BUDGET_GB` | env_key | std::env::var("FF_MEM_BUDGET_GB") |  |  |  |  |

| `FF_MEM_TRIP_FRAC` | env_key | std::env::var("FF_MEM_TRIP_FRAC") |  |  |  |  |

| `FF_NO_MEM_WALL` | env_key | std::env::var("FF_NO_MEM_WALL") |  |  |  |  |

| `arm` | function | arm() -> Self |  |  |  |  |

| `armed` | function | armed(&self) -> bool |  |  |  |  |

| `declared_budget_bytes` | function | declared_budget_bytes() -> Option<u64> |  |  |  |  |

| `hit` | function | hit(&self) -> bool |  |  |  |  |

| `latch` | function | latch() |  |  |  |  |

| `latched` | function | latched() -> bool |  |  |  |  |

| `peak_resident_bytes` | function | peak_resident_bytes() -> Option<u64> |  |  |  |  |

| `resident_bytes` | function | resident_bytes() -> Option<u64> |  |  |  |  |

| `unarmed` | function | unarmed() -> Self |  |  |  |  |

| `MemWall` | struct | MemWall { trip_at: Option<u64> } |  |  |  |  |

| `FF_NOV_R_CAP` | env_key | std::env::var("FF_NOV_R_CAP") |  |  |  |  |

| `FF_NUMNOV` | env_key | std::env::var("FF_NUMNOV") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `from_env` | function | from_env() -> Self |  |  |  |  |

| `r_partition_facts` | function | r_partition_facts( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[crate::types::NumPre], r_cap: usize, ) -> Vec<u32> |  |  |  |  |

| `search` | function | search( task: &PackedTask, threads: usize, max_eval: usize, forbidden: &[bool], slice: Option<(crate::clock::Clock, f64)>, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `search_driver` | function | search_driver( task: &PackedTask, max_eval: usize, forbidden: &[bool], slice: Option<(crate::clock::Clock, f64)>, cfg: &DriverCfg, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `search_light` | function | search_light( task: &PackedTask, max_eval: usize, forbidden: &[bool], ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `search_subgoal` | function | search_subgoal( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[crate::types::NumPre], threads: usize, max_eval: usize, forbidden: &[bool], slice: Option<(crate::clock::Clock, f64)>, ) -> Option<(Vec<usize>, usize)> |  |  |  |  |

| `DriverCfg` | struct | DriverCfg { pub partition: bool, pub width2: bool, pub r_cap: usize } |  |  |  |  |

| `OperatorCompileError` | enum | OperatorCompileError { EmptyName, ConflictingEffect(String), NoApplicableState, MissingTargetState { from: String, facts: BTreeSet<String>, }, AmbiguousTargetState { from: String, candidates: Vec<String>, } } |  |  |  |  |

| `compile_operator` | function | compile_operator( spec: &OperatorSpec, states: &[State], ) -> Result<CompiledOperator, OperatorCompileError> |  |  |  |  |

| `CompiledOperator` | struct | CompiledOperator { pub task: Task, pub transitions: Vec<Transition> } |  |  |  |  |

| `OperatorEffects` | struct | OperatorEffects { pub add: BTreeSet<String>, pub delete: BTreeSet<String> } |  |  |  |  |

| `OperatorSpec` | struct | OperatorSpec { pub name: String, pub preconditions: BTreeSet<String>, pub effects: OperatorEffects, pub cost: u64 } |  |  |  |  |

| `FF_NO_HMAX_SPRINT` | env_key | std::env::var("FF_NO_HMAX_SPRINT") |  |  |  |  |

| `FF_NO_INC_LMCUT` | env_key | std::env::var("FF_NO_INC_LMCUT") |  |  |  |  |

| `FF_NO_LMCUT` | env_key | std::env::var("FF_NO_LMCUT") |  |  |  |  |

| `FF_NO_NODECAP_REFILL` | env_key | std::env::var("FF_NO_NODECAP_REFILL") |  |  |  |  |

| `FF_OPT_GATE_MARGIN` | env_key | std::env::var("FF_OPT_GATE_MARGIN") |  |  |  |  |

| `FF_OPT_NO_NUMFOLD` | env_key | std::env::var("FF_OPT_NO_NUMFOLD") |  |  |  |  |

| `FF_OPT_NO_NUMH` | env_key | std::env::var("FF_OPT_NO_NUMH") |  |  |  |  |

| `FF_OPT_NO_RESUME` | env_key | std::env::var("FF_OPT_NO_RESUME") |  |  |  |  |

| `FF_OPT_NO_ROOTGATE` | env_key | std::env::var("FF_OPT_NO_ROOTGATE") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `solve` | function | solve( task: &PackedTask, cf: Option<usize>, max_nodes: usize, orbit: Option<&crate::orbits::OrbitMap>, ) -> OptOutcome |  |  |  |  |

| `OptOutcome` | struct | OptOutcome { pub ops: Option<Vec<usize>>, pub cost: f64, pub expanded: usize, pub evaluated: usize, pub proven: bool, pub reject: Option<String>, pub heuristic: &'static str, pub clock_tripped: bool } |  |  |  |  |

| `FF_NO_ORBIT` | env_key | std::env::var("FF_NO_ORBIT") |  |  |  |  |

| `FF_NO_ORBIT_CLASSICAL` | env_key | std::env::var("FF_NO_ORBIT_CLASSICAL") |  |  |  |  |

| `FF_ORBIT_DEBUG` | env_key | std::env::var("FF_ORBIT_DEBUG") |  |  |  |  |

| `FF_ORBIT_ISO` | env_key | std::env::var("FF_ORBIT_ISO") |  |  |  |  |

| `canonical_key` | function | canonical_key( &self, task: &PackedTask, state: &State, agenda: &[(i64, usize)], ) -> (crate::packed::StateKey, Vec<(i64, usize)>) |  |  |  |  |

| `canonical_skey` | function | canonical_skey( &self, task: &PackedTask, state: &State, cost_fluent: Option<usize>, ) -> crate::packed::StateKey |  |  |  |  |

| `canonical_skey_hash` | function | canonical_skey_hash( &self, task: &PackedTask, state: &State, cost_fluent: Option<usize>, ) -> u64 |  |  |  |  |

| `detect` | function | detect(domain: &Domain, problem: &Problem, task: &PackedTask) -> Option<OrbitMap> |  |  |  |  |

| `detect_classical` | function | detect_classical(domain: &Domain, problem: &Problem, task: &PackedTask) -> Option<OrbitMap> |  |  |  |  |

| `detect_classical_iso` | function | detect_classical_iso( domain: &Domain, problem: &Problem, task: &PackedTask, ) -> Option<OrbitMap> |  |  |  |  |

| `detect_iso` | function | detect_iso(domain: &Domain, problem: &Problem, task: &PackedTask) -> Option<OrbitMap> |  |  |  |  |

| `gen_key` | function | gen_key(&self, op: usize, classes: &[Vec<u16>]) -> Option<(u32, Vec<u16>)> |  |  |  |  |

| `goal_free_view` | function | goal_free_view(&self) -> Option<OrbitMap> |  |  |  |  |

| `iso_active` | function | iso_active(&self) -> bool |  |  |  |  |

| `iso_goal_witness` | function | iso_goal_witness( &self, task: &PackedTask, state: &State, goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<Vec<Vec<u16>>> |  |  |  |  |

| `iso_remap_op` | function | iso_remap_op(&self, sigma: &[Vec<u16>], op: usize) -> usize |  |  |  |  |

| `iso_untouched_goal` | function | iso_untouched_goal(&self) -> Option<&[u32]> |  |  |  |  |

| `stabilizer_classes` | function | stabilizer_classes(&self, state: &State, agenda: &[(f64, usize)]) -> Vec<Vec<u16>> |  |  |  |  |

| `IsoGoal` | struct | IsoGoal { desig: Vec<(u32, Vec<u16>)>, untouched: Vec<u32>, goal: Vec<u32> } |  |  |  |  |

| `Orbit` | struct | Orbit { pub facts: Vec<Vec<u32>>, pub fluent_slots: Vec<Vec<usize>>, pub ops: Vec<Vec<usize>> } |  |  |  |  |

| `OrbitMap` | struct | OrbitMap { pub orbits: Vec<Orbit>, pub iso: Option<IsoGoal>, pub op_owner: FxHashMap<usize, (usize, usize, usize)>, pub goal_bound: Vec<bool>, frozen: Vec<bool>, fact_fams: Vec<Family>, fact_touch: Vec<(u32, u32, Vec<u16>)>, op_fams: Vec<Family>, op_touch: FxHashMap<usize, (u32, Vec<u16>)>, flu_fams: Vec<Family>, flu_touch: Vec<(u32, u32, Vec<u16>)> } |  |  |  |  |

| `preamble` | function | preamble(threads: usize) -> String |  |  |  |  |

| `render` | function | render(task: &PackedTask, result: &PlanResult, threads: usize) -> (String, i32) |  |  |  |  |

| `applicable_ops` | function | applicable_ops(&self, s: &State, out: &mut Vec<u32>) |  |  |  |  |

| `apply` | function | apply(&self, oi: usize, s: &State) -> State |  |  |  |  |

| `build_succ` | function | build_succ(pre_pos: &Csr<u32>, n_facts: usize, n_ops: usize) -> (Csr<u32>, Vec<u32>) |  |  |  |  |

| `cond_effs` | function | cond_effs(&self, oi: usize) -> impl Iterator<Item = &CondEff> + Clone |  |  |  |  |

| `fact_id` | function | fact_id(&self, disp: &str) -> Option<usize> |  |  |  |  |

| `finish` | function | finish(self) -> Csr<T> |  |  |  |  |

| `fluent_id` | function | fluent_id(&self, disp: &str) -> Option<usize> |  |  |  |  |

| `goal_met` | function | goal_met(&self, s: &State) -> bool |  |  |  |  |

| `goal_met_with` | function | goal_met_with(&self, s: &State, goal_pos: &[u32], goal_num: &[NumPre]) -> bool |  |  |  |  |

| `initial` | function | initial(&self) -> State |  |  |  |  |

| `n_cond_effs` | function | n_cond_effs(&self, oi: usize) -> usize |  |  |  |  |

| `new` | function | new() -> Self |  |  |  |  |

| `op_applicable` | function | op_applicable(&self, oi: usize, s: &State) -> bool |  |  |  |  |

| `push_row` | function | push_row(&mut self, items: impl IntoIterator<Item = T>) |  |  |  |  |

| `slice` | function | slice(&self, i: usize) -> &[T] |  |  |  |  |

| `state_key` | function | state_key(&self, s: &State) -> StateKey |  |  |  |  |

| `state_key_eq` | function | state_key_eq(&self, a: &State, b: &State, cost_fluent: Option<usize>) -> bool |  |  |  |  |

| `state_key_hash` | function | state_key_hash(&self, s: &State, cost_fluent: Option<usize>) -> u64 |  |  |  |  |

| `state_key_with_cost` | function | state_key_with_cost(&self, s: &State, cost_fluent: Option<usize>) -> StateKey |  |  |  |  |

| `static_fluent` | function | static_fluent(&self, disp: &str) -> Option<f64> |  |  |  |  |

| `CondEff` | struct | CondEff { pub cond_pos: Vec<u32>, pub cond_neg: Vec<u32>, pub cond_num: Vec<NumPre>, pub add: Vec<u32>, pub del: Vec<u32>, pub num: Vec<NumEff> } |  |  |  |  |

| `Csr` | struct | Csr { pub flat: Arc<[T]>, pub off: Arc<[u32]> } |  |  |  |  |

| `CsrBuilder` | struct | CsrBuilder { pub flat: Vec<T>, pub off: Vec<u32> } |  |  |  |  |

| `PackedTask` | struct | PackedTask { pub n_facts: usize, pub words: usize, pub n_ops: usize, pub op_display: Arc<[String]>, pub pre_pos: Csr<u32>, pub succ_by_fact: Csr<u32>, pub succ_always: Arc<[u32]>, pub add: Csr<u32>, pub del: Csr<u32>, pub pre_num: Csr<NumPre>, pub num_eff: Csr<NumEff>, pub cond: Csr<CondEff>, pub shared_cond: Arc<[CondEff]>, pub monitored: Arc<[bool]>, pub add_by_fact: Csr<u32>, pub neff_by_fluent: Csr<u32>, pub relevant_fluent: Vec<bool>, pub rel_fluents: Vec<u32>, pub init_bits: Vec<u64>, pub fv0: Vec<f64>, pub fdef0: Vec<bool>, pub goal_pos: Vec<u32>, pub goal_num: Vec<NumPre>, pub charge_pre_num: bool, pub pair_end: Option<Vec<u32>>, pub trpg: Option<Arc<crate::heuristic::TrpgInfo>>, pub fact_names: Arc<[String]>, pub fluent_names: Arc<[String]>, pub static_fluents: Arc<[(String, f64)]>, pub n_easy: usize, pub n_hard: usize, pub n_reach_facts: usize, pub n_reach_actions: usize, pub n_relevant_fluents: usize } |  |  |  |  |

| `State` | struct | State { pub bits: Vec<u64>, pub fv: Vec<f64>, pub fdef: Vec<bool> } |  |  |  |  |

| `StateKey` | struct | StateKey { pub bits: Vec<u64>, pub vals: Vec<i64> } |  |  |  |  |

| `MIN_PAR` | const | MIN_PAR: usize |  |  |  |  |

| `FFDP_THREADS` | env_key | std::env::var("FFDP_THREADS") |  |  |  |  |

| `num_threads` | function | num_threads() -> usize |  |  |  |  |

| `par_map` | function | par_map(items: &[T], threads: usize, f: F) -> Vec<R> |  |  |  |  |

| `par_map_with` | function | par_map_with(items: &[T], threads: usize, init: I, f: F) -> Vec<R> |  |  |  |  |

| `parse_domain` | function | parse_domain(src: &str) -> Result<Domain, ParseError> |  |  |  |  |

| `parse_problem` | function | parse_problem(src: &str) -> Result<Problem, ParseError> |  |  |  |  |

| `:STRIPS` | str_key | SUPPORTED = ":STRIPS" |  |  |  |  |

| `interaction_partition` | function | interaction_partition(task: &PackedTask, groups: &[Vec<u32>]) -> Vec<Subgoal> |  |  |  |  |

| `interaction_partition_of` | function | interaction_partition_of( task: &PackedTask, groups: &[Vec<u32>], goals: &[u32], excluded_vars: &FxHashSet<usize>, ) -> Vec<Subgoal> |  |  |  |  |

| `is_empty` | function | is_empty(&self) -> bool |  |  |  |  |

| `merge_at` | function | merge_at(groups: &mut Vec<Subgoal>, i: usize, j: usize) -> usize |  |  |  |  |

| `merge_with_neighbor` | function | merge_with_neighbor(groups: &mut Vec<Subgoal>, i: usize) -> usize |  |  |  |  |

| `partition` | function | partition(task: &PackedTask) -> Vec<Subgoal> |  |  |  |  |

| `
    (define (domain t) (:requirements :strips)
      (:predicates (done1) (tok-a) (tok-b))
      (:action grab :precondition (tok-a)
        :effect (and (done1) (not (tok-a)) (tok-b)))
      (:action swap :precondition (tok-b)
        :effect (and (not (tok-b)) (tok-a))))` | str_key | DOM = "
    (define (domain t) (:requirements :strips)
      (:predicates (done1) (tok-a) (tok-b))
      (:action grab :precondition (tok-a)
        :effect (and (done1) (not (tok-a)) (tok-b)))
      (:action swap :precondition (tok-b)
        :effect (and (not (tok-b)) (tok-a))))" |  |  |  |  |

| `(define (problem p) (:domain t)
      (:init (tok-a)) (:goal (and (done1) (tok-b))))` | str_key | PRB = "(define (problem p) (:domain t)
      (:init (tok-a)) (:goal (and (done1) (tok-b))))" |  |  |  |  |

| `Subgoal` | struct | Subgoal { pub pos: Vec<u32>, pub num: Vec<NumPre> } |  |  |  |  |

| `COST` | const | COST: &str |  |  |  |  |

| `COST_DISP` | const | COST_DISP: &str |  |  |  |  |

| `FF_DEADLINE_WEIGHT` | env_key | std::env::var("FF_DEADLINE_WEIGHT") |  |  |  |  |

| `FF_ESPC_TRAJ_PAIRS` | env_key | std::env::var("FF_ESPC_TRAJ_PAIRS") |  |  |  |  |

| `FF_PREF_COMPILED` | env_key | std::env::var("FF_PREF_COMPILED") |  |  |  |  |

| `FF_PREF_COST_WEIGHT` | env_key | std::env::var("FF_PREF_COST_WEIGHT") |  |  |  |  |

| `FF_PREF_EVAL_BUDGET` | env_key | std::env::var("FF_PREF_EVAL_BUDGET") |  |  |  |  |

| `FF_PREF_GREEDY` | env_key | std::env::var("FF_PREF_GREEDY") |  |  |  |  |

| `FF_PREF_NO_BARRIER` | env_key | std::env::var("FF_PREF_NO_BARRIER") |  |  |  |  |

| `FF_PREF_NO_ESCALATE` | env_key | std::env::var("FF_PREF_NO_ESCALATE") |  |  |  |  |

| `FF_PREF_NO_RESTARTS` | env_key | std::env::var("FF_PREF_NO_RESTARTS") |  |  |  |  |

| `FF_PREF_NO_SEED` | env_key | std::env::var("FF_PREF_NO_SEED") |  |  |  |  |

| `FF_PREF_NO_SELECT` | env_key | std::env::var("FF_PREF_NO_SELECT") |  |  |  |  |

| `FF_PREF_NO_STATIC` | env_key | std::env::var("FF_PREF_NO_STATIC") |  |  |  |  |

| `FF_PREF_NUMLEGACY` | env_key | std::env::var("FF_PREF_NUMLEGACY") |  |  |  |  |

| `FF_PREF_SEED` | env_key | std::env::var("FF_PREF_SEED") |  |  |  |  |

| `FF_PREF_SEED3` | env_key | std::env::var("FF_PREF_SEED3") |  |  |  |  |

| `FF_PREF_SEED_BOUND` | env_key | std::env::var("FF_PREF_SEED_BOUND") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_RES_THRESH` | env_key | std::env::var("FF_RES_THRESH") |  |  |  |  |

| `FF_RES_WEIGHT` | env_key | std::env::var("FF_RES_WEIGHT") |  |  |  |  |

| `close_seed` | function | close_seed( task: &PackedTask, cost_fluent: usize, forgos: &[(usize, f64)], prefix: &[usize], ) -> Option<(Vec<usize>, f64)> |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Compiled |  |  |  |  |

| `display_metric` | function | display_metric(&self, optimized: f64) -> f64 |  |  |  |  |

| `hard_goal_plan` | function | hard_goal_plan( domain: &Domain, problem: &Problem, threads: usize, cfg: SearchCfg, ) -> Option<Vec<String>> |  |  |  |  |

| `hard_goal_seed` | function | hard_goal_seed( domain: &Domain, problem: &Problem, compiled: &PackedTask, threads: usize, cfg: SearchCfg, ) -> Option<Vec<usize>> |  |  |  |  |

| `has_preferences` | function | has_preferences(problem: &Problem) -> bool |  |  |  |  |

| `is_pddl3` | function | is_pddl3(problem: &Problem) -> bool |  |  |  |  |

| `lift_seed` | function | lift_seed(compiled: &PackedTask, names: &[String]) -> Option<Vec<usize>> |  |  |  |  |

| `metric_optimize` | function | metric_optimize( task: &PackedTask, cost_fluent: usize, forgos: &[(usize, f64)], groups: &[Vec<u32>], folded_metric: bool, threads: usize, ) -> Option<MetricResult> |  |  |  |  |

| `metric_optimize_seeded` | function | metric_optimize_seeded( task: &PackedTask, cost_fluent: usize, forgos: &[(usize, f64)], groups: &[Vec<u32>], folded_metric: bool, threads: usize, seed: Option<&[usize]>, ) -> Option<SeededResult> |  |  |  |  |

| `pref_weights` | function | pref_weights(domain: &Domain, problem: &Problem) -> HashMap<String, f64> |  |  |  |  |

| `preferences` | function | preferences(goal: &Formula, objs: &HashMap<Sym, Vec<Sym>>) -> Vec<(String, Formula)> |  |  |  |  |

| `(TOTAL-COST)` | str_key | COST_DISP = "(TOTAL-COST)" |  |  |  |  |

| `P3ENDED` | str_key | ENDED = "P3ENDED" |  |  |  |  |

| `P3PLANNING` | str_key | PLANNING = "P3PLANNING" |  |  |  |  |

| `TOTAL-COST` | str_key | COST = "TOTAL-COST" |  |  |  |  |

| `Compiled` | struct | Compiled { pub domain: Domain, pub problem: Problem, pub minimize: bool, pub maximized: bool, pub metric_konst: f64, pub n_prefs: usize, pub warn_other: bool, pub unsupported: Option<String>, pub synthetic: HashSet<String>, pub forgos: Vec<(String, f64)>, pub folded_metric: bool } |  |  |  |  |

| `MetricResult` | struct | MetricResult { pub ops: Vec<usize>, pub cost: f64, pub iterations: usize, pub proven: bool } |  |  |  |  |

| `PhaseTail` | struct | PhaseTail { pub end_op: usize, pub prefs: Vec<(Vec<usize>, usize)> } |  |  |  |  |

| `SeededResult` | struct | SeededResult { pub result: MetricResult, pub from_seed: bool } |  |  |  |  |

| `Validity` | enum | Validity { Valid, Invalid(String) } |  |  |  |  |

| `parse_classical` | function | parse_classical(src: &str) -> Vec<(String, Vec<String>)> |  |  |  |  |

| `parse_timed` | function | parse_timed(src: &str) -> Result<TimedPlan, String> |  |  |  |  |

| `validate_plan` | function | validate_plan( domain_src: &str, problem_src: &str, plan_src: &str, ) -> Result<Validity, String> |  |  |  |  |

| `(define (domain c)` | str_key | CLASSICAL_DOMAIN = "(define (domain c)" |  |  |  |  |

| `(define (domain ct)` | str_key | TEMPORAL_DOMAIN = "(define (domain ct)" |  |  |  |  |

| `(define (problem p) (:domain c)` | str_key | CLASSICAL_PROBLEM = "(define (problem p) (:domain c)" |  |  |  |  |

| `FF_SAT_CLASSICAL` | env_key | std::env::var("FF_SAT_CLASSICAL") |  |  |  |  |

| `run_ff` | function | run_ff(domain_src: &str, problem_src: &str, opts: &crate::Options) -> (String, i32) |  |  |  |  |

| `run_planner` | function | run_planner( domain_src: &str, problem_src: &str, opts: &crate::Options, ipc: bool, ) -> (String, i32) |  |  |  |  |

| `grounding budget reached! no plan found within budget (grounding NOT finished).` | str_key | GROUNDING_WALL_LINE = "grounding budget reached! no plan found within budget (grounding NOT finished)." |  |  |  |  |

| `PlannerError` | enum | PlannerError { EmptyInitialState, UnknownState { state: String, }, InvalidProbabilityMass { state: String, action: String, mass: u64, }, ResourceBound { resource: String, limit: usize, }, NoPlan, HierarchyCycle { task: String, }, UnknownTask { task: String, }, NoMethod { task: String, }, WorkflowCycle, WipBoundExceeded { queue: String, current: u64, max: u64, }, CapabilityUncovered { item: String, missing: BTreeSet<String>, }, AuthorityUnbound { tool: String, }, VerifierUnbound { tool: String, }, ReceiptUnbound { tool: String, }, InvalidRdfProjection { reason: String, }, Timeout { elapsed_ms: u128, limit_ms: u128, } } |  |  |  |  |

| `solve_planning_type` | function | solve_planning_type( request: &UniversalPlanningRequest, ) -> Result<UniversalPlan, PlannerError> |  |  |  |  |

| `Agent` | struct | Agent { pub id: String, pub capabilities: BTreeSet<String>, pub capacity: u64, pub current_wip: u64 } |  |  |  |  |

| `Goal` | struct | Goal { pub facts: BTreeSet<String>, pub numeric_min: BTreeMap<String, i64>, pub numeric_max: BTreeMap<String, i64> } |  |  |  |  |

| `Method` | struct | Method { pub id: String, pub task: String, pub subtasks: Vec<String> } |  |  |  |  |

| `PlanStep` | struct | PlanStep { pub action: String, pub from: Option<String>, pub to: Option<String>, pub start: u64, pub duration: u64, pub agent: Option<String>, pub tool: Option<String> } |  |  |  |  |

| `PlannerLimits` | struct | PlannerLimits { pub max_depth: usize, pub max_states: usize, pub max_iterations: usize, pub max_wall_ms: u64 } |  |  |  |  |

| `PlanningProblem` | struct | PlanningProblem { pub states: Vec<State>, pub initial_states: Vec<String>, pub goal: Goal, pub unsafe_states: BTreeSet<String>, pub soft_goal_facts: BTreeMap<String, u64>, pub transitions: Vec<Transition>, pub tasks: Vec<Task>, pub root_tasks: Vec<String>, pub methods: Vec<Method>, pub workflow_edges: Vec<WorkflowEdge>, pub queues: Vec<QueueState>, pub agents: Vec<Agent>, pub tools: Vec<Tool>, pub rdf: Vec<RdfTriple> } |  |  |  |  |

| `PolicyEntry` | struct | PolicyEntry { pub state: String, pub action: String, pub outcomes: Vec<PolicyOutcome> } |  |  |  |  |

| `PolicyOutcome` | struct | PolicyOutcome { pub state: String, pub probability_ppm: u32, pub observation: Option<String> } |  |  |  |  |

| `QueueState` | struct | QueueState { pub id: String, pub current_wip: u64, pub max_wip: u64 } |  |  |  |  |

| `RdfTriple` | struct | RdfTriple { pub subject: String, pub predicate: String, pub object: String } |  |  |  |  |

| `State` | struct | State { pub id: String, pub facts: BTreeSet<String>, pub fluents: BTreeMap<String, i64> } |  |  |  |  |

| `Task` | struct | Task { pub id: String, pub primitive_action: Option<String>, pub requires: BTreeSet<String> } |  |  |  |  |

| `Tool` | struct | Tool { pub id: String, pub capabilities: BTreeSet<String>, pub authority_bound: bool, pub verifier_bound: bool, pub receipt_bound: bool } |  |  |  |  |

| `Transition` | struct | Transition { pub action: String, pub from: String, pub to: String, pub cost: u64, pub duration: u64, pub reward: i64, pub probability_ppm: u32, pub observation: Option<String>, pub requires: BTreeSet<String> } |  |  |  |  |

| `UniversalPlan` | struct | UniversalPlan { pub planning_type: Option<PlanningType>, pub solved: bool, pub steps: Vec<PlanStep>, pub policy: Vec<PolicyEntry>, pub decomposition: Vec<String>, pub notes: Vec<String> } |  |  |  |  |

| `UniversalPlanningRequest` | struct | UniversalPlanningRequest { pub planning_type: PlanningType, pub problem: PlanningProblem, pub limits: PlannerLimits } |  |  |  |  |

| `WorkflowEdge` | struct | WorkflowEdge { pub before: String, pub after: String } |  |  |  |  |

| `ALL` | const | ALL: [Self; 18] |  |  |  |  |

| `PlanningCapability` | enum | PlanningCapability { DeterministicState, SequentialPlan, ActionCosts, OptimalityProof, NumericFluents, DurativeActions, TemporalValidation, SoftGoals, StochasticTransitions, NondeterministicTransitions, Policy, PolicyValidation, StrongCyclicValidation, BeliefState, OpenLoopPlan, ObservationBranching, CompoundTasks, Methods, PartialOrder, ReceiptJoin, QueueState, WipBounds, ResolutionObligations, AgentCapabilities, CoordinationPolicy, AdmittedGraph, DeterministicProjection, DelegationEnvelope, ToolCapabilities, AuthorityBinding, PrimitiveClosure } |  |  |  |  |

| `PlanningRail` | enum | PlanningRail { NativeDeterministic, NativeProbabilistic, NativeNondeterministic, NativeBeliefState, NativeHierarchical, NativeWorkflow, NativeFlowConstrained, NativeMultiAgent, GraphProjection, Delegation, CapabilityBinding } |  |  |  |  |

| `PlanningRouteError` | enum | PlanningRouteError { EmptySubject, MissingCapabilities { missing: BTreeSet<PlanningCapability>, }, AuthorityUnbound, VerifierUnbound, ReceiptUnbound } |  |  |  |  |

| `PlanningType` | enum | PlanningType { Classical, CostOptimal, Numeric, Temporal, Preferences, Probabilistic, Fond, Conformant, Contingent, Hierarchical, PartialOrder, Workflow, FlowConstrained, ResolutionAdaptive, MultiAgent, RdfDerived, A2aDelegated, McpBound } |  |  |  |  |

| `rail` | function | rail(self) -> PlanningRail |  |  |  |  |

| `required_capabilities` | function | required_capabilities(self) -> BTreeSet<PlanningCapability> |  |  |  |  |

| `route_planning_request` | function | route_planning_request( request: &PlanningRequest, ) -> Result<PlanningRoute, PlanningRouteError> |  |  |  |  |

| `token` | function | token(self) -> &'static str |  |  |  |  |

| `PlanningRequest` | struct | PlanningRequest { pub subject: String, pub planning_type: PlanningType, pub available_capabilities: BTreeSet<PlanningCapability>, pub authority_bound: bool, pub verifier_bound: bool, pub receipt_bound: bool } |  |  |  |  |

| `PlanningRoute` | struct | PlanningRoute { pub subject: String, pub planning_type: PlanningType, pub rail: PlanningRail, pub required_capabilities: BTreeSet<PlanningCapability> } |  |  |  |  |

| `PolicyGuarantee` | enum | PolicyGuarantee { Strong, StrongCyclic, Invalid } |  |  |  |  |

| `PolicyIssue` | enum | PolicyIssue { UnknownInitialState { state: String, }, DuplicatePolicyState { state: String, }, UnknownPolicyState { state: String, }, PolicyOnGoalState { state: String, }, MissingPolicyEntry { state: String, }, UnknownAction { state: String, action: String, }, UnknownTransitionTarget { state: String, action: String, target: String, }, InvalidProbabilityMass { state: String, action: String, mass: u64, }, OutcomeMismatch { state: String, action: String, }, UnsafeReachableState { state: String, }, NoGoalProgress { state: String, } } |  |  |  |  |

| `validate_fond_policy` | function | validate_fond_policy( problem: &PlanningProblem, plan: &UniversalPlan, ) -> PolicyValidationReport |  |  |  |  |

| `PolicyValidationReport` | struct | PolicyValidationReport { pub valid: bool, pub guarantee: PolicyGuarantee, pub reachable_states: Vec<String>, pub reachable_goals: Vec<String>, pub issues: Vec<PolicyIssue> } |  |  |  |  |

| `FF_PORTFOLIO_SLICED` | env_key | std::env::var("FF_PORTFOLIO_SLICED") |  |  |  |  |

| `solve` | function | solve(task: &PackedTask, threads: usize, cfg: SearchCfg) -> Outcome |  |  |  |  |

| `Outcome` | struct | Outcome { pub ops: Option<Vec<usize>>, pub evaluated: usize, pub winner: Option<&'static str> } |  |  |  |  |

| `PpddlError` | enum | PpddlError { Syntax(String), DomainParse(ParseError), ProblemParse(ParseError), Derived(String), Unsupported(String), InvalidProbability(String), InvalidOptions(String), OutcomeLimit { action: String, limit: usize }, StateLimit { limit: usize }, TransitionLimit { limit: usize }, GroundingFailed, GroundingDivergence { action: String, expected: usize, observed: usize, }, InitialOutcomeLimit { limit: usize }, RewardViolation(String), PolicyLimit { limit: usize }, ValueTableLimit { limit: usize } } |  |  |  |  |

| `ProbabilisticObjective` | enum | ProbabilisticObjective { Auto, MaximizeGoalProbability, MinimizeGoalProbability, MaximizeExpectedReward, MinimizeExpectedReward, MaximizeExpectedMetric, MinimizeExpectedMetric } |  |  |  |  |

| `:PROBABILISTIC-EFFECTS` | str_key | PROB_REQ = ":PROBABILISTIC-EFFECTS" |  |  |  |  |

| `:REWARDS` | str_key | REWARD_REQ = ":REWARDS" |  |  |  |  |

| `PPDDL-A` | str_key | VARIANT_PREFIX = "PPDDL-A" |  |  |  |  |

| `PPDDL-INIT-PENDING` | str_key | INIT_PENDING = "PPDDL-INIT-PENDING" |  |  |  |  |

| `PPDDL-INITIALIZE` | str_key | INIT_ACTION = "PPDDL-INITIALIZE" |  |  |  |  |

| `PPDDL-MARKER-A` | str_key | MARKER_PREFIX = "PPDDL-MARKER-A" |  |  |  |  |

| `InitialStateProbability` | struct | InitialStateProbability { pub state: usize, pub probability: f64, pub goal: bool } |  |  |  |  |

| `PolicyDecision` | struct | PolicyDecision { pub state: usize, pub remaining: Option<usize>, pub action: String, pub args: Vec<String>, pub value: f64, pub outcomes: Vec<PolicyOutcome> } |  |  |  |  |

| `PolicyOutcome` | struct | PolicyOutcome { pub probability: f64, pub next_state: usize, pub reward: f64, pub goal: bool } |  |  |  |  |

| `PolicyValidation` | struct | PolicyValidation { pub valid: bool, pub checked_decisions: usize, pub max_probability_error: f64, pub errors: Vec<String> } |  |  |  |  |

| `PpddlParseReport` | struct | PpddlParseReport { pub ok: bool, pub domain: Option<String>, pub problem: Option<String>, pub probabilistic_actions: usize, pub normalized_outcomes: usize, pub initial_outcomes: usize, pub uses_rewards: bool, pub goal_reward: Option<String>, pub error: Option<String> } |  |  |  |  |

| `ProbabilisticOptions` | struct | ProbabilisticOptions { pub objective: ProbabilisticObjective, pub horizon: Option<usize>, pub discount: f64, pub epsilon: f64, pub max_iterations: usize, pub max_states: usize, pub max_transitions: usize, pub max_outcomes_per_action: usize, pub max_policy_entries: usize, pub max_value_cells: usize, pub max_initial_outcomes: usize, pub simulation_max_steps: usize, pub threads: usize } |  |  |  |  |

| `ProbabilisticSolution` | struct | ProbabilisticSolution { pub solved: bool, pub objective: ProbabilisticObjective, pub initial_value: f64, pub initial_distribution: Vec<InitialStateProbability>, pub states: Vec<ProbabilisticState>, pub initial_action: Option<String>, pub horizon: Option<usize>, pub discount: f64, pub declared_metric: Option<String>, pub policy: Vec<PolicyDecision>, pub statistics: ProbabilisticStatistics, pub notes: Vec<String> } |  |  |  |  |

| `ProbabilisticState` | struct | ProbabilisticState { pub id: usize, pub facts: Vec<String>, pub fluents: BTreeMap<String, f64>, pub goal: bool, pub initial_probability: f64 } |  |  |  |  |

| `ProbabilisticStatistics` | struct | ProbabilisticStatistics { pub grounded_facts: usize, pub grounded_outcome_operators: usize, pub grounded_actions: usize, pub initial_states: usize, pub reachable_states: usize, pub transitions: usize, pub iterations: usize, pub converged: bool, pub threads: usize } |  |  |  |  |

| `SimulationReport` | struct | SimulationReport { pub episodes: usize, pub reached_goal: usize, pub goal_rate: f64, pub average_reward: f64, pub average_discounted_reward: f64, pub average_steps: f64, pub seed: u64 } |  |  |  |  |

| `parse_ppddl` | function | parse_ppddl(domain_src: &str, problem_src: &str) -> PpddlParseReport |  |  |  |  |

| `solve_ppddl` | function | solve_ppddl( domain_src: &str, problem_src: &str, options: &ProbabilisticOptions, ) -> Result<ProbabilisticSolution, PpddlError> |  |  |  |  |

| `validate_ppddl_policy` | function | validate_ppddl_policy( domain_src: &str, problem_src: &str, options: &ProbabilisticOptions, solution: &ProbabilisticSolution, ) -> Result<PolicyValidation, PpddlError> |  |  |  |  |

| `simulate_ppddl` | function | simulate_ppddl( domain_src: &str, problem_src: &str, options: &ProbabilisticOptions, episodes: usize, seed: u64, ) -> Result<SimulationReport, PpddlError> |  |  |  |  |

| `decompose_production` | function | decompose_production( domain: &str, problem: &str, options: &Options, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Decomposition> |  |  |  |  |

| `goal_met` | function | goal_met(&self) -> bool |  |  |  |  |

| `mind_bytes` | function | mind_bytes(&self) -> usize |  |  |  |  |

| `new` | function | new( domain: &str, problem: &str, options: &Options, limits: ProductionLimits, ) -> Result<Self, PublicError> |  |  |  |  |

| `parse_production` | function | parse_production( source: &str, max_input_bytes: usize, request_id: Option<&str>, ) -> OperationEnvelope<ParseReport> |  |  |  |  |

| `replan` | function | replan( &self, max_evaluated: usize, memory_mb: Option<usize>, request_id: Option<&str>, ) -> OperationEnvelope<Solution> |  |  |  |  |

| `solve_ppddl_production` | function | solve_ppddl_production( domain: &str, problem: &str, options: &ProbabilisticOptions, max_input_bytes: usize, max_output_bytes: usize, request_id: Option<&str>, ) -> OperationEnvelope<ProbabilisticSolution> |  |  |  |  |

| `trace_production` | function | trace_production( domain: &str, problem: &str, plan: &[(String, Vec<String>)], limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Vec<StateSnapshot>> |  |  |  |  |

| `validate_plan_production` | function | validate_plan_production( domain: &str, problem: &str, plan: &str, max_input_bytes: usize, max_plan_bytes: usize, request_id: Option<&str>, ) -> OperationEnvelope<PlanValidationEvidence> |  |  |  |  |

| `world_bytes` | function | world_bytes(&self) -> usize |  |  |  |  |

| `(define (domain smoke) (:requirements :strips) ` | str_key | DOMAIN = "(define (domain smoke) (:requirements :strips) " |  |  |  |  |

| `(define (problem smoke-p) (:domain smoke) ` | str_key | PROBLEM = "(define (problem smoke-p) (:domain smoke) " |  |  |  |  |

| `ferroplan.production-surface.v1` | str_key | PRODUCTION_SURFACE_HASH_DOMAIN = "ferroplan.production-surface.v1" |  |  |  |  |

| `PlanValidationEvidence` | struct | PlanValidationEvidence { pub valid: bool, pub reason: Option<String> } |  |  |  |  |

| `ProductionSession` | struct | ProductionSession { inner: Session, domain: String, problem: String, limits: ProductionLimits, input_fingerprint: String } |  |  |  |  |

| `decompose_production` | function | decompose_production( domain: &str, problem: &str, options: &Options, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Decomposition> |  |  |  |  |

| `explain_production` | function | explain_production( domain: &str, problem: &str, plan: &Plan, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Explanation> |  |  |  |  |

| `(define (domain smoke) (:requirements :strips) ` | str_key | DOMAIN = "(define (domain smoke) (:requirements :strips) " |  |  |  |  |

| `(define (problem smoke-p) (:domain smoke) ` | str_key | PROBLEM = "(define (problem smoke-p) (:domain smoke) " |  |  |  |  |

| `ferroplan.production-decompose.v1` | str_key | DECOMPOSE_HASH_DOMAIN = "ferroplan.production-decompose.v1" |  |  |  |  |

| `ferroplan.production-explain.v1` | str_key | EXPLAIN_HASH_DOMAIN = "ferroplan.production-explain.v1" |  |  |  |  |

| `depth_reached` | function | depth_reached(&self) -> u32 |  |  |  |  |

| `from_predecessors` | function | from_predecessors( predecessors: &[Vec<u32>], prohibited: &[u32], max_depth: u32, ) -> Self |  |  |  |  |

| `from_successors` | function | from_successors( successors: &[Vec<u32>], prohibited: &[u32], max_depth: u32, ) -> Self |  |  |  |  |

| `is_safe` | function | is_safe(&self, state: u32) -> bool |  |  |  |  |

| `saturated` | function | saturated(&self) -> bool |  |  |  |  |

| `unsafe_count` | function | unsafe_count(&self) -> usize |  |  |  |  |

| `BackwardSafeSet` | struct | BackwardSafeSet { n_states: usize, unsafe_words: Vec<u64>, depth_reached: u32, saturated: bool } |  |  |  |  |

| `CANDIDATE_AUTHORITY` | const | CANDIDATE_AUTHORITY: &str |  |  |  |  |

| `CAPABILITY_MANIFEST_SCHEMA` | const | CAPABILITY_MANIFEST_SCHEMA: &str |  |  |  |  |

| `OPERATION_ENVELOPE_SCHEMA` | const | OPERATION_ENVELOPE_SCHEMA: &str |  |  |  |  |

| `AuthorityClass` | enum | AuthorityClass { CandidateOnly, EvidenceOnly, PresentationOnly } |  |  |  |  |

| `CompatibilityClass` | enum | CompatibilityClass { Semver, VersionedSchema } |  |  |  |  |

| `DeterminismClass` | enum | DeterminismClass { Exact, OutcomeEquivalent, NotApplicable } |  |  |  |  |

| `InterfaceKind` | enum | InterfaceKind { RustLibrary, NativeCli, PythonAbi3, BrowserWasm, BevyGui, McpPlus, Plugin, Documentation, ReleasePipeline } |  |  |  |  |

| `ManifestError` | enum | ManifestError { Schema(String), NonCanonicalOrder, DuplicateId(String), MissingField { id: String, field: String }, MissingEvidence(String), NonCanonicalEvidence(String), MissingSourceIdentity, Serialization } |  |  |  |  |

| `OutcomeClass` | enum | OutcomeClass { Solved, NoPlan, LimitExceeded, Refused, Failed } |  |  |  |  |

| `ReadinessState` | enum | ReadinessState { Unknown, Declared, Partial, Admitted, Blocked, Unsupported, Refused } |  |  |  |  |

| `ReplayClass` | enum | ReplayClass { Exact, Outcome, BuildReproducible, NotApplicable } |  |  |  |  |

| `SecurityClass` | enum | SecurityClass { UntrustedInput, LocalPresentation, BuildControl } |  |  |  |  |

| `ValidationStatus` | enum | ValidationStatus { Valid, NotApplicable, Failed } |  |  |  |  |

| `capability_manifest` | function | capability_manifest() -> CapabilityManifest |  |  |  |  |

| `evaluate_readiness` | function | evaluate_readiness( source_identity: impl Into<String>, evidence: I, ) -> Result<ReadinessReport, ManifestError> |  |  |  |  |

| `fingerprint` | function | fingerprint(&self) -> Result<String, ManifestError> |  |  |  |  |

| `new` | function | new(code: impl Into<String>, message: impl Into<String>, retryable: bool) -> Self |  |  |  |  |

| `production_input_fingerprint` | function | production_input_fingerprint(domain: &str, problem: &str, options: &Options) -> String |  |  |  |  |

| `solve_production` | function | solve_production( domain: &str, problem: &str, options: &Options, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Solution> |  |  |  |  |

| `validate` | function | validate(&self) -> Result<(), ManifestError> |  |  |  |  |

| `(define (domain smoke) (:requirements :strips) ` | str_key | DOMAIN = "(define (domain smoke) (:requirements :strips) " |  |  |  |  |

| `(define (problem smoke-p) (:domain smoke) ` | str_key | PROBLEM = "(define (problem smoke-p) (:domain smoke) " |  |  |  |  |

| `candidate_only` | str_key | CANDIDATE_AUTHORITY = "candidate_only" |  |  |  |  |

| `ferroplan.capabilities.v1` | str_key | CAPABILITY_MANIFEST_SCHEMA = "ferroplan.capabilities.v1" |  |  |  |  |

| `ferroplan.capability-manifest.v1` | str_key | MANIFEST_HASH_DOMAIN = "ferroplan.capability-manifest.v1" |  |  |  |  |

| `ferroplan.operation.v1` | str_key | OPERATION_ENVELOPE_SCHEMA = "ferroplan.operation.v1" |  |  |  |  |

| `ferroplan.production-input.v1` | str_key | INPUT_HASH_DOMAIN = "ferroplan.production-input.v1" |  |  |  |  |

| `BuildIdentity` | struct | BuildIdentity { pub product_version: String, pub source_revision: Option<String>, pub manifest_fingerprint: Option<String> } |  |  |  |  |

| `CapabilityContract` | struct | CapabilityContract { pub id: String, pub version: String, pub owner: String, pub component: String, pub interface: InterfaceKind, pub authority: AuthorityClass, pub determinism: DeterminismClass, pub replay: ReplayClass, pub input_schema: String, pub output_schema: String, pub resource_profile: String, pub failure_contract: String, pub telemetry_contract: String, pub compatibility: CompatibilityClass, pub security: SecurityClass, pub shipped: bool, pub required_evidence: Vec<String> } |  |  |  |  |

| `CapabilityEvaluation` | struct | CapabilityEvaluation { pub capability_id: String, pub state: ReadinessState, pub satisfied_evidence: Vec<String>, pub missing_evidence: Vec<String> } |  |  |  |  |

| `CapabilityManifest` | struct | CapabilityManifest { pub schema_version: String, pub product_version: String, pub authority_notice: String, pub capabilities: Vec<CapabilityContract> } |  |  |  |  |

| `OperationEnvelope` | struct | OperationEnvelope { pub schema_version: String, pub request_id: String, pub capability_id: String, pub capability_version: String, pub build_identity: BuildIdentity, pub input_fingerprint: String, pub authority: String, pub outcome: OutcomeClass, pub validation: ValidationStatus, pub elapsed_micros: u64, pub counters: BTreeMap<String, u64>, pub warnings: Vec<String>, pub payload: Option<T>, pub error: Option<PublicError> } |  |  |  |  |

| `ProductionLimits` | struct | ProductionLimits { pub max_domain_bytes: usize, pub max_problem_bytes: usize, pub max_evaluated: usize, pub max_plan_steps: usize, pub max_output_bytes: usize, pub max_workers: usize } |  |  |  |  |

| `PublicError` | struct | PublicError { pub code: String, pub message: String, pub retryable: bool } |  |  |  |  |

| `ReadinessReport` | struct | ReadinessReport { pub schema_version: String, pub product_version: String, pub source_identity: String, pub manifest_fingerprint: String, pub evaluator_version: String, pub overall_state: ReadinessState, pub capabilities: Vec<CapabilityEvaluation> } |  |  |  |  |

| `ff_plan` | function | ff_plan(task: &PackedTask, ops: &[usize]) -> String |  |  |  |  |

| `ipc_plan` | function | ipc_plan(task: &PackedTask, ops: &[usize], metric: Option<f64>) -> String |  |  |  |  |

| `metric_footer` | function | metric_footer( cost: f64, iterations: usize, n_prefs: usize, threads: usize, warn_other: bool, ) -> String |  |  |  |  |

| `preamble` | function | preamble(threads: usize) -> String |  |  |  |  |

| `timing` | function | timing(stats: &Stats, threads: usize) -> String |  |  |  |  |

| `Solved` | enum | Solved { Plan(Vec<usize>, Stats), Unsolvable { capped: bool, } } |  |  |  |  |

| `FF_NO_LAMA` | env_key | std::env::var("FF_NO_LAMA") |  |  |  |  |

| `FF_RESLM` | env_key | std::env::var("FF_RESLM") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `solve` | function | solve( task: &PackedTask, threads: usize, cfg: crate::search::SearchCfg, mutex_groups: &[Vec<u32>], orbit: Option<&crate::orbits::OrbitMap>, ) -> Solved |  |  |  |  |

| `Stats` | struct | Stats { pub init_groups: usize, pub final_groups: usize, pub merges: usize, pub fallback: bool } |  |  |  |  |

| `detect_resources` | function | detect_resources(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Vec<ResourceVar> |  |  |  |  |

| `occupancy` | function | occupancy(&self, bits: &[u64]) -> u32 |  |  |  |  |

| `trip_bound` | function | trip_bound(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Option<TripBound> |  |  |  |  |

| `trips` | function | trips(&self, bits: &[u64]) -> i64 |  |  |  |  |

| `(define (domain ctr) (:requirements :typing)
      (:types count)
      (:predicates (avail ?s - count) (nxt ?lo ?hi - count))
      (:action consume :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?b ?a))
        :effect (and (not (avail ?a)) (avail ?b)))
      (:action restore :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?a ?b))
        :effect (and (not (avail ?a)) (avail ?b))))` | str_key | DOM = "(define (domain ctr) (:requirements :typing)
      (:types count)
      (:predicates (avail ?s - count) (nxt ?lo ?hi - count))
      (:action consume :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?b ?a))
        :effect (and (not (avail ?a)) (avail ?b)))
      (:action restore :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?a ?b))
        :effect (and (not (avail ?a)) (avail ?b))))" |  |  |  |  |

| `(define (domain tinytrans)
      (:requirements :strips :typing)
      (:types loc pkg cap)
      (:predicates (tat ?l - loc) (pat ?p - pkg ?l - loc) (pin ?p - pkg)
                   (cap ?c - cap) (nxt ?a ?b - cap))
      (:action mv :parameters (?a ?b - loc)
        :precondition (tat ?a) :effect (and (not (tat ?a)) (tat ?b)))
      (:action pick :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pat ?p ?l) (nxt ?a ?b) (cap ?b))
        :effect (and (not (pat ?p ?l)) (pin ?p) (cap ?a) (not (cap ?b))))
      (:action drop :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pin ?p) (nxt ?a ?b) (cap ?a))
        :effect (and (not (pin ?p)) (pat ?p ?l) (cap ?b) (not (cap ?a)))))` | str_key | TDOM = "(define (domain tinytrans)
      (:requirements :strips :typing)
      (:types loc pkg cap)
      (:predicates (tat ?l - loc) (pat ?p - pkg ?l - loc) (pin ?p - pkg)
                   (cap ?c - cap) (nxt ?a ?b - cap))
      (:action mv :parameters (?a ?b - loc)
        :precondition (tat ?a) :effect (and (not (tat ?a)) (tat ?b)))
      (:action pick :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pat ?p ?l) (nxt ?a ?b) (cap ?b))
        :effect (and (not (pat ?p ?l)) (pin ?p) (cap ?a) (not (cap ?b))))
      (:action drop :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pin ?p) (nxt ?a ?b) (cap ?a))
        :effect (and (not (pin ?p)) (pat ?p ?l) (cap ?b) (not (cap ?a)))))" |  |  |  |  |

| `(define (problem ctr1) (:domain ctr)
      (:objects c0 c1 c2 c3 - count)
      (:init (avail c3) (nxt c0 c1) (nxt c1 c2) (nxt c2 c3))
      (:goal (avail c0)))` | str_key | PROB = "(define (problem ctr1) (:domain ctr)
      (:objects c0 c1 c2 c3 - count)
      (:init (avail c3) (nxt c0 c1) (nxt c1 c2) (nxt c2 c3))
      (:goal (avail c0)))" |  |  |  |  |

| `(define (problem tt1) (:domain tinytrans)
      (:objects l1 l2 l3 - loc p1 p2 p3 - pkg c0 c1 c2 - cap)
      (:init (tat l1) (pat p1 l1) (pat p2 l1) (pat p3 l1)
             (cap c2) (nxt c0 c1) (nxt c1 c2))
      (:goal (and (pat p1 l2) (pat p2 l2) (pat p3 l3))))` | str_key | TPROB = "(define (problem tt1) (:domain tinytrans)
      (:objects l1 l2 l3 - loc p1 p2 p3 - pkg c0 c1 c2 - cap)
      (:init (tat l1) (pat p1 l1) (pat p2 l1) (pat p3 l1)
             (cap c2) (nxt c0 c1) (nxt c1 c2))
      (:goal (and (pat p1 l2) (pat p2 l2) (pat p3 l3))))" |  |  |  |  |

| `ResourceVar` | struct | ResourceVar { pub members: Vec<(u32, u32)> } |  |  |  |  |

| `TripBound` | struct | TripBound { pub goals: Vec<u32>, pub pool: i64 } |  |  |  |  |

| `FF_NO_SAT_LAYERGEN` | env_key | std::env::var("FF_NO_SAT_LAYERGEN") |  |  |  |  |

| `FF_NO_SAT_RATEBAIL` | env_key | std::env::var("FF_NO_SAT_RATEBAIL") |  |  |  |  |

| `FF_SAT_BRANCH` | env_key | std::env::var("FF_SAT_BRANCH") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `from_env` | function | from_env() -> Self |  |  |  |  |

| `requires_concurrency` | function | requires_concurrency(domain: &Domain, problem: &Problem) -> bool |  |  |  |  |

| `solve_classical` | function | solve_classical( task: &PackedTask, groups: &[Vec<u32>], cfg: &SatCfg, ) -> SatOutcome<Vec<usize>> |  |  |  |  |

| `solve_temporal` | function | solve_temporal( domain: &Domain, problem: &Problem, threads: usize, cfg: &SatCfg, ) -> SatOutcome<TimedPlan> |  |  |  |  |

| `solve_temporal_within` | function | solve_temporal_within( domain: &Domain, problem: &Problem, threads: usize, cfg: &SatCfg, budget_secs: Option<f64>, ) -> SatOutcome<TimedPlan> |  |  |  |  |

| `SatCfg` | struct | SatCfg { pub max_horizon: usize, pub conflicts_per_horizon: u64, pub cap_lits: u64 } |  |  |  |  |

| `SatOutcome` | struct | SatOutcome { pub plan: Option<P>, pub notes: Vec<String>, pub proven_at_every_horizon: bool, pub grounded_facts: usize, pub grounded_actions: usize } |  |  |  |  |

| `DEFAULT_MAX_EVAL` | const | DEFAULT_MAX_EVAL: usize |  |  |  |  |

| `PlanResult` | enum | PlanResult { Plan { ops: Vec<usize>, advance: Vec<i32>, evaluated: usize, max_g: usize, }, Unsolvable { evaluated: usize, capped: bool, } } |  |  |  |  |

| `FF_CLM` | env_key | std::env::var("FF_CLM") |  |  |  |  |

| `FF_HTRACE` | env_key | std::env::var("FF_HTRACE") |  |  |  |  |

| `FF_LEN_ANYTIME` | env_key | std::env::var("FF_LEN_ANYTIME") |  |  |  |  |

| `FF_MEM_BUDGET_GB` | env_key | std::env::var("FF_MEM_BUDGET_GB") |  |  |  |  |

| `FF_NOVDRIVER_ONLY` | env_key | std::env::var("FF_NOVDRIVER_ONLY") |  |  |  |  |

| `FF_NOVELTY` | env_key | std::env::var("FF_NOVELTY") |  |  |  |  |

| `FF_NOVELTY_ONLY` | env_key | std::env::var("FF_NOVELTY_ONLY") |  |  |  |  |

| `FF_NOVLIGHT` | env_key | std::env::var("FF_NOVLIGHT") |  |  |  |  |

| `FF_NOVLIGHT_ONLY` | env_key | std::env::var("FF_NOVLIGHT_ONLY") |  |  |  |  |

| `FF_NOV_OLD` | env_key | std::env::var("FF_NOV_OLD") |  |  |  |  |

| `FF_NO_EHC_WALLCAP` | env_key | std::env::var("FF_NO_EHC_WALLCAP") |  |  |  |  |

| `FF_NO_ENRICH` | env_key | std::env::var("FF_NO_ENRICH") |  |  |  |  |

| `FF_NO_LAMA` | env_key | std::env::var("FF_NO_LAMA") |  |  |  |  |

| `FF_NO_NODECAP_REFILL` | env_key | std::env::var("FF_NO_NODECAP_REFILL") |  |  |  |  |

| `FF_NO_NOVELTY` | env_key | std::env::var("FF_NO_NOVELTY") |  |  |  |  |

| `FF_NO_NOVLIGHT` | env_key | std::env::var("FF_NO_NOVLIGHT") |  |  |  |  |

| `FF_NO_REFILL` | env_key | std::env::var("FF_NO_REFILL") |  |  |  |  |

| `FF_NO_RUNG_WALLCAP` | env_key | std::env::var("FF_NO_RUNG_WALLCAP") |  |  |  |  |

| `FF_REPORT_RESERVE_SECS` | env_key | std::env::var("FF_REPORT_RESERVE_SECS") |  |  |  |  |

| `FF_RESLM` | env_key | std::env::var("FF_RESLM") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_SEARCH_NODE_CAP` | env_key | std::env::var("FF_SEARCH_NODE_CAP") |  |  |  |  |

| `FF_TIME_LIMIT` | env_key | std::env::var("FF_TIME_LIMIT") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `arm_wall_limit` | function | arm_wall_limit() |  |  |  |  |

| `cancelled` | function | cancelled(&self) -> bool |  |  |  |  |

| `cost` | function | cost(&self, s: &State) -> f64 |  |  |  |  |

| `from_weights` | function | from_weights(weight_g: f64, weight_h: f64, max_eval: Option<usize>) -> Self |  |  |  |  |

| `holds` | function | holds(&self, s: &State) -> bool |  |  |  |  |

| `plan` | function | plan( task: &PackedTask, threads: usize, cfg: SearchCfg, ehc_first: bool, orbit: Option<&crate::orbits::OrbitMap>, ) -> PlanOutcome |  |  |  |  |

| `plan_avoiding` | function | plan_avoiding( task: &PackedTask, threads: usize, cfg: SearchCfg, ehc_first: bool, forbidden: &[bool], orbit: Option<&crate::orbits::OrbitMap>, ) -> PlanOutcome |  |  |  |  |

| `search` | function | search(task: &PackedTask, threads: usize, cfg: SearchCfg) -> PlanResult |  |  |  |  |

| `search_from` | function | search_from( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: Option<usize>, cost_bound: f64, threads: usize, cfg: SearchCfg, forbidden: &[bool], sat: Option<&SatGuidance>, closure: Option<&ClosureCost>, orbit: Option<&crate::orbits::OrbitMap>, ) -> PlanResult |  |  |  |  |

| `solve_closure_bounded` | function | solve_closure_bounded( task: &PackedTask, goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, bound: f64, closure: &ClosureCost, forbidden: &[bool], threads: usize, cfg: SearchCfg, sat: Option<&SatGuidance>, ) -> (Option<Vec<usize>>, usize, bool) |  |  |  |  |

| `solve_subgoal` | function | solve_subgoal( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], threads: usize, cfg: SearchCfg, orbit: Option<&crate::orbits::OrbitMap>, ) -> Option<Vec<usize>> |  |  |  |  |

| `solve_subgoal_avoiding` | function | solve_subgoal_avoiding( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], forbidden: &[bool], threads: usize, cfg: SearchCfg, ) -> Option<Vec<usize>> |  |  |  |  |

| `solve_subgoal_bounded` | function | solve_subgoal_bounded( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, bound: f64, threads: usize, cfg: SearchCfg, sat: Option<&SatGuidance>, ) -> (Option<Vec<usize>>, usize, bool) |  |  |  |  |

| `solve_subgoal_guided` | function | solve_subgoal_guided( task: &PackedTask, start: &State, goal_pos: &[u32], goal_num: &[NumPre], forbidden: &[bool], threads: usize, cfg: SearchCfg, sat: Option<&SatGuidance>, ) -> (Option<Vec<usize>>, usize) |  |  |  |  |

| `with_cost_h` | function | with_cost_h(mut self, cost_fluent: usize) -> Self |  |  |  |  |

| `with_cost_weight` | function | with_cost_weight(mut self, w_c: f64) -> Self |  |  |  |  |

| `ClosureCost` | struct | ClosureCost { pub prefs: Vec<(f64, PrefPhi)> } |  |  |  |  |

| `PlanOutcome` | struct | PlanOutcome { pub ops: Option<Vec<usize>>, pub evaluated: usize, pub ehc_fell_back: bool, pub capped: bool } |  |  |  |  |

| `PrefPhi` | struct | PrefPhi { pub disjuncts: Vec<(Vec<u32>, Vec<NumPre>)> } |  |  |  |  |

| `SatGuidance` | struct | SatGuidance { pub prefs: Vec<(PrefPhi, i64)>, pub res: Vec<crate::resource::ResourceVar>, pub res_weight: i64, pub res_thresh: i64, pub deadline: Vec<(u32, u32, i64)>, pub deadline_weight: i64 } |  |  |  |  |

| `SearchCfg` | struct | SearchCfg { pub w_g: i64, pub w_h: i64, pub max_eval: usize, pub w_c: f64, pub h_cost: Option<usize>, pub anytime: bool, pub g_bound: usize, pub len_anytime: bool, pub w_lm: i64, pub w_res: i64, pub pref_ops: bool, pub node_bytes_target: Option<usize>, pub deadline: Option<(crate::clock::Clock, f64)>, pub ehc_wall_frac: Option<f64> } |  |  |  |  |

| `select` | function | select( task: &PackedTask, groups: &[Vec<u32>], weights: &[f64], dnf: &FxHashMap<usize, Vec<Vec<u32>>>, banned: &crate::hash::FxHashSet<u32>, ) -> Option<Selection> |  |  |  |  |

| `Selection` | struct | Selection { pub chosen: Vec<(usize, Vec<u32>)>, pub bound: f64, pub capped: bool } |  |  |  |  |

| `ThinkVerdict` | enum | ThinkVerdict { Solved, Capped, Exhausted } |  |  |  |  |

| `apply_start` | function | apply_start(&mut self, name: &str) -> Result<(), String> |  |  |  |  |

| `elapse` | function | elapse(&mut self, dt: f64) -> Result<Vec<String>, String> |  |  |  |  |

| `fact` | function | fact(&self, name: &str) -> Option<bool> |  |  |  |  |

| `fluent` | function | fluent(&self, name: &str) -> Option<f64> |  |  |  |  |

| `fork` | function | fork(&self) -> Session |  |  |  |  |

| `goal_met` | function | goal_met(&self) -> bool |  |  |  |  |

| `mind_bytes` | function | mind_bytes(&self) -> usize |  |  |  |  |

| `new` | function | new(domain_src: &str, problem_src: &str, opts: &Options) -> Result<Session, String> |  |  |  |  |

| `observe` | function | observe(&mut self, sight: &[(&str, bool)]) -> Result<Vec<String>, String> |  |  |  |  |

| `plan_still_valid` | function | plan_still_valid(&self, plan: &Plan, from_step: usize) -> bool |  |  |  |  |

| `replan` | function | replan(&self) -> Solution |  |  |  |  |

| `replan_budgeted` | function | replan_budgeted(&self, max_evaluated: usize, memory_mb: Option<usize>) -> Solution |  |  |  |  |

| `replan_following` | function | replan_following( &self, prior: &Plan, from_step: usize, max_evaluated: usize, memory_mb: Option<usize>, ) -> Solution |  |  |  |  |

| `restrict_ops` | function | restrict_ops(&mut self, mut keep: impl FnMut(&str) -> bool) |  |  |  |  |

| `set_fact` | function | set_fact(&mut self, name: &str, value: bool) -> Result<(), String> |  |  |  |  |

| `set_fluent` | function | set_fluent(&mut self, name: &str, value: f64) -> Result<(), String> |  |  |  |  |

| `set_goal` | function | set_goal(&mut self, goal: &str) -> Result<(), String> |  |  |  |  |

| `set_timed_fact` | function | set_timed_fact(&mut self, dt: f64, name: &str, value: bool) -> Result<(), String> |  |  |  |  |

| `state_fingerprint` | function | state_fingerprint(&self) -> String |  |  |  |  |

| `think` | function | think(&self, budget: &ThinkBudget) -> Think |  |  |  |  |

| `think_following` | function | think_following(&self, prior: &Plan, from_step: usize, budget: &ThinkBudget) -> Think |  |  |  |  |

| `world_bytes` | function | world_bytes(&self) -> usize |  |  |  |  |

| `
        (define (domain gate) (:requirements :strips :numeric-fluents)
          (:predicates (done))
          (:functions (permit))
          (:action act :precondition (>= (permit) 1) :effect (done)))` | str_key | GDOM = "
        (define (domain gate) (:requirements :strips :numeric-fluents)
          (:predicates (done))
          (:functions (permit))
          (:action act :precondition (>= (permit) 1) :effect (done)))" |  |  |  |  |

| `
        (define (domain shop)
          (:requirements :strips :typing :durative-actions :numeric-fluents)
          (:types worker)
          (:predicates (idle ?w - worker) (built ?w - worker))
          (:functions (build-time ?w - worker))
          (:durative-action build
            :parameters (?w - worker)
            :duration (= ?duration (build-time ?w))
            :condition (at start (idle ?w))
            :effect (and (at start (not (idle ?w))) (at end (built ?w)))))` | str_key | DDOM = "
        (define (domain shop)
          (:requirements :strips :typing :durative-actions :numeric-fluents)
          (:types worker)
          (:predicates (idle ?w - worker) (built ?w - worker))
          (:functions (build-time ?w - worker))
          (:durative-action build
            :parameters (?w - worker)
            :duration (= ?duration (build-time ?w))
            :condition (at start (idle ?w))
            :effect (and (at start (not (idle ?w))) (at end (built ?w)))))" |  |  |  |  |

| `
        (define (problem g) (:domain gate)
          (:init (= (permit) 0))
          (:goal (done)))` | str_key | GPRB = "
        (define (problem g) (:domain gate)
          (:init (= (permit) 0))
          (:goal (done)))" |  |  |  |  |

| `
        (define (problem job) (:domain shop)
          (:objects w1 - worker)
          (:init (idle w1) (= (build-time w1) 5))
          (:goal (built w1)))` | str_key | DPRB = "
        (define (problem job) (:domain shop)
          (:objects w1 - worker)
          (:init (idle w1) (= (build-time w1) 5))
          (:goal (built w1)))" |  |  |  |  |

| `
    (define (domain farm) (:requirements :strips :typing :numeric-fluents)
      (:types agent place)
      (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
      (:functions (grain))
      (:action walk :parameters (?a - agent ?from ?to - place)
        :precondition (and (at ?a ?from) (road ?from ?to))
        :effect (and (not (at ?a ?from)) (at ?a ?to)))
      (:action harvest :parameters (?a - agent ?p - place)
        :precondition (and (at ?a ?p) (fertile ?p))
        :effect (increase (grain) 1)))` | str_key | DOM = "
    (define (domain farm) (:requirements :strips :typing :numeric-fluents)
      (:types agent place)
      (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
      (:functions (grain))
      (:action walk :parameters (?a - agent ?from ?to - place)
        :precondition (and (at ?a ?from) (road ?from ?to))
        :effect (and (not (at ?a ?from)) (at ?a ?to)))
      (:action harvest :parameters (?a - agent ?p - place)
        :precondition (and (at ?a ?p) (fertile ?p))
        :effect (increase (grain) 1)))" |  |  |  |  |

| `
    (define (domain lamp) (:requirements :strips :negative-preconditions)
      (:predicates (on) (broken))
      (:action switch-on :precondition (and (not (on)) (not (broken))) :effect (on))
      (:action switch-off :precondition (on) :effect (not (on))))` | str_key | NEG_DOM = "
    (define (domain lamp) (:requirements :strips :negative-preconditions)
      (:predicates (on) (broken))
      (:action switch-on :precondition (and (not (on)) (not (broken))) :effect (on))
      (:action switch-off :precondition (on) :effect (not (on))))" |  |  |  |  |

| `
    (define (domain rollers) (:requirements :strips :typing)
      (:types ball room)
      (:predicates (at ?b - ball ?r - room) (link ?x ?y - room)
                   (goal-room ?r - room) (home ?b - ball))
      (:action roll :parameters (?b - ball ?from ?to - room)
        :precondition (and (at ?b ?from) (link ?from ?to))
        :effect (and (not (at ?b ?from)) (at ?b ?to)))
      (:action park :parameters (?b - ball ?r - room)
        :precondition (and (at ?b ?r) (goal-room ?r))
        :effect (home ?b)))` | str_key | ORB_DOM = "
    (define (domain rollers) (:requirements :strips :typing)
      (:types ball room)
      (:predicates (at ?b - ball ?r - room) (link ?x ?y - room)
                   (goal-room ?r - room) (home ?b - ball))
      (:action roll :parameters (?b - ball ?from ?to - room)
        :precondition (and (at ?b ?from) (link ?from ?to))
        :effect (and (not (at ?b ?from)) (at ?b ?to)))
      (:action park :parameters (?b - ball ?r - room)
        :precondition (and (at ?b ?r) (goal-room ?r))
        :effect (home ?b)))" |  |  |  |  |

| `
    (define (domain seqshop) (:requirements :strips :typing :durative-actions)
      (:types w)
      (:predicates (idle ?x - w) (staged ?x - w) (built ?x - w) (power))
      (:durative-action stage1 :parameters (?x - w) :duration (= ?duration 5)
        :condition (at start (idle ?x))
        :effect (and (at start (not (idle ?x))) (at end (staged ?x))))
      (:durative-action stage2 :parameters (?x - w) :duration (= ?duration 5)
        :condition (and (at start (staged ?x)) (at start (power)))
        :effect (at end (built ?x)))
      (:durative-action grid :parameters () :duration (= ?duration 1)
        :condition (at start (power))
        :effect (and (at start (not (power))) (at end (power)))))` | str_key | SEQ_DOM = "
    (define (domain seqshop) (:requirements :strips :typing :durative-actions)
      (:types w)
      (:predicates (idle ?x - w) (staged ?x - w) (built ?x - w) (power))
      (:durative-action stage1 :parameters (?x - w) :duration (= ?duration 5)
        :condition (at start (idle ?x))
        :effect (and (at start (not (idle ?x))) (at end (staged ?x))))
      (:durative-action stage2 :parameters (?x - w) :duration (= ?duration 5)
        :condition (and (at start (staged ?x)) (at start (power)))
        :effect (at end (built ?x)))
      (:durative-action grid :parameters () :duration (= ?duration 1)
        :condition (at start (power))
        :effect (and (at start (not (power))) (at end (power)))))" |  |  |  |  |

| `
    (define (domain shop) (:requirements :strips :typing :durative-actions)
      (:types job machine)
      (:predicates (todo ?j - job) (done ?j - job) (up ?m - machine) (fast ?m - machine)
                   (slow ?m - machine))
      (:durative-action run-fast :parameters (?j - job ?m - machine)
        :duration (= ?duration 2)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (fast ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action run-slow :parameters (?j - job ?m - machine)
        :duration (= ?duration 8)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (slow ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action maintain :parameters (?m - machine)
        :duration (= ?duration 1)
        :condition (at start (up ?m))
        :effect (and (at start (not (up ?m))) (at end (up ?m)))))` | str_key | SHOP_DOM = "
    (define (domain shop) (:requirements :strips :typing :durative-actions)
      (:types job machine)
      (:predicates (todo ?j - job) (done ?j - job) (up ?m - machine) (fast ?m - machine)
                   (slow ?m - machine))
      (:durative-action run-fast :parameters (?j - job ?m - machine)
        :duration (= ?duration 2)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (fast ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action run-slow :parameters (?j - job ?m - machine)
        :duration (= ?duration 8)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (slow ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action maintain :parameters (?m - machine)
        :duration (= ?duration 1)
        :condition (at start (up ?m))
        :effect (and (at start (not (up ?m))) (at end (up ?m)))))" |  |  |  |  |

| `
    (define (domain workshop) (:requirements :strips :typing :durative-actions)
      (:types worker)
      (:predicates (idle ?w - worker) (built ?w - worker))
      (:durative-action build
        :parameters (?w - worker)
        :duration (= ?duration 5)
        :condition (at start (idle ?w))
        :effect (and (at start (not (idle ?w))) (at end (built ?w)))))` | str_key | TDOM = "
    (define (domain workshop) (:requirements :strips :typing :durative-actions)
      (:types worker)
      (:predicates (idle ?w - worker) (built ?w - worker))
      (:durative-action build
        :parameters (?w - worker)
        :duration (= ?duration 5)
        :condition (at start (idle ?w))
        :effect (and (at start (not (idle ?w))) (at end (built ?w)))))" |  |  |  |  |

| `
    (define (problem p) (:domain farm)
      (:objects v1 - agent hut field - place)
      (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
      (:goal (>= (grain) 2)))` | str_key | PRB = "
    (define (problem p) (:domain farm)
      (:objects v1 - agent hut field - place)
      (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
      (:goal (>= (grain) 2)))" |  |  |  |  |

| `
    (define (problem p) (:domain lamp)
      (:init) (:goal (on)))` | str_key | NEG_PRB = "
    (define (problem p) (:domain lamp)
      (:init) (:goal (on)))" |  |  |  |  |

| `
    (define (problem p) (:domain rollers)
      (:objects b1 b2 - ball ra rb - room)
      (:init (at b1 ra) (at b2 ra) (link ra rb) (link rb ra) (goal-room rb))
      (:goal (and (home b1) (home b2))))` | str_key | ORB_PRB = "
    (define (problem p) (:domain rollers)
      (:objects b1 b2 - ball ra rb - room)
      (:init (at b1 ra) (at b2 ra) (link ra rb) (link rb ra) (goal-room rb))
      (:goal (and (home b1) (home b2))))" |  |  |  |  |

| `
    (define (problem p) (:domain seqshop)
      (:objects w1 - w)
      (:init (idle w1) (power))
      (:goal (built w1)))` | str_key | SEQ_PRB = "
    (define (problem p) (:domain seqshop)
      (:objects w1 - w)
      (:init (idle w1) (power))
      (:goal (built w1)))" |  |  |  |  |

| `
    (define (problem p) (:domain shop)
      (:objects j1 j2 - job f s - machine)
      (:init (todo j1) (todo j2) (up f) (up s) (fast f) (slow s))
      (:goal (and (done j1) (done j2))))` | str_key | SHOP_PRB = "
    (define (problem p) (:domain shop)
      (:objects j1 j2 - job f s - machine)
      (:init (todo j1) (todo j2) (up f) (up s) (fast f) (slow s))
      (:goal (and (done j1) (done j2))))" |  |  |  |  |

| `
    (define (problem shift) (:domain workshop)
      (:objects w1 w2 - worker)
      (:init (idle w1) (idle w2))
      (:goal (and (built w1) (built w2))))` | str_key | TPRB = "
    (define (problem shift) (:domain workshop)
      (:objects w1 w2 - worker)
      (:init (idle w1) (idle w2))
      (:goal (and (built w1) (built w2))))" |  |  |  |  |

| `urn:ferroplan:session-state:v1` | str_key | DOMAIN = "urn:ferroplan:session-state:v1" |  |  |  |  |

| `Session` | struct | Session { task: PackedTask, threads: usize, weight_g: f64, weight_h: f64, max_evaluated: Option<usize>, ehc_first: bool, fact_ids: Arc<FxHashMap<String, u32>>, dynamic: Arc<[bool]>, fluent_ids: Arc<FxHashMap<String, u32>>, temporal: Option<Arc<crate::temporal::TemporalCompiled>>, tier: crate::features::DemandMode, running_preds: Vec<String>, op_ids: Arc<FxHashMap<String, usize>>, mirror: Arc<FxHashMap<u32, u32>>, forbidden: Vec<bool>, timed: Vec<(f64, u32, bool)>, til_setters: Arc<FxHashMap<(u32, bool), usize>>, running: Vec<(f64, usize)>, lifted: Option<Arc<(crate::types::Domain, crate::types::Problem)>>, goal_formula: Formula } |  |  |  |  |

| `Think` | struct | Think { pub solution: Solution, pub capped: bool, pub spent_ms: u64, pub spent_evals: usize, pub verdict: ThinkVerdict } |  |  |  |  |

| `ThinkBudget` | struct | ThinkBudget { pub max_evaluated: Option<usize>, pub wall_ms: Option<u64>, pub memory_mb: Option<usize> } |  |  |  |  |

| `UNWALLED_EVALS` | const | UNWALLED_EVALS: usize |  |  |  |  |

| `WALL_FRAC` | const | WALL_FRAC: f64 |  |  |  |  |

| `Bet` | enum | Bet { First, Rest } |  |  |  |  |

| `FF_NO_TCOMPRESS` | env_key | std::env::var("FF_NO_TCOMPRESS") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> (Domain, Problem) |  |  |  |  |

| `declines` | function | declines(domain: &Domain, problem: &Problem) -> Option<&'static str> |  |  |  |  |

| `lay_out` | function | lay_out( domain: &Domain, task: &PackedTask, ops: &[usize], shift: bool, ) -> Option<TimedPlan> |  |  |  |  |

| `solve` | function | solve(domain: &Domain, problem: &Problem, threads: usize, bet: Bet) -> Option<TimedPlan> |  |  |  |  |

| `FF_H_ENDGATE` | env_key | std::env::var("FF_H_ENDGATE") |  |  |  |  |

| `FF_LAX_HELPFUL` | env_key | std::env::var("FF_LAX_HELPFUL") |  |  |  |  |

| `FF_NOREL` | env_key | std::env::var("FF_NOREL") |  |  |  |  |

| `FF_NO_LADDER_DEDUP` | env_key | std::env::var("FF_NO_LADDER_DEDUP") |  |  |  |  |

| `FF_NO_SAT` | env_key | std::env::var("FF_NO_SAT") |  |  |  |  |

| `FF_NO_TSUCC` | env_key | std::env::var("FF_NO_TSUCC") |  |  |  |  |

| `FF_NO_TSYMM` | env_key | std::env::var("FF_NO_TSYMM") |  |  |  |  |

| `FF_ORBIT_DEBUG` | env_key | std::env::var("FF_ORBIT_DEBUG") |  |  |  |  |

| `FF_ORBIT_GEN` | env_key | std::env::var("FF_ORBIT_GEN") |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `FF_TAGENDA_W` | env_key | std::env::var("FF_TAGENDA_W") |  |  |  |  |

| `FF_TAGENDA_W_PRUNE` | env_key | std::env::var("FF_TAGENDA_W_PRUNE") |  |  |  |  |

| `FF_TB_FREE_G` | env_key | std::env::var("FF_TB_FREE_G") |  |  |  |  |

| `FF_TDEMAND_W` | env_key | std::env::var("FF_TDEMAND_W") |  |  |  |  |

| `FF_TEMPORAL_ABS_KEY` | env_key | std::env::var("FF_TEMPORAL_ABS_KEY") |  |  |  |  |

| `FF_TEMPORAL_NODE_CAP` | env_key | std::env::var("FF_TEMPORAL_NODE_CAP") |  |  |  |  |

| `FF_TEVAL_BUDGET` | env_key | std::env::var("FF_TEVAL_BUDGET") |  |  |  |  |

| `FF_TLAMA` | env_key | std::env::var("FF_TLAMA") |  |  |  |  |

| `FF_TLIFO` | env_key | std::env::var("FF_TLIFO") |  |  |  |  |

| `FF_TRPG` | env_key | std::env::var("FF_TRPG") |  |  |  |  |

| `FF_WALL_DEBUG` | env_key | std::env::var("FF_WALL_DEBUG") |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> TemporalCompiled |  |  |  |  |

| `is_temporal` | function | is_temporal(domain: &Domain) -> bool |  |  |  |  |

| `prepare` | function | prepare(domain: &'a Domain, problem: &'a Problem) -> Option<Self> |  |  |  |  |

| `score` | function | score(&self, plan: &TimedPlan) -> Option<SoftScore> |  |  |  |  |

| `score_soft` | function | score_soft(domain: &Domain, problem: &Problem, plan: &TimedPlan) -> Option<SoftScore> |  |  |  |  |

| `solve` | function | solve(domain: &Domain, problem: &Problem, threads: usize) -> Option<TimedPlan> |  |  |  |  |

| `solve_scored` | function | solve_scored(domain: &Domain, problem: &Problem, threads: usize) -> Option<ScoredPlan> |  |  |  |  |

| `to_ipc` | function | to_ipc(&self) -> String |  |  |  |  |

| `validate` | function | validate(domain: &Domain, problem: &Problem, plan: &TimedPlan) -> Result<(), String> |  |  |  |  |

| `ScoredPlan` | struct | ScoredPlan { pub plan: TimedPlan, pub score: Option<SoftScore>, pub unscored: bool } |  |  |  |  |

| `SnapInfo` | struct | SnapInfo { pub start_action: Sym, pub end_action: Sym, pub running_pred: Sym, pub duration: Duration, pub invariant: Formula, pub params: Vec<(Sym, Sym)> } |  |  |  |  |

| `SoftScore` | struct | SoftScore { pub metric: Option<f64>, pub violated: Vec<String>, pub satisfied: usize } |  |  |  |  |

| `SoftScorer` | struct | SoftScorer { domain: &'a Domain, problem: &'a Problem, objs: HashMap<Sym, Vec<Sym>>, goal_prefs: Vec<(String, Formula)>, exp: crate::constraints::Expanded, c: TemporalCompiled, task: PackedTask } |  |  |  |  |

| `TemporalCompiled` | struct | TemporalCompiled { pub domain: Domain, pub problem: Problem, pub snaps: Vec<SnapInfo>, pub til_ops: Vec<(f64, Sym)> } |  |  |  |  |

| `TimedPlan` | struct | TimedPlan { pub steps: Vec<TimedStep>, pub makespan: f64 } |  |  |  |  |

| `TimedStep` | struct | TimedStep { pub time: f64, pub action: String, pub duration: Option<f64> } |  |  |  |  |

| `trace` | function | trace( domain_src: &str, problem_src: &str, plan: &[(String, Vec<String>)], ) -> Result<Vec<StateSnapshot>, String> |  |  |  |  |

| `
    (define (domain logi) (:requirements :typing)
      (:types location truck)
      (:predicates (at ?t - truck ?l - location) (road ?a ?b - location))
      (:action drive :parameters (?t - truck ?from ?to - location)
        :precondition (and (at ?t ?from) (road ?from ?to))
        :effect (and (not (at ?t ?from)) (at ?t ?to))))` | str_key | DOM = "
    (define (domain logi) (:requirements :typing)
      (:types location truck)
      (:predicates (at ?t - truck ?l - location) (road ?a ?b - location))
      (:action drive :parameters (?t - truck ?from ?to - location)
        :precondition (and (at ?t ?from) (road ?from ?to))
        :effect (and (not (at ?t ?from)) (at ?t ?to))))" |  |  |  |  |

| `
    (define (problem p) (:domain logi)
      (:objects a b - location  t1 - truck)
      (:init (at t1 a) (road a b))
      (:goal (at t1 b)))` | str_key | PRB = "
    (define (problem p) (:domain logi)
      (:objects a b - location  t1 - truck)
      (:init (at t1 a) (road a b))
      (:goal (at t1 b)))" |  |  |  |  |

| `StateSnapshot` | struct | StateSnapshot { pub facts: Vec<String>, pub fluents: Vec<(String, f64)> } |  |  |  |  |

| `FF_RES_DEBUG` | env_key | std::env::var("FF_RES_DEBUG") |  |  |  |  |

| `solve` | function | solve(domain: &Domain, problem: &Problem, threads: usize) -> Option<TimedPlan> |  |  |  |  |

| `n_actors` | function | n_actors(domain: &Domain, problem: &Problem) -> usize |  |  |  |  |

| `reschedule` | function | reschedule(domain: &Domain, problem: &Problem, plan: &TimedPlan) -> Option<TimedPlan> |  |  |  |  |

| `single_actor_problem` | function | single_actor_problem(domain: &Domain, problem: &Problem) -> Problem |  |  |  |  |

| `DURATION_PSEUDO` | const | DURATION_PSEUDO: &str |  |  |  |  |

| `AssignOp` | enum | AssignOp { Assign, Increase, Decrease, ScaleUp, ScaleDown } |  |  |  |  |

| `CompOp` | enum | CompOp { Lt, Le, Eq, Ge, Gt } |  |  |  |  |

| `Constraint` | enum | Constraint { And(Vec<Constraint>), Forall(Vec<(Sym, Sym)>, Box<Constraint>), Pref(Option<Sym>, Box<Constraint>), Always(Formula), Sometime(Formula), AtMostOnce(Formula), SometimeAfter(Formula, Formula), SometimeBefore(Formula, Formula), AtEnd(Formula), Within(f64, Formula), AlwaysWithin(f64, Formula, Formula), HoldDuring(f64, f64, Formula), HoldAfter(f64, Formula) } |  |  |  |  |

| `Effect` | enum | Effect { Add(Sym, Vec<Term>), Del(Sym, Vec<Term>), Num(AssignOp, Sym, Vec<Term>, Expr), And(Vec<Effect>), When(Formula, Box<Effect>), Forall(Vec<(Sym, Sym)>, Box<Effect>) } |  |  |  |  |

| `Expr` | enum | Expr { Num(f64), Fluent(Sym, Vec<Term>), Add(Box<Expr>, Box<Expr>), Sub(Box<Expr>, Box<Expr>), Mul(Box<Expr>, Box<Expr>), Div(Box<Expr>, Box<Expr>), Neg(Box<Expr>) } |  |  |  |  |

| `Formula` | enum | Formula { And(Vec<Formula>), Or(Vec<Formula>), Not(Box<Formula>), Atom(Sym, Vec<Term>), Comp(CompOp, Expr, Expr), Forall(Vec<(Sym, Sym)>, Box<Formula>), Exists(Vec<(Sym, Sym)>, Box<Formula>), Eq(Term, Term), Pref(Option<Sym>, Box<Formula>), True, False } |  |  |  |  |

| `MetricDir` | enum | MetricDir { Minimize, Maximize } |  |  |  |  |

| `NExpr` | enum | NExpr { Num(f64), Fluent(u32), Add(Box<NExpr>, Box<NExpr>), Sub(Box<NExpr>, Box<NExpr>), Mul(Box<NExpr>, Box<NExpr>), Div(Box<NExpr>, Box<NExpr>), Neg(Box<NExpr>) } |  |  |  |  |

| `Term` | enum | Term { Var(Sym), Const(Sym) } |  |  |  |  |

| `TimeSpec` | enum | TimeSpec { Start, End, All } |  |  |  |  |

| `chosen` | function | chosen(&self) -> Option<&Expr> |  |  |  |  |

| `collect_fluents` | function | collect_fluents(&self, out: &mut Vec<u32>) |  |  |  |  |

| `eval` | function | eval(&self, fv: &[f64], def: &[bool]) -> Option<f64> |  |  |  |  |

| `eval_numpre` | function | eval_numpre(np: &NumPre, fv: &[f64], def: &[bool]) -> Option<bool> |  |  |  |  |

| `fixed` | function | fixed(e: Expr) -> Self |  |  |  |  |

| `new` | function | new(line: u32, message: impl Into<String>) -> Self |  |  |  |  |

| `?DURATION` | str_key | DURATION_PSEUDO = "?DURATION" |  |  |  |  |

| `Action` | struct | Action { pub name: Sym, pub params: Vec<(Sym, Sym)>, pub precond: Formula, pub effect: Effect, pub monitored: bool } |  |  |  |  |

| `DerivedRule` | struct | DerivedRule { pub head: Sym, pub params: Vec<(Sym, Sym)>, pub body: Formula } |  |  |  |  |

| `Domain` | struct | Domain { pub name: Sym, pub requirements: Vec<Sym>, pub types: Vec<Sym>, pub type_parent: Vec<(Sym, Sym)>, pub constants: Vec<(Sym, Sym)>, pub predicates: Vec<(Sym, Vec<Sym>)>, pub functions: Vec<(Sym, Vec<Sym>)>, pub actions: Vec<Action>, pub durative_actions: Vec<DurativeAction>, pub constraints: Vec<Constraint>, pub derived: Vec<DerivedRule>, pub monitors: Vec<Effect> } |  |  |  |  |

| `Duration` | struct | Duration { pub min: Option<Expr>, pub max: Option<Expr> } |  |  |  |  |

| `DurativeAction` | struct | DurativeAction { pub name: Sym, pub params: Vec<(Sym, Sym)>, pub duration: Duration, pub conditions: Vec<(TimeSpec, Formula)>, pub effects: Vec<(TimeSpec, Effect)> } |  |  |  |  |

| `NumEff` | struct | NumEff { pub op: AssignOp, pub target: u32, pub value: NExpr } |  |  |  |  |

| `NumPre` | struct | NumPre { pub op: CompOp, pub lhs: NExpr, pub rhs: NExpr } |  |  |  |  |

| `ParseError` | struct | ParseError { pub line: u32, pub message: String } |  |  |  |  |

| `Problem` | struct | Problem { pub name: Sym, pub domain_name: Sym, pub objects: Vec<(Sym, Sym)>, pub init_atoms: Vec<(Sym, Vec<Sym>)>, pub init_fluents: Vec<((Sym, Vec<Sym>), f64)>, pub til: Vec<TimedLiteral>, pub goal: Formula, pub constraints: Vec<Constraint>, pub metric: Option<(MetricDir, Expr)> } |  |  |  |  |

| `TimedLiteral` | struct | TimedLiteral { pub time: f64, pub add: bool, pub pred: Sym, pub args: Vec<Sym> } |  |  |  |  |

| `verify` | function | verify( domain_src: &str, problem_src: &str, plan: &[(String, Vec<String>)], ) -> Result<Verified, String> |  |  |  |  |

| `Verified` | struct | Verified { pub metric: f64, pub hard_goal_met: bool, pub satisfied: usize, pub violated: usize, pub constraints_met: bool, pub constraint_failures: Vec<String>, pub constraint_prefs: Vec<(String, bool)> } |  |  |  |  |

| `PredKind` | enum | PredKind { Edge, Position, Property } |  |  |  |  |

| `build` | function | build(domain: &Domain, problem: &Problem) -> Self |  |  |  |  |

| `domain_to_pddl` | function | domain_to_pddl( name: &str, requirements: &str, types: &[(String, String)], predicates: &[(String, Vec<String>)], actions_raw: &[String], ) -> String |  |  |  |  |

| `dynamic_predicates` | function | dynamic_predicates(domain: &Domain) -> BTreeSet<String> |  |  |  |  |

| `goal_facts` | function | goal_facts(problem: &Problem) -> Vec<(String, Vec<String>)> |  |  |  |  |

| `positions_at` | function | positions_at(&self, facts: &[String]) -> HashMap<String, Option<String>> |  |  |  |  |

| `to_pddl` | function | to_pddl( name: &str, domain_name: &str, objects: &[(String, String)], init: &[(String, Vec<String>)], goal: &[(String, Vec<String>)], ) -> String |  |  |  |  |

| `
    (define (domain logi) (:requirements :typing)
      (:types location truck package)
      (:predicates (at ?x - truck ?l - location) (road ?a ?b - location)
                   (in ?p - package ?t - truck) (delivered ?p - package))
      (:action drive :parameters (?t - truck ?from ?to - location)
        :precondition (and (at ?t ?from) (road ?from ?to))
        :effect (and (not (at ?t ?from)) (at ?t ?to))))` | str_key | DOM = "
    (define (domain logi) (:requirements :typing)
      (:types location truck package)
      (:predicates (at ?x - truck ?l - location) (road ?a ?b - location)
                   (in ?p - package ?t - truck) (delivered ?p - package))
      (:action drive :parameters (?t - truck ?from ?to - location)
        :precondition (and (at ?t ?from) (road ?from ?to))
        :effect (and (not (at ?t ?from)) (at ?t ?to))))" |  |  |  |  |

| `
    (define (problem p) (:domain logi)
      (:objects a b c - location  t1 - truck  p1 - package)
      (:init (at t1 a) (road a b) (road b c) (in p1 t1))
      (:goal (delivered p1)))` | str_key | PRB = "
    (define (problem p) (:domain logi)
      (:objects a b c - location  t1 - truck  p1 - package)
      (:init (at t1 a) (road a b) (road b c) (in p1 t1))
      (:goal (delivered p1)))" |  |  |  |  |

| `AT` | str_key | POSITION_NAMES = "AT" |  |  |  |  |

| `ROAD` | str_key | EDGE_NAMES = "ROAD" |  |  |  |  |

| `VizEdge` | struct | VizEdge { pub a: String, pub b: String, pub pred: String } |  |  |  |  |

| `VizGraph` | struct | VizGraph { pub nodes: Vec<VizNode>, pub edges: Vec<VizEdge>, pub mobiles: Vec<VizMobile>, pub props_by_object: BTreeMap<String, Vec<String>>, pub goal_by_object: BTreeMap<String, Vec<String>>, pub pred_kind: BTreeMap<String, PredKind>, pub location_types: BTreeSet<String> } |  |  |  |  |

| `VizMobile` | struct | VizMobile { pub object: String, pub ty: String, pub at: Option<String>, pub at_raw: Option<String> } |  |  |  |  |

| `VizNode` | struct | VizNode { pub object: String, pub ty: String } |  |  |  |  |

| `
(define (domain roads)
  (:requirements :strips :typing :action-costs)
  (:types loc)
  (:constants a b c - loc)
  (:predicates (at ?l - loc))
  (:functions (total-cost) - number)
  (:action hop
    :parameters ()
    :precondition (at a)
    :effect (and (not (at a)) (at c) (increase (total-cost) 10)))
  (:action step1
    :parameters ()
    :precondition (at a)
    :effect (and (not (at a)) (at b) (increase (total-cost) 1)))
  (:action step2
    :parameters ()
    :precondition (at b)
    :effect (and (not (at b)) (at c) (increase (total-cost) 1))))
` | str_key | ROADS_DOMAIN = "
(define (domain roads)
  (:requirements :strips :typing :action-costs)
  (:types loc)
  (:constants a b c - loc)
  (:predicates (at ?l - loc))
  (:functions (total-cost) - number)
  (:action hop
    :parameters ()
    :precondition (at a)
    :effect (and (not (at a)) (at c) (increase (total-cost) 10)))
  (:action step1
    :parameters ()
    :precondition (at a)
    :effect (and (not (at a)) (at b) (increase (total-cost) 1)))
  (:action step2
    :parameters ()
    :precondition (at b)
    :effect (and (not (at b)) (at c) (increase (total-cost) 1))))
" |  |  |  |  |

| `
(define (problem roads-1) (:domain roads)
  (:init (at a) (= (total-cost) 0))
  (:goal (at c))
  (:metric minimize (total-cost)))
` | str_key | ROADS_PROBLEM = "
(define (problem roads-1) (:domain roads)
  (:init (at a) (= (total-cost) 0))
  (:goal (at c))
  (:metric minimize (total-cost)))
" |  |  |  |  |

| `(define (domain adl1)
 (:requirements :typing :adl :negative-preconditions)
 (:types item)
 (:predicates (tagged ?x - item) (done) (linked ?a - item ?b - item))
 (:action tag :parameters (?x - item) :precondition (not (tagged ?x)) :effect (tagged ?x))
 (:action link :parameters (?a - item ?b - item)
   :precondition (and (not (= ?a ?b)) (tagged ?a) (tagged ?b))
   :effect (linked ?a ?b))
 (:action finish :parameters ()
   :precondition (and (forall (?x - item) (tagged ?x))
                      (exists (?a - item ?b - item) (linked ?a ?b)))
   :effect (done)))` | str_key | DOM = "(define (domain adl1)
 (:requirements :typing :adl :negative-preconditions)
 (:types item)
 (:predicates (tagged ?x - item) (done) (linked ?a - item ?b - item))
 (:action tag :parameters (?x - item) :precondition (not (tagged ?x)) :effect (tagged ?x))
 (:action link :parameters (?a - item ?b - item)
   :precondition (and (not (= ?a ?b)) (tagged ?a) (tagged ?b))
   :effect (linked ?a ?b))
 (:action finish :parameters ()
   :precondition (and (forall (?x - item) (tagged ?x))
                      (exists (?a - item ?b - item) (linked ?a ?b)))
   :effect (done)))" |  |  |  |  |

| `(define (domain briefcase)
 (:requirements :typing :adl)
 (:types obj loc)
 (:predicates (at-bc ?l - loc) (inbc ?o - obj) (at ?o - obj ?l - loc))
 (:action move :parameters (?from ?to - loc)
   :precondition (at-bc ?from)
   :effect (and (at-bc ?to) (not (at-bc ?from))
                (forall (?o - obj)
                  (when (inbc ?o) (and (at ?o ?to) (not (at ?o ?from)))))))
 (:action putin :parameters (?o - obj ?l - loc)
   :precondition (and (at-bc ?l) (at ?o ?l))
   :effect (inbc ?o))
 (:action takeout :parameters (?o - obj)
   :precondition (inbc ?o)
   :effect (not (inbc ?o))))` | str_key | BRIEFCASE = "(define (domain briefcase)
 (:requirements :typing :adl)
 (:types obj loc)
 (:predicates (at-bc ?l - loc) (inbc ?o - obj) (at ?o - obj ?l - loc))
 (:action move :parameters (?from ?to - loc)
   :precondition (at-bc ?from)
   :effect (and (at-bc ?to) (not (at-bc ?from))
                (forall (?o - obj)
                  (when (inbc ?o) (and (at ?o ?to) (not (at ?o ?from)))))))
 (:action putin :parameters (?o - obj ?l - loc)
   :precondition (and (at-bc ?l) (at ?o ?l))
   :effect (inbc ?o))
 (:action takeout :parameters (?o - obj)
   :precondition (inbc ?o)
   :effect (not (inbc ?o))))" |  |  |  |  |

| `(define (domain toggle)
 (:requirements :adl)
 (:predicates (on) (marker))
 (:action flip :parameters ()
   :precondition (marker)
   :effect (and (when (on) (not (on))) (when (not (on)) (on)))))` | str_key | TOGGLE = "(define (domain toggle)
 (:requirements :adl)
 (:predicates (on) (marker))
 (:action flip :parameters ()
   :precondition (marker)
   :effect (and (when (on) (not (on))) (when (not (on)) (on)))))" |  |  |  |  |

| `(define (domain g)
 (:requirements :strips :typing)
 (:types loc)
 (:predicates (at ?l - loc) (link ?a - loc ?b - loc))
 (:action move :parameters (?a ?b - loc)
   :precondition (and (at ?a) (link ?a ?b)) :effect (and (at ?b) (not (at ?a)))))` | str_key | GRID = "(define (domain g)
 (:requirements :strips :typing)
 (:types loc)
 (:predicates (at ?l - loc) (link ?a - loc ?b - loc))
 (:action move :parameters (?a ?b - loc)
   :precondition (and (at ?a) (link ?a ?b)) :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (domain t)
 (:requirements :strips :typing)
 (:types loc pkg)
 (:predicates (truck-at ?l - loc) (pkg-at ?p - pkg ?l - loc) (in ?p - pkg) (road ?a ?b - loc))
 (:action drive :parameters (?a ?b - loc)
   :precondition (and (truck-at ?a) (road ?a ?b)) :effect (and (truck-at ?b) (not (truck-at ?a))))
 (:action load :parameters (?p - pkg ?l - loc)
   :precondition (and (pkg-at ?p ?l) (truck-at ?l)) :effect (and (in ?p) (not (pkg-at ?p ?l))))
 (:action unload :parameters (?p - pkg ?l - loc)
   :precondition (and (in ?p) (truck-at ?l)) :effect (and (pkg-at ?p ?l) (not (in ?p)))))` | str_key | TRANSPORT = "(define (domain t)
 (:requirements :strips :typing)
 (:types loc pkg)
 (:predicates (truck-at ?l - loc) (pkg-at ?p - pkg ?l - loc) (in ?p - pkg) (road ?a ?b - loc))
 (:action drive :parameters (?a ?b - loc)
   :precondition (and (truck-at ?a) (road ?a ?b)) :effect (and (truck-at ?b) (not (truck-at ?a))))
 (:action load :parameters (?p - pkg ?l - loc)
   :precondition (and (pkg-at ?p ?l) (truck-at ?l)) :effect (and (in ?p) (not (pkg-at ?p ?l))))
 (:action unload :parameters (?p - pkg ?l - loc)
   :precondition (and (in ?p) (truck-at ?l)) :effect (and (pkg-at ?p ?l) (not (in ?p)))))" |  |  |  |  |

| `parse_domain` | str_key | TARGETS_PARSER = "parse_domain" |  |  |  |  |

| `preprocess` | str_key | TARGETS_PREPROCESS = "preprocess" |  |  |  |  |

| `CALL_BUDGET_CHILD` | env_key | std::env::var("CALL_BUDGET_CHILD") |  |  |  |  |

| `FERROPLAN_CORPUS_DIR` | env_key | std::env::var_os("FERROPLAN_CORPUS_DIR") |  |  |  |  |

| `FERROPLAN_ORACLE_DIR` | env_key | std::env::var_os("FERROPLAN_ORACLE_DIR") |  |  |  |  |

| `FERROPLAN_RUN_DIR` | env_key | std::env::var_os("FERROPLAN_RUN_DIR") |  |  |  |  |

| `HOME` | env_key | std::env::var_os("HOME") |  |  |  |  |

| `corpus_dir` | function | corpus_dir() -> PathBuf |  |  |  |  |

| `corpus_ipc_dir` | function | corpus_ipc_dir() -> PathBuf |  |  |  |  |

| `differential_run_dir` | function | differential_run_dir() -> PathBuf |  |  |  |  |

| `harness_present` | function | harness_present(what: &str, path: &Path) -> bool |  |  |  |  |

| `oracle_dir` | function | oracle_dir() -> PathBuf |  |  |  |  |

| `oracle_runner` | function | oracle_runner() -> PathBuf |  |  |  |  |

| `PROVENANCE_DIFF_T61` | const | PROVENANCE_DIFF_T61: &[&str] |  |  |  |  |

| `PROVENANCE_FUZZ_T31` | const | PROVENANCE_FUZZ_T31: &[&str] |  |  |  |  |

| `base_sizes` | function | base_sizes(rng: &mut Rng) -> Sizes |  |  |  |  |

| `below` | function | below(&mut self, n: u64) -> u64 |  |  |  |  |

| `chance` | function | chance(&mut self, percent: u64) -> bool |  |  |  |  |

| `draw_for` | function | draw_for(seed: u64, sizes: Sizes) -> (String, String) |  |  |  |  |

| `draw_valid` | function | draw_valid(seed: u64, sizes: Sizes) -> (String, String) |  |  |  |  |

| `generate` | function | generate(seed: u64, sizes: Sizes) -> Model |  |  |  |  |

| `generate_valid` | function | generate_valid(seed: u64, sizes: Sizes) -> Model |  |  |  |  |

| `halve_sizes` | function | halve_sizes(s: Sizes) -> Sizes |  |  |  |  |

| `mutation_of` | function | mutation_of(seed: u64) -> bool |  |  |  |  |

| `new` | function | new(seed: u64) -> Self |  |  |  |  |

| `next_u64` | function | next_u64(&mut self) -> u64 |  |  |  |  |

| `pick_idx` | function | pick_idx(&mut self, len: usize) -> usize |  |  |  |  |

| `range` | function | range(&mut self, lo: u64, hi: u64) -> u64 |  |  |  |  |

| `range_usize` | function | range_usize(&mut self, lo: usize, hi: usize) -> usize |  |  |  |  |

| `render` | function | render(model: &Model, problem_name: &str) -> (String, String) |  |  |  |  |

| `render_with_provenance` | function | render_with_provenance( model: &Model, problem_name: &str, header: &[&str], ) -> (String, String) |  |  |  |  |

| `sizes_for` | function | sizes_for(seed: u64) -> Sizes |  |  |  |  |

| `;; differential-fuzz: self-authored seeded VALID draw (ticket fond-htn-61,` | str_key | PROVENANCE_DIFF_T61 = ";; differential-fuzz: self-authored seeded VALID draw (ticket fond-htn-61," |  |  |  |  |

| `;; fuzz-found: self-authored seeded draw (ticket fond-htn-31,` | str_key | PROVENANCE_FUZZ_T31 = ";; fuzz-found: self-authored seeded draw (ticket fond-htn-31," |  |  |  |  |

| `Model` | struct | Model { types: Vec<String>, preds: Vec<(String, Vec<usize>)>, actions: Vec<ActionM>, tasks: Vec<(String, Vec<(String, usize)>)>, methods: Vec<MethodM>, objects: Vec<(String, usize)>, init: Vec<LitO>, goal: Vec<LitO>, root: Vec<(String, CallM)> } |  |  |  |  |

| `Rng` | struct | Rng { u64 } |  |  |  |  |

| `Sizes` | struct | Sizes { pub types: usize, pub preds: usize, pub tasks: usize, pub actions: usize, pub objects: usize, pub subs: usize, pub root_subs: usize } |  |  |  |  |

| `
(define (domain cond-pref)
  (:requirements :typing :durative-actions :preferences)
  (:types thing)
  (:predicates (clean ?x - thing) (ready ?x - thing) (moved ?x - thing) (quiet))
  (:durative-action move
    :parameters (?x - thing)
    :duration (= ?duration 1)
    :condition (and (preference pc (at start (clean ?x)))
                    (preference pall (at start (forall (?y - thing) (clean ?y))))
                    (at start (ready ?x))
                    (at start (preference pq (quiet)))
                    (preference po (over all (quiet))))
    :effect (and (at start (not (ready ?x))) (at end (moved ?x)))))
` | str_key | COND_DOMAIN = "
(define (domain cond-pref)
  (:requirements :typing :durative-actions :preferences)
  (:types thing)
  (:predicates (clean ?x - thing) (ready ?x - thing) (moved ?x - thing) (quiet))
  (:durative-action move
    :parameters (?x - thing)
    :duration (= ?duration 1)
    :condition (and (preference pc (at start (clean ?x)))
                    (preference pall (at start (forall (?y - thing) (clean ?y))))
                    (at start (ready ?x))
                    (at start (preference pq (quiet)))
                    (preference po (over all (quiet))))
    :effect (and (at start (not (ready ?x))) (at end (moved ?x)))))
" |  |  |  |  |

| `
(define (domain cp-mini)
  (:requirements :strips :durative-actions :constraints :preferences)
  (:predicates (fresh-work) (fresh-wave) (done) (waved) (never-obtainable))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 5)
    :condition (at start (fresh-work))
    :effect (and (at start (not (fresh-work))) (at end (done))))
  (:durative-action wave
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-wave))
    :effect (and (at start (not (fresh-wave))) (at end (waved)))))
` | str_key | DOMAIN = "
(define (domain cp-mini)
  (:requirements :strips :durative-actions :constraints :preferences)
  (:predicates (fresh-work) (fresh-wave) (done) (waved) (never-obtainable))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 5)
    :condition (at start (fresh-work))
    :effect (and (at start (not (fresh-work))) (at end (done))))
  (:durative-action wave
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-wave))
    :effect (and (at start (not (fresh-wave))) (at end (waved)))))
" |  |  |  |  |

| `
(define (problem cond-pref-1) (:domain cond-pref)
  (:objects a b - thing)
  (:init (clean a) (ready a) (ready b) (quiet))
  (:goal (and (moved a) (moved b)))
  (:metric minimize (+ (* 1 (is-violated pc)) (* 10 (is-violated pall))
                       (* 100 (is-violated pq)) (* 1000 (is-violated po)))))
` | str_key | COND_PROBLEM = "
(define (problem cond-pref-1) (:domain cond-pref)
  (:objects a b - thing)
  (:init (clean a) (ready a) (ready b) (quiet))
  (:goal (and (moved a) (moved b)))
  (:metric minimize (+ (* 1 (is-violated pc)) (* 10 (is-violated pall))
                       (* 100 (is-violated pq)) (* 1000 (is-violated po)))))
" |  |  |  |  |

| `
(define (problem cp-mixed) (:domain cp-mini)
  (:init (fresh-work) (fresh-wave))
  (:goal (and (done) (preference gp (waved))))
  (:constraints (and (preference cp (sometime (waved)))
                     (preference cq (sometime (never-obtainable)))))
  (:metric minimize (+ (* 2 (is-violated gp))
                       (* 3 (is-violated cp))
                       (* 5 (is-violated cq)))))
` | str_key | P_MIXED = "
(define (problem cp-mixed) (:domain cp-mini)
  (:init (fresh-work) (fresh-wave))
  (:goal (and (done) (preference gp (waved))))
  (:constraints (and (preference cp (sometime (waved)))
                     (preference cq (sometime (never-obtainable)))))
  (:metric minimize (+ (* 2 (is-violated gp))
                       (* 3 (is-violated cp))
                       (* 5 (is-violated cq)))))
" |  |  |  |  |

| `
(define (problem cp-sat) (:domain cp-mini)
  (:init (fresh-work) (fresh-wave))
  (:goal (and (done) (preference gp (waved))))
  (:constraints (preference cp (sometime (waved))))
  (:metric minimize (+ (* 2 (is-violated gp)) (* 3 (is-violated cp)))))
` | str_key | P_SATISFIABLE = "
(define (problem cp-sat) (:domain cp-mini)
  (:init (fresh-work) (fresh-wave))
  (:goal (and (done) (preference gp (waved))))
  (:constraints (preference cp (sometime (waved))))
  (:metric minimize (+ (* 2 (is-violated gp)) (* 3 (is-violated cp)))))
" |  |  |  |  |

| `(define (domain sw)
  (:requirements :strips :constraints)
  (:predicates (on) (off) (lamp) (used))
  (:action flip-on :precondition (off) :effect (and (not (off)) (on)))
  (:action flip-off :precondition (on) :effect (and (not (on)) (off)))
  (:action light :precondition (on) :effect (and (lamp) (used))))` | str_key | DOM = "(define (domain sw)
  (:requirements :strips :constraints)
  (:predicates (on) (off) (lamp) (used))
  (:action flip-on :precondition (off) :effect (and (not (off)) (on)))
  (:action flip-off :precondition (on) :effect (and (not (on)) (off)))
  (:action light :precondition (on) :effect (and (lamp) (used))))" |  |  |  |  |

| `(define (domain sws)
  (:requirements :strips :constraints)
  (:predicates (on) (off) (lamp) (used) (linked))
  (:action flip-on :precondition (off) :effect (and (not (off)) (on)))
  (:action flip-off :precondition (on) :effect (and (not (on)) (off)))
  (:action light :precondition (on) :effect (and (lamp) (used))))` | str_key | DOMS = "(define (domain sws)
  (:requirements :strips :constraints)
  (:predicates (on) (off) (lamp) (used) (linked))
  (:action flip-on :precondition (off) :effect (and (not (off)) (on)))
  (:action flip-off :precondition (on) :effect (and (not (on)) (off)))
  (:action light :precondition (on) :effect (and (lamp) (used))))" |  |  |  |  |

| `../../../examples/daily_agent_methods/domain.ppddl` | str_key | DOMAIN = "../../../examples/daily_agent_methods/domain.ppddl" |  |  |  |  |

| `../../../examples/daily_agent_methods/method-catalog.json` | str_key | CATALOG = "../../../examples/daily_agent_methods/method-catalog.json" |  |  |  |  |

| `../../../examples/daily_agent_methods/problem-2026-07-31.ppddl` | str_key | PROBLEM = "../../../examples/daily_agent_methods/problem-2026-07-31.ppddl" |  |  |  |  |

| `../../../examples/daily_agent_methods/receipt-2026-07-31.json` | str_key | RECEIPT = "../../../examples/daily_agent_methods/receipt-2026-07-31.json" |  |  |  |  |

| `
(define (domain acc)
  (:requirements :durative-actions :numeric-fluents)
  (:functions (x))
  (:durative-action step :parameters () :duration (= ?duration 1)
    :condition () :effect (at end (increase (x) 1))))
` | str_key | SINGLE = "
(define (domain acc)
  (:requirements :durative-actions :numeric-fluents)
  (:functions (x))
  (:durative-action step :parameters () :duration (= ?duration 1)
    :condition () :effect (at end (increase (x) 1))))
" |  |  |  |  |

| `
(define (domain mk)
  (:requirements :durative-actions :numeric-fluents)
  (:functions (a) (b))
  (:durative-action make-a :parameters () :duration (= ?duration 2)
    :condition () :effect (at end (increase (a) 1)))
  (:durative-action make-b :parameters () :duration (= ?duration 3)
    :condition () :effect (at end (increase (b) 1))))
` | str_key | TWO_DELIVERABLES = "
(define (domain mk)
  (:requirements :durative-actions :numeric-fluents)
  (:functions (a) (b))
  (:durative-action make-a :parameters () :duration (= ?duration 2)
    :condition () :effect (at end (increase (a) 1)))
  (:durative-action make-b :parameters () :duration (= ?duration 3)
    :condition () :effect (at end (increase (b) 1))))
" |  |  |  |  |

| `(define (problem p) (:domain acc) (:init (= (x) 0)) (:goal (>= (x) 3)))` | str_key | SINGLE_PROB = "(define (problem p) (:domain acc) (:init (= (x) 0)) (:goal (>= (x) 3)))" |  |  |  |  |

| `(define (problem p) (:domain mk)
  (:init (= (a) 0) (= (b) 0))
  (:goal (and (>= (a) 1) (>= (b) 1))))` | str_key | TWO_PROB = "(define (problem p) (:domain mk)
  (:init (= (a) 0) (= (b) 0))
  (:goal (and (>= (a) 1) (>= (b) 1))))" |  |  |  |  |

| `DIFFERENTIAL_FUZZ_FRESH` | env_key | std::env::var("DIFFERENTIAL_FUZZ_FRESH") |  |  |  |  |

| `CARGO_MANIFEST_DIR` | str_key | LEDGER_PATH = "CARGO_MANIFEST_DIR" |  |  |  |  |

| `flexible` | str_key | ORACLE_MODE = "flexible" |  |  |  |  |

| `redecomposition` | str_key | KNOWN_DIVERGENCES = "redecomposition" |  |  |  |  |

| `FF_H_ENDGATE` | env_key | std::env::var("FF_H_ENDGATE") |  |  |  |  |

| `
(define (domain endgate)
  (:predicates (ga) (gb))
  (:action snap-start :parameters () :effect (ga))
  (:action snap-end   :parameters () :effect (gb)))` | str_key | SNAP_DOM = "
(define (domain endgate)
  (:predicates (ga) (gb))
  (:action snap-start :parameters () :effect (ga))
  (:action snap-end   :parameters () :effect (gb)))" |  |  |  |  |

| `ENRICH_CAP` | env_key | std::env::var("ENRICH_CAP") |  |  |  |  |

| `ENRICH_CHILD` | env_key | std::env::var("ENRICH_CHILD") |  |  |  |  |

| `ENRICH_K` | env_key | std::env::var("ENRICH_K") |  |  |  |  |

| `
(define (domain adlfold)
  (:requirements :adl :typing :numeric-fluents)
  (:types box)
  (:predicates (open ?b - box) (heavy ?b - box) (moved ?b - box))
  (:functions (weight ?b - box) (carried))
  (:action move
    :parameters (?b - box)
    :precondition (not (moved ?b))
    :effect (and (moved ?b)
                 (when (heavy ?b) (increase (carried) (weight ?b))))))` | str_key | ADL_DOM = "
(define (domain adlfold)
  (:requirements :adl :typing :numeric-fluents)
  (:types box)
  (:predicates (open ?b - box) (heavy ?b - box) (moved ?b - box))
  (:functions (weight ?b - box) (carried))
  (:action move
    :parameters (?b - box)
    :precondition (not (moved ?b))
    :effect (and (moved ?b)
                 (when (heavy ?b) (increase (carried) (weight ?b))))))" |  |  |  |  |

| `
(define (domain costfold)
  (:requirements :strips :typing :action-costs)
  (:types loc)
  (:predicates (at ?l - loc) (road ?a ?b - loc) (visited ?l - loc))
  (:functions (total-cost) (toll ?a ?b - loc))
  (:action go
    :parameters (?a ?b - loc)
    :precondition (and (at ?a) (road ?a ?b))
    :effect (and (not (at ?a)) (at ?b) (visited ?b)
                 (increase (total-cost) (toll ?a ?b)))))` | str_key | COSTS_DOM = "
(define (domain costfold)
  (:requirements :strips :typing :action-costs)
  (:types loc)
  (:predicates (at ?l - loc) (road ?a ?b - loc) (visited ?l - loc))
  (:functions (total-cost) (toll ?a ?b - loc))
  (:action go
    :parameters (?a ?b - loc)
    :precondition (and (at ?a) (road ?a ?b))
    :effect (and (not (at ?a)) (at ?b) (visited ?b)
                 (increase (total-cost) (toll ?a ?b)))))" |  |  |  |  |

| `
(define (domain durexpr)
  (:requirements :strips :durative-actions :numeric-fluents)
  (:predicates (ready) (boosted) (done))
  (:functions (speed) (progress))
  (:durative-action boost
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (ready))
    :effect (and (at start (not (ready)))
                 (at start (increase (speed) 2))
                 (at end (boosted))))
  (:durative-action work
    :parameters ()
    :duration (= ?duration (speed))
    :condition (at start (boosted))
    :effect (and (at start (increase (progress) ?duration))
                 (at end (done)))))` | str_key | DUREXPR_DOM = "
(define (domain durexpr)
  (:requirements :strips :durative-actions :numeric-fluents)
  (:predicates (ready) (boosted) (done))
  (:functions (speed) (progress))
  (:durative-action boost
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (ready))
    :effect (and (at start (not (ready)))
                 (at start (increase (speed) 2))
                 (at end (boosted))))
  (:durative-action work
    :parameters ()
    :duration (= ?duration (speed))
    :condition (at start (boosted))
    :effect (and (at start (increase (progress) ?duration))
                 (at end (done)))))" |  |  |  |  |

| `
(define (domain minitpp)
  (:requirements :strips :typing :numeric-fluents)
  (:types place goods)
  (:predicates (at ?p - place) (link ?a ?b - place))
  (:functions (price ?g - goods ?p - place) (bought ?g - goods)
              (request ?g - goods) (spent))
  (:action drive
    :parameters (?a ?b - place)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (not (at ?a)) (at ?b)))
  (:action buy
    :parameters (?g - goods ?p - place)
    :precondition (and (at ?p) (< (bought ?g) (request ?g)))
    :effect (and (increase (bought ?g) 1)
                 (increase (spent) (price ?g ?p)))))` | str_key | MINITPP_DOM = "
(define (domain minitpp)
  (:requirements :strips :typing :numeric-fluents)
  (:types place goods)
  (:predicates (at ?p - place) (link ?a ?b - place))
  (:functions (price ?g - goods ?p - place) (bought ?g - goods)
              (request ?g - goods) (spent))
  (:action drive
    :parameters (?a ?b - place)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (not (at ?a)) (at ?b)))
  (:action buy
    :parameters (?g - goods ?p - place)
    :precondition (and (at ?p) (< (bought ?g) (request ?g)))
    :effect (and (increase (bought ?g) 1)
                 (increase (spent) (price ?g ?p)))))" |  |  |  |  |

| `
(define (problem adlfold-1) (:domain adlfold)
  (:objects b1 b2 b3 - box)
  (:init (heavy b1) (heavy b3)
         (= (weight b1) 10) (= (weight b2) 1) (= (weight b3) 3)
         (= (carried) 0))
  (:goal (and (moved b1) (moved b2) (moved b3))))` | str_key | ADL_PRB = "
(define (problem adlfold-1) (:domain adlfold)
  (:objects b1 b2 b3 - box)
  (:init (heavy b1) (heavy b3)
         (= (weight b1) 10) (= (weight b2) 1) (= (weight b3) 3)
         (= (carried) 0))
  (:goal (and (moved b1) (moved b2) (moved b3))))" |  |  |  |  |

| `
(define (problem costfold-1) (:domain costfold)
  (:objects a b c d - loc)
  (:init (at a) (road a b) (road b c) (road a c) (road c d)
         (= (toll a b) 1) (= (toll b c) 1) (= (toll a c) 5) (= (toll c d) 2)
         (= (total-cost) 0))
  (:goal (visited d))
  (:metric minimize (total-cost)))` | str_key | COSTS_PRB = "
(define (problem costfold-1) (:domain costfold)
  (:objects a b c d - loc)
  (:init (at a) (road a b) (road b c) (road a c) (road c d)
         (= (toll a b) 1) (= (toll b c) 1) (= (toll a c) 5) (= (toll c d) 2)
         (= (total-cost) 0))
  (:goal (visited d))
  (:metric minimize (total-cost)))" |  |  |  |  |

| `
(define (problem durexpr-1) (:domain durexpr)
  (:init (ready) (= (speed) 1) (= (progress) 0))
  (:goal (and (done) (>= (progress) 3))))` | str_key | DUREXPR_PRB = "
(define (problem durexpr-1) (:domain durexpr)
  (:init (ready) (= (speed) 1) (= (progress) 0))
  (:goal (and (done) (>= (progress) 3))))" |  |  |  |  |

| `
(define (problem minitpp-1) (:domain minitpp)
  (:objects p1 p2 p3 - place g1 g2 - goods)
  (:init (at p1) (link p1 p2) (link p2 p3) (link p2 p1) (link p3 p2)
         (= (price g1 p1) 4) (= (price g1 p2) 2) (= (price g1 p3) 7)
         (= (price g2 p1) 5) (= (price g2 p2) 9) (= (price g2 p3) 1)
         (= (bought g1) 0) (= (bought g2) 0)
         (= (request g1) 2) (= (request g2) 1)
         (= (spent) 0))
  (:goal (and (>= (bought g1) (request g1)) (>= (bought g2) (request g2)))))` | str_key | MINITPP_PRB = "
(define (problem minitpp-1) (:domain minitpp)
  (:objects p1 p2 p3 - place g1 g2 - goods)
  (:init (at p1) (link p1 p2) (link p2 p3) (link p2 p1) (link p3 p2)
         (= (price g1 p1) 4) (= (price g1 p2) 2) (= (price g1 p3) 7)
         (= (price g2 p1) 5) (= (price g2 p2) 9) (= (price g2 p3) 1)
         (= (bought g1) 0) (= (bought g2) 0)
         (= (request g1) 2) (= (request g2) 1)
         (= (spent) 0))
  (:goal (and (>= (bought g1) (request g1)) (>= (bought g2) (request g2)))))" |  |  |  |  |

| `strong FOND fixed point` | str_key | STRONG_NOTE = "strong FOND fixed point" |  |  |  |  |

| `strong-cyclic FOND fixpoint` | str_key | STRONG_CYCLIC_NOTE = "strong-cyclic FOND fixpoint" |  |  |  |  |

| `tireworld` | str_key | DOMAINS = "tireworld" |  |  |  |  |

| `drop-retry` | str_key | ALL = "drop-retry" |  |  |  |  |

| `fixtures/fond-htn-micro/both-branches-deadend/domain.hddl` | str_key | BOTH_BRANCHES_DEADEND = "fixtures/fond-htn-micro/both-branches-deadend/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/drop-retry/domain.hddl` | str_key | DROP_RETRY = "fixtures/fond-htn-micro/drop-retry/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/grow-loop/domain.hddl` | str_key | GROW_LOOP = "fixtures/fond-htn-micro/grow-loop/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/plain-chain/domain.hddl` | str_key | PLAIN_CHAIN = "fixtures/fond-htn-micro/plain-chain/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/sense-then-branch/domain.hddl` | str_key | SENSE_THEN_BRANCH = "fixtures/fond-htn-micro/sense-then-branch/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/supervisor-fail/domain.hddl` | str_key | SUPERVISOR_FAIL = "fixtures/fond-htn-micro/supervisor-fail/domain.hddl" |  |  |  |  |

| `fixtures/fond-htn-micro/tray-dirty/domain.hddl` | str_key | TRAY_DIRTY = "fixtures/fond-htn-micro/tray-dirty/domain.hddl" |  |  |  |  |

| `CARGO_MANIFEST_DIR` | str_key | FIXTURE_DIR = "CARGO_MANIFEST_DIR" |  |  |  |  |

| `fixtures/fond-htn/oracle-goldens.json` | str_key | GOLDENS_RAW = "fixtures/fond-htn/oracle-goldens.json" |  |  |  |  |

| `ticket floor is 300+ instances` | str_key | _ = "ticket floor is 300+ instances" |  |  |  |  |

| `drop-retry` | str_key | CASES = "drop-retry" |  |  |  |  |

| `fixtures/fond-unsafe/abyss-avoid-problem.hddl` | str_key | ABYSS_AVOID_PROBLEM = "fixtures/fond-unsafe/abyss-avoid-problem.hddl" |  |  |  |  |

| `fixtures/fond-unsafe/abyss-avoid.hddl` | str_key | ABYSS_AVOID_DOMAIN = "fixtures/fond-unsafe/abyss-avoid.hddl" |  |  |  |  |

| `fixtures/fond-unsafe/abyss-both-problem.hddl` | str_key | ABYSS_BOTH_PROBLEM = "fixtures/fond-unsafe/abyss-both-problem.hddl" |  |  |  |  |

| `fixtures/fond-unsafe/abyss-both.hddl` | str_key | ABYSS_BOTH_DOMAIN = "fixtures/fond-unsafe/abyss-both.hddl" |  |  |  |  |

| `freecell_learned_ecai_16` | str_key | CASES = "freecell_learned_ecai_16" |  |  |  |  |

| `GROUND_WALL_CHILD` | env_key | std::env::var("GROUND_WALL_CHILD") |  |  |  |  |

| `freecell_learned_ecai_16` | str_key | CASES = "freecell_learned_ecai_16" |  |  |  |  |

| `abstract-task-without-decomposition-domain.hddl` | str_key | CORPUS_ACCEPTED_SCOPE_GAPS = "abstract-task-without-decomposition-domain.hddl" |  |  |  |  |

| `TIMEOUT(normalized)` | str_key | TIMEOUT_TAG = "TIMEOUT(normalized)" |  |  |  |  |

| `solve:TIMEOUT(normalized)` | str_key | TIMEOUT_SOLVE = "solve:TIMEOUT(normalized)" |  |  |  |  |

| `blocksworld_gtohp` | str_key | INSTANCES = "blocksworld_gtohp" |  |  |  |  |

| `PCP_1` | str_key | KNOWN_MISMATCHES = "PCP_1" |  |  |  |  |

| `INC_LMCUT_CHILD` | env_key | std::env::var("INC_LMCUT_CHILD") |  |  |  |  |

| `assemblyhierarchical` | str_key | DOMAINS = "assemblyhierarchical" |  |  |  |  |

| `lamps` | str_key | SAMPLED = "lamps" |  |  |  |  |

| `LADDER_DEDUP_CHILD` | env_key | std::env::var("LADDER_DEDUP_CHILD") |  |  |  |  |

| `LADDER_WALL_CHILD` | env_key | std::env::var("LADDER_WALL_CHILD") |  |  |  |  |

| `
(define (domain chain)
  (:requirements :strips)
  (:predicates (a) (b) (c))
  (:action ab :parameters () :precondition (a) :effect (b))
  (:action bc :parameters () :precondition (b) :effect (c)))
` | str_key | CHAIN = "
(define (domain chain)
  (:requirements :strips)
  (:predicates (a) (b) (c))
  (:action ab :parameters () :precondition (a) :effect (b))
  (:action bc :parameters () :precondition (b) :effect (c)))
" |  |  |  |  |

| `MCV_ROUTE_CHILD` | env_key | std::env::var("MCV_ROUTE_CHILD") |  |  |  |  |

| `MCV_STRAT_CHILD` | env_key | std::env::var("MCV_STRAT_CHILD") |  |  |  |  |

| `(define (domain fixroute) (:requirements :typing)
  (:types item key - object)
  (:predicates (tok ?a ?b - item ?c - key) (done ?a - item) (seeded))
  (:action seed :parameters ()
    :precondition (and) :effect (and (seeded) (tok i1 i2 k1)))
  (:action reap :parameters (?a ?b - item ?c - key)
    :precondition (and (seeded) (tok ?a ?b ?c))
    :effect (done ?a)))` | str_key | ROUTE_DOM = "(define (domain fixroute) (:requirements :typing)
  (:types item key - object)
  (:predicates (tok ?a ?b - item ?c - key) (done ?a - item) (seeded))
  (:action seed :parameters ()
    :precondition (and) :effect (and (seeded) (tok i1 i2 k1)))
  (:action reap :parameters (?a ?b - item ?c - key)
    :precondition (and (seeded) (tok ?a ?b ?c))
    :effect (done ?a)))" |  |  |  |  |

| `MEM_WALL_CHILD` | env_key | std::env::var("MEM_WALL_CHILD") |  |  |  |  |

| `0.25` | str_key | BUDGET_GB = "0.25" |  |  |  |  |

| `FERROPLAN_MEMORY_STRESS_CHILD` | str_key | CHILD_ENV = "FERROPLAN_MEMORY_STRESS_CHILD" |  |  |  |  |

| `predicates-500` | str_key | SAMPLED_CASES = "predicates-500" |  |  |  |  |

| `FERROPLAN_REQUIRE_MFW_ORACLE` | env_key | std::env::var_os("FERROPLAN_REQUIRE_MFW_ORACLE") |  |  |  |  |

| `MFW_PLANNER_ORACLE_PYTHONPATH` | env_key | std::env::var("MFW_PLANNER_ORACLE_PYTHONPATH") |  |  |  |  |

| `
(define (domain nb)
  (:requirements :strips :action-costs)
  (:predicates (have-a) (have-b) (blocked))
  (:functions (total-cost) - number)
  (:action get-a
    :parameters ()
    :precondition ()
    :effect (and (have-a) (increase (total-cost) 3)))
  (:action get-b
    :parameters ()
    :precondition (blocked)
    :effect (and (have-b) (increase (total-cost) 1))))
` | str_key | NB_DOMAIN = "
(define (domain nb)
  (:requirements :strips :action-costs)
  (:predicates (have-a) (have-b) (blocked))
  (:functions (total-cost) - number)
  (:action get-a
    :parameters ()
    :precondition ()
    :effect (and (have-a) (increase (total-cost) 3)))
  (:action get-b
    :parameters ()
    :precondition (blocked)
    :effect (and (have-b) (increase (total-cost) 1))))
" |  |  |  |  |

| `
(define (problem nb-1) (:domain nb)
  (:init (= (total-cost) 0))
  (:goal (and (preference pa (have-a)) (preference pb (have-b))))
  (:metric maximize (- 15 (+ (total-cost)
                             (* (is-violated pa) 10)
                             (* (is-violated pb) 5)))))
` | str_key | NB_PROBLEM = "
(define (problem nb-1) (:domain nb)
  (:init (= (total-cost) 0))
  (:goal (and (preference pa (have-a)) (preference pb (have-b))))
  (:metric maximize (- 15 (+ (total-cost)
                             (* (is-violated pa) 10)
                             (* (is-violated pb) 5)))))
" |  |  |  |  |

| `NODE_CAP_RLIMIT_CHILD` | env_key | std::env::var("NODE_CAP_RLIMIT_CHILD") |  |  |  |  |

| `NOVDRIVER_CHILD` | env_key | std::env::var("NOVDRIVER_CHILD") |  |  |  |  |

| `NUMFOLD_CHILD` | env_key | std::env::var("NUMFOLD_CHILD") |  |  |  |  |

| `NUMOPT_ARM_CHILD` | env_key | std::env::var("NUMOPT_ARM_CHILD") |  |  |  |  |

| `../../../benchmarks/bench/chained-band-domain.pddl` | str_key | CHAIN_DOM = "../../../benchmarks/bench/chained-band-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/chained-band-i1.pddl` | str_key | CHAIN_PRB = "../../../benchmarks/bench/chained-band-i1.pddl" |  |  |  |  |

| `../../../benchmarks/bench/fo-sailing-domain.pddl` | str_key | FOSAIL_DOM = "../../../benchmarks/bench/fo-sailing-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/fo-sailing-i8.pddl` | str_key | FOSAIL_I8 = "../../../benchmarks/bench/fo-sailing-i8.pddl" |  |  |  |  |

| `../../../benchmarks/bench/sailing-band-domain.pddl` | str_key | SAIL_DOM = "../../../benchmarks/bench/sailing-band-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/sailing-band-i1.pddl` | str_key | SAIL_PRB = "../../../benchmarks/bench/sailing-band-i1.pddl" |  |  |  |  |

| `../../../benchmarks/bench/trader-cycle-domain.pddl` | str_key | TRADE_DOM = "../../../benchmarks/bench/trader-cycle-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/trader-cycle-i1.pddl` | str_key | TRADE_PRB = "../../../benchmarks/bench/trader-cycle-i1.pddl" |  |  |  |  |

| `../../../benchmarks/bench/watering-line-domain.pddl` | str_key | WATER_DOM = "../../../benchmarks/bench/watering-line-domain.pddl" |  |  |  |  |

| `../../../benchmarks/bench/watering-line-i1.pddl` | str_key | WATER_PRB = "../../../benchmarks/bench/watering-line-i1.pddl" |  |  |  |  |

| `OPT_WALL_CHILD` | env_key | std::env::var("OPT_WALL_CHILD") |  |  |  |  |

| `
(define (domain fees)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget) (alldone))
  (:functions (total-cost) (fee ?g - gadget))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (fee ?g))))
  (:action finish
    :parameters (?g - gadget)
    :precondition (done ?g)
    :effect (alldone)))
` | str_key | FEE_DOM = "
(define (domain fees)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget) (alldone))
  (:functions (total-cost) (fee ?g - gadget))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (fee ?g))))
  (:action finish
    :parameters (?g - gadget)
    :precondition (done ?g)
    :effect (alldone)))
" |  |  |  |  |

| `
(define (domain gadgets)
  (:requirements :strips :typing :action-costs)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) 1))))
` | str_key | GADGET_DOM_CONST = "
(define (domain gadgets)
  (:requirements :strips :typing :action-costs)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) 1))))
" |  |  |  |  |

| `
(define (domain gadgets-dyn)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost) (surcharge))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (surcharge))))
  (:action bump
    :parameters ()
    :precondition (and)
    :effect (increase (surcharge) 1)))
` | str_key | GADGET_DOM_DYN = "
(define (domain gadgets-dyn)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost) (surcharge))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (surcharge))))
  (:action bump
    :parameters ()
    :precondition (and)
    :effect (increase (surcharge) 1)))
" |  |  |  |  |

| `
(define (domain gadgets-static)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost) (surcharge))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (surcharge)))))
` | str_key | GADGET_DOM_STATIC = "
(define (domain gadgets-static)
  (:requirements :strips :typing :action-costs :numeric-fluents)
  (:types gadget)
  (:predicates (fresh ?g - gadget) (done ?g - gadget))
  (:functions (total-cost) (surcharge))
  (:action prep
    :parameters (?g - gadget)
    :precondition (fresh ?g)
    :effect (and (not (fresh ?g)) (done ?g) (increase (total-cost) (surcharge)))))
" |  |  |  |  |

| `
(define (domain mini-snack)
  (:requirements :strips :typing)
  (:types child bread sandwich tray)
  (:predicates (at-kitchen-bread ?b - bread) (at-kitchen-sandwich ?s - sandwich)
               (notexist ?s - sandwich) (ontray ?s - sandwich ?t - tray)
               (served ?c - child) (waiting ?c - child))
  (:action make
    :parameters (?s - sandwich ?b - bread)
    :precondition (and (notexist ?s) (at-kitchen-bread ?b))
    :effect (and (not (notexist ?s)) (not (at-kitchen-bread ?b))
                 (at-kitchen-sandwich ?s)))
  (:action put
    :parameters (?s - sandwich ?t - tray)
    :precondition (at-kitchen-sandwich ?s)
    :effect (and (not (at-kitchen-sandwich ?s)) (ontray ?s ?t)))
  (:action serve
    :parameters (?s - sandwich ?t - tray ?c - child)
    :precondition (and (ontray ?s ?t) (waiting ?c))
    :effect (and (not (ontray ?s ?t)) (not (waiting ?c)) (served ?c))))
` | str_key | SNACK_DOM = "
(define (domain mini-snack)
  (:requirements :strips :typing)
  (:types child bread sandwich tray)
  (:predicates (at-kitchen-bread ?b - bread) (at-kitchen-sandwich ?s - sandwich)
               (notexist ?s - sandwich) (ontray ?s - sandwich ?t - tray)
               (served ?c - child) (waiting ?c - child))
  (:action make
    :parameters (?s - sandwich ?b - bread)
    :precondition (and (notexist ?s) (at-kitchen-bread ?b))
    :effect (and (not (notexist ?s)) (not (at-kitchen-bread ?b))
                 (at-kitchen-sandwich ?s)))
  (:action put
    :parameters (?s - sandwich ?t - tray)
    :precondition (at-kitchen-sandwich ?s)
    :effect (and (not (at-kitchen-sandwich ?s)) (ontray ?s ?t)))
  (:action serve
    :parameters (?s - sandwich ?t - tray ?c - child)
    :precondition (and (ontray ?s ?t) (waiting ?c))
    :effect (and (not (ontray ?s ?t)) (not (waiting ?c)) (served ?c))))
" |  |  |  |  |

| `
(define (problem fees-1)
  (:domain fees)
  (:objects g1 g2 - gadget)
  (:init (fresh g1) (fresh g2) (= (total-cost) 0)
         (= (fee g1) 1) (= (fee g2) 9))
  (:goal (alldone))
  (:metric minimize (total-cost)))
` | str_key | FEE_PRB = "
(define (problem fees-1)
  (:domain fees)
  (:objects g1 g2 - gadget)
  (:init (fresh g1) (fresh g2) (= (total-cost) 0)
         (= (fee g1) 1) (= (fee g2) 9))
  (:goal (alldone))
  (:metric minimize (total-cost)))
" |  |  |  |  |

| `
(define (problem mini-snack-1)
  (:domain mini-snack)
  (:objects c1 c2 c3 - child b1 b2 b3 - bread s1 s2 s3 s4 - sandwich t1 - tray)
  (:init (waiting c1) (waiting c2) (waiting c3)
         (at-kitchen-bread b1) (at-kitchen-bread b2) (at-kitchen-bread b3)
         (notexist s1) (notexist s2) (notexist s3) (notexist s4))
  (:goal (and (served c1) (served c2) (served c3))))
` | str_key | SNACK_PRB = "
(define (problem mini-snack-1)
  (:domain mini-snack)
  (:objects c1 c2 c3 - child b1 b2 b3 - bread s1 s2 s3 s4 - sandwich t1 - tray)
  (:init (waiting c1) (waiting c2) (waiting c3)
         (at-kitchen-bread b1) (at-kitchen-bread b2) (at-kitchen-bread b3)
         (notexist s1) (notexist s2) (notexist s3) (notexist s4))
  (:goal (and (served c1) (served c2) (served c3))))
" |  |  |  |  |

| `
    (define (problem iso-paint-uniform)
      (:domain iso-paint)
      (:objects b1 b2 b3 b4 - block)
      (:init (clean b1) (clean b2) (clean b3) (clean b4))
      (:goal (and (red b1) (red b2) (red b3) (red b4))))
    ` | str_key | UNIFORM_PRB = "
    (define (problem iso-paint-uniform)
      (:domain iso-paint)
      (:objects b1 b2 b3 b4 - block)
      (:init (clean b1) (clean b2) (clean b3) (clean b4))
      (:goal (and (red b1) (red b2) (red b3) (red b4))))
    " |  |  |  |  |

| `
(define (domain iso-bake)
  (:requirements :strips :typing :durative-actions)
  (:types piece)
  (:predicates (raw ?p - piece) (made ?p - piece)
               (fancy ?p - piece) (plain ?p - piece) (free))
  (:durative-action make
    :parameters (?p - piece)
    :duration (= ?duration 2)
    :condition (and (at start (raw ?p)) (over all (free)))
    :effect (and (at start (not (raw ?p))) (at end (made ?p))))
  (:durative-action finish-fancy
    :parameters (?p - piece)
    :duration (= ?duration 1)
    :condition (at start (made ?p))
    :effect (at end (fancy ?p)))
  (:durative-action finish-plain
    :parameters (?p - piece)
    :duration (= ?duration 1)
    :condition (at start (made ?p))
    :effect (at end (plain ?p))))
` | str_key | BAKE_DOM = "
(define (domain iso-bake)
  (:requirements :strips :typing :durative-actions)
  (:types piece)
  (:predicates (raw ?p - piece) (made ?p - piece)
               (fancy ?p - piece) (plain ?p - piece) (free))
  (:durative-action make
    :parameters (?p - piece)
    :duration (= ?duration 2)
    :condition (and (at start (raw ?p)) (over all (free)))
    :effect (and (at start (not (raw ?p))) (at end (made ?p))))
  (:durative-action finish-fancy
    :parameters (?p - piece)
    :duration (= ?duration 1)
    :condition (at start (made ?p))
    :effect (at end (fancy ?p)))
  (:durative-action finish-plain
    :parameters (?p - piece)
    :duration (= ?duration 1)
    :condition (at start (made ?p))
    :effect (at end (plain ?p))))
" |  |  |  |  |

| `
(define (domain iso-paint)
  (:requirements :strips :typing)
  (:types block)
  (:predicates (clean ?b - block) (red ?b - block) (blue ?b - block))
  (:action paint-red
    :parameters (?b - block)
    :precondition (clean ?b)
    :effect (and (not (clean ?b)) (red ?b)))
  (:action paint-blue
    :parameters (?b - block)
    :precondition (clean ?b)
    :effect (and (not (clean ?b)) (blue ?b))))
` | str_key | PAINT_DOM = "
(define (domain iso-paint)
  (:requirements :strips :typing)
  (:types block)
  (:predicates (clean ?b - block) (red ?b - block) (blue ?b - block))
  (:action paint-red
    :parameters (?b - block)
    :precondition (clean ?b)
    :effect (and (not (clean ?b)) (red ?b)))
  (:action paint-blue
    :parameters (?b - block)
    :precondition (clean ?b)
    :effect (and (not (clean ?b)) (blue ?b))))
" |  |  |  |  |

| `
(define (problem iso-bake-1)
  (:domain iso-bake)
  (:objects a b c - piece)
  (:init (raw a) (raw b) (raw c) (free))
  (:goal (and (fancy a) (plain b))))
` | str_key | BAKE_PRB = "
(define (problem iso-bake-1)
  (:domain iso-bake)
  (:objects a b c - piece)
  (:init (raw a) (raw b) (raw c) (free))
  (:goal (and (fancy a) (plain b))))
" |  |  |  |  |

| `
(define (problem iso-paint-1)
  (:domain iso-paint)
  (:objects b1 b2 b3 b4 - block)
  (:init (clean b1) (clean b2) (clean b3) (clean b4))
  (:goal (and (red b1) (blue b2))))
` | str_key | PAINT_PRB = "
(define (problem iso-paint-1)
  (:domain iso-paint)
  (:objects b1 b2 b3 b4 - block)
  (:init (clean b1) (clean b2) (clean b3) (clean b4))
  (:goal (and (red b1) (blue b2))))
" |  |  |  |  |

| `
(define (problem iso-paint-2)
  (:domain iso-paint)
  (:objects b1 b2 b3 b4 - block)
  (:init (clean b1) (clean b2) (clean b3) (clean b4))
  (:goal (and (red b1) (blue b3))))
` | str_key | PAINT_PRB_B3 = "
(define (problem iso-paint-2)
  (:domain iso-paint)
  (:objects b1 b2 b3 b4 - block)
  (:init (clean b1) (clean b2) (clean b3) (clean b4))
  (:goal (and (red b1) (blue b3))))
" |  |  |  |  |

| `
(define (domain fuel-gap)
  (:requirements :typing :durative-actions :numeric-fluents)
  (:types rig)
  (:predicates (idle) (hot) (done) (dipped) (refilled))
  (:functions (level))
  (:durative-action run
    :parameters (?r - rig)
    :duration (= ?duration 10)
    :condition (and (at start (idle)) (over all (>= (level) 1)))
    :effect (and (at start (not (idle))) (at start (hot))
                 (at end (not (hot))) (at end (done))))
  (:durative-action topup
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (idle))
    :effect (at start (increase (level) 2)))
  (:durative-action dip
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (hot))
    :effect (and (at start (dipped)) (at start (decrease (level) 2))))
  (:durative-action refill
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (dipped))
    :effect (and (at start (refilled)) (at start (increase (level) 2)))))
` | str_key | FUEL_DOM = "
(define (domain fuel-gap)
  (:requirements :typing :durative-actions :numeric-fluents)
  (:types rig)
  (:predicates (idle) (hot) (done) (dipped) (refilled))
  (:functions (level))
  (:durative-action run
    :parameters (?r - rig)
    :duration (= ?duration 10)
    :condition (and (at start (idle)) (over all (>= (level) 1)))
    :effect (and (at start (not (idle))) (at start (hot))
                 (at end (not (hot))) (at end (done))))
  (:durative-action topup
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (idle))
    :effect (at start (increase (level) 2)))
  (:durative-action dip
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (hot))
    :effect (and (at start (dipped)) (at start (decrease (level) 2))))
  (:durative-action refill
    :parameters (?r - rig)
    :duration (= ?duration 1)
    :condition (at start (dipped))
    :effect (and (at start (refilled)) (at start (increase (level) 2)))))
" |  |  |  |  |

| `
(define (domain kiln-gap)
  (:requirements :typing :durative-actions :timed-initial-literals)
  (:types piece)
  (:predicates (ready) (raw ?p - piece) (prepped ?p - piece) (baked ?p - piece))
  (:durative-action prep
    :parameters (?p - piece)
    :duration (= ?duration 6)
    :condition (at start (raw ?p))
    :effect (and (at start (not (raw ?p))) (at end (prepped ?p))))
  (:durative-action bake
    :parameters (?p - piece)
    :duration (= ?duration 3)
    :condition (and (at start (prepped ?p)) (over all (ready)))
    :effect (at end (baked ?p))))
` | str_key | KILN_DOM = "
(define (domain kiln-gap)
  (:requirements :typing :durative-actions :timed-initial-literals)
  (:types piece)
  (:predicates (ready) (raw ?p - piece) (prepped ?p - piece) (baked ?p - piece))
  (:durative-action prep
    :parameters (?p - piece)
    :duration (= ?duration 6)
    :condition (at start (raw ?p))
    :effect (and (at start (not (raw ?p))) (at end (prepped ?p))))
  (:durative-action bake
    :parameters (?p - piece)
    :duration (= ?duration 3)
    :condition (and (at start (prepped ?p)) (over all (ready)))
    :effect (at end (baked ?p))))
" |  |  |  |  |

| `
(define (domain mini-tms)
  (:requirements :strips :typing :durative-actions)
  (:types piece)
  (:predicates (raw ?p - piece) (made ?p - piece) (glued ?a ?b - piece) (free))
  (:durative-action make
    :parameters (?p - piece)
    :duration (= ?duration 2)
    :condition (and (at start (raw ?p)) (over all (free)))
    :effect (and (at start (not (raw ?p))) (at end (made ?p))))
  (:durative-action glue
    :parameters (?a ?b - piece)
    :duration (= ?duration 3)
    :condition (and (at start (made ?a)) (at start (made ?b)))
    :effect (at end (glued ?a ?b))))
` | str_key | MINI_DOM = "
(define (domain mini-tms)
  (:requirements :strips :typing :durative-actions)
  (:types piece)
  (:predicates (raw ?p - piece) (made ?p - piece) (glued ?a ?b - piece) (free))
  (:durative-action make
    :parameters (?p - piece)
    :duration (= ?duration 2)
    :condition (and (at start (raw ?p)) (over all (free)))
    :effect (and (at start (not (raw ?p))) (at end (made ?p))))
  (:durative-action glue
    :parameters (?a ?b - piece)
    :duration (= ?duration 3)
    :condition (and (at start (made ?a)) (at start (made ?b)))
    :effect (at end (glued ?a ?b))))
" |  |  |  |  |

| `
(define (problem fuel-gap-1)
  (:domain fuel-gap)
  (:objects r1 - rig)
  (:init (idle) (= (level) 2))
  (:goal (and (done) (dipped) (refilled)))
  (:metric minimize (total-time)))
` | str_key | FUEL_PROB = "
(define (problem fuel-gap-1)
  (:domain fuel-gap)
  (:objects r1 - rig)
  (:init (idle) (= (level) 2))
  (:goal (and (done) (dipped) (refilled)))
  (:metric minimize (total-time)))
" |  |  |  |  |

| `
(define (problem kiln-gap-1)
  (:domain kiln-gap)
  (:objects p1 - piece)
  (:init (raw p1) (ready)
         (at 8 (not (ready)))
         (at 8.001 (ready)))
  (:goal (baked p1))
  (:metric minimize (total-time)))
` | str_key | KILN_PROB = "
(define (problem kiln-gap-1)
  (:domain kiln-gap)
  (:objects p1 - piece)
  (:init (raw p1) (ready)
         (at 8 (not (ready)))
         (at 8.001 (ready)))
  (:goal (baked p1))
  (:metric minimize (total-time)))
" |  |  |  |  |

| `
(define (problem mini-tms-1)
  (:domain mini-tms)
  (:objects a1 b1 a2 b2 - piece)
  (:init (raw a1) (raw b1) (raw a2) (raw b2) (free))
  (:goal (and (glued a1 b1) (glued a2 b2))))
` | str_key | MINI_PROB = "
(define (problem mini-tms-1)
  (:domain mini-tms)
  (:objects a1 b1 a2 b2 - piece)
  (:init (raw a1) (raw b1) (raw a2) (raw b2) (free))
  (:goal (and (glued a1 b1) (glued a2 b2))))
" |  |  |  |  |

| `(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room) (free ?g - gripper))
  (:functions (cost))
  (:action move :parameters (?from ?to - room)
    :precondition (at-robby ?from) :effect (and (not (at-robby ?from)) (at-robby ?to)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r)) :effect (not (at ?b ?r))))` | str_key | DOM = "(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room) (free ?g - gripper))
  (:functions (cost))
  (:action move :parameters (?from ?to - room)
    :precondition (at-robby ?from) :effect (and (not (at-robby ?from)) (at-robby ?to)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r)) :effect (not (at ?b ?r))))" |  |  |  |  |

| `(define (problem gripper-1) (:domain gripper)
  (:objects rooma roomb - room b1 b2 - ball left right - gripper)
  (:init (at-robby rooma) (at b1 rooma) (= (cost) 0))
  (:goal (at b1 roomb))
  (:metric minimize (cost)))` | str_key | PROB = "(define (problem gripper-1) (:domain gripper)
  (:objects rooma roomb - room b1 b2 - ball left right - gripper)
  (:init (at-robby rooma) (at b1 rooma) (= (cost) 0))
  (:goal (at b1 roomb))
  (:metric minimize (cost)))" |  |  |  |  |

| `(define (domain mk)
 (:requirements :strips :typing :adl :fluents)
 (:types item)
 (:predicates (special ?x - item) (can ?x - item))
 (:functions (total-cost))
 (:action make :parameters (?x - item) :precondition (can ?x) :effect (special ?x)))` | str_key | MARK = "(define (domain mk)
 (:requirements :strips :typing :adl :fluents)
 (:types item)
 (:predicates (special ?x - item) (can ?x - item))
 (:functions (total-cost))
 (:action make :parameters (?x - item) :precondition (can ?x) :effect (special ?x)))" |  |  |  |  |

| `(define (domain sp)
 (:requirements :strips :adl :fluents)
 (:predicates (ready) (done))
 (:functions (total-cost))
 (:action go :parameters ()
   :precondition (preference want (ready))
   :effect (done)))` | str_key | SOFTPRE = "(define (domain sp)
 (:requirements :strips :adl :fluents)
 (:predicates (ready) (done))
 (:functions (total-cost))
 (:action go :parameters ()
   :precondition (preference want (ready))
   :effect (done)))" |  |  |  |  |

| `(define (domain chain)
  (:predicates (p0) (p1) (p2) (p3))
  (:action s1 :precondition (p0) :effect (p1))
  (:action s2 :precondition (p1) :effect (p2))
  (:action s3 :precondition (p2) :effect (p3)))` | str_key | DOM = "(define (domain chain)
  (:predicates (p0) (p1) (p2) (p3))
  (:action s1 :precondition (p0) :effect (p1))
  (:action s2 :precondition (p1) :effect (p2))
  (:action s3 :precondition (p2) :effect (p3)))" |  |  |  |  |

| `PREF_CHASE_WALL_CHILD` | env_key | std::env::var("PREF_CHASE_WALL_CHILD") |  |  |  |  |

| `(define (domain corridor)
 (:requirements :strips :typing :preferences)
 (:types cell)
 (:predicates (at ?c - cell) (adj ?a ?b - cell) (lit ?c - cell) (visited ?c - cell))
 (:action move :parameters (?a ?b - cell)
   :precondition (and (at ?a) (adj ?a ?b) (preference darkstep (lit ?a)))
   :effect (and (not (at ?a)) (at ?b) (visited ?b)))
 (:action light :parameters (?a - cell)
   :precondition (at ?a)
   :effect (lit ?a)))` | str_key | CORRIDOR = "(define (domain corridor)
 (:requirements :strips :typing :preferences)
 (:types cell)
 (:predicates (at ?c - cell) (adj ?a ?b - cell) (lit ?c - cell) (visited ?c - cell))
 (:action move :parameters (?a ?b - cell)
   :precondition (and (at ?a) (adj ?a ?b) (preference darkstep (lit ?a)))
   :effect (and (not (at ?a)) (at ?b) (visited ?b)))
 (:action light :parameters (?a - cell)
   :precondition (at ?a)
   :effect (lit ?a)))" |  |  |  |  |

| `(define (problem walk) (:domain corridor)
 (:objects c0 c1 c2 c3 s1 - cell)
 (:init (at c0)
        (adj c0 c1) (adj c1 c2) (adj c2 c3) (adj c1 s1) (adj s1 c1))
 (:goal (and (at c3) (preference sidetrip (visited s1))))
 (:metric minimize (+ (is-violated darkstep) (* 5 (is-violated sidetrip)))))` | str_key | WALK = "(define (problem walk) (:domain corridor)
 (:objects c0 c1 c2 c3 s1 - cell)
 (:init (at c0)
        (adj c0 c1) (adj c1 c2) (adj c2 c3) (adj c1 s1) (adj s1 c1))
 (:goal (and (at c3) (preference sidetrip (visited s1))))
 (:metric minimize (+ (is-violated darkstep) (* 5 (is-violated sidetrip)))))" |  |  |  |  |

| `REFILL_CHILD` | env_key | std::env::var("REFILL_CHILD") |  |  |  |  |

| `(define (domain d) (:requirements :strips)
  (:predicates (a) (b))
  (:action go :parameters () :precondition (a) :effect (and (not (a)) (b))))` | str_key | GOOD_DOM = "(define (domain d) (:requirements :strips)
  (:predicates (a) (b))
  (:action go :parameters () :precondition (a) :effect (and (not (a)) (b))))" |  |  |  |  |

| `(define (problem p) (:domain d) (:init (a)) (:goal (b)))` | str_key | GOOD_PROB = "(define (problem p) (:domain d) (:init (a)) (:goal (b)))" |  |  |  |  |

| `fixtures/sa2a-v26.9.17/sa2a-v26.9.17-domain.hddl` | str_key | DOMAIN = "fixtures/sa2a-v26.9.17/sa2a-v26.9.17-domain.hddl" |  |  |  |  |

| `fixtures/sa2a-v26.9.17/sa2a-v26.9.17-problem.hddl` | str_key | PROBLEM = "fixtures/sa2a-v26.9.17/sa2a-v26.9.17-problem.hddl" |  |  |  |  |

| `SAT_PROMO_CHILD` | env_key | std::env::var("SAT_PROMO_CHILD") |  |  |  |  |

| `
(define (domain sat-rc)
  (:requirements :strips :durative-actions)
  (:predicates (light) (open) (fresh-shine) (fresh-mend) (fresh-deliver)
               (fresh-door) (mended) (delivered))
  (:durative-action shine
    :parameters ()
    :duration (= ?duration 20)
    :condition (at start (fresh-shine))
    :effect (and (at start (not (fresh-shine)))
                 (at start (light))
                 (at end (not (light)))))
  (:durative-action mend
    :parameters ()
    :duration (= ?duration 9)
    :condition (and (at start (fresh-mend)) (over all (light)))
    :effect (and (at start (not (fresh-mend))) (at end (mended))))
  (:durative-action deliver
    :parameters ()
    :duration (= ?duration 6)
    :condition (and (at start (fresh-deliver)) (at end (open)))
    :effect (and (at start (not (fresh-deliver))) (at end (delivered))))
  (:durative-action door
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-door))
    :effect (and (at start (not (fresh-door)))
                 (at start (open))
                 (at end (not (open))))))
` | str_key | RC_DOMAIN = "
(define (domain sat-rc)
  (:requirements :strips :durative-actions)
  (:predicates (light) (open) (fresh-shine) (fresh-mend) (fresh-deliver)
               (fresh-door) (mended) (delivered))
  (:durative-action shine
    :parameters ()
    :duration (= ?duration 20)
    :condition (at start (fresh-shine))
    :effect (and (at start (not (fresh-shine)))
                 (at start (light))
                 (at end (not (light)))))
  (:durative-action mend
    :parameters ()
    :duration (= ?duration 9)
    :condition (and (at start (fresh-mend)) (over all (light)))
    :effect (and (at start (not (fresh-mend))) (at end (mended))))
  (:durative-action deliver
    :parameters ()
    :duration (= ?duration 6)
    :condition (and (at start (fresh-deliver)) (at end (open)))
    :effect (and (at start (not (fresh-deliver))) (at end (delivered))))
  (:durative-action door
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-door))
    :effect (and (at start (not (fresh-door)))
                 (at start (open))
                 (at end (not (open))))))
" |  |  |  |  |

| `
(define (problem sat-rc-1) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend) (fresh-deliver) (fresh-door))
  (:goal (and (mended) (delivered))))
` | str_key | RC_PROBLEM = "
(define (problem sat-rc-1) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend) (fresh-deliver) (fresh-door))
  (:goal (and (mended) (delivered))))
" |  |  |  |  |

| `
(define (problem sat-rc-env) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend))
  (:goal (mended)))
` | str_key | ENVELOPE_PROBLEM = "
(define (problem sat-rc-env) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend))
  (:goal (mended)))
" |  |  |  |  |

| `FERROPLAN_VAL` | env_key | std::env::var("FERROPLAN_VAL") |  |  |  |  |

| `
(define (domain sat-micro)
  (:requirements :strips :typing)
  (:types loc)
  (:predicates (at ?l - loc) (adj ?a ?b - loc))
  (:action move
    :parameters (?a ?b - loc)
    :precondition (and (at ?a) (adj ?a ?b))
    :effect (and (not (at ?a)) (at ?b))))
` | str_key | MICRO_DOMAIN = "
(define (domain sat-micro)
  (:requirements :strips :typing)
  (:types loc)
  (:predicates (at ?l - loc) (adj ?a ?b - loc))
  (:action move
    :parameters (?a ?b - loc)
    :precondition (and (at ?a) (adj ?a ?b))
    :effect (and (not (at ?a)) (at ?b))))
" |  |  |  |  |

| `
(define (domain sat-rc)
  (:requirements :strips :durative-actions)
  (:predicates (light) (open) (fresh-shine) (fresh-mend) (fresh-deliver)
               (fresh-door) (mended) (delivered))
  (:durative-action shine
    :parameters ()
    :duration (= ?duration 20)
    :condition (at start (fresh-shine))
    :effect (and (at start (not (fresh-shine)))
                 (at start (light))
                 (at end (not (light)))))
  (:durative-action mend
    :parameters ()
    :duration (= ?duration 9)
    :condition (and (at start (fresh-mend)) (over all (light)))
    :effect (and (at start (not (fresh-mend))) (at end (mended))))
  (:durative-action deliver
    :parameters ()
    :duration (= ?duration 6)
    :condition (and (at start (fresh-deliver)) (at end (open)))
    :effect (and (at start (not (fresh-deliver))) (at end (delivered))))
  (:durative-action door
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-door))
    :effect (and (at start (not (fresh-door)))
                 (at start (open))
                 (at end (not (open))))))
` | str_key | RC_DOMAIN = "
(define (domain sat-rc)
  (:requirements :strips :durative-actions)
  (:predicates (light) (open) (fresh-shine) (fresh-mend) (fresh-deliver)
               (fresh-door) (mended) (delivered))
  (:durative-action shine
    :parameters ()
    :duration (= ?duration 20)
    :condition (at start (fresh-shine))
    :effect (and (at start (not (fresh-shine)))
                 (at start (light))
                 (at end (not (light)))))
  (:durative-action mend
    :parameters ()
    :duration (= ?duration 9)
    :condition (and (at start (fresh-mend)) (over all (light)))
    :effect (and (at start (not (fresh-mend))) (at end (mended))))
  (:durative-action deliver
    :parameters ()
    :duration (= ?duration 6)
    :condition (and (at start (fresh-deliver)) (at end (open)))
    :effect (and (at start (not (fresh-deliver))) (at end (delivered))))
  (:durative-action door
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (fresh-door))
    :effect (and (at start (not (fresh-door)))
                 (at start (open))
                 (at end (not (open))))))
" |  |  |  |  |

| `
(define (problem sat-micro-1) (:domain sat-micro)
  (:objects l1 l2 l3 - loc)
  (:init (at l1) (adj l1 l2) (adj l2 l3))
  (:goal (at l3)))
` | str_key | MICRO_PROBLEM = "
(define (problem sat-micro-1) (:domain sat-micro)
  (:objects l1 l2 l3 - loc)
  (:init (at l1) (adj l1 l2) (adj l2 l3))
  (:goal (at l3)))
" |  |  |  |  |

| `
(define (problem sat-rc-1) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend) (fresh-deliver) (fresh-door))
  (:goal (and (mended) (delivered))))
` | str_key | RC_PROBLEM = "
(define (problem sat-rc-1) (:domain sat-rc)
  (:init (fresh-shine) (fresh-mend) (fresh-deliver) (fresh-door))
  (:goal (and (mended) (delivered))))
" |  |  |  |  |

| `SCALING_LADDER_RESULTS` | env_key | std::env::var("SCALING_LADDER_RESULTS") |  |  |  |  |

| `SCALING_LADDER_RSS_KB` | env_key | std::env::var("SCALING_LADDER_RSS_KB") |  |  |  |  |

| `| family | n | m | ground_actions | ground_methods | states | transitions | parse_ms | ground_ms | translate_ms | solve_ms | outcome |` | str_key | SWEEP_HEADER = "| family | n | m | ground_actions | ground_methods | states | transitions | parse_ms | ground_ms | translate_ms | solve_ms | outcome |" |  |  |  |  |

| `TCOMPRESS_TCONC_CHILD` | env_key | std::env::var("TCOMPRESS_TCONC_CHILD") |  |  |  |  |

| `(define (domain shop)
 (:requirements :typing :durative-actions)
 (:types job machine)
 (:predicates (todo ?j - job) (done ?j - job) (fits ?j - job ?m - machine)
              (free ?m - machine) (busy ?m - machine))
 (:durative-action run
   :parameters (?j - job ?m - machine)
   :duration (= ?duration 3)
   :condition (and (at start (todo ?j)) (at start (free ?m))
                   (over all (fits ?j ?m)) (at end (busy ?m)))
   :effect (and (at start (not (todo ?j))) (at start (not (free ?m))) (at start (busy ?m))
                (at end (not (busy ?m))) (at end (free ?m)) (at end (done ?j)))))` | str_key | SHOP = "(define (domain shop)
 (:requirements :typing :durative-actions)
 (:types job machine)
 (:predicates (todo ?j - job) (done ?j - job) (fits ?j - job ?m - machine)
              (free ?m - machine) (busy ?m - machine))
 (:durative-action run
   :parameters (?j - job ?m - machine)
   :duration (= ?duration 3)
   :condition (and (at start (todo ?j)) (at start (free ?m))
                   (over all (fits ?j ?m)) (at end (busy ?m)))
   :effect (and (at start (not (todo ?j))) (at start (not (free ?m))) (at start (busy ?m))
                (at end (not (busy ?m))) (at end (free ?m)) (at end (done ?j)))))" |  |  |  |  |

| `(define (domain mr)
  (:requirements :durative-actions :numeric-fluents)
  (:predicates (ready))
  (:functions (raw) (mid) (top))
  (:durative-action gather :parameters ()
    :duration (= ?duration 1)
    :condition (at start (ready))
    :effect (at end (increase (raw) 1)))
  (:durative-action refine :parameters ()
    :duration (= ?duration 1)
    :condition (at start (>= (raw) 1))
    :effect (and (at start (decrease (raw) 1)) (at end (increase (mid) 1))))
  (:durative-action assemble :parameters ()
    :duration (= ?duration 1)
    :condition (at start (>= (mid) 1))
    :effect (and (at start (decrease (mid) 1)) (at end (increase (top) 1)))))` | str_key | DOM = "(define (domain mr)
  (:requirements :durative-actions :numeric-fluents)
  (:predicates (ready))
  (:functions (raw) (mid) (top))
  (:durative-action gather :parameters ()
    :duration (= ?duration 1)
    :condition (at start (ready))
    :effect (at end (increase (raw) 1)))
  (:durative-action refine :parameters ()
    :duration (= ?duration 1)
    :condition (at start (>= (raw) 1))
    :effect (and (at start (decrease (raw) 1)) (at end (increase (mid) 1))))
  (:durative-action assemble :parameters ()
    :duration (= ?duration 1)
    :condition (at start (>= (mid) 1))
    :effect (and (at start (decrease (mid) 1)) (at end (increase (top) 1)))))" |  |  |  |  |

| `(define (problem mr3) (:domain mr)
  (:init (ready) (= (raw) 0) (= (mid) 0) (= (top) 0))
  (:goal (>= (top) 3)))` | str_key | PROB = "(define (problem mr3) (:domain mr)
  (:init (ready) (= (raw) 0) (= (mid) 0) (= (top) 0))
  (:goal (>= (top) 3)))" |  |  |  |  |

| `ESCALATION_CHILD` | env_key | std::env::var("ESCALATION_CHILD") |  |  |  |  |

| `
(define (domain crew)
  (:requirements :typing :durative-actions :numeric-fluents)
  (:types task)
  (:predicates (done ?t - task))
  (:functions (avail))
  (:durative-action do
    :parameters (?t - task)
    :duration (= ?duration 5)
    :condition (at start (>= (avail) 1))
    :effect (and (at start (decrease (avail) 1))
                 (at end (increase (avail) 1))
                 (at end (done ?t)))))
` | str_key | RESOURCE_DOM = "
(define (domain crew)
  (:requirements :typing :durative-actions :numeric-fluents)
  (:types task)
  (:predicates (done ?t - task))
  (:functions (avail))
  (:durative-action do
    :parameters (?t - task)
    :duration (= ?duration 5)
    :condition (at start (>= (avail) 1))
    :effect (and (at start (decrease (avail) 1))
                 (at end (increase (avail) 1))
                 (at end (done ?t)))))
" |  |  |  |  |

| `
(define (domain gate)
  (:requirements :durative-actions)
  (:predicates (open) (through))
  (:durative-action pass
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (open))
    :effect (at end (through))))
` | str_key | TIL_DOM = "
(define (domain gate)
  (:requirements :durative-actions)
  (:predicates (open) (through))
  (:durative-action pass
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (open))
    :effect (at end (through))))
" |  |  |  |  |

| `
(define (domain ineq)
  (:requirements :durative-actions)
  (:predicates (done))
  (:durative-action work
    :parameters ()
    :duration (and (>= ?duration 2) (<= ?duration 5))
    :condition ()
    :effect (at end (done))))
` | str_key | INEQ_DOM = "
(define (domain ineq)
  (:requirements :durative-actions)
  (:predicates (done))
  (:durative-action work
    :parameters ()
    :duration (and (>= ?duration 2) (<= ?duration 5))
    :condition ()
    :effect (at end (done))))
" |  |  |  |  |

| `
(define (domain t)
  (:requirements :strips :durative-actions :numeric-fluents)
  (:predicates (at) (goal) (light))
  (:durative-action act
    :parameters ()
    :duration (= ?duration 3)
    :condition (and (at start (at)) (over all (light)))
    :effect (and (at start (not (at))) (at end (goal)))))` | str_key | DUR_DOM = "
(define (domain t)
  (:requirements :strips :durative-actions :numeric-fluents)
  (:predicates (at) (goal) (light))
  (:durative-action act
    :parameters ()
    :duration (= ?duration 3)
    :condition (and (at start (at)) (over all (light)))
    :effect (and (at start (not (at))) (at end (goal)))))" |  |  |  |  |

| `
(define (domain temporal-test)
  (:requirements :strips :typing :durative-actions :numeric-fluents)
  (:types location)
  (:predicates (at ?l - location) (connected ?a ?b - location) (free))
  (:functions (dist ?a ?b - location))
  (:durative-action move
    :parameters (?from ?to - location)
    :duration (= ?duration (dist ?from ?to))
    :condition (and (at start (at ?from))
                    (at start (connected ?from ?to))
                    (over all (free)))
    :effect (and (at start (not (at ?from)))
                 (at end (at ?to)))))
` | str_key | DOM = "
(define (domain temporal-test)
  (:requirements :strips :typing :durative-actions :numeric-fluents)
  (:types location)
  (:predicates (at ?l - location) (connected ?a ?b - location) (free))
  (:functions (dist ?a ?b - location))
  (:durative-action move
    :parameters (?from ?to - location)
    :duration (= ?duration (dist ?from ?to))
    :condition (and (at start (at ?from))
                    (at start (connected ?from ?to))
                    (over all (free)))
    :effect (and (at start (not (at ?from)))
                 (at end (at ?to)))))
" |  |  |  |  |

| `(define (problem g) (:domain gate)
  (:init (at 5 (open)))
  (:goal (through)))` | str_key | TIL_PROB = "(define (problem g) (:domain gate)
  (:init (at 5 (open)))
  (:goal (through)))" |  |  |  |  |

| `(define (problem p) (:domain t) (:init (at) (light)) (:goal (goal)))` | str_key | DUR_PROB = "(define (problem p) (:domain t) (:init (at) (light)) (:goal (goal)))" |  |  |  |  |

| `(define (problem w) (:domain ineq) (:init) (:goal (done)))` | str_key | INEQ_PROB = "(define (problem w) (:domain ineq) (:init) (:goal (done)))" |  |  |  |  |

| `(define (domain tconstr)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (home) (done) (flag))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (home))
    :effect (at end (done)))
  (:durative-action raise
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (home))
    :effect (at end (flag))))` | str_key | ATEND_DOM = "(define (domain tconstr)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (home) (done) (flag))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (home))
    :effect (at end (done)))
  (:durative-action raise
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (home))
    :effect (at end (flag))))" |  |  |  |  |

| `(define (domain tresp)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (idle) (alarm) (handled) (done))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (idle))
    :effect (and (at start (not (idle))) (at start (alarm)) (at end (done))))
  (:durative-action quiet-work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (idle))
    :effect (and (at start (not (idle))) (at end (done))))
  (:durative-action respond
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (alarm))
    :effect (at end (handled))))` | str_key | RESPOND_DOM = "(define (domain tresp)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (idle) (alarm) (handled) (done))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (idle))
    :effect (and (at start (not (idle))) (at start (alarm)) (at end (done))))
  (:durative-action quiet-work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (idle))
    :effect (and (at start (not (idle))) (at end (done))))
  (:durative-action respond
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (alarm))
    :effect (at end (handled))))" |  |  |  |  |

| `(define (domain twin)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (home) (done) (flag))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (home))
    :effect (at end (done)))
  (:durative-action quick-flag
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (home))
    :effect (at end (flag)))
  (:durative-action slow-flag
    :parameters ()
    :duration (= ?duration 6)
    :condition (at start (home))
    :effect (at end (flag))))` | str_key | WITHIN_DOM = "(define (domain twin)
  (:requirements :strips :durative-actions :constraints)
  (:predicates (home) (done) (flag))
  (:durative-action work
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (home))
    :effect (at end (done)))
  (:durative-action quick-flag
    :parameters ()
    :duration (= ?duration 1)
    :condition (at start (home))
    :effect (at end (flag)))
  (:durative-action slow-flag
    :parameters ()
    :duration (= ?duration 6)
    :condition (at start (home))
    :effect (at end (flag))))" |  |  |  |  |

| `TGROUND_WALL_CHILD` | env_key | std::env::var("TGROUND_WALL_CHILD") |  |  |  |  |

| `(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move
    :parameters (?from - room ?to - room)
    :precondition (at-robby ?from)
    :effect (and (at-robby ?to) (not (at-robby ?from))))
  (:action pick
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))` | str_key | GRIPPER_DOMAIN = "(define (domain gripper)
  (:requirements :strips :typing)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:action move
    :parameters (?from - room ?to - room)
    :precondition (at-robby ?from)
    :effect (and (at-robby ?to) (not (at-robby ?from))))
  (:action pick
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g))))
  (:action drop
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)))))" |  |  |  |  |

| `(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))` | str_key | CORRIDOR_DOMAIN = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))" |  |  |  |  |

| `(define (problem corridor)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c d))
  (:goal (at d)))` | str_key | CORRIDOR_PROBLEM = "(define (problem corridor)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c) (link c d))
  (:goal (at d)))" |  |  |  |  |

| `(define (problem dead-end)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c))
  (:goal (at d)))` | str_key | DEAD_END_PROBLEM = "(define (problem dead-end)
  (:domain rooms)
  (:objects a b c d - room)
  (:init (at a) (link a b) (link b c))
  (:goal (at d)))" |  |  |  |  |

| `(define (problem gripper-4)
  (:domain gripper)
  (:objects rooma roomb - room b1 b2 b3 b4 - ball left right - gripper)
  (:init (at-robby rooma) (free left) (free right)
         (at b1 rooma) (at b2 rooma) (at b3 rooma) (at b4 rooma))
  (:goal (and (at b1 roomb) (at b2 roomb) (at b3 roomb) (at b4 roomb))))` | str_key | GRIPPER_PROBLEM = "(define (problem gripper-4)
  (:domain gripper)
  (:objects rooma roomb - room b1 b2 b3 b4 - ball left right - gripper)
  (:init (at-robby rooma) (free left) (free right)
         (at b1 rooma) (at b2 rooma) (at b3 rooma) (at b4 rooma))
  (:goal (and (at b1 roomb) (at b2 roomb) (at b3 roomb) (at b4 roomb))))" |  |  |  |  |

| `pcp_1` | str_key | CASES = "pcp_1" |  |  |  |  |

| `TSEARCH_WALL_CHILD` | env_key | std::env::var("TSEARCH_WALL_CHILD") |  |  |  |  |

| `
(define (domain z0)
  (:requirements :strips :durative-actions)
  (:predicates (a) (g) (h2))
  (:durative-action zap
    :parameters ()
    :duration (= ?duration 0)
    :condition (at start (a))
    :effect (at start (g)))
  (:durative-action chain
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (g))
    :effect (at end (h2))))
` | str_key | DOMAIN = "
(define (domain z0)
  (:requirements :strips :durative-actions)
  (:predicates (a) (g) (h2))
  (:durative-action zap
    :parameters ()
    :duration (= ?duration 0)
    :condition (at start (a))
    :effect (at start (g)))
  (:durative-action chain
    :parameters ()
    :duration (= ?duration 2)
    :condition (at start (g))
    :effect (at end (h2))))
" |  |  |  |  |


<!-- ============================================================= -->
<!-- AGENT-FORBIDDEN-END: nothing below this line may describe     -->
<!-- code behavior.                                                -->
<!-- ============================================================= -->
