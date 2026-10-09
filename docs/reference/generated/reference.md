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

| `model::*` | use | model::* |  |  |  |  |


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


### crates/ferroplan-hddl/src/probabilistic.rs

| `has_probabilistic` | function | has_probabilistic(src: &str) -> bool |  |  |  |  |

| `preprocess` | function | preprocess(src: &str) -> Result<(String, BTreeMap<String, Vec<String>>), ParseError> |  |  |  |  |


### crates/ferroplan-hddl/src/translate.rs

| `TranslateError` | enum | TranslateError { UnsupportedNegativeGoal, UnsupportedGoalConnective(String), MalformedTermEquality { found: usize, }, UnboundVariable(String), TaskNetworkDepthExceeded { addr: String, limit: usize, }, Timeout { elapsed_ms: u128, limit_ms: u128, }, MemoryLimitExceeded { states: usize, limit: usize, }, Ground(GroundError) } |  |  |  |  |

| `translate` | function | translate( ir: &GroundedIR, limits: &TranslateLimits, ) -> Result<PlanningProblem, TranslateError> |  |  |  |  |

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

| `Client` | struct | Client { child: Child, stdin: Option<ChildStdin>, stdout: BufReader<ChildStdout>, next_id: i64 } |  |  |  |  |


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


### crates/ferroplan/src/api.rs

| `Mode` | enum | Mode { Auto, Ff, Partition, Pddl3, Temporal, Portfolio, Optimal, Sat } |  |  |  |  |

| `Search` | enum | Search { Auto, Ehc, BestFirst, EhcThenBestFirst } |  |  |  |  |

| `SolveError` | enum | SolveError { DomainParse(crate::types::ParseError), ProblemParse(crate::types::ParseError), EmptyType { kind: String, pred: String, ty: String, }, Derived(String), Unsupported(String) } |  |  |  |  |

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

| `accepted` | function | accepted(&self) -> bool |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> |  |  |  |  |

| `expand` | function | expand(domain: &Domain, problem: &Problem) -> Result<Expanded, String> |  |  |  |  |

| `gate` | function | gate(domain: &Domain, problem: &Problem) -> Result<Option<(Domain, Problem)>, String> |  |  |  |  |

| `hard_only_gated` | function | hard_only_gated( domain: &Domain, problem: &Problem, ) -> Result<Option<(Domain, Problem)>, String> |  |  |  |  |

| `new` | function | new(traj: &'a Traj) -> Self |  |  |  |  |

| `op_name` | function | op_name(&self) -> &'static str |  |  |  |  |

| `step` | function | step(&mut self, holds: &mut dyn FnMut(&Formula) -> bool) |  |  |  |  |

| `step_at` | function | step_at(&mut self, time: f64, holds: &mut dyn FnMut(&Formula) -> bool) |  |  |  |  |

| `Expanded` | struct | Expanded { pub hard: Vec<Traj>, pub soft: Vec<(String, Vec<Traj>)> } |  |  |  |  |

| `Fold` | struct | Fold { traj: &'a Traj, ok: bool, seen: bool, holding: bool, pending: bool, safe: bool, last: bool, due: f64 } |  |  |  |  |


### crates/ferroplan/src/costs.rs

| `improve` | function | improve( task: &PackedTask, cf: usize, ops: Vec<usize>, first_cost: f64, threads: usize, base: SearchCfg, spent: usize, orbit: Option<&crate::orbits::OrbitMap>, ) -> CostOutcome |  |  |  |  |

| `improve_length` | function | improve_length( task: &PackedTask, ops: Vec<usize>, threads: usize, base: SearchCfg, spent: usize, ) -> (Vec<usize>, usize, bool) |  |  |  |  |

| `metric_fluent` | function | metric_fluent(problem: &Problem) -> Option<String> |  |  |  |  |

| `optimize_text` | function | optimize_text( problem: &Problem, task: &PackedTask, optimize: bool, threads: usize, cfg: SearchCfg, ops: &mut Vec<usize>, orbit: Option<&crate::orbits::OrbitMap>, ) -> Option<(f64, &'static str)> |  |  |  |  |

| `plan_cost` | function | plan_cost(task: &PackedTask, cf: usize, ops: &[usize]) -> Option<f64> |  |  |  |  |

| `CostOutcome` | struct | CostOutcome { pub ops: Vec<usize>, pub cost: f64, pub improved: bool, pub proven: bool, pub evaluated: usize } |  |  |  |  |


### crates/ferroplan/src/derived.rs

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> |  |  |  |  |


### crates/ferroplan/src/espc.rs

| `espc_optimize` | function | espc_optimize( task: &PackedTask, cost_fluent: usize, sat: &mut SatGuidance, seed: Option<(Vec<usize>, f64)>, part: Option<EspcPartition>, threads: usize, cfg: SearchCfg, ) -> Option<EspcResult> |  |  |  |  |

| `EspcPartition` | struct | EspcPartition { pub comps: Vec<Subgoal>, pub tail: PhaseTail, pub assoc: FxHashMap<u32, Vec<u32>> } |  |  |  |  |

| `EspcResult` | struct | EspcResult { pub ops: Vec<usize>, pub cost: f64, pub iterations: usize } |  |  |  |  |


### crates/ferroplan/src/eve.rs

| `MAX_PRIMARY_ACTIVATORS` | const | MAX_PRIMARY_ACTIVATORS: usize |  |  |  |  |

| `EveError` | enum | EveError { Missing { field: String }, SplitRequired { directive: SplitDirective } } |  |  |  |  |

| `EveStage` | enum | EveStage { GroundHumanPurpose, ProjectGenesis, DecomposeHddl, GovernUncertaintyPpddl, ManufactureGgen, ExposeMcpPlus, ActuateBrce, ObserveOcel2, ConformTruexKernel, AdmitReceipt, ReplayTruex } |  |  |  |  |

| `PlanningRegime` | enum | PlanningRegime { Deterministic, Probabilistic } |  |  |  |  |

| `enter` | function | enter(request: EveRequest) -> Result<EveHandoff, EveError> |  |  |  |  |

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

| `ground` | function | ground(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_fixpoint` | function | ground_fixpoint(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_stratified` | function | ground_stratified(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_stratified_walled` | function | ground_stratified_walled(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_task` | function | ground_task(domain: &Domain, problem: &Problem, threads: usize) -> Option<PackedTask> |  |  |  |  |

| `initial_state` | function | initial_state(t: &PackedTask) -> State |  |  |  |  |

| `objects_by_type` | function | objects_by_type(domain: &Domain, problem: &Problem) -> HashMap<Sym, Vec<Sym>> |  |  |  |  |

| `crate::packed::PackedTask as Task` | use | crate::packed::PackedTask as Task |  |  |  |  |


### crates/ferroplan/src/hash.rs

| `FxHasher` | struct | FxHasher { hash: u64 } |  |  |  |  |


### crates/ferroplan/src/hddl.rs

| `HddlError` | enum | HddlError { Parse(String), Ground(String), Translate(String), Planner(PlannerError), RootTaskMismatch { root_task: String, problem_root_network: Vec<String>, }, Timeout { elapsed_ms: u128, limit_ms: u128, }, WorkerPanicked(String) } |  |  |  |  |

| `adapt_problem` | function | adapt_problem(p: ferroplan_hddl::translate::PlanningProblem) -> PlanningProblem |  |  |  |  |

| `solve_hddl` | function | solve_hddl( domain_src: &str, problem_src: &str, limits: &PlannerLimits, ) -> Result<UniversalPlan, HddlError> |  |  |  |  |

| `solve_hddl_from_eve` | function | solve_hddl_from_eve( handoff: &EveHandoff, limits: &PlannerLimits, ) -> Result<UniversalPlan, HddlError> |  |  |  |  |


### crates/ferroplan/src/heuristic.rs

| `T_BUILD` | const | T_BUILD: std::sync::atomic::AtomicU64 |  |  |  |  |

| `T_EXTRACT` | const | T_EXTRACT: std::sync::atomic::AtomicU64 |  |  |  |  |

| `T_RESET` | const | T_RESET: std::sync::atomic::AtomicU64 |  |  |  |  |

| `extraction_need_facts` | function | extraction_need_facts(sc: &Scratch) -> Vec<(u32, u32)> |  |  |  |  |

| `helpful_needed_adders` | function | helpful_needed_adders( task: &PackedTask, sc: &Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> Vec<u32> |  |  |  |  |

| `new` | function | new(task: &PackedTask) -> Self |  |  |  |  |

| `reachability_layers` | function | reachability_layers( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> (Vec<u32>, Vec<u32>) |  |  |  |  |

| `relaxed` | function | relaxed( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> Option<i32> |  |  |  |  |

| `relaxed_costed` | function | relaxed_costed( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, ) -> Option<i32> |  |  |  |  |

| `relaxed_helpful` | function | relaxed_helpful( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<(i32, Vec<u32>)> |  |  |  |  |

| `relaxed_plan_cost` | function | relaxed_plan_cost( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, ) -> Option<f64> |  |  |  |  |

| `relaxed_to` | function | relaxed_to( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<i32> |  |  |  |  |

| `Scratch` | struct | Scratch { reached: Vec<bool>, fact_layer: Vec<u32>, op_layer: Vec<u32>, gen: u32, op_stamp: Vec<u32>, applicable: Vec<u32>, lb: Vec<f64>, ub: Vec<f64>, selected: Vec<u32>, need_fact: Vec<u32>, queue: Vec<u32>, num_applied: Vec<u32>, cond_ops: Vec<u32>, helpful: Vec<u32>, fact_time: Vec<f64>, op_time: Vec<f64> } |  |  |  |  |

| `TrpgInfo` | struct | TrpgInfo { pub start_of: Vec<u32>, pub lag: Vec<f64>, pub floor: Vec<f64>, pub windows: Vec<Vec<TrpgWindow>> } |  |  |  |  |

| `TrpgWindow` | struct | TrpgWindow { pub fact: u32, pub providers: Vec<(u32, f64)>, pub close: f64 } |  |  |  |  |


### crates/ferroplan/src/introspect.rs

| `explain` | function | explain(domain_src: &str, problem_src: &str, plan: &Plan) -> Result<Explanation, String> |  |  |  |  |

| `CausalLink` | struct | CausalLink { pub provider: Option<usize>, pub consumer: usize, pub fact: String } |  |  |  |  |

| `Explanation` | struct | Explanation { pub kind: String, pub causal_links: Vec<CausalLink>, pub invariant_spans: Vec<InvariantSpan>, pub preferences: Vec<PrefReport> } |  |  |  |  |

| `InvariantSpan` | struct | InvariantSpan { pub step: usize, pub action: String, pub start: f64, pub end: f64, pub conditions: Vec<String> } |  |  |  |  |

| `PrefReport` | struct | PrefReport { pub name: String, pub satisfied: bool, pub weight: f64 } |  |  |  |  |


### crates/ferroplan/src/invariants.rs

| `synthesize` | function | synthesize(domain: &Domain, task: &PackedTask) -> Vec<Vec<u32>> |  |  |  |  |


### crates/ferroplan/src/lama.rs

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

| `solve` | function | solve( task: &PackedTask, cf: Option<usize>, max_nodes: usize, orbit: Option<&crate::orbits::OrbitMap>, ) -> OptOutcome |  |  |  |  |

| `OptOutcome` | struct | OptOutcome { pub ops: Option<Vec<usize>>, pub cost: f64, pub expanded: usize, pub evaluated: usize, pub proven: bool, pub reject: Option<String>, pub heuristic: &'static str, pub clock_tripped: bool } |  |  |  |  |


### crates/ferroplan/src/orbits.rs

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

| `num_threads` | function | num_threads() -> usize |  |  |  |  |

| `par_map` | function | par_map(items: &[T], threads: usize, f: F) -> Vec<R> |  |  |  |  |

| `par_map_with` | function | par_map_with(items: &[T], threads: usize, init: I, f: F) -> Vec<R> |  |  |  |  |


### crates/ferroplan/src/parser.rs

| `parse_domain` | function | parse_domain(src: &str) -> Result<Domain, ParseError> |  |  |  |  |

| `parse_problem` | function | parse_problem(src: &str) -> Result<Problem, ParseError> |  |  |  |  |


### crates/ferroplan/src/partition.rs

| `interaction_partition` | function | interaction_partition(task: &PackedTask, groups: &[Vec<u32>]) -> Vec<Subgoal> |  |  |  |  |

| `interaction_partition_of` | function | interaction_partition_of( task: &PackedTask, groups: &[Vec<u32>], goals: &[u32], excluded_vars: &FxHashSet<usize>, ) -> Vec<Subgoal> |  |  |  |  |

| `is_empty` | function | is_empty(&self) -> bool |  |  |  |  |

| `merge_at` | function | merge_at(groups: &mut Vec<Subgoal>, i: usize, j: usize) -> usize |  |  |  |  |

| `merge_with_neighbor` | function | merge_with_neighbor(groups: &mut Vec<Subgoal>, i: usize) -> usize |  |  |  |  |

| `partition` | function | partition(task: &PackedTask) -> Vec<Subgoal> |  |  |  |  |

| `Subgoal` | struct | Subgoal { pub pos: Vec<u32>, pub num: Vec<NumPre> } |  |  |  |  |


### crates/ferroplan/src/pddl3.rs

| `COST` | const | COST: &str |  |  |  |  |

| `COST_DISP` | const | COST_DISP: &str |  |  |  |  |

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

| `Compiled` | struct | Compiled { pub domain: Domain, pub problem: Problem, pub minimize: bool, pub maximized: bool, pub metric_konst: f64, pub n_prefs: usize, pub warn_other: bool, pub unsupported: Option<String>, pub synthetic: HashSet<String>, pub forgos: Vec<(String, f64)>, pub folded_metric: bool } |  |  |  |  |

| `MetricResult` | struct | MetricResult { pub ops: Vec<usize>, pub cost: f64, pub iterations: usize, pub proven: bool } |  |  |  |  |

| `PhaseTail` | struct | PhaseTail { pub end_op: usize, pub prefs: Vec<(Vec<usize>, usize)> } |  |  |  |  |

| `SeededResult` | struct | SeededResult { pub result: MetricResult, pub from_seed: bool } |  |  |  |  |


### crates/ferroplan/src/plan.rs

| `Validity` | enum | Validity { Valid, Invalid(String) } |  |  |  |  |

| `parse_classical` | function | parse_classical(src: &str) -> Vec<(String, Vec<String>)> |  |  |  |  |

| `parse_timed` | function | parse_timed(src: &str) -> Result<TimedPlan, String> |  |  |  |  |

| `validate_plan` | function | validate_plan( domain_src: &str, problem_src: &str, plan_src: &str, ) -> Result<Validity, String> |  |  |  |  |


### crates/ferroplan/src/planner.rs

| `run_ff` | function | run_ff(domain_src: &str, problem_src: &str, opts: &crate::Options) -> (String, i32) |  |  |  |  |

| `run_planner` | function | run_planner( domain_src: &str, problem_src: &str, opts: &crate::Options, ipc: bool, ) -> (String, i32) |  |  |  |  |


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

| `solve` | function | solve(task: &PackedTask, threads: usize, cfg: SearchCfg) -> Outcome |  |  |  |  |

| `Outcome` | struct | Outcome { pub ops: Option<Vec<usize>>, pub evaluated: usize, pub winner: Option<&'static str> } |  |  |  |  |


### crates/ferroplan/src/ppddl.rs

| `PpddlError` | enum | PpddlError { Syntax(String), DomainParse(ParseError), ProblemParse(ParseError), Derived(String), Unsupported(String), InvalidProbability(String), InvalidOptions(String), OutcomeLimit { action: String, limit: usize }, StateLimit { limit: usize }, TransitionLimit { limit: usize }, GroundingFailed, GroundingDivergence { action: String, expected: usize, observed: usize, }, InitialOutcomeLimit { limit: usize }, RewardViolation(String), PolicyLimit { limit: usize }, ValueTableLimit { limit: usize } } |  |  |  |  |

| `ProbabilisticObjective` | enum | ProbabilisticObjective { Auto, MaximizeGoalProbability, MinimizeGoalProbability, MaximizeExpectedReward, MinimizeExpectedReward, MaximizeExpectedMetric, MinimizeExpectedMetric } |  |  |  |  |

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

| `PlanValidationEvidence` | struct | PlanValidationEvidence { pub valid: bool, pub reason: Option<String> } |  |  |  |  |

| `ProductionSession` | struct | ProductionSession { inner: Session, domain: String, problem: String, limits: ProductionLimits, input_fingerprint: String } |  |  |  |  |


### crates/ferroplan/src/production_explain.rs

| `decompose_production` | function | decompose_production( domain: &str, problem: &str, options: &Options, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Decomposition> |  |  |  |  |

| `explain_production` | function | explain_production( domain: &str, problem: &str, plan: &Plan, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Explanation> |  |  |  |  |


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

| `solve` | function | solve( task: &PackedTask, threads: usize, cfg: crate::search::SearchCfg, mutex_groups: &[Vec<u32>], orbit: Option<&crate::orbits::OrbitMap>, ) -> Solved |  |  |  |  |

| `Stats` | struct | Stats { pub init_groups: usize, pub final_groups: usize, pub merges: usize, pub fallback: bool } |  |  |  |  |


### crates/ferroplan/src/resource.rs

| `detect_resources` | function | detect_resources(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Vec<ResourceVar> |  |  |  |  |

| `occupancy` | function | occupancy(&self, bits: &[u64]) -> u32 |  |  |  |  |

| `trip_bound` | function | trip_bound(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Option<TripBound> |  |  |  |  |

| `trips` | function | trips(&self, bits: &[u64]) -> i64 |  |  |  |  |

| `ResourceVar` | struct | ResourceVar { pub members: Vec<(u32, u32)> } |  |  |  |  |

| `TripBound` | struct | TripBound { pub goals: Vec<u32>, pub pool: i64 } |  |  |  |  |


### crates/ferroplan/src/sat.rs

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

| `Session` | struct | Session { task: PackedTask, threads: usize, weight_g: f64, weight_h: f64, max_evaluated: Option<usize>, ehc_first: bool, fact_ids: Arc<FxHashMap<String, u32>>, dynamic: Arc<[bool]>, fluent_ids: Arc<FxHashMap<String, u32>>, temporal: Option<Arc<crate::temporal::TemporalCompiled>>, tier: crate::features::DemandMode, running_preds: Vec<String>, op_ids: Arc<FxHashMap<String, usize>>, mirror: Arc<FxHashMap<u32, u32>>, forbidden: Vec<bool>, timed: Vec<(f64, u32, bool)>, til_setters: Arc<FxHashMap<(u32, bool), usize>>, running: Vec<(f64, usize)>, lifted: Option<Arc<(crate::types::Domain, crate::types::Problem)>>, goal_formula: Formula } |  |  |  |  |

| `Think` | struct | Think { pub solution: Solution, pub capped: bool, pub spent_ms: u64, pub spent_evals: usize, pub verdict: ThinkVerdict } |  |  |  |  |

| `ThinkBudget` | struct | ThinkBudget { pub max_evaluated: Option<usize>, pub wall_ms: Option<u64>, pub memory_mb: Option<usize> } |  |  |  |  |


### crates/ferroplan/src/tcompress.rs

| `UNWALLED_EVALS` | const | UNWALLED_EVALS: usize |  |  |  |  |

| `WALL_FRAC` | const | WALL_FRAC: f64 |  |  |  |  |

| `Bet` | enum | Bet { First, Rest } |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> (Domain, Problem) |  |  |  |  |

| `declines` | function | declines(domain: &Domain, problem: &Problem) -> Option<&'static str> |  |  |  |  |

| `lay_out` | function | lay_out( domain: &Domain, task: &PackedTask, ops: &[usize], shift: bool, ) -> Option<TimedPlan> |  |  |  |  |

| `solve` | function | solve(domain: &Domain, problem: &Problem, threads: usize, bet: Bet) -> Option<TimedPlan> |  |  |  |  |


### crates/ferroplan/src/temporal.rs

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

| `StateSnapshot` | struct | StateSnapshot { pub facts: Vec<String>, pub fluents: Vec<(String, f64)> } |  |  |  |  |


### crates/ferroplan/src/tresolve.rs

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

| `VizEdge` | struct | VizEdge { pub a: String, pub b: String, pub pred: String } |  |  |  |  |

| `VizGraph` | struct | VizGraph { pub nodes: Vec<VizNode>, pub edges: Vec<VizEdge>, pub mobiles: Vec<VizMobile>, pub props_by_object: BTreeMap<String, Vec<String>>, pub goal_by_object: BTreeMap<String, Vec<String>>, pub pred_kind: BTreeMap<String, PredKind>, pub location_types: BTreeSet<String> } |  |  |  |  |

| `VizMobile` | struct | VizMobile { pub object: String, pub ty: String, pub at: Option<String>, pub at_raw: Option<String> } |  |  |  |  |

| `VizNode` | struct | VizNode { pub object: String, pub ty: String } |  |  |  |  |


### crates/ferroplan/tests/common/external.rs

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

| `Model` | struct | Model { types: Vec<String>, preds: Vec<(String, Vec<usize>)>, actions: Vec<ActionM>, tasks: Vec<(String, Vec<(String, usize)>)>, methods: Vec<MethodM>, objects: Vec<(String, usize)>, init: Vec<LitO>, goal: Vec<LitO>, root: Vec<(String, CallM)> } |  |  |  |  |

| `Rng` | struct | Rng { u64 } |  |  |  |  |

| `Sizes` | struct | Sizes { pub types: usize, pub preds: usize, pub tasks: usize, pub actions: usize, pub objects: usize, pub subs: usize, pub root_subs: usize } |  |  |  |  |



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

| `model::*` | use | model::* |  |  |  |  |

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

| `has_probabilistic` | function | has_probabilistic(src: &str) -> bool |  |  |  |  |

| `preprocess` | function | preprocess(src: &str) -> Result<(String, BTreeMap<String, Vec<String>>), ParseError> |  |  |  |  |

| `TranslateError` | enum | TranslateError { UnsupportedNegativeGoal, UnsupportedGoalConnective(String), MalformedTermEquality { found: usize, }, UnboundVariable(String), TaskNetworkDepthExceeded { addr: String, limit: usize, }, Timeout { elapsed_ms: u128, limit_ms: u128, }, MemoryLimitExceeded { states: usize, limit: usize, }, Ground(GroundError) } |  |  |  |  |

| `translate` | function | translate( ir: &GroundedIR, limits: &TranslateLimits, ) -> Result<PlanningProblem, TranslateError> |  |  |  |  |

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

| `DOM` | const | DOM: &str |  |  |  |  |

| `PROB` | const | PROB: &str |  |  |  |  |

| `call` | function | call(&mut self, tool: &str, args: Value) -> Value |  |  |  |  |

| `call_json` | function | call_json(&mut self, tool: &str, args: Value) -> Value |  |  |  |  |

| `call_text` | function | call_text(&mut self, tool: &str, args: Value) -> (String, bool) |  |  |  |  |

| `finish` | function | finish(mut self) |  |  |  |  |

| `notify` | function | notify(&mut self, method: &str) |  |  |  |  |

| `request` | function | request(&mut self, method: &str, params: Value) -> Value |  |  |  |  |

| `start` | function | start() -> Client |  |  |  |  |

| `Client` | struct | Client { child: Child, stdin: Option<ChildStdin>, stdout: BufReader<ChildStdout>, next_id: i64 } |  |  |  |  |

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

| `Mode` | enum | Mode { Auto, Ff, Partition, Pddl3, Temporal, Portfolio, Optimal, Sat } |  |  |  |  |

| `Search` | enum | Search { Auto, Ehc, BestFirst, EhcThenBestFirst } |  |  |  |  |

| `SolveError` | enum | SolveError { DomainParse(crate::types::ParseError), ProblemParse(crate::types::ParseError), EmptyType { kind: String, pred: String, ty: String, }, Derived(String), Unsupported(String) } |  |  |  |  |

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

| `accepted` | function | accepted(&self) -> bool |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> |  |  |  |  |

| `expand` | function | expand(domain: &Domain, problem: &Problem) -> Result<Expanded, String> |  |  |  |  |

| `gate` | function | gate(domain: &Domain, problem: &Problem) -> Result<Option<(Domain, Problem)>, String> |  |  |  |  |

| `hard_only_gated` | function | hard_only_gated( domain: &Domain, problem: &Problem, ) -> Result<Option<(Domain, Problem)>, String> |  |  |  |  |

| `new` | function | new(traj: &'a Traj) -> Self |  |  |  |  |

| `op_name` | function | op_name(&self) -> &'static str |  |  |  |  |

| `step` | function | step(&mut self, holds: &mut dyn FnMut(&Formula) -> bool) |  |  |  |  |

| `step_at` | function | step_at(&mut self, time: f64, holds: &mut dyn FnMut(&Formula) -> bool) |  |  |  |  |

| `Expanded` | struct | Expanded { pub hard: Vec<Traj>, pub soft: Vec<(String, Vec<Traj>)> } |  |  |  |  |

| `Fold` | struct | Fold { traj: &'a Traj, ok: bool, seen: bool, holding: bool, pending: bool, safe: bool, last: bool, due: f64 } |  |  |  |  |

| `improve` | function | improve( task: &PackedTask, cf: usize, ops: Vec<usize>, first_cost: f64, threads: usize, base: SearchCfg, spent: usize, orbit: Option<&crate::orbits::OrbitMap>, ) -> CostOutcome |  |  |  |  |

| `improve_length` | function | improve_length( task: &PackedTask, ops: Vec<usize>, threads: usize, base: SearchCfg, spent: usize, ) -> (Vec<usize>, usize, bool) |  |  |  |  |

| `metric_fluent` | function | metric_fluent(problem: &Problem) -> Option<String> |  |  |  |  |

| `optimize_text` | function | optimize_text( problem: &Problem, task: &PackedTask, optimize: bool, threads: usize, cfg: SearchCfg, ops: &mut Vec<usize>, orbit: Option<&crate::orbits::OrbitMap>, ) -> Option<(f64, &'static str)> |  |  |  |  |

| `plan_cost` | function | plan_cost(task: &PackedTask, cf: usize, ops: &[usize]) -> Option<f64> |  |  |  |  |

| `CostOutcome` | struct | CostOutcome { pub ops: Vec<usize>, pub cost: f64, pub improved: bool, pub proven: bool, pub evaluated: usize } |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> |  |  |  |  |

| `espc_optimize` | function | espc_optimize( task: &PackedTask, cost_fluent: usize, sat: &mut SatGuidance, seed: Option<(Vec<usize>, f64)>, part: Option<EspcPartition>, threads: usize, cfg: SearchCfg, ) -> Option<EspcResult> |  |  |  |  |

| `EspcPartition` | struct | EspcPartition { pub comps: Vec<Subgoal>, pub tail: PhaseTail, pub assoc: FxHashMap<u32, Vec<u32>> } |  |  |  |  |

| `EspcResult` | struct | EspcResult { pub ops: Vec<usize>, pub cost: f64, pub iterations: usize } |  |  |  |  |

| `MAX_PRIMARY_ACTIVATORS` | const | MAX_PRIMARY_ACTIVATORS: usize |  |  |  |  |

| `EveError` | enum | EveError { Missing { field: String }, SplitRequired { directive: SplitDirective } } |  |  |  |  |

| `EveStage` | enum | EveStage { GroundHumanPurpose, ProjectGenesis, DecomposeHddl, GovernUncertaintyPpddl, ManufactureGgen, ExposeMcpPlus, ActuateBrce, ObserveOcel2, ConformTruexKernel, AdmitReceipt, ReplayTruex } |  |  |  |  |

| `PlanningRegime` | enum | PlanningRegime { Deterministic, Probabilistic } |  |  |  |  |

| `enter` | function | enter(request: EveRequest) -> Result<EveHandoff, EveError> |  |  |  |  |

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

| `ground` | function | ground(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_fixpoint` | function | ground_fixpoint(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_stratified` | function | ground_stratified(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_stratified_walled` | function | ground_stratified_walled(domain: &Domain, problem: &Problem, threads: usize) -> Outcome |  |  |  |  |

| `ground_task` | function | ground_task(domain: &Domain, problem: &Problem, threads: usize) -> Option<PackedTask> |  |  |  |  |

| `initial_state` | function | initial_state(t: &PackedTask) -> State |  |  |  |  |

| `objects_by_type` | function | objects_by_type(domain: &Domain, problem: &Problem) -> HashMap<Sym, Vec<Sym>> |  |  |  |  |

| `crate::packed::PackedTask as Task` | use | crate::packed::PackedTask as Task |  |  |  |  |

| `FxHasher` | struct | FxHasher { hash: u64 } |  |  |  |  |

| `HddlError` | enum | HddlError { Parse(String), Ground(String), Translate(String), Planner(PlannerError), RootTaskMismatch { root_task: String, problem_root_network: Vec<String>, }, Timeout { elapsed_ms: u128, limit_ms: u128, }, WorkerPanicked(String) } |  |  |  |  |

| `adapt_problem` | function | adapt_problem(p: ferroplan_hddl::translate::PlanningProblem) -> PlanningProblem |  |  |  |  |

| `solve_hddl` | function | solve_hddl( domain_src: &str, problem_src: &str, limits: &PlannerLimits, ) -> Result<UniversalPlan, HddlError> |  |  |  |  |

| `solve_hddl_from_eve` | function | solve_hddl_from_eve( handoff: &EveHandoff, limits: &PlannerLimits, ) -> Result<UniversalPlan, HddlError> |  |  |  |  |

| `T_BUILD` | const | T_BUILD: std::sync::atomic::AtomicU64 |  |  |  |  |

| `T_EXTRACT` | const | T_EXTRACT: std::sync::atomic::AtomicU64 |  |  |  |  |

| `T_RESET` | const | T_RESET: std::sync::atomic::AtomicU64 |  |  |  |  |

| `extraction_need_facts` | function | extraction_need_facts(sc: &Scratch) -> Vec<(u32, u32)> |  |  |  |  |

| `helpful_needed_adders` | function | helpful_needed_adders( task: &PackedTask, sc: &Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> Vec<u32> |  |  |  |  |

| `new` | function | new(task: &PackedTask) -> Self |  |  |  |  |

| `reachability_layers` | function | reachability_layers( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> (Vec<u32>, Vec<u32>) |  |  |  |  |

| `relaxed` | function | relaxed( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], ) -> Option<i32> |  |  |  |  |

| `relaxed_costed` | function | relaxed_costed( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, ) -> Option<i32> |  |  |  |  |

| `relaxed_helpful` | function | relaxed_helpful( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<(i32, Vec<u32>)> |  |  |  |  |

| `relaxed_plan_cost` | function | relaxed_plan_cost( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], cost_fluent: usize, ) -> Option<f64> |  |  |  |  |

| `relaxed_to` | function | relaxed_to( task: &PackedTask, sc: &mut Scratch, bits: &[u64], fv: &[f64], def: &[bool], goal_pos: &[u32], goal_num: &[NumPre], ) -> Option<i32> |  |  |  |  |

| `Scratch` | struct | Scratch { reached: Vec<bool>, fact_layer: Vec<u32>, op_layer: Vec<u32>, gen: u32, op_stamp: Vec<u32>, applicable: Vec<u32>, lb: Vec<f64>, ub: Vec<f64>, selected: Vec<u32>, need_fact: Vec<u32>, queue: Vec<u32>, num_applied: Vec<u32>, cond_ops: Vec<u32>, helpful: Vec<u32>, fact_time: Vec<f64>, op_time: Vec<f64> } |  |  |  |  |

| `TrpgInfo` | struct | TrpgInfo { pub start_of: Vec<u32>, pub lag: Vec<f64>, pub floor: Vec<f64>, pub windows: Vec<Vec<TrpgWindow>> } |  |  |  |  |

| `TrpgWindow` | struct | TrpgWindow { pub fact: u32, pub providers: Vec<(u32, f64)>, pub close: f64 } |  |  |  |  |

| `explain` | function | explain(domain_src: &str, problem_src: &str, plan: &Plan) -> Result<Explanation, String> |  |  |  |  |

| `CausalLink` | struct | CausalLink { pub provider: Option<usize>, pub consumer: usize, pub fact: String } |  |  |  |  |

| `Explanation` | struct | Explanation { pub kind: String, pub causal_links: Vec<CausalLink>, pub invariant_spans: Vec<InvariantSpan>, pub preferences: Vec<PrefReport> } |  |  |  |  |

| `InvariantSpan` | struct | InvariantSpan { pub step: usize, pub action: String, pub start: f64, pub end: f64, pub conditions: Vec<String> } |  |  |  |  |

| `PrefReport` | struct | PrefReport { pub name: String, pub satisfied: bool, pub weight: f64 } |  |  |  |  |

| `synthesize` | function | synthesize(domain: &Domain, task: &PackedTask) -> Vec<Vec<u32>> |  |  |  |  |

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

| `solve` | function | solve( task: &PackedTask, cf: Option<usize>, max_nodes: usize, orbit: Option<&crate::orbits::OrbitMap>, ) -> OptOutcome |  |  |  |  |

| `OptOutcome` | struct | OptOutcome { pub ops: Option<Vec<usize>>, pub cost: f64, pub expanded: usize, pub evaluated: usize, pub proven: bool, pub reject: Option<String>, pub heuristic: &'static str, pub clock_tripped: bool } |  |  |  |  |

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

| `num_threads` | function | num_threads() -> usize |  |  |  |  |

| `par_map` | function | par_map(items: &[T], threads: usize, f: F) -> Vec<R> |  |  |  |  |

| `par_map_with` | function | par_map_with(items: &[T], threads: usize, init: I, f: F) -> Vec<R> |  |  |  |  |

| `parse_domain` | function | parse_domain(src: &str) -> Result<Domain, ParseError> |  |  |  |  |

| `parse_problem` | function | parse_problem(src: &str) -> Result<Problem, ParseError> |  |  |  |  |

| `interaction_partition` | function | interaction_partition(task: &PackedTask, groups: &[Vec<u32>]) -> Vec<Subgoal> |  |  |  |  |

| `interaction_partition_of` | function | interaction_partition_of( task: &PackedTask, groups: &[Vec<u32>], goals: &[u32], excluded_vars: &FxHashSet<usize>, ) -> Vec<Subgoal> |  |  |  |  |

| `is_empty` | function | is_empty(&self) -> bool |  |  |  |  |

| `merge_at` | function | merge_at(groups: &mut Vec<Subgoal>, i: usize, j: usize) -> usize |  |  |  |  |

| `merge_with_neighbor` | function | merge_with_neighbor(groups: &mut Vec<Subgoal>, i: usize) -> usize |  |  |  |  |

| `partition` | function | partition(task: &PackedTask) -> Vec<Subgoal> |  |  |  |  |

| `Subgoal` | struct | Subgoal { pub pos: Vec<u32>, pub num: Vec<NumPre> } |  |  |  |  |

| `COST` | const | COST: &str |  |  |  |  |

| `COST_DISP` | const | COST_DISP: &str |  |  |  |  |

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

| `Compiled` | struct | Compiled { pub domain: Domain, pub problem: Problem, pub minimize: bool, pub maximized: bool, pub metric_konst: f64, pub n_prefs: usize, pub warn_other: bool, pub unsupported: Option<String>, pub synthetic: HashSet<String>, pub forgos: Vec<(String, f64)>, pub folded_metric: bool } |  |  |  |  |

| `MetricResult` | struct | MetricResult { pub ops: Vec<usize>, pub cost: f64, pub iterations: usize, pub proven: bool } |  |  |  |  |

| `PhaseTail` | struct | PhaseTail { pub end_op: usize, pub prefs: Vec<(Vec<usize>, usize)> } |  |  |  |  |

| `SeededResult` | struct | SeededResult { pub result: MetricResult, pub from_seed: bool } |  |  |  |  |

| `Validity` | enum | Validity { Valid, Invalid(String) } |  |  |  |  |

| `parse_classical` | function | parse_classical(src: &str) -> Vec<(String, Vec<String>)> |  |  |  |  |

| `parse_timed` | function | parse_timed(src: &str) -> Result<TimedPlan, String> |  |  |  |  |

| `validate_plan` | function | validate_plan( domain_src: &str, problem_src: &str, plan_src: &str, ) -> Result<Validity, String> |  |  |  |  |

| `run_ff` | function | run_ff(domain_src: &str, problem_src: &str, opts: &crate::Options) -> (String, i32) |  |  |  |  |

| `run_planner` | function | run_planner( domain_src: &str, problem_src: &str, opts: &crate::Options, ipc: bool, ) -> (String, i32) |  |  |  |  |

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

| `solve` | function | solve(task: &PackedTask, threads: usize, cfg: SearchCfg) -> Outcome |  |  |  |  |

| `Outcome` | struct | Outcome { pub ops: Option<Vec<usize>>, pub evaluated: usize, pub winner: Option<&'static str> } |  |  |  |  |

| `PpddlError` | enum | PpddlError { Syntax(String), DomainParse(ParseError), ProblemParse(ParseError), Derived(String), Unsupported(String), InvalidProbability(String), InvalidOptions(String), OutcomeLimit { action: String, limit: usize }, StateLimit { limit: usize }, TransitionLimit { limit: usize }, GroundingFailed, GroundingDivergence { action: String, expected: usize, observed: usize, }, InitialOutcomeLimit { limit: usize }, RewardViolation(String), PolicyLimit { limit: usize }, ValueTableLimit { limit: usize } } |  |  |  |  |

| `ProbabilisticObjective` | enum | ProbabilisticObjective { Auto, MaximizeGoalProbability, MinimizeGoalProbability, MaximizeExpectedReward, MinimizeExpectedReward, MaximizeExpectedMetric, MinimizeExpectedMetric } |  |  |  |  |

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

| `PlanValidationEvidence` | struct | PlanValidationEvidence { pub valid: bool, pub reason: Option<String> } |  |  |  |  |

| `ProductionSession` | struct | ProductionSession { inner: Session, domain: String, problem: String, limits: ProductionLimits, input_fingerprint: String } |  |  |  |  |

| `decompose_production` | function | decompose_production( domain: &str, problem: &str, options: &Options, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Decomposition> |  |  |  |  |

| `explain_production` | function | explain_production( domain: &str, problem: &str, plan: &Plan, limits: &ProductionLimits, request_id: Option<&str>, ) -> OperationEnvelope<Explanation> |  |  |  |  |

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

| `solve` | function | solve( task: &PackedTask, threads: usize, cfg: crate::search::SearchCfg, mutex_groups: &[Vec<u32>], orbit: Option<&crate::orbits::OrbitMap>, ) -> Solved |  |  |  |  |

| `Stats` | struct | Stats { pub init_groups: usize, pub final_groups: usize, pub merges: usize, pub fallback: bool } |  |  |  |  |

| `detect_resources` | function | detect_resources(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Vec<ResourceVar> |  |  |  |  |

| `occupancy` | function | occupancy(&self, bits: &[u64]) -> u32 |  |  |  |  |

| `trip_bound` | function | trip_bound(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Option<TripBound> |  |  |  |  |

| `trips` | function | trips(&self, bits: &[u64]) -> i64 |  |  |  |  |

| `ResourceVar` | struct | ResourceVar { pub members: Vec<(u32, u32)> } |  |  |  |  |

| `TripBound` | struct | TripBound { pub goals: Vec<u32>, pub pool: i64 } |  |  |  |  |

| `from_env` | function | from_env() -> Self |  |  |  |  |

| `requires_concurrency` | function | requires_concurrency(domain: &Domain, problem: &Problem) -> bool |  |  |  |  |

| `solve_classical` | function | solve_classical( task: &PackedTask, groups: &[Vec<u32>], cfg: &SatCfg, ) -> SatOutcome<Vec<usize>> |  |  |  |  |

| `solve_temporal` | function | solve_temporal( domain: &Domain, problem: &Problem, threads: usize, cfg: &SatCfg, ) -> SatOutcome<TimedPlan> |  |  |  |  |

| `solve_temporal_within` | function | solve_temporal_within( domain: &Domain, problem: &Problem, threads: usize, cfg: &SatCfg, budget_secs: Option<f64>, ) -> SatOutcome<TimedPlan> |  |  |  |  |

| `SatCfg` | struct | SatCfg { pub max_horizon: usize, pub conflicts_per_horizon: u64, pub cap_lits: u64 } |  |  |  |  |

| `SatOutcome` | struct | SatOutcome { pub plan: Option<P>, pub notes: Vec<String>, pub proven_at_every_horizon: bool, pub grounded_facts: usize, pub grounded_actions: usize } |  |  |  |  |

| `DEFAULT_MAX_EVAL` | const | DEFAULT_MAX_EVAL: usize |  |  |  |  |

| `PlanResult` | enum | PlanResult { Plan { ops: Vec<usize>, advance: Vec<i32>, evaluated: usize, max_g: usize, }, Unsolvable { evaluated: usize, capped: bool, } } |  |  |  |  |

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

| `Session` | struct | Session { task: PackedTask, threads: usize, weight_g: f64, weight_h: f64, max_evaluated: Option<usize>, ehc_first: bool, fact_ids: Arc<FxHashMap<String, u32>>, dynamic: Arc<[bool]>, fluent_ids: Arc<FxHashMap<String, u32>>, temporal: Option<Arc<crate::temporal::TemporalCompiled>>, tier: crate::features::DemandMode, running_preds: Vec<String>, op_ids: Arc<FxHashMap<String, usize>>, mirror: Arc<FxHashMap<u32, u32>>, forbidden: Vec<bool>, timed: Vec<(f64, u32, bool)>, til_setters: Arc<FxHashMap<(u32, bool), usize>>, running: Vec<(f64, usize)>, lifted: Option<Arc<(crate::types::Domain, crate::types::Problem)>>, goal_formula: Formula } |  |  |  |  |

| `Think` | struct | Think { pub solution: Solution, pub capped: bool, pub spent_ms: u64, pub spent_evals: usize, pub verdict: ThinkVerdict } |  |  |  |  |

| `ThinkBudget` | struct | ThinkBudget { pub max_evaluated: Option<usize>, pub wall_ms: Option<u64>, pub memory_mb: Option<usize> } |  |  |  |  |

| `UNWALLED_EVALS` | const | UNWALLED_EVALS: usize |  |  |  |  |

| `WALL_FRAC` | const | WALL_FRAC: f64 |  |  |  |  |

| `Bet` | enum | Bet { First, Rest } |  |  |  |  |

| `compile` | function | compile(domain: &Domain, problem: &Problem) -> (Domain, Problem) |  |  |  |  |

| `declines` | function | declines(domain: &Domain, problem: &Problem) -> Option<&'static str> |  |  |  |  |

| `lay_out` | function | lay_out( domain: &Domain, task: &PackedTask, ops: &[usize], shift: bool, ) -> Option<TimedPlan> |  |  |  |  |

| `solve` | function | solve(domain: &Domain, problem: &Problem, threads: usize, bet: Bet) -> Option<TimedPlan> |  |  |  |  |

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

| `StateSnapshot` | struct | StateSnapshot { pub facts: Vec<String>, pub fluents: Vec<(String, f64)> } |  |  |  |  |

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

| `VizEdge` | struct | VizEdge { pub a: String, pub b: String, pub pred: String } |  |  |  |  |

| `VizGraph` | struct | VizGraph { pub nodes: Vec<VizNode>, pub edges: Vec<VizEdge>, pub mobiles: Vec<VizMobile>, pub props_by_object: BTreeMap<String, Vec<String>>, pub goal_by_object: BTreeMap<String, Vec<String>>, pub pred_kind: BTreeMap<String, PredKind>, pub location_types: BTreeSet<String> } |  |  |  |  |

| `VizMobile` | struct | VizMobile { pub object: String, pub ty: String, pub at: Option<String>, pub at_raw: Option<String> } |  |  |  |  |

| `VizNode` | struct | VizNode { pub object: String, pub ty: String } |  |  |  |  |

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

| `Model` | struct | Model { types: Vec<String>, preds: Vec<(String, Vec<usize>)>, actions: Vec<ActionM>, tasks: Vec<(String, Vec<(String, usize)>)>, methods: Vec<MethodM>, objects: Vec<(String, usize)>, init: Vec<LitO>, goal: Vec<LitO>, root: Vec<(String, CallM)> } |  |  |  |  |

| `Rng` | struct | Rng { u64 } |  |  |  |  |

| `Sizes` | struct | Sizes { pub types: usize, pub preds: usize, pub tasks: usize, pub actions: usize, pub objects: usize, pub subs: usize, pub root_subs: usize } |  |  |  |  |


<!-- ============================================================= -->
<!-- AGENT-FORBIDDEN-END: nothing below this line may describe     -->
<!-- code behavior.                                                -->
<!-- ============================================================= -->
