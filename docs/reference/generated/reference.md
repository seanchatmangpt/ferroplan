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

| advance | function | advance(time: Res<Time>, mut plan: ResMut<Plan>) |  |  |  |  |

| animate | function | animate(
    plan: Res<Plan>,
    scene: Res<Scene>,
    nodes: Query<(&NodeObj, &Transform) |  |  |  |  |

| controls | function | controls(
    keys: Res<ButtonInput<KeyCode>>,
    scene: Res<Scene>,
    editor: Res<crate::blocks::Editor>,
    mut plan: ResMut<Plan>,
    mut job: ResMut<SolveJob>,
) |  |  |  |  |

| frac | function | frac(&self) |  |  |  |  |

| poll_solve | function | poll_solve(mut job: ResMut<SolveJob>, mut plan: ResMut<Plan>) |  |  |  |  |

| span | function | span(&self) |  |  |  |  |

| start_frac | function | start_frac(&self, step: &Step, idx: usize) |  |  |  |  |

| Plan | struct |  |  |  |  |  |

| SolveJob | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/blocks.rs

| Act | enum |  |  |  |  |  |

| DragKind | enum |  |  |  |  |  |

| Focus | enum |  |  |  |  |  |

| LitLoc | enum |  |  |  |  |  |

| Mode | enum |  |  |  |  |  |

| Zone | enum |  |  |  |  |  |

| editor_drag | function | editor_drag(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut drag: ResMut<Drag>,
    mut editor: ResMut<Editor>,
    grips: Query<(&DragKind, &RelativeCursorPosition) |  |  |  |  |

| handle_clicks | function | handle_clicks(
    interactions: Query<(&Interaction, &Act) |  |  |  |  |

| rebuild | function | rebuild(
    mut commands: Commands,
    mut editor: ResMut<Editor>,
    roots: Query<Entity, With<EditorRoot>>,
) |  |  |  |  |

| scroll_editor | function | scroll_editor(
    mut wheel: MessageReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    editor: Res<Editor>,
    mut q: Query<&mut ScrollPosition, With<EditorRoot>>,
) |  |  |  |  |

| text_input | function | text_input(mut evr: MessageReader<KeyboardInput>, mut editor: ResMut<Editor>) |  |  |  |  |

| toggle_editor | function | toggle_editor(
    keys: Res<ButtonInput<KeyCode>>,
    scene: Res<Scene>,
    mut editor: ResMut<Editor>,
) |  |  |  |  |

| Drag | struct |  |  |  |  |  |

| Editor | struct |  |  |  |  |  |

| EditorRoot | struct |  |  |  |  |  |

| Ghost | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/gantt.rs

| gantt_now | function | gantt_now(plan: Res<Plan>, mut now: Query<&mut Node, With<GanttNow>>) |  |  |  |  |

| gantt_visibility | function | gantt_visibility(
    plan: Res<Plan>,
    state: Res<GanttState>,
    editor: Res<crate::blocks::Editor>,
    mut panel: Query<&mut Visibility, With<GanttPanel>>,
) |  |  |  |  |

| rebuild_gantt | function | rebuild_gantt(
    mut commands: Commands,
    plan: Res<Plan>,
    mut state: ResMut<GanttState>,
    track: Query<Entity, With<GanttTrack>>,
    bars: Query<Entity, With<GanttBar>>,
) |  |  |  |  |

| setup_gantt | function | setup_gantt(mut commands: Commands) |  |  |  |  |

| toggle_gantt | function | toggle_gantt(
    keys: Res<ButtonInput<KeyCode>>,
    editor: Res<crate::blocks::Editor>,
    mut state: ResMut<GanttState>,
) |  |  |  |  |

| GanttBar | struct |  |  |  |  |  |

| GanttNow | struct |  |  |  |  |  |

| GanttPanel | struct |  |  |  |  |  |

| GanttState | struct |  |  |  |  |  |

| GanttTrack | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/icons.rs

| IconShape | enum |  |  |  |  |  |

| color_for | function | color_for(ty: &str) |  |  |  |  |

| mat_handle | function | mat_handle(
    materials: &mut Assets<ColorMaterial>,
    cache: &mut MatCache,
    color: Color,
) |  |  |  |  |

| mesh_handle | function | mesh_handle(
    meshes: &mut Assets<Mesh>,
    cache: &mut MeshCache,
    shape: IconShape,
    size: f32,
) |  |  |  |  |

| shape_for | function | shape_for(ty: &str) |  |  |  |  |


### crates/ferroplan-bevy/src/interact.rs

| draw_selection | function | draw_selection(
    mut gizmos: Gizmos,
    selected: Res<Selected>,
    nodes: Query<(&NodeObj, &Transform) |  |  |  |  |

| interact | function | interact(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    editor: Res<crate::blocks::Editor>,
    cam_q: Query<(&Camera, &GlobalTransform) |  |  |  |  |

| DragState | struct |  |  |  |  |  |

| Selected | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/scene.rs

| camera_nav | function | camera_nav(
    mouse: Res<ButtonInput<MouseButton>>,
    editor: Res<crate::blocks::Editor>,
    mut motion: MessageReader<bevy::input::mouse::MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    mut cam: Query<(&mut Transform, &mut Projection) |  |  |  |  |

| draw_edges | function | draw_edges(
    mut gizmos: Gizmos,
    scene: Res<Scene>,
    plan: Res<crate::anim::Plan>,
    nodes: Query<(&NodeObj, &Transform) |  |  |  |  |

| handle_drops | function | handle_drops(mut drops: MessageReader<FileDragAndDrop>, mut scene: ResMut<Scene>) |  |  |  |  |

| load_src | function | load_src(&mut self, src: &str) |  |  |  |  |

| respawn_graph | function | respawn_graph(
    mut commands: Commands,
    mut scene: ResMut<Scene>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing: Query<Entity, With<GraphItem>>,
) |  |  |  |  |

| setup | function | setup(mut commands: Commands) |  |  |  |  |

| FanOffset | struct |  |  |  |  |  |

| GraphItem | struct |  |  |  |  |  |

| MainCamera | struct |  |  |  |  |  |

| MobileObj | struct |  |  |  |  |  |

| NodeObj | struct |  |  |  |  |  |

| Scene | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/transport.rs

| rebuild_notches | function | rebuild_notches(
    mut commands: Commands,
    plan: Res<Plan>,
    mut state: ResMut<Transport>,
    track: Query<Entity, With<ScrubTrack>>,
    notches: Query<Entity, With<StepNotch>>,
) |  |  |  |  |

| setup_transport | function | setup_transport(mut commands: Commands) |  |  |  |  |

| transport_input | function | transport_input(
    mouse: Res<ButtonInput<MouseButton>>,
    mut transport: ResMut<Transport>,
    mut plan: ResMut<Plan>,
    play_btn: Query<&Interaction, (With<PlayButton>, Changed<Interaction>) |  |  |  |  |

| transport_sync | function | transport_sync(
    plan: Res<Plan>,
    mut fill: Query<&mut Node, (With<ScrubFill>, Without<Playhead>) |  |  |  |  |

| transport_visibility | function | transport_visibility(plan: Res<Plan>, mut bar: Query<&mut Visibility, With<TransportBar>>) |  |  |  |  |

| PlayButton | struct |  |  |  |  |  |

| PlayIcon | struct |  |  |  |  |  |

| Playhead | struct |  |  |  |  |  |

| ScrubFill | struct |  |  |  |  |  |

| ScrubTrack | struct |  |  |  |  |  |

| StepNotch | struct |  |  |  |  |  |

| Transport | struct |  |  |  |  |  |

| TransportBar | struct |  |  |  |  |  |

| TransportLabel | struct |  |  |  |  |  |


### crates/ferroplan-bevy/src/ui.rs

| setup_ui | function | setup_ui(mut commands: Commands) |  |  |  |  |

| update_info | function | update_info(
    scene: Res<Scene>,
    selected: Res<Selected>,
    plan: Res<Plan>,
    mut q: Query<&mut Text, With<InfoText>>,
) |  |  |  |  |

| InfoText | struct |  |  |  |  |  |


### crates/ferroplan-cli/src/generated/options.rs

| ModeArg | enum |  |  |  |  |  |

| ObjectiveArg | enum |  |  |  |  |  |

| SearchArg | enum |  |  |  |  |  |


### crates/ferroplan-cli/src/harvest/compile.rs

| compile_pack | function | compile_pack(pack: &ObservationPack, output_dir: &Path) |  |  |  |  |

| replay_pack | function | replay_pack(
    pack: &ObservationPack,
    expected: &HarvestReceipt,
    output_dir: &Path,
) |  |  |  |  |


### crates/ferroplan-cli/src/harvest/extract.rs

| extract_operators | function | extract_operators(report: &AdmissionReport) |  |  |  |  |


### crates/ferroplan-cli/src/harvest/gh.rs

| collect_with_gh | function | collect_with_gh(
    repositories: &[String],
    window: ObservationWindow,
    max_pages: usize,
) |  |  |  |  |


### crates/ferroplan-cli/src/harvest/mod.rs

| admit | function | admit(pack: &ObservationPack) |  |  |  |  |

| digest_bytes | function | digest_bytes(bytes: &[u8]) |  |  |  |  |

| load_observation_pack | function | load_observation_pack(path: &Path) |  |  |  |  |

| load_receipt | function | load_receipt(path: &Path) |  |  |  |  |

| receipt_exit_code | function | receipt_exit_code(receipt: &HarvestReceipt) |  |  |  |  |

| save_observation_pack | function | save_observation_pack(path: &Path, pack: &ObservationPack) |  |  |  |  |

| validate_pack | function | validate_pack(pack: &ObservationPack) |  |  |  |  |

| validate_window | function | validate_window(window: &ObservationWindow) |  |  |  |  |


### crates/ferroplan-cli/src/harvest/model.rs

| ActuationClass | enum |  |  |  |  |  |

| AdmissionLevel | enum |  |  |  |  |  |

| ExecutionResult | enum |  |  |  |  |  |

| FinalState | enum |  |  |  |  |  |

| GallCheckpoint | enum |  |  |  |  |  |

| RefusalCode | enum |  |  |  |  |  |

| ReplayState | enum |  |  |  |  |  |

| AdmissionReport | struct |  |  |  |  |  |

| AdmittedWork | struct |  |  |  |  |  |

| ArtifactEvidence | struct |  |  |  |  |  |

| EvidenceRef | struct |  |  |  |  |  |

| ExcludedWork | struct |  |  |  |  |  |

| ExecutionEvidence | struct |  |  |  |  |  |

| HarvestReceipt | struct |  |  |  |  |  |

| MethodCatalog | struct |  |  |  |  |  |

| ObservationPack | struct |  |  |  |  |  |

| ObservationWindow | struct |  |  |  |  |  |

| ObservedOutcome | struct |  |  |  |  |  |

| ObservedWorkItem | struct |  |  |  |  |  |

| OperatorOutcome | struct |  |  |  |  |  |

| OutputArtifact | struct |  |  |  |  |  |

| PlanningOperator | struct |  |  |  |  |  |

| SourceRevision | struct |  |  |  |  |  |

| TransportFailure | struct |  |  |  |  |  |

| ValidationRecord | struct |  |  |  |  |  |

| ValidationSummary | struct |  |  |  |  |  |


### crates/ferroplan-hddl/examples/validate_files.rs

| validate_pair | function | validate_pair(domain_src: &str, problem_src: &str) |  |  |  |  |

| validate_paths | function | validate_paths(domain_path: &str, problem_path: &str) |  |  |  |  |

| Summary | struct |  |  |  |  |  |


### crates/ferroplan-hddl/src/ast.rs

| Effect | enum |  |  |  |  |  |

| GoalDesc | enum |  |  |  |  |  |

| Literal | enum |  |  |  |  |  |

| NumericValue | enum |  |  |  |  |  |

| Term | enum |  |  |  |  |  |

| ActionDef | struct |  |  |  |  |  |

| AtomicFormula | struct |  |  |  |  |  |

| ConstraintDef | struct |  |  |  |  |  |

| Domain | struct |  |  |  |  |  |

| MethodDef | struct |  |  |  |  |  |

| NumericFluentDecl | struct |  |  |  |  |  |

| OrderEdge | struct |  |  |  |  |  |

| PredicateDef | struct |  |  |  |  |  |

| Problem | struct |  |  |  |  |  |

| Subtask | struct |  |  |  |  |  |

| TaskCall | struct |  |  |  |  |  |

| TaskDef | struct |  |  |  |  |  |

| TaskNetwork | struct |  |  |  |  |  |

| TypeDef | struct |  |  |  |  |  |

| TypedObject | struct |  |  |  |  |  |

| TypedParam | struct |  |  |  |  |  |


### crates/ferroplan-hddl/src/grounder.rs

| GroundError | enum |  |  |  |  |  |

| GroundGoal | enum |  |  |  |  |  |

| action_applicable | function | action_applicable(
    action: &GroundAction,
    facts: &BTreeSet<String>,
) |  |  |  |  |

| build_type_closure | function | build_type_closure(
    domain: &Domain,
) |  |  |  |  |

| compute_reachability | function | compute_reachability(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    initial_facts: &BTreeSet<String>,
    limits: &GroundingLimits,
) |  |  |  |  |

| compute_task_relevance | function | compute_task_relevance(
    domain: &Domain,
    problem: &Problem,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
) |  |  |  |  |

| evaluate_ground_goal | function | evaluate_ground_goal(
    goal: &GroundGoal,
    facts: &BTreeSet<String>,
) |  |  |  |  |

| evaluate_ground_goal_with_budget | function | evaluate_ground_goal_with_budget(
    goal: &GroundGoal,
    facts: &BTreeSet<String>,
    budget: usize,
) |  |  |  |  |

| ground | function | ground(
    domain: &Domain,
    problem: &Problem,
    limits: &GroundingLimits,
) |  |  |  |  |

| ground_actions | function | ground_actions(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
) |  |  |  |  |

| ground_actions_reachable | function | ground_actions_reachable(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    reachability: &ReachabilityInfo,
) |  |  |  |  |

| ground_actions_relevant | function | ground_actions_relevant(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    relevance: &TaskRelevanceInfo,
    reachability: Option<&ReachabilityInfo>,
) |  |  |  |  |

| ground_initial_facts | function | ground_initial_facts(problem: &Problem) |  |  |  |  |

| ground_methods | function | ground_methods(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
) |  |  |  |  |

| ground_methods_reachable | function | ground_methods_reachable(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    reachability: &ReachabilityInfo,
) |  |  |  |  |

| ground_methods_relevant | function | ground_methods_relevant(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    relevance: &TaskRelevanceInfo,
    reachability: Option<&ReachabilityInfo>,
) |  |  |  |  |

| ground_root_network | function | ground_root_network(
    problem: &Problem,
    objects_by_type: &BTreeMap<String, Vec<String>>,
) |  |  |  |  |

| index_objects_by_type | function | index_objects_by_type(
    domain: &Domain,
    problem: &Problem,
    closure: &BTreeMap<String, BTreeSet<String>>,
) |  |  |  |  |

| GroundAction | struct |  |  |  |  |  |

| GroundConditional | struct |  |  |  |  |  |

| GroundEffectBranch | struct |  |  |  |  |  |

| GroundMethod | struct |  |  |  |  |  |

| GroundRootNetwork | struct |  |  |  |  |  |

| GroundSubtask | struct |  |  |  |  |  |

| GroundedIR | struct |  |  |  |  |  |

| GroundingLimits | struct |  |  |  |  |  |

| ReachabilityInfo | struct |  |  |  |  |  |

| TaskRelevanceInfo | struct |  |  |  |  |  |


### crates/ferroplan-hddl/src/parser.rs

| ParseError | enum |  |  |  |  |  |

| parse_domain | function | parse_domain(src: &str) |  |  |  |  |

| parse_domain_with_budget | function | parse_domain_with_budget(src: &str, max_depth: usize) |  |  |  |  |

| parse_problem | function | parse_problem(src: &str) |  |  |  |  |

| parse_problem_with_budget | function | parse_problem_with_budget(src: &str, max_depth: usize) |  |  |  |  |


### crates/ferroplan-hddl/src/probabilistic.rs

| has_probabilistic | function | has_probabilistic(src: &str) |  |  |  |  |

| preprocess | function | preprocess(src: &str) |  |  |  |  |


### crates/ferroplan-hddl/src/translate.rs

| TranslateError | enum |  |  |  |  |  |

| translate | function | translate(
    ir: &GroundedIR,
    limits: &TranslateLimits,
) |  |  |  |  |

| Goal | struct |  |  |  |  |  |

| Method | struct |  |  |  |  |  |

| PlanningProblem | struct |  |  |  |  |  |

| State | struct |  |  |  |  |  |

| Task | struct |  |  |  |  |  |

| Transition | struct |  |  |  |  |  |

| TranslateLimits | struct |  |  |  |  |  |


### crates/ferroplan-hddl/src/validate.rs

| DuplicateKind | enum |  |  |  |  |  |

| ValidationError | enum |  |  |  |  |  |

| ValidationWarning | enum |  |  |  |  |  |

| validate_domain | function | validate_domain(domain: &Domain) |  |  |  |  |

| validate_domain_with_warnings | function | validate_domain_with_warnings(
    domain: &Domain,
) |  |  |  |  |

| validate_problem | function | validate_problem(domain: &Domain, problem: &Problem) |  |  |  |  |

| validate_problem_with_warnings | function | validate_problem_with_warnings(
    domain: &Domain,
    problem: &Problem,
) |  |  |  |  |


### crates/ferroplan-mcp/tests/common/mod.rs

| call | function | call(&mut self, tool: &str, args: Value) |  |  |  |  |

| call_json | function | call_json(&mut self, tool: &str, args: Value) |  |  |  |  |

| call_text | function | call_text(&mut self, tool: &str, args: Value) |  |  |  |  |

| finish | function | finish(mut self) |  |  |  |  |

| notify | function | notify(&mut self, method: &str) |  |  |  |  |

| request | function | request(&mut self, method: &str, params: Value) |  |  |  |  |

| start | function | start() |  |  |  |  |

| Client | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/authority.rs

| Authority | enum |  |  |  |  |  |

| permits | function | permits(g: Authority, n: Authority) |  |  |  |  |


### crates/ferroplan-runtime/src/backoff.rs

| exponential | function | exponential(base: u64, attempt: u32, cap: u64) |  |  |  |  |


### crates/ferroplan-runtime/src/budget.rs

| new | function | new(n: u32) |  |  |  |  |

| remaining | function | remaining(&self) |  |  |  |  |

| take | function | take(&mut self) |  |  |  |  |

| Budget | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/capability.rs

| compatible | function | compatible(available: &[Capability], required: &str) |  |  |  |  |

| Capability | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/circuit.rs

| open | function | open(&self) |  |  |  |  |

| record_failure | function | record_failure(&mut self) |  |  |  |  |

| Circuit | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/coordinator.rs

| fail | function | fail(&mut self, id: &str) |  |  |  |  |

| Coordinator | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/deadline.rs

| after | function | after(d: Duration) |  |  |  |  |

| expired | function | expired(&self) |  |  |  |  |

| Deadline | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/edge.rs

| exclude | function | exclude(&mut self) |  |  |  |  |

| Edge | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/epoch.rs

| next | function | next(self) |  |  |  |  |

| Epoch | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/evidence.rs

| admitted | function | admitted(xs: &[Observation]) |  |  |  |  |


### crates/ferroplan-runtime/src/exact_subject.rs

| admitted | function | admitted(&self) |  |  |  |  |

| ExactSubject | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/failure.rs

| FailureClass | enum |  |  |  |  |  |


### crates/ferroplan-runtime/src/graph.rs

| exclude | function | exclude(&mut self, id: &str) |  |  |  |  |

| lawful | function | lawful(&self) |  |  |  |  |

| Graph | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/hddl.rs

| Task | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/health.rs

| Health | enum |  |  |  |  |  |

| eligible | function | eligible(h: Health) |  |  |  |  |


### crates/ferroplan-runtime/src/idempotency.rs

| key | function | key(subject: &str, epoch: u64, provider: &str) |  |  |  |  |


### crates/ferroplan-runtime/src/lease.rs

| valid_for | function | valid_for(&self, o: &str, e: u64) |  |  |  |  |

| Lease | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/observation.rs

| Observation | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/ocel.rs

| bound | function | bound(&self) |  |  |  |  |

| Event | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/outcome.rs

| Outcome | enum |  |  |  |  |  |

| is_success | function | is_success(&self) |  |  |  |  |


### crates/ferroplan-runtime/src/plan.rs

| valid | function | valid(&self) |  |  |  |  |

| Plan | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/poly_evidence.rs

| exact | function | exact(&self) |  |  |  |  |

| EvidenceSet | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/portfolio.rs

| ranked | function | ranked(mut ps: Vec<Plan>) |  |  |  |  |


### crates/ferroplan-runtime/src/powl.rs

| acyclic | function | acyclic(e: &[Order]) |  |  |  |  |

| Order | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/provider.rs

| Provider | trait |  |  |  |  |  |


### crates/ferroplan-runtime/src/ptd.rs

| changed | function | changed(a: &EpochArtifact, b: &EpochArtifact) |  |  |  |  |

| EpochArtifact | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/receipt.rs

| exact | function | exact(&self) |  |  |  |  |

| Receipt | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/reconcile.rs

| reconcile | function | reconcile(g: &mut Graph, s: &[(String, Health) |  |  |  |  |


### crates/ferroplan-runtime/src/recovery.rs

| exclude_failed | function | exclude_failed(g: &mut Graph, edge: &str) |  |  |  |  |


### crates/ferroplan-runtime/src/registry.rs

| contains | function | contains(&self, id: &str) |  |  |  |  |

| register | function | register(&mut self, id: impl Into<String>) |  |  |  |  |

| Registry | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/replay.rs

| same_decision | function | same_decision(a: &Receipt, b: &Receipt) |  |  |  |  |


### crates/ferroplan-runtime/src/runtime.rs

| next_edge | function | next_edge(&self) |  |  |  |  |


### crates/ferroplan-runtime/src/scheduler.rs

| is_empty | function | is_empty(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| pop_next | function | pop_next(&mut self) |  |  |  |  |

| push | function | push(&mut self, x: T) |  |  |  |  |

| Scheduler | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/state.rs

| State | enum |  |  |  |  |  |

| allowed | function | allowed(a: State, b: State) |  |  |  |  |


### crates/ferroplan-runtime/src/supervision.rs

| Restart | enum |  |  |  |  |  |

| should_restart | function | should_restart(r: Restart, abnormal: bool) |  |  |  |  |


### crates/ferroplan-runtime/src/telemetry.rs

| success_rate | function | success_rate(&self) |  |  |  |  |

| Counters | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/tla_evidence.rs

| actionable | function | actionable(&self) |  |  |  |  |

| Counterexample | struct |  |  |  |  |  |


### crates/ferroplan-runtime/src/trimtab.rs

| ModelRole | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/analyze_conflict.rs

| add | function | add(&mut self, level: usize) |  |  |  |  |

| analyze_conflict | function | analyze_conflict(ctx: &mut Context, conflict: Conflict) |  |  |  |  |

| clause | function | clause(&self) |  |  |  |  |

| involved | function | involved(&self) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| test | function | test(&self, level: usize) |  |  |  |  |

| AnalyzeConflict | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/assumptions.rs

| EnqueueAssumption | enum |  |  |  |  |  |

| assumption_levels | function | assumption_levels(&self) |  |  |  |  |

| enqueue_assumption | function | enqueue_assumption(ctx: &mut Context) |  |  |  |  |

| full_restart | function | full_restart(&mut self) |  |  |  |  |

| set_assumptions | function | set_assumptions(ctx: &mut Context, user_assumptions: &[Lit]) |  |  |  |  |

| user_failed_core | function | user_failed_core(&self) |  |  |  |  |

| Assumptions | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/binary.rs

| add_binary_clause | function | add_binary_clause(&mut self, lits: [Lit; 2]) |  |  |  |  |

| count | function | count(&self) |  |  |  |  |

| implied | function | implied(&self, lit: Lit) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| simplify_binary | function | simplify_binary(ctx: &mut Context) |  |  |  |  |

| BinaryClauses | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/cdcl.rs

| conflict_step | function | conflict_step(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/clause.rs

| header | function | header(&self) |  |  |  |  |

| header_mut | function | header_mut(&mut self) |  |  |  |  |

| lits | function | lits(&self) |  |  |  |  |

| lits_mut | function | lits_mut(&mut self) |  |  |  |  |

| Clause | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/clause/activity.rs

| bump_clause_activity | function | bump_clause_activity(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| decay_clause_activities | function | decay_clause_activities(ctx: &mut Context) |  |  |  |  |

| ClauseActivity | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/clause/alloc.rs

| add_clause | function | add_clause(&mut self, mut header: ClauseHeader, lits: &[Lit]) |  |  |  |  |

| buffer_size | function | buffer_size(&self) |  |  |  |  |

| check_bounds | function | check_bounds(&self, cref: ClauseRef, len: usize) |  |  |  |  |

| clause | function | clause(&self, cref: ClauseRef) |  |  |  |  |

| clause_mut | function | clause_mut(&mut self, cref: ClauseRef) |  |  |  |  |

| header | function | header(&self, cref: ClauseRef) |  |  |  |  |

| header_mut | function | header_mut(&mut self, cref: ClauseRef) |  |  |  |  |

| header_unchecked_mut | function | header_unchecked_mut(&mut self, cref: ClauseRef) |  |  |  |  |

| lits_ptr_mut_unchecked | function | lits_ptr_mut_unchecked(&mut self, cref: ClauseRef) |  |  |  |  |

| with_capacity | function | with_capacity(capacity: usize) |  |  |  |  |

| ClauseAlloc | struct |  |  |  |  |  |

| ClauseRef | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/clause/assess.rs

| assess_learned_clause | function | assess_learned_clause(ctx: &mut Context, lits: &[Lit]) |  |  |  |  |

| bump_clause | function | bump_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |


### crates/ferroplan-sat/src/clause/db.rs

| Tier | enum |  |  |  |  |  |

| add_clause | function | add_clause(ctx: &mut Context, header: ClauseHeader, lits: &[Lit]) |  |  |  |  |

| count | function | count() |  |  |  |  |

| count_by_tier | function | count_by_tier(&self, tier: Tier) |  |  |  |  |

| delete_clause | function | delete_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| from_index | function | from_index(index: usize) |  |  |  |  |

| set_clause_tier | function | set_clause_tier(ctx: &mut Context, cref: ClauseRef, tier: Tier) |  |  |  |  |

| try_delete_clause | function | try_delete_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| ClauseDb | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/clause/gc.rs

| collect_garbage | function | collect_garbage(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/clause/header.rs

| active | function | active(&self) |  |  |  |  |

| activity | function | activity(&self) |  |  |  |  |

| deleted | function | deleted(&self) |  |  |  |  |

| glue | function | glue(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| mark | function | mark(&self) |  |  |  |  |

| new | function | new() |  |  |  |  |

| set_active | function | set_active(&mut self, active: bool) |  |  |  |  |

| set_activity | function | set_activity(&mut self, activity: f32) |  |  |  |  |

| set_deleted | function | set_deleted(&mut self, deleted: bool) |  |  |  |  |

| set_glue | function | set_glue(&mut self, glue: usize) |  |  |  |  |

| set_len | function | set_len(&mut self, length: usize) |  |  |  |  |

| set_mark | function | set_mark(&mut self, mark: bool) |  |  |  |  |

| set_tier | function | set_tier(&mut self, tier: Tier) |  |  |  |  |

| tier | function | tier(&self) |  |  |  |  |

| ClauseHeader | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/clause/reduce.rs

| dedup_and_mark_by_tier | function | dedup_and_mark_by_tier(ctx: &mut Context, tier: Tier) |  |  |  |  |

| reduce_locals | function | reduce_locals(ctx: &mut Context) |  |  |  |  |

| reduce_mids | function | reduce_mids(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/cnf.rs

| is_empty | function | is_empty(&self) |  |  |  |  |

| iter | function | iter(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| new | function | new() |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| var_count | function | var_count(&self) |  |  |  |  |

| CnfFormula | struct |  |  |  |  |  |

| ExtendFormula | trait |  |  |  |  |  |


### crates/ferroplan-sat/src/config.rs

| SolverConfig | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/context.rs

| set_var_count | function | set_var_count(ctx: &mut Context, count: usize) |  |  |  |  |

| Context | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/decision.rs

| make_decision | function | make_decision(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/decision/vsids.rs

| bump | function | bump(&mut self, var: Var) |  |  |  |  |

| decay | function | decay(&mut self) |  |  |  |  |

| make_available | function | make_available(&mut self, var: Var) |  |  |  |  |

| make_unavailable | function | make_unavailable(&mut self, var: Var) |  |  |  |  |

| reset | function | reset(&mut self, var: Var) |  |  |  |  |

| seed | function | seed(&mut self, var: Var, activity: f32) |  |  |  |  |

| set_decay | function | set_decay(&mut self, decay: f32) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| var_count | function | var_count(&self) |  |  |  |  |

| Vsids | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/dimacs.rs

| DimacsError | enum |  |  |  |  |  |

| parse_dimacs | function | parse_dimacs(input: impl io::BufRead) |  |  |  |  |

| parse_dimacs_str | function | parse_dimacs_str(input: &str) |  |  |  |  |

| write_dimacs | function | write_dimacs(target: &mut impl io::Write, formula: &CnfFormula) |  |  |  |  |


### crates/ferroplan-sat/src/glue.rs

| compute_glue | function | compute_glue(tmp_flags: &mut TmpFlags, impl_graph: &ImplGraph, lits: &[Lit]) |  |  |  |  |


### crates/ferroplan-sat/src/lit.rs

| code | function | code(self) |  |  |  |  |

| from_code | function | from_code(code: usize) |  |  |  |  |

| from_dimacs | function | from_dimacs(number: isize) |  |  |  |  |

| from_index | function | from_index(index: usize) |  |  |  |  |

| from_var | function | from_var(var: Var, polarity: bool) |  |  |  |  |

| index | function | index(self) |  |  |  |  |

| is_negative | function | is_negative(self) |  |  |  |  |

| is_positive | function | is_positive(self) |  |  |  |  |

| lit | function | lit(self, polarity: bool) |  |  |  |  |

| map_var | function | map_var(self, f: impl FnOnce(Var) |  |  |  |  |

| max_count | function | max_count() |  |  |  |  |

| max_var | function | max_var() |  |  |  |  |

| negative | function | negative(self) |  |  |  |  |

| positive | function | positive(self) |  |  |  |  |

| to_dimacs | function | to_dimacs(self) |  |  |  |  |

| var | function | var(self) |  |  |  |  |

| Lit | struct |  |  |  |  |  |

| Var | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/load.rs

| load_clause | function | load_clause(ctx: &mut Context, user_lits: &[Lit]) |  |  |  |  |


### crates/ferroplan-sat/src/model.rs

| assignment | function | assignment(&self) |  |  |  |  |

| reconstruct_global_model | function | reconstruct_global_model(ctx: &mut Context) |  |  |  |  |

| Model | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/prop.rs

| propagate | function | propagate(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/prop/assignment.rs

| assign_lit | function | assign_lit(&mut self, lit: Lit) |  |  |  |  |

| assignment | function | assignment(&self) |  |  |  |  |

| backtrack | function | backtrack(ctx: &mut Context, level: usize) |  |  |  |  |

| clear | function | clear(&mut self) |  |  |  |  |

| current_level | function | current_level(&self) |  |  |  |  |

| enqueue_assignment | function | enqueue_assignment(
    assignment: &mut Assignment,
    impl_graph: &mut ImplGraph,
    trail: &mut Trail,
    lit: Lit,
    reason: Reason,
) |  |  |  |  |

| fast_option_eq | function | fast_option_eq(a: Option<bool>, b: Option<bool>) |  |  |  |  |

| full_restart | function | full_restart(ctx: &mut Context) |  |  |  |  |

| last_var_value | function | last_var_value(&self, var: Var) |  |  |  |  |

| lit_is_false | function | lit_is_false(&self, lit: Lit) |  |  |  |  |

| lit_is_true | function | lit_is_true(&self, lit: Lit) |  |  |  |  |

| lit_is_unk | function | lit_is_unk(&self, lit: Lit) |  |  |  |  |

| lit_value | function | lit_value(&self, lit: Lit) |  |  |  |  |

| new_decision_level | function | new_decision_level(&mut self) |  |  |  |  |

| pop_queue | function | pop_queue(&mut self) |  |  |  |  |

| queue_head | function | queue_head(&self) |  |  |  |  |

| restart | function | restart(ctx: &mut Context) |  |  |  |  |

| set_var | function | set_var(&mut self, var: Var, assignment: Option<bool>) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| trail | function | trail(&self) |  |  |  |  |

| unassign_var | function | unassign_var(&mut self, var: Var) |  |  |  |  |

| var_value | function | var_value(&self, var: Var) |  |  |  |  |

| Assignment | struct |  |  |  |  |  |

| Trail | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/prop/binary.rs

| propagate_binary | function | propagate_binary(ctx: &mut Context, lit: Lit) |  |  |  |  |


### crates/ferroplan-sat/src/prop/graph.rs

| Conflict | enum |  |  |  |  |  |

| Reason | enum |  |  |  |  |  |

| is_removed_unit | function | is_removed_unit(&self, var: Var) |  |  |  |  |

| is_unit | function | is_unit(&self) |  |  |  |  |

| level | function | level(&self, var: Var) |  |  |  |  |

| reason | function | reason(&self, var: Var) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| update_reason | function | update_reason(&mut self, var: Var, reason: Reason) |  |  |  |  |

| update_removed_unit | function | update_removed_unit(&mut self, var: Var) |  |  |  |  |

| ImplGraph | struct |  |  |  |  |  |

| ImplNode | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/prop/long.rs

| propagate_long | function | propagate_long(ctx: &mut Context, lit: Lit) |  |  |  |  |


### crates/ferroplan-sat/src/prop/watch.rs

| add_watch | function | add_watch(&mut self, lit: Lit, watch: Watch) |  |  |  |  |

| disable | function | disable(&mut self) |  |  |  |  |

| enable_watchlists | function | enable_watchlists(ctx: &mut Context) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| watch_clause | function | watch_clause(&mut self, cref: ClauseRef, lits: [Lit; 2]) |  |  |  |  |

| watched_by_mut | function | watched_by_mut(&mut self, lit: Lit) |  |  |  |  |

| Watch | struct |  |  |  |  |  |

| Watchlists | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/schedule.rs

| schedule_step | function | schedule_step(ctx: &mut Context) |  |  |  |  |

| Schedule | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/schedule/luby.rs

| advance | function | advance(&mut self) |  |  |  |  |

| LubySequence | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/solver.rs

| SolverError | enum |  |  |  |  |  |

| add_formula | function | add_formula(&mut self, formula: &CnfFormula) |  |  |  |  |

| assume | function | assume(&mut self, assumptions: &[Lit]) |  |  |  |  |

| failed_core | function | failed_core(&self) |  |  |  |  |

| is_recoverable | function | is_recoverable(&self) |  |  |  |  |

| model | function | model(&self) |  |  |  |  |

| new | function | new() |  |  |  |  |

| seed_activity | function | seed_activity(&mut self, seeds: &[(usize, f32) |  |  |  |  |

| set_conflict_limit | function | set_conflict_limit(&mut self, limit: Option<u64>) |  |  |  |  |

| solve | function | solve(&mut self) |  |  |  |  |

| Solver | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/state.rs

| SatState | enum |  |  |  |  |  |

| SolverState | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/tmp.rs

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| TmpData | struct |  |  |  |  |  |

| TmpFlags | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/unit_simplify.rs

| prove_units | function | prove_units(ctx: &mut Context) |  |  |  |  |

| resurrect_unit | function | resurrect_unit(ctx: &mut Context, lit: Lit) |  |  |  |  |

| unit_simplify | function | unit_simplify(ctx: &mut Context) |  |  |  |  |


### crates/ferroplan-sat/src/variables.rs

| existing_user_from_solver | function | existing_user_from_solver(&self, solver: Var) |  |  |  |  |

| global_from_solver | function | global_from_solver(&self) |  |  |  |  |

| global_from_solver_mut | function | global_from_solver_mut(&mut self) |  |  |  |  |

| global_from_user | function | global_from_user(&self) |  |  |  |  |

| global_from_user_mut | function | global_from_user_mut(&mut self) |  |  |  |  |

| global_var_iter | function | global_var_iter(&self) |  |  |  |  |

| global_watermark | function | global_watermark(&self) |  |  |  |  |

| initialize_solver_var | function | initialize_solver_var(ctx: &mut Context, solver: Var, global: Var) |  |  |  |  |

| new_user_var | function | new_user_var(ctx: &mut Context) |  |  |  |  |

| next_unmapped_solver | function | next_unmapped_solver(&self) |  |  |  |  |

| next_unmapped_user | function | next_unmapped_user(&self) |  |  |  |  |

| remove_solver_var | function | remove_solver_var(ctx: &mut Context, solver: Var) |  |  |  |  |

| solver_from_global | function | solver_from_global(&self) |  |  |  |  |

| solver_from_global_mut | function | solver_from_global_mut(&mut self) |  |  |  |  |

| solver_from_user | function | solver_from_user(ctx: &mut Context, user: Var) |  |  |  |  |

| solver_from_user_lits | function | solver_from_user_lits(ctx: &mut Context, solver_lits: &mut Vec<Lit>, user_lits: &[Lit]) |  |  |  |  |

| solver_var_present | function | solver_var_present(&self, solver: Var) |  |  |  |  |

| solver_watermark | function | solver_watermark(&self) |  |  |  |  |

| user_from_global | function | user_from_global(&self) |  |  |  |  |

| user_var_iter | function | user_var_iter(&self) |  |  |  |  |

| user_watermark | function | user_watermark(&self) |  |  |  |  |

| var_data_global | function | var_data_global(&self, global: Var) |  |  |  |  |

| var_data_global_mut | function | var_data_global_mut(&mut self, global: Var) |  |  |  |  |

| var_data_solver_mut | function | var_data_solver_mut(&mut self, solver: Var) |  |  |  |  |

| Variables | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/variables/data.rs

| user_default | function | user_default() |  |  |  |  |

| VarData | struct |  |  |  |  |  |


### crates/ferroplan-sat/src/variables/var_map.rs

| bwd | function | bwd(&self) |  |  |  |  |

| bwd_mut | function | bwd_mut(&mut self) |  |  |  |  |

| fwd | function | fwd(&self) |  |  |  |  |

| fwd_mut | function | fwd_mut(&mut self) |  |  |  |  |

| get | function | get(&self, from: Var) |  |  |  |  |

| insert | function | insert(&mut self, into: Var, from: Var) |  |  |  |  |

| remove | function | remove(&mut self, from: Var) |  |  |  |  |

| watermark | function | watermark(&self) |  |  |  |  |

| VarBiMap | struct |  |  |  |  |  |

| VarBiMapMut | struct |  |  |  |  |  |

| VarMap | struct |  |  |  |  |  |


### crates/ferroplan-wasm/src/dfcm_route.rs

| admit_candidates | function | admit_candidates(
    candidates: &[ProbeCandidate],
    goal_limit: usize,
    surface: &str,
) |  |  |  |  |

| corridor_problem | function | corridor_problem(n: usize) |  |  |  |  |

| probe_all | function | probe_all(
    parent: &Session,
    candidates: &[ProbeCandidate],
    evals: usize,
    mem_mb: usize,
) |  |  |  |  |

| probe_candidate | function | probe_candidate(
    parent: &Session,
    candidate: &ProbeCandidate,
    evals: usize,
    mem_mb: usize,
) |  |  |  |  |

| repair | function | repair(
    inner: &Session,
    plan: &mut Option<Plan>,
    cursor: &mut usize,
    evals: usize,
    mem_mb: usize,
) |  |  |  |  |

| ProbeCandidate | struct |  |  |  |  |  |


### crates/ferroplan-wasm/src/lib.rs

| advance | function | advance(&mut self) |  |  |  |  |

| apply_start | function | apply_start(&mut self, name: &str) |  |  |  |  |

| drop_plan | function | drop_plan(&mut self) |  |  |  |  |

| elapse | function | elapse(&mut self, dt: f64) |  |  |  |  |

| explain | function | explain(domain: &str, problem: &str, plan_json: &str) |  |  |  |  |

| fact | function | fact(&self, name: &str) |  |  |  |  |

| fluent | function | fluent(&self, name: &str) |  |  |  |  |

| fond_validate | function | fond_validate(problem_json: &str, plan_json: &str) |  |  |  |  |

| fork | function | fork(&self) |  |  |  |  |

| goal_met | function | goal_met(&self) |  |  |  |  |

| has_plan | function | has_plan(&self) |  |  |  |  |

| mind_bytes | function | mind_bytes(&self) |  |  |  |  |

| new | function | new(domain: &str, problem: &str) |  |  |  |  |

| observe | function | observe(&mut self, sight_json: &str) |  |  |  |  |

| plan | function | plan(
        domain: &str,
        problem: &str,
        mode: Option<String>,
        flags: Option<String>,
        search: Option<String>,
    ) |  |  |  |  |

| plan_production | function | plan_production(
        domain: &str,
        problem: &str,
        mode: Option<String>,
        search: Option<String>,
        max_evaluated: Option<usize>,
        max_plan_steps: Option<usize>,
        max_output_bytes: Option<usize>,
        request_id: Option<String>,
    ) |  |  |  |  |

| plan_valid_json | function | plan_valid_json(&self, plan_json: &str, from: usize) |  |  |  |  |

| probe_json | function | probe_json(&self, candidates_json: &str, evals: usize, mem_mb: usize) |  |  |  |  |

| readiness | function | readiness() |  |  |  |  |

| repair | function | repair(&mut self, evals: usize, mem_mb: usize) |  |  |  |  |

| replan_following | function | replan_following(&mut self, evals: usize, mem_mb: usize) |  |  |  |  |

| restrict_contains | function | restrict_contains(&mut self, filter: String) |  |  |  |  |

| restrict_prefix_claims | function | restrict_prefix_claims(&mut self, prefix: String, claimed: String) |  |  |  |  |

| set_fact | function | set_fact(&mut self, name: &str, value: bool) |  |  |  |  |

| set_fluent | function | set_fluent(&mut self, name: &str, value: f64) |  |  |  |  |

| set_goal | function | set_goal(&mut self, goal: &str) |  |  |  |  |

| set_timed_fact | function | set_timed_fact(&mut self, dt: f64, name: &str, value: bool) |  |  |  |  |

| step_json | function | step_json(&self) |  |  |  |  |

| suffix_json | function | suffix_json(&self) |  |  |  |  |

| think | function | think(&mut self, evals: usize, mem_mb: usize) |  |  |  |  |

| valid | function | valid(&self) |  |  |  |  |

| version | function | version() |  |  |  |  |

| world_bytes | function | world_bytes(&self) |  |  |  |  |

| WasmSession | struct |  |  |  |  |  |


### crates/ferroplan-wasm/src/probe_guard.rs

| contradictory_fact | function | contradictory_fact(sight: &[(String, bool) |  |  |  |  |

| fact_key | function | fact_key(fact: &str) |  |  |  |  |


### crates/ferroplan/src/api.rs

| Mode | enum |  |  |  |  |  |

| Search | enum |  |  |  |  |  |

| SolveError | enum |  |  |  |  |  |

| decompose | function | decompose(
    domain_src: &str,
    problem_src: &str,
    opts: &Options,
) |  |  |  |  |

| parse | function | parse(src: &str) |  |  |  |  |

| solve | function | solve(domain_src: &str, problem_src: &str, opts: &Options) |  |  |  |  |

| Contract | struct |  |  |  |  |  |

| Decomposition | struct |  |  |  |  |  |

| DomainSummary | struct |  |  |  |  |  |

| Options | struct |  |  |  |  |  |

| ParseReport | struct |  |  |  |  |  |

| Plan | struct |  |  |  |  |  |

| ProblemSummary | struct |  |  |  |  |  |

| Solution | struct |  |  |  |  |  |

| Statistics | struct |  |  |  |  |  |

| Step | struct |  |  |  |  |  |


### crates/ferroplan/src/bitset.rs

| clear | function | clear(w: &mut [u64], i: usize) |  |  |  |  |

| count | function | count(w: &[u64]) |  |  |  |  |

| set | function | set(w: &mut [u64], i: usize) |  |  |  |  |

| test | function | test(w: &[u64], i: usize) |  |  |  |  |

| words_for | function | words_for(n_bits: usize) |  |  |  |  |


### crates/ferroplan/src/clock.rs

| elapsed_ms | function | elapsed_ms(&self) |  |  |  |  |

| elapsed_secs | function | elapsed_secs(&self) |  |  |  |  |

| elapsed_us | function | elapsed_us(&self) |  |  |  |  |

| now | function | now() |  |  |  |  |

| Clock | struct |  |  |  |  |  |


### crates/ferroplan/src/constraints.rs

| Traj | enum |  |  |  |  |  |

| accepted | function | accepted(&self) |  |  |  |  |

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |

| expand | function | expand(domain: &Domain, problem: &Problem) |  |  |  |  |

| gate | function | gate(domain: &Domain, problem: &Problem) |  |  |  |  |

| hard_only_gated | function | hard_only_gated(
    domain: &Domain,
    problem: &Problem,
) |  |  |  |  |

| new | function | new(traj: &'a Traj) |  |  |  |  |

| op_name | function | op_name(&self) |  |  |  |  |

| step | function | step(&mut self, holds: &mut dyn FnMut(&Formula) |  |  |  |  |

| step_at | function | step_at(&mut self, time: f64, holds: &mut dyn FnMut(&Formula) |  |  |  |  |

| Expanded | struct |  |  |  |  |  |

| Fold | struct |  |  |  |  |  |


### crates/ferroplan/src/costs.rs

| improve | function | improve(
    task: &PackedTask,
    cf: usize,
    ops: Vec<usize>,
    first_cost: f64,
    threads: usize,
    base: SearchCfg,
    spent: usize,
    
    
    
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| improve_length | function | improve_length(
    task: &PackedTask,
    ops: Vec<usize>,
    threads: usize,
    base: SearchCfg,
    spent: usize,
) |  |  |  |  |

| metric_fluent | function | metric_fluent(problem: &Problem) |  |  |  |  |

| optimize_text | function | optimize_text(
    problem: &Problem,
    task: &PackedTask,
    optimize: bool,
    threads: usize,
    cfg: SearchCfg,
    ops: &mut Vec<usize>,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| plan_cost | function | plan_cost(task: &PackedTask, cf: usize, ops: &[usize]) |  |  |  |  |

| CostOutcome | struct |  |  |  |  |  |


### crates/ferroplan/src/derived.rs

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |


### crates/ferroplan/src/espc.rs

| espc_optimize | function | espc_optimize(
    task: &PackedTask,
    cost_fluent: usize,
    sat: &mut SatGuidance,
    seed: Option<(Vec<usize>, f64) |  |  |  |  |

| EspcPartition | struct |  |  |  |  |  |

| EspcResult | struct |  |  |  |  |  |


### crates/ferroplan/src/eve.rs

| EveError | enum |  |  |  |  |  |

| EveStage | enum |  |  |  |  |  |

| PlanningRegime | enum |  |  |  |  |  |

| enter | function | enter(request: EveRequest) |  |  |  |  |

| Activator | struct |  |  |  |  |  |

| CapabilityTarget | struct |  |  |  |  |  |

| Eve | struct |  |  |  |  |  |

| EveHandoff | struct |  |  |  |  |  |

| EveRequest | struct |  |  |  |  |  |

| GenesisProjection | struct |  |  |  |  |  |

| GenesisWorld | struct |  |  |  |  |  |

| GgenManufacturingRequest | struct |  |  |  |  |  |

| GroundedGoal | struct |  |  |  |  |  |

| HddlDecompositionRequest | struct |  |  |  |  |  |

| HddlSurface | struct |  |  |  |  |  |

| HumanPurpose | struct |  |  |  |  |  |

| ManufactureTarget | struct |  |  |  |  |  |

| McpPlusHandoff | struct |  |  |  |  |  |

| PpddlPolicyRequest | struct |  |  |  |  |  |

| PpddlSurface | struct |  |  |  |  |  |

| SplitDirective | struct |  |  |  |  |  |

| TruexContinuation | struct |  |  |  |  |  |


### crates/ferroplan/src/features.rs

| DemandMode | enum |  |  |  |  |  |

| clear_overrides | function | clear_overrides() |  |  |  |  |

| demand_mode | function | demand_mode() |  |  |  |  |

| escalate | function | escalate() |  |  |  |  |

| espc | function | espc() |  |  |  |  |

| set_escalate_override | function | set_escalate_override(on: bool) |  |  |  |  |

| set_espc_override | function | set_espc_override(on: bool) |  |  |  |  |

| set_overrides | function | set_overrides(tdemand: bool, tdecomp: bool, tconc: bool) |  |  |  |  |

| tconc | function | tconc() |  |  |  |  |

| tdecomp | function | tdecomp() |  |  |  |  |

| tdemand | function | tdemand() |  |  |  |  |


### crates/ferroplan/src/ground.rs

| Outcome | enum |  |  |  |  |  |

| ground | function | ground(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| ground_fixpoint | function | ground_fixpoint(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| ground_stratified | function | ground_stratified(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| ground_stratified_walled | function | ground_stratified_walled(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| ground_task | function | ground_task(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| initial_state | function | initial_state(t: &PackedTask) |  |  |  |  |

| objects_by_type | function | objects_by_type(domain: &Domain, problem: &Problem) |  |  |  |  |


### crates/ferroplan/src/hash.rs

| FxHasher | struct |  |  |  |  |  |


### crates/ferroplan/src/hddl.rs

| HddlError | enum |  |  |  |  |  |

| adapt_problem | function | adapt_problem(p: ferroplan_hddl::translate::PlanningProblem) |  |  |  |  |

| solve_hddl | function | solve_hddl(
    domain_src: &str,
    problem_src: &str,
    limits: &PlannerLimits,
) |  |  |  |  |

| solve_hddl_from_eve | function | solve_hddl_from_eve(
    handoff: &EveHandoff,
    limits: &PlannerLimits,
) |  |  |  |  |


### crates/ferroplan/src/heuristic.rs

| extraction_need_facts | function | extraction_need_facts(sc: &Scratch) |  |  |  |  |

| helpful_needed_adders | function | helpful_needed_adders(
    task: &PackedTask,
    sc: &Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
) |  |  |  |  |

| new | function | new(task: &PackedTask) |  |  |  |  |

| reachability_layers | function | reachability_layers(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
) |  |  |  |  |

| relaxed | function | relaxed(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
) |  |  |  |  |

| relaxed_costed | function | relaxed_costed(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
) |  |  |  |  |

| relaxed_helpful | function | relaxed_helpful(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
) |  |  |  |  |

| relaxed_plan_cost | function | relaxed_plan_cost(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
) |  |  |  |  |

| relaxed_to | function | relaxed_to(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
) |  |  |  |  |

| Scratch | struct |  |  |  |  |  |

| TrpgInfo | struct |  |  |  |  |  |

| TrpgWindow | struct |  |  |  |  |  |


### crates/ferroplan/src/introspect.rs

| explain | function | explain(domain_src: &str, problem_src: &str, plan: &Plan) |  |  |  |  |

| CausalLink | struct |  |  |  |  |  |

| Explanation | struct |  |  |  |  |  |

| InvariantSpan | struct |  |  |  |  |  |

| PrefReport | struct |  |  |  |  |  |


### crates/ferroplan/src/invariants.rs

| synthesize | function | synthesize(domain: &Domain, task: &PackedTask) |  |  |  |  |


### crates/ferroplan/src/lama.rs

| search | function | search(
    task: &PackedTask,
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |

| search_subgoal | function | search_subgoal(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[crate::types::NumPre],
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
    len_anytime: bool,
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |


### crates/ferroplan/src/landmarks.rs

| goal_landmarks | function | goal_landmarks(task: &PackedTask) |  |  |  |  |

| landmarks_for | function | landmarks_for(
    task: &PackedTask,
    start: &crate::packed::State,
    goal_pos: &[u32],
) |  |  |  |  |


### crates/ferroplan/src/lexer.rs

| Tok | enum |  |  |  |  |  |

| lex | function | lex(input: &str) |  |  |  |  |


### crates/ferroplan/src/mem.rs

| arm | function | arm() |  |  |  |  |

| armed | function | armed(&self) |  |  |  |  |

| declared_budget_bytes | function | declared_budget_bytes() |  |  |  |  |

| hit | function | hit(&self) |  |  |  |  |

| latch | function | latch() |  |  |  |  |

| latched | function | latched() |  |  |  |  |

| peak_resident_bytes | function | peak_resident_bytes() |  |  |  |  |

| resident_bytes | function | resident_bytes() |  |  |  |  |

| unarmed | function | unarmed() |  |  |  |  |

| MemWall | struct |  |  |  |  |  |


### crates/ferroplan/src/novelty.rs

| from_env | function | from_env() |  |  |  |  |

| r_partition_facts | function | r_partition_facts(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[crate::types::NumPre],
    r_cap: usize,
) |  |  |  |  |

| search | function | search(
    task: &PackedTask,
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |

| search_driver | function | search_driver(
    task: &PackedTask,
    max_eval: usize,
    forbidden: &[bool],
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |

| search_light | function | search_light(
    task: &PackedTask,
    max_eval: usize,
    forbidden: &[bool],
) |  |  |  |  |

| search_subgoal | function | search_subgoal(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[crate::types::NumPre],
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |

| DriverCfg | struct |  |  |  |  |  |


### crates/ferroplan/src/operator_compiler.rs

| OperatorCompileError | enum |  |  |  |  |  |

| compile_operator | function | compile_operator(
    spec: &OperatorSpec,
    states: &[State],
) |  |  |  |  |

| CompiledOperator | struct |  |  |  |  |  |

| OperatorEffects | struct |  |  |  |  |  |

| OperatorSpec | struct |  |  |  |  |  |


### crates/ferroplan/src/optimal.rs

| solve | function | solve(
    task: &PackedTask,
    cf: Option<usize>,
    max_nodes: usize,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| OptOutcome | struct |  |  |  |  |  |


### crates/ferroplan/src/orbits.rs

| canonical_key | function | canonical_key(
        &self,
        task: &PackedTask,
        state: &State,
        agenda: &[(i64, usize) |  |  |  |  |

| canonical_skey | function | canonical_skey(
        &self,
        task: &PackedTask,
        state: &State,
        cost_fluent: Option<usize>,
    ) |  |  |  |  |

| canonical_skey_hash | function | canonical_skey_hash(
        &self,
        task: &PackedTask,
        state: &State,
        cost_fluent: Option<usize>,
    ) |  |  |  |  |

| detect | function | detect(domain: &Domain, problem: &Problem, task: &PackedTask) |  |  |  |  |

| detect_classical | function | detect_classical(domain: &Domain, problem: &Problem, task: &PackedTask) |  |  |  |  |

| detect_classical_iso | function | detect_classical_iso(
    domain: &Domain,
    problem: &Problem,
    task: &PackedTask,
) |  |  |  |  |

| detect_iso | function | detect_iso(domain: &Domain, problem: &Problem, task: &PackedTask) |  |  |  |  |

| gen_key | function | gen_key(&self, op: usize, classes: &[Vec<u16>]) |  |  |  |  |

| goal_free_view | function | goal_free_view(&self) |  |  |  |  |

| iso_active | function | iso_active(&self) |  |  |  |  |

| iso_goal_witness | function | iso_goal_witness(
        &self,
        task: &PackedTask,
        state: &State,
        goal_pos: &[u32],
        goal_num: &[NumPre],
    ) |  |  |  |  |

| iso_remap_op | function | iso_remap_op(&self, sigma: &[Vec<u16>], op: usize) |  |  |  |  |

| iso_untouched_goal | function | iso_untouched_goal(&self) |  |  |  |  |

| stabilizer_classes | function | stabilizer_classes(&self, state: &State, agenda: &[(f64, usize) |  |  |  |  |

| IsoGoal | struct |  |  |  |  |  |

| Orbit | struct |  |  |  |  |  |

| OrbitMap | struct |  |  |  |  |  |


### crates/ferroplan/src/output.rs

| preamble | function | preamble(threads: usize) |  |  |  |  |

| render | function | render(task: &PackedTask, result: &PlanResult, threads: usize) |  |  |  |  |


### crates/ferroplan/src/packed.rs

| applicable_ops | function | applicable_ops(&self, s: &State, out: &mut Vec<u32>) |  |  |  |  |

| apply | function | apply(&self, oi: usize, s: &State) |  |  |  |  |

| build_succ | function | build_succ(pre_pos: &Csr<u32>, n_facts: usize, n_ops: usize) |  |  |  |  |

| cond_effs | function | cond_effs(&self, oi: usize) |  |  |  |  |

| fact_id | function | fact_id(&self, disp: &str) |  |  |  |  |

| finish | function | finish(self) |  |  |  |  |

| fluent_id | function | fluent_id(&self, disp: &str) |  |  |  |  |

| goal_met | function | goal_met(&self, s: &State) |  |  |  |  |

| goal_met_with | function | goal_met_with(&self, s: &State, goal_pos: &[u32], goal_num: &[NumPre]) |  |  |  |  |

| initial | function | initial(&self) |  |  |  |  |

| n_cond_effs | function | n_cond_effs(&self, oi: usize) |  |  |  |  |

| new | function | new() |  |  |  |  |

| op_applicable | function | op_applicable(&self, oi: usize, s: &State) |  |  |  |  |

| push_row | function | push_row(&mut self, items: impl IntoIterator<Item = T>) |  |  |  |  |

| slice | function | slice(&self, i: usize) |  |  |  |  |

| state_key | function | state_key(&self, s: &State) |  |  |  |  |

| state_key_eq | function | state_key_eq(&self, a: &State, b: &State, cost_fluent: Option<usize>) |  |  |  |  |

| state_key_hash | function | state_key_hash(&self, s: &State, cost_fluent: Option<usize>) |  |  |  |  |

| state_key_with_cost | function | state_key_with_cost(&self, s: &State, cost_fluent: Option<usize>) |  |  |  |  |

| static_fluent | function | static_fluent(&self, disp: &str) |  |  |  |  |

| CondEff | struct |  |  |  |  |  |

| Csr | struct |  |  |  |  |  |

| CsrBuilder | struct |  |  |  |  |  |

| PackedTask | struct |  |  |  |  |  |

| State | struct |  |  |  |  |  |

| StateKey | struct |  |  |  |  |  |


### crates/ferroplan/src/par.rs

| num_threads | function | num_threads() |  |  |  |  |


### crates/ferroplan/src/parser.rs

| parse_domain | function | parse_domain(src: &str) |  |  |  |  |

| parse_problem | function | parse_problem(src: &str) |  |  |  |  |


### crates/ferroplan/src/partition.rs

| interaction_partition | function | interaction_partition(task: &PackedTask, groups: &[Vec<u32>]) |  |  |  |  |

| interaction_partition_of | function | interaction_partition_of(
    task: &PackedTask,
    groups: &[Vec<u32>],
    goals: &[u32],
    excluded_vars: &FxHashSet<usize>,
) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| merge_at | function | merge_at(groups: &mut Vec<Subgoal>, i: usize, j: usize) |  |  |  |  |

| merge_with_neighbor | function | merge_with_neighbor(groups: &mut Vec<Subgoal>, i: usize) |  |  |  |  |

| partition | function | partition(task: &PackedTask) |  |  |  |  |

| Subgoal | struct |  |  |  |  |  |


### crates/ferroplan/src/pddl3.rs

| close_seed | function | close_seed(
    task: &PackedTask,
    cost_fluent: usize,
    forgos: &[(usize, f64) |  |  |  |  |

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |

| display_metric | function | display_metric(&self, optimized: f64) |  |  |  |  |

| hard_goal_plan | function | hard_goal_plan(
    domain: &Domain,
    problem: &Problem,
    threads: usize,
    cfg: SearchCfg,
) |  |  |  |  |

| hard_goal_seed | function | hard_goal_seed(
    domain: &Domain,
    problem: &Problem,
    compiled: &PackedTask,
    threads: usize,
    cfg: SearchCfg,
) |  |  |  |  |

| has_preferences | function | has_preferences(problem: &Problem) |  |  |  |  |

| is_pddl3 | function | is_pddl3(problem: &Problem) |  |  |  |  |

| lift_seed | function | lift_seed(compiled: &PackedTask, names: &[String]) |  |  |  |  |

| metric_optimize | function | metric_optimize(
    task: &PackedTask,
    cost_fluent: usize,
    forgos: &[(usize, f64) |  |  |  |  |

| metric_optimize_seeded | function | metric_optimize_seeded(
    task: &PackedTask,
    cost_fluent: usize,
    forgos: &[(usize, f64) |  |  |  |  |

| pref_weights | function | pref_weights(domain: &Domain, problem: &Problem) |  |  |  |  |

| preferences | function | preferences(goal: &Formula, objs: &HashMap<Sym, Vec<Sym>>) |  |  |  |  |

| Compiled | struct |  |  |  |  |  |

| MetricResult | struct |  |  |  |  |  |

| PhaseTail | struct |  |  |  |  |  |

| SeededResult | struct |  |  |  |  |  |


### crates/ferroplan/src/plan.rs

| Validity | enum |  |  |  |  |  |

| parse_classical | function | parse_classical(src: &str) |  |  |  |  |

| parse_timed | function | parse_timed(src: &str) |  |  |  |  |

| validate_plan | function | validate_plan(
    domain_src: &str,
    problem_src: &str,
    plan_src: &str,
) |  |  |  |  |


### crates/ferroplan/src/planner.rs

| run_ff | function | run_ff(domain_src: &str, problem_src: &str, opts: &crate::Options) |  |  |  |  |

| run_planner | function | run_planner(
    domain_src: &str,
    problem_src: &str,
    opts: &crate::Options,
    ipc: bool,
) |  |  |  |  |


### crates/ferroplan/src/planning_runtime.rs

| PlannerError | enum |  |  |  |  |  |

| solve_planning_type | function | solve_planning_type(
    request: &UniversalPlanningRequest,
) |  |  |  |  |

| Agent | struct |  |  |  |  |  |

| Goal | struct |  |  |  |  |  |

| Method | struct |  |  |  |  |  |

| PlanStep | struct |  |  |  |  |  |

| PlannerLimits | struct |  |  |  |  |  |

| PlanningProblem | struct |  |  |  |  |  |

| PolicyEntry | struct |  |  |  |  |  |

| PolicyOutcome | struct |  |  |  |  |  |

| QueueState | struct |  |  |  |  |  |

| RdfTriple | struct |  |  |  |  |  |

| State | struct |  |  |  |  |  |

| Task | struct |  |  |  |  |  |

| Tool | struct |  |  |  |  |  |

| Transition | struct |  |  |  |  |  |

| UniversalPlan | struct |  |  |  |  |  |

| UniversalPlanningRequest | struct |  |  |  |  |  |

| WorkflowEdge | struct |  |  |  |  |  |


### crates/ferroplan/src/planning_types.rs

| PlanningCapability | enum |  |  |  |  |  |

| PlanningRail | enum |  |  |  |  |  |

| PlanningRouteError | enum |  |  |  |  |  |

| PlanningType | enum |  |  |  |  |  |

| rail | function | rail(self) |  |  |  |  |

| required_capabilities | function | required_capabilities(self) |  |  |  |  |

| route_planning_request | function | route_planning_request(
    request: &PlanningRequest,
) |  |  |  |  |

| token | function | token(self) |  |  |  |  |

| PlanningRequest | struct |  |  |  |  |  |

| PlanningRoute | struct |  |  |  |  |  |


### crates/ferroplan/src/policy_validation.rs

| PolicyGuarantee | enum |  |  |  |  |  |

| PolicyIssue | enum |  |  |  |  |  |

| validate_fond_policy | function | validate_fond_policy(
    problem: &PlanningProblem,
    plan: &UniversalPlan,
) |  |  |  |  |

| PolicyValidationReport | struct |  |  |  |  |  |


### crates/ferroplan/src/portfolio.rs

| solve | function | solve(task: &PackedTask, threads: usize, cfg: SearchCfg) |  |  |  |  |

| Outcome | struct |  |  |  |  |  |


### crates/ferroplan/src/ppddl.rs

| PpddlError | enum |  |  |  |  |  |

| ProbabilisticObjective | enum |  |  |  |  |  |

| InitialStateProbability | struct |  |  |  |  |  |

| PolicyDecision | struct |  |  |  |  |  |

| PolicyOutcome | struct |  |  |  |  |  |

| PolicyValidation | struct |  |  |  |  |  |

| PpddlParseReport | struct |  |  |  |  |  |

| ProbabilisticOptions | struct |  |  |  |  |  |

| ProbabilisticSolution | struct |  |  |  |  |  |

| ProbabilisticState | struct |  |  |  |  |  |

| ProbabilisticStatistics | struct |  |  |  |  |  |

| SimulationReport | struct |  |  |  |  |  |


### crates/ferroplan/src/ppddl/compile/part07.rs

| parse_ppddl | function | parse_ppddl(domain_src: &str, problem_src: &str) |  |  |  |  |


### crates/ferroplan/src/ppddl/solver/part03.rs

| solve_ppddl | function | solve_ppddl(
    domain_src: &str,
    problem_src: &str,
    options: &ProbabilisticOptions,
) |  |  |  |  |


### crates/ferroplan/src/ppddl/solver/part04.rs

| validate_ppddl_policy | function | validate_ppddl_policy(
    domain_src: &str,
    problem_src: &str,
    options: &ProbabilisticOptions,
    solution: &ProbabilisticSolution,
) |  |  |  |  |


### crates/ferroplan/src/ppddl/solver/part05.rs

| simulate_ppddl | function | simulate_ppddl(
    domain_src: &str,
    problem_src: &str,
    options: &ProbabilisticOptions,
    episodes: usize,
    seed: u64,
) |  |  |  |  |


### crates/ferroplan/src/production.rs

| decompose_production | function | decompose_production(
    domain: &str,
    problem: &str,
    options: &Options,
    limits: &ProductionLimits,
    request_id: Option<&str>,
) |  |  |  |  |

| goal_met | function | goal_met(&self) |  |  |  |  |

| mind_bytes | function | mind_bytes(&self) |  |  |  |  |

| new | function | new(
        domain: &str,
        problem: &str,
        options: &Options,
        limits: ProductionLimits,
    ) |  |  |  |  |

| parse_production | function | parse_production(
    source: &str,
    max_input_bytes: usize,
    request_id: Option<&str>,
) |  |  |  |  |

| replan | function | replan(
        &self,
        max_evaluated: usize,
        memory_mb: Option<usize>,
        request_id: Option<&str>,
    ) |  |  |  |  |

| solve_ppddl_production | function | solve_ppddl_production(
    domain: &str,
    problem: &str,
    options: &ProbabilisticOptions,
    max_input_bytes: usize,
    max_output_bytes: usize,
    request_id: Option<&str>,
) |  |  |  |  |

| trace_production | function | trace_production(
    domain: &str,
    problem: &str,
    plan: &[(String, Vec<String>) |  |  |  |  |

| validate_plan_production | function | validate_plan_production(
    domain: &str,
    problem: &str,
    plan: &str,
    max_input_bytes: usize,
    max_plan_bytes: usize,
    request_id: Option<&str>,
) |  |  |  |  |

| world_bytes | function | world_bytes(&self) |  |  |  |  |

| PlanValidationEvidence | struct |  |  |  |  |  |

| ProductionSession | struct |  |  |  |  |  |


### crates/ferroplan/src/production_explain.rs

| decompose_production | function | decompose_production(
    domain: &str,
    problem: &str,
    options: &Options,
    limits: &ProductionLimits,
    request_id: Option<&str>,
) |  |  |  |  |

| explain_production | function | explain_production(
    domain: &str,
    problem: &str,
    plan: &Plan,
    limits: &ProductionLimits,
    request_id: Option<&str>,
) |  |  |  |  |


### crates/ferroplan/src/reachability.rs

| depth_reached | function | depth_reached(&self) |  |  |  |  |

| from_predecessors | function | from_predecessors(
        predecessors: &[Vec<u32>],
        prohibited: &[u32],
        max_depth: u32,
    ) |  |  |  |  |

| from_successors | function | from_successors(
        successors: &[Vec<u32>],
        prohibited: &[u32],
        max_depth: u32,
    ) |  |  |  |  |

| is_safe | function | is_safe(&self, state: u32) |  |  |  |  |

| saturated | function | saturated(&self) |  |  |  |  |

| unsafe_count | function | unsafe_count(&self) |  |  |  |  |

| BackwardSafeSet | struct |  |  |  |  |  |


### crates/ferroplan/src/readiness.rs

| AuthorityClass | enum |  |  |  |  |  |

| CompatibilityClass | enum |  |  |  |  |  |

| DeterminismClass | enum |  |  |  |  |  |

| InterfaceKind | enum |  |  |  |  |  |

| ManifestError | enum |  |  |  |  |  |

| OutcomeClass | enum |  |  |  |  |  |

| ReadinessState | enum |  |  |  |  |  |

| ReplayClass | enum |  |  |  |  |  |

| SecurityClass | enum |  |  |  |  |  |

| ValidationStatus | enum |  |  |  |  |  |

| capability_manifest | function | capability_manifest() |  |  |  |  |

| fingerprint | function | fingerprint(&self) |  |  |  |  |

| new | function | new(code: impl Into<String>, message: impl Into<String>, retryable: bool) |  |  |  |  |

| production_input_fingerprint | function | production_input_fingerprint(domain: &str, problem: &str, options: &Options) |  |  |  |  |

| solve_production | function | solve_production(
    domain: &str,
    problem: &str,
    options: &Options,
    limits: &ProductionLimits,
    request_id: Option<&str>,
) |  |  |  |  |

| validate | function | validate(&self) |  |  |  |  |

| BuildIdentity | struct |  |  |  |  |  |

| CapabilityContract | struct |  |  |  |  |  |

| CapabilityEvaluation | struct |  |  |  |  |  |

| CapabilityManifest | struct |  |  |  |  |  |

| OperationEnvelope | struct |  |  |  |  |  |

| ProductionLimits | struct |  |  |  |  |  |

| PublicError | struct |  |  |  |  |  |

| ReadinessReport | struct |  |  |  |  |  |


### crates/ferroplan/src/report.rs

| ff_plan | function | ff_plan(task: &PackedTask, ops: &[usize]) |  |  |  |  |

| ipc_plan | function | ipc_plan(task: &PackedTask, ops: &[usize], metric: Option<f64>) |  |  |  |  |

| metric_footer | function | metric_footer(
    cost: f64,
    iterations: usize,
    n_prefs: usize,
    threads: usize,
    warn_other: bool,
) |  |  |  |  |

| preamble | function | preamble(threads: usize) |  |  |  |  |

| timing | function | timing(stats: &Stats, threads: usize) |  |  |  |  |


### crates/ferroplan/src/resolve.rs

| Solved | enum |  |  |  |  |  |

| solve | function | solve(
    task: &PackedTask,
    threads: usize,
    cfg: crate::search::SearchCfg,
    mutex_groups: &[Vec<u32>],
    
    
    
    
    
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| Stats | struct |  |  |  |  |  |


### crates/ferroplan/src/resource.rs

| detect_resources | function | detect_resources(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) |  |  |  |  |

| occupancy | function | occupancy(&self, bits: &[u64]) |  |  |  |  |

| trip_bound | function | trip_bound(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) |  |  |  |  |

| trips | function | trips(&self, bits: &[u64]) |  |  |  |  |

| ResourceVar | struct |  |  |  |  |  |

| TripBound | struct |  |  |  |  |  |


### crates/ferroplan/src/sat.rs

| from_env | function | from_env() |  |  |  |  |

| requires_concurrency | function | requires_concurrency(domain: &Domain, problem: &Problem) |  |  |  |  |

| solve_classical | function | solve_classical(
    task: &PackedTask,
    groups: &[Vec<u32>],
    cfg: &SatCfg,
) |  |  |  |  |

| solve_temporal | function | solve_temporal(
    domain: &Domain,
    problem: &Problem,
    threads: usize,
    cfg: &SatCfg,
) |  |  |  |  |

| solve_temporal_within | function | solve_temporal_within(
    domain: &Domain,
    problem: &Problem,
    threads: usize,
    cfg: &SatCfg,
    budget_secs: Option<f64>,
) |  |  |  |  |

| SatCfg | struct |  |  |  |  |  |

| SatOutcome | struct |  |  |  |  |  |


### crates/ferroplan/src/search.rs

| PlanResult | enum |  |  |  |  |  |

| arm_wall_limit | function | arm_wall_limit() |  |  |  |  |

| cancelled | function | cancelled(&self) |  |  |  |  |

| cost | function | cost(&self, s: &State) |  |  |  |  |

| from_weights | function | from_weights(weight_g: f64, weight_h: f64, max_eval: Option<usize>) |  |  |  |  |

| holds | function | holds(&self, s: &State) |  |  |  |  |

| plan | function | plan(
    task: &PackedTask,
    threads: usize,
    cfg: SearchCfg,
    ehc_first: bool,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| plan_avoiding | function | plan_avoiding(
    task: &PackedTask,
    threads: usize,
    cfg: SearchCfg,
    ehc_first: bool,
    forbidden: &[bool],
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| search | function | search(task: &PackedTask, threads: usize, cfg: SearchCfg) |  |  |  |  |

| search_from | function | search_from(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: Option<usize>,
    cost_bound: f64,
    threads: usize,
    cfg: SearchCfg,
    forbidden: &[bool],
    sat: Option<&SatGuidance>,
    closure: Option<&ClosureCost>,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| solve_closure_bounded | function | solve_closure_bounded(
    task: &PackedTask,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
    bound: f64,
    closure: &ClosureCost,
    forbidden: &[bool],
    threads: usize,
    cfg: SearchCfg,
    sat: Option<&SatGuidance>,
) |  |  |  |  |

| solve_subgoal | function | solve_subgoal(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    threads: usize,
    cfg: SearchCfg,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| solve_subgoal_avoiding | function | solve_subgoal_avoiding(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    forbidden: &[bool],
    threads: usize,
    cfg: SearchCfg,
) |  |  |  |  |

| solve_subgoal_bounded | function | solve_subgoal_bounded(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
    bound: f64,
    threads: usize,
    cfg: SearchCfg,
    sat: Option<&SatGuidance>,
) |  |  |  |  |

| solve_subgoal_guided | function | solve_subgoal_guided(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    forbidden: &[bool],
    threads: usize,
    cfg: SearchCfg,
    sat: Option<&SatGuidance>,
) |  |  |  |  |

| with_cost_h | function | with_cost_h(mut self, cost_fluent: usize) |  |  |  |  |

| with_cost_weight | function | with_cost_weight(mut self, w_c: f64) |  |  |  |  |

| ClosureCost | struct |  |  |  |  |  |

| PlanOutcome | struct |  |  |  |  |  |

| PrefPhi | struct |  |  |  |  |  |

| SatGuidance | struct |  |  |  |  |  |

| SearchCfg | struct |  |  |  |  |  |


### crates/ferroplan/src/selection.rs

| select | function | select(
    task: &PackedTask,
    groups: &[Vec<u32>],
    weights: &[f64],
    dnf: &FxHashMap<usize, Vec<Vec<u32>>>,
    banned: &crate::hash::FxHashSet<u32>,
) |  |  |  |  |

| Selection | struct |  |  |  |  |  |


### crates/ferroplan/src/session.rs

| ThinkVerdict | enum |  |  |  |  |  |

| apply_start | function | apply_start(&mut self, name: &str) |  |  |  |  |

| elapse | function | elapse(&mut self, dt: f64) |  |  |  |  |

| fact | function | fact(&self, name: &str) |  |  |  |  |

| fluent | function | fluent(&self, name: &str) |  |  |  |  |

| fork | function | fork(&self) |  |  |  |  |

| goal_met | function | goal_met(&self) |  |  |  |  |

| mind_bytes | function | mind_bytes(&self) |  |  |  |  |

| new | function | new(domain_src: &str, problem_src: &str, opts: &Options) |  |  |  |  |

| observe | function | observe(&mut self, sight: &[(&str, bool) |  |  |  |  |

| plan_still_valid | function | plan_still_valid(&self, plan: &Plan, from_step: usize) |  |  |  |  |

| replan | function | replan(&self) |  |  |  |  |

| replan_budgeted | function | replan_budgeted(&self, max_evaluated: usize, memory_mb: Option<usize>) |  |  |  |  |

| replan_following | function | replan_following(
        &self,
        prior: &Plan,
        from_step: usize,
        max_evaluated: usize,
        memory_mb: Option<usize>,
    ) |  |  |  |  |

| restrict_ops | function | restrict_ops(&mut self, mut keep: impl FnMut(&str) |  |  |  |  |

| set_fact | function | set_fact(&mut self, name: &str, value: bool) |  |  |  |  |

| set_fluent | function | set_fluent(&mut self, name: &str, value: f64) |  |  |  |  |

| set_goal | function | set_goal(&mut self, goal: &str) |  |  |  |  |

| set_timed_fact | function | set_timed_fact(&mut self, dt: f64, name: &str, value: bool) |  |  |  |  |

| state_fingerprint | function | state_fingerprint(&self) |  |  |  |  |

| think | function | think(&self, budget: &ThinkBudget) |  |  |  |  |

| think_following | function | think_following(&self, prior: &Plan, from_step: usize, budget: &ThinkBudget) |  |  |  |  |

| world_bytes | function | world_bytes(&self) |  |  |  |  |

| Session | struct |  |  |  |  |  |

| Think | struct |  |  |  |  |  |

| ThinkBudget | struct |  |  |  |  |  |


### crates/ferroplan/src/tcompress.rs

| Bet | enum |  |  |  |  |  |

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |

| declines | function | declines(domain: &Domain, problem: &Problem) |  |  |  |  |

| lay_out | function | lay_out(
    domain: &Domain,
    task: &PackedTask,
    ops: &[usize],
    shift: bool,
) |  |  |  |  |

| solve | function | solve(domain: &Domain, problem: &Problem, threads: usize, bet: Bet) |  |  |  |  |


### crates/ferroplan/src/temporal.rs

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |

| is_temporal | function | is_temporal(domain: &Domain) |  |  |  |  |

| prepare | function | prepare(domain: &'a Domain, problem: &'a Problem) |  |  |  |  |

| score | function | score(&self, plan: &TimedPlan) |  |  |  |  |

| score_soft | function | score_soft(domain: &Domain, problem: &Problem, plan: &TimedPlan) |  |  |  |  |

| solve | function | solve(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| solve_scored | function | solve_scored(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| to_ipc | function | to_ipc(&self) |  |  |  |  |

| validate | function | validate(domain: &Domain, problem: &Problem, plan: &TimedPlan) |  |  |  |  |

| ScoredPlan | struct |  |  |  |  |  |

| SnapInfo | struct |  |  |  |  |  |

| SoftScore | struct |  |  |  |  |  |

| SoftScorer | struct |  |  |  |  |  |

| TemporalCompiled | struct |  |  |  |  |  |

| TimedPlan | struct |  |  |  |  |  |

| TimedStep | struct |  |  |  |  |  |


### crates/ferroplan/src/trace.rs

| trace | function | trace(
    domain_src: &str,
    problem_src: &str,
    plan: &[(String, Vec<String>) |  |  |  |  |

| StateSnapshot | struct |  |  |  |  |  |


### crates/ferroplan/src/tresolve.rs

| solve | function | solve(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |


### crates/ferroplan/src/tsched.rs

| n_actors | function | n_actors(domain: &Domain, problem: &Problem) |  |  |  |  |

| reschedule | function | reschedule(domain: &Domain, problem: &Problem, plan: &TimedPlan) |  |  |  |  |

| single_actor_problem | function | single_actor_problem(domain: &Domain, problem: &Problem) |  |  |  |  |


### crates/ferroplan/src/types.rs

| AssignOp | enum |  |  |  |  |  |

| CompOp | enum |  |  |  |  |  |

| Constraint | enum |  |  |  |  |  |

| Effect | enum |  |  |  |  |  |

| Expr | enum |  |  |  |  |  |

| Formula | enum |  |  |  |  |  |

| MetricDir | enum |  |  |  |  |  |

| NExpr | enum |  |  |  |  |  |

| Term | enum |  |  |  |  |  |

| TimeSpec | enum |  |  |  |  |  |

| chosen | function | chosen(&self) |  |  |  |  |

| collect_fluents | function | collect_fluents(&self, out: &mut Vec<u32>) |  |  |  |  |

| eval | function | eval(&self, fv: &[f64], def: &[bool]) |  |  |  |  |

| eval_numpre | function | eval_numpre(np: &NumPre, fv: &[f64], def: &[bool]) |  |  |  |  |

| fixed | function | fixed(e: Expr) |  |  |  |  |

| new | function | new(line: u32, message: impl Into<String>) |  |  |  |  |

| Action | struct |  |  |  |  |  |

| DerivedRule | struct |  |  |  |  |  |

| Domain | struct |  |  |  |  |  |

| Duration | struct |  |  |  |  |  |

| DurativeAction | struct |  |  |  |  |  |

| NumEff | struct |  |  |  |  |  |

| NumPre | struct |  |  |  |  |  |

| ParseError | struct |  |  |  |  |  |

| Problem | struct |  |  |  |  |  |

| TimedLiteral | struct |  |  |  |  |  |


### crates/ferroplan/src/verify.rs

| verify | function | verify(
    domain_src: &str,
    problem_src: &str,
    plan: &[(String, Vec<String>) |  |  |  |  |

| Verified | struct |  |  |  |  |  |


### crates/ferroplan/src/viz.rs

| PredKind | enum |  |  |  |  |  |

| build | function | build(domain: &Domain, problem: &Problem) |  |  |  |  |

| domain_to_pddl | function | domain_to_pddl(
    name: &str,
    requirements: &str,
    types: &[(String, String) |  |  |  |  |

| dynamic_predicates | function | dynamic_predicates(domain: &Domain) |  |  |  |  |

| goal_facts | function | goal_facts(problem: &Problem) |  |  |  |  |

| positions_at | function | positions_at(&self, facts: &[String]) |  |  |  |  |

| to_pddl | function | to_pddl(
    name: &str,
    domain_name: &str,
    objects: &[(String, String) |  |  |  |  |

| VizEdge | struct |  |  |  |  |  |

| VizGraph | struct |  |  |  |  |  |

| VizMobile | struct |  |  |  |  |  |

| VizNode | struct |  |  |  |  |  |


### crates/ferroplan/tests/common/external.rs

| corpus_dir | function | corpus_dir() |  |  |  |  |

| corpus_ipc_dir | function | corpus_ipc_dir() |  |  |  |  |

| differential_run_dir | function | differential_run_dir() |  |  |  |  |

| harness_present | function | harness_present(what: &str, path: &Path) |  |  |  |  |

| oracle_dir | function | oracle_dir() |  |  |  |  |

| oracle_runner | function | oracle_runner() |  |  |  |  |


### crates/ferroplan/tests/common/mod.rs

| base_sizes | function | base_sizes(rng: &mut Rng) |  |  |  |  |

| below | function | below(&mut self, n: u64) |  |  |  |  |

| chance | function | chance(&mut self, percent: u64) |  |  |  |  |

| draw_for | function | draw_for(seed: u64, sizes: Sizes) |  |  |  |  |

| draw_valid | function | draw_valid(seed: u64, sizes: Sizes) |  |  |  |  |

| generate | function | generate(seed: u64, sizes: Sizes) |  |  |  |  |

| generate_valid | function | generate_valid(seed: u64, sizes: Sizes) |  |  |  |  |

| halve_sizes | function | halve_sizes(s: Sizes) |  |  |  |  |

| mutation_of | function | mutation_of(seed: u64) |  |  |  |  |

| new | function | new(seed: u64) |  |  |  |  |

| next_u64 | function | next_u64(&mut self) |  |  |  |  |

| pick_idx | function | pick_idx(&mut self, len: usize) |  |  |  |  |

| range | function | range(&mut self, lo: u64, hi: u64) |  |  |  |  |

| range_usize | function | range_usize(&mut self, lo: usize, hi: usize) |  |  |  |  |

| render | function | render(model: &Model, problem_name: &str) |  |  |  |  |

| render_with_provenance | function | render_with_provenance(
    model: &Model,
    problem_name: &str,
    header: &[&str],
) |  |  |  |  |

| sizes_for | function | sizes_for(seed: u64) |  |  |  |  |

| Model | struct |  |  |  |  |  |

| Rng | struct |  |  |  |  |  |

| Sizes | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/artifact/board_md.rs

| from_rows | function | from_rows(rows: &[RawRow]) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| render | function | render(
    header: &BoardHeader,
    summary: &[VariantSummary],
    score_against: Option<&str>,
) |  |  |  |  |

| summarize_variants | function | summarize_variants(rows: &[RawRow], reference: Option<&Reference>) |  |  |  |  |

| BoardHeader | struct |  |  |  |  |  |

| Reference | struct |  |  |  |  |  |

| VariantSummary | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/artifact/conditions.rs

| ConditionsError | enum |  |  |  |  |  |

| Slot | enum |  |  |  |  |  |

| as_option | function | as_option(&self) |  |  |  |  |

| at | function | at(&self) |  |  |  |  |

| competitors_total | function | competitors_total(&self) |  |  |  |  |

| idle_pct | function | idle_pct(&self) |  |  |  |  |

| is_present | function | is_present(&self) |  |  |  |  |

| new | function | new(
        at: Option<Number>,
        idle_pct: Option<Number>,
        competitors_total: Option<Number>,
    ) |  |  |  |  |

| observe | function | observe(&mut self, r: &Reading<'_>) |  |  |  |  |

| of | function | of(self_exclusion: Vec<String>) |  |  |  |  |

| parse | function | parse(text: &str, path: &str) |  |  |  |  |

| percentile | function | percentile(v: &[f64], p: f64) |  |  |  |  |

| rollup_from_timeline | function | rollup_from_timeline(timeline: &[TimelineEntry]) |  |  |  |  |

| statistics_median | function | statistics_median(v: &[f64]) |  |  |  |  |

| summarize | function | summarize(r: &Rollup, ended: &str, provenance: Option<&Provenance>) |  |  |  |  |

| to_json | function | to_json(&self) |  |  |  |  |

| Competitors | struct |  |  |  |  |  |

| Conditions | struct |  |  |  |  |  |

| CpuSpeedLimit | struct |  |  |  |  |  |

| IdlePct | struct |  |  |  |  |  |

| LoadAvg | struct |  |  |  |  |  |

| Provenance | struct |  |  |  |  |  |

| Reading | struct |  |  |  |  |  |

| Rollup | struct |  |  |  |  |  |

| SwapMb | struct |  |  |  |  |  |

| TimelineEntry | struct |  |  |  |  |  |

| TimelineRollup | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/corpus.rs

| instances | function | instances(v: &Variant, max: usize, warnings: &mut Vec<String>) |  |  |  |  |

| variants | function | variants(corpus: &Path, ipcs: &[String], selects: &dyn Fn(&str) |  |  |  |  |

| Instance | struct |  |  |  |  |  |

| Variant | struct |  |  |  |  |  |

| Walk | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/db/lock.rs

| LockError | enum |  |  |  |  |  |

| acquire | function | acquire(dir: &Path) |  |  |  |  |

| path | function | path(&self) |  |  |  |  |

| DirLock | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/db/mod.rs

| DbError | enum |  |  |  |  |  |

| open | function | open(dir: &Path) |  |  |  |  |

| path | function | path(&self) |  |  |  |  |

| reader | function | reader(&self) |  |  |  |  |

| writer | function | writer(&self) |  |  |  |  |

| Db | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/db/model.rs

| Cleanliness | enum |  |  |  |  |  |

| PassVerdict | enum |  |  |  |  |  |

| RunState | enum |  |  |  |  |  |

| TimingQuality | enum |  |  |  |  |  |

| ValReason | enum |  |  |  |  |  |

| as_str | function | as_str(self) |  |  |  |  |

| identity | function | identity(&self) |  |  |  |  |

| of | function | of(i: &Instance) |  |  |  |  |

| parse | function | parse(s: &str) |  |  |  |  |

| sort_key | function | sort_key(label: &str) |  |  |  |  |

| to_instance | function | to_instance(&self) |  |  |  |  |

| AttemptRec | struct |  |  |  |  |  |

| BoardFacts | struct |  |  |  |  |  |

| BoardKey | struct |  |  |  |  |  |

| BoardPassRec | struct |  |  |  |  |  |

| EngineFacts | struct |  |  |  |  |  |

| EngineKey | struct |  |  |  |  |  |

| EventRec | struct |  |  |  |  |  |

| InstanceKey | struct |  |  |  |  |  |

| LiveChild | struct |  |  |  |  |  |

| Measured | struct |  |  |  |  |  |

| RunRecord | struct |  |  |  |  |  |

| SamplePoint | struct |  |  |  |  |  |

| SampleRec | struct |  |  |  |  |  |

| ThrottleWindowRec | struct |  |  |  |  |  |

| VariantKey | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/db/read.rs

| attempts_for | function | attempts_for(
        &self,
        board_id: i64,
        engine_id: i64,
        variant: &str,
        label: &str,
    ) |  |  |  |  |

| banked_instances | function | banked_instances(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| boards_named | function | boards_named(&self, name: &str) |  |  |  |  |

| canary_baseline | function | canary_baseline(
        &self,
        label: &str,
        window: usize,
        pct: f64,
    ) |  |  |  |  |

| canary_max_between | function | canary_max_between(&self, start_ts: f64, end_ts: f64) |  |  |  |  |

| clean_instances | function | clean_instances(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| competitors_between | function | competitors_between(
        &self,
        start_ts: f64,
        end_ts: f64,
    ) |  |  |  |  |

| conn | function | conn(&self) |  |  |  |  |

| engine_by_hash | function | engine_by_hash(&self, blake3: &str) |  |  |  |  |

| engines_for_board | function | engines_for_board(&self, board_id: i64) |  |  |  |  |

| engines_matching | function | engines_matching(&self, needle: &str) |  |  |  |  |

| export_rows | function | export_rows(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| live_children | function | live_children(&self) |  |  |  |  |

| next_attempt | function | next_attempt(
        &self,
        board_id: i64,
        engine_id: i64,
        ipc: Option<&str>,
        variant: &str,
        label: &str,
    ) |  |  |  |  |

| open | function | open(path: &Path) |  |  |  |  |

| pass_verdict | function | pass_verdict(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| prior_peak_rss | function | prior_peak_rss(&self, variant: &str, label: &str) |  |  |  |  |

| run_census | function | run_census(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| runs_between | function | runs_between(
        &self,
        engine_id: i64,
        start_ts: f64,
        end_ts: f64,
    ) |  |  |  |  |

| sample_count | function | sample_count(&self, pass: Option<i64>) |  |  |  |  |

| samples_between | function | samples_between(&self, start_ts: f64, end_ts: f64) |  |  |  |  |

| solo_attempts | function | solo_attempts(
        &self,
        board_id: i64,
        engine_id: i64,
        variant: &str,
        label: &str,
    ) |  |  |  |  |

| swap_growth_between | function | swap_growth_between(&self, start_ts: f64, end_ts: f64) |  |  |  |  |

| throttle_windows_between | function | throttle_windows_between(
        &self,
        start_ts: f64,
        end_ts: f64,
    ) |  |  |  |  |

| timing_census | function | timing_census(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| val_ok | function | val_ok(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| val_rejected | function | val_rejected(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| val_unavailable | function | val_unavailable(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| verdicts_for | function | verdicts_for(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| window_gate | function | window_gate(
        &self,
        start_ts: f64,
        end_ts: f64,
        interval: f64,
        pass: Option<i64>,
    ) |  |  |  |  |

| Reader | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/db/rebuild.rs

| board_facts | function | board_facts(spec: &BoardSpec, m: &Manifest, first: Option<&RawRow>) |  |  |  |  |

| board_key_from_manifest | function | board_key_from_manifest(m: &Manifest, spec: &BoardSpec) |  |  |  |  |

| export | function | export(reader: &Reader, board_id: i64, engine_id: i64) |  |  |  |  |

| export_to | function | export_to(
    reader: &Reader,
    board_id: i64,
    engine_id: i64,
    path: &Path,
) |  |  |  |  |

| rebuild_from_artifacts | function | rebuild_from_artifacts(
    writer: &WriterHandle,
    manifest: &Manifest,
    dir: &Path,
    val_unavailable: Option<&ValUnavailable>,
) |  |  |  |  |

| RebuiltBoard | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/db/schema.rs

| MigrateError | enum |  |  |  |  |  |

| migrate | function | migrate(conn: &Connection) |  |  |  |  |


### crucible/crates/crucible-core/src/db/writer.rs

| board_pass | function | board_pass(&self, p: BoardPassRec) |  |  |  |  |

| canary | function | canary(&self, at: f64, label: String, secs: f64, solo: bool) |  |  |  |  |

| child_gone | function | child_gone(&self, pid: i32) |  |  |  |  |

| child_spawned | function | child_spawned(&self, c: LiveChild) |  |  |  |  |

| child_stopped | function | child_stopped(&self, pid: i32, stopped: bool) |  |  |  |  |

| event | function | event(&self, e: EventRec) |  |  |  |  |

| flush | function | flush(&self) |  |  |  |  |

| handle | function | handle(&self) |  |  |  |  |

| resolve | function | resolve(
        &self,
        board: BoardKey,
        board_facts: BoardFacts,
        engine: EngineKey,
        engine_facts: EngineFacts,
    ) |  |  |  |  |

| run | function | run(&self, rec: RunRecord) |  |  |  |  |

| sample | function | sample(&self, s: SampleRec) |  |  |  |  |

| start | function | start(conn: Connection) |  |  |  |  |

| take_error | function | take_error(&self) |  |  |  |  |

| throttle_close | function | throttle_close(&self, id: i64, ended_at: f64) |  |  |  |  |

| throttle_open | function | throttle_open(&self, w: ThrottleWindowRec) |  |  |  |  |

| Writer | struct |  |  |  |  |  |

| WriterHandle | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/exec/env.rs

| build | function | build(
    timeout_secs: u64,
    mem_gb: f64,
    board_env: &BTreeMap<String, String>,
) |  |  |  |  |

| validate | function | validate(
    timeout_secs: u64,
    mem_gb: f64,
    board_env: &BTreeMap<String, String>,
) |  |  |  |  |


### crucible/crates/crucible-core/src/exec/mod.rs

| Ctl | enum |  |  |  |  |  |

| ExecError | enum |  |  |  |  |  |

| Killed | enum |  |  |  |  |  |

| install_interrupt_handler | function | install_interrupt_handler() |  |  |  |  |

| interrupted | function | interrupted() |  |  |  |  |

| set_interrupted | function | set_interrupted(on: bool) |  |  |  |  |

| RunOutcome | struct |  |  |  |  |  |

| RunRequest | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/exec/orphan.rs

| Reaped | enum |  |  |  |  |  |

| armed | function | armed(&self) |  |  |  |  |

| disarm | function | disarm(&mut self) |  |  |  |  |

| install_panic_reaper | function | install_panic_reaper() |  |  |  |  |

| new | function | new(pgid: Pid) |  |  |  |  |

| pgid | function | pgid(&self) |  |  |  |  |

| pid | function | pid(&self) |  |  |  |  |

| reap_registered | function | reap_registered() |  |  |  |  |

| record | function | record(pid: Pid, run_id: Option<i64>, id: &ProcIdentity, spawned_at: f64) |  |  |  |  |

| signalled | function | signalled(&self) |  |  |  |  |

| GroupGuard | struct |  |  |  |  |  |

| LiveChild | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/monitor/games.rs

| busiest | function | busiest(&self, procs: &[Proc]) |  |  |  |  |

| game_pids | function | game_pids(&self, procs: &[Proc]) |  |  |  |  |

| GameRules | struct |  |  |  |  |  |

| Proc | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/monitor/sample.rs

| attribute | function | attribute(ps_output: &str, exclude: &dyn Fn(&str) |  |  |  |  |

| is_clean | function | is_clean(&self) |  |  |  |  |

| Sample | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/monitor/throttle.rs

| Level | enum |  |  |  |  |  |

| Reason | enum |  |  |  |  |  |

| level | function | level(&self) |  |  |  |  |

| new | function | new(cfg: Config) |  |  |  |  |

| on_sample | function | on_sample(&mut self, s: &Sample, g: &GameState, now: Instant) |  |  |  |  |

| set_manual_hold | function | set_manual_hold(&mut self, on: bool) |  |  |  |  |

| Config | struct |  |  |  |  |  |

| GameState | struct |  |  |  |  |  |

| Throttle | struct |  |  |  |  |  |

| Transition | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/platform/generic.rs

| Generic | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/platform/macos.rs

| mach_ticks_to_ns | function | mach_ticks_to_ns(ticks: u64) |  |  |  |  |

| MacOs | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/platform/mod.rs

| MemCap | enum |  |  |  |  |  |

| bytes | function | bytes(self) |  |  |  |  |

| host | function | host() |  |  |  |  |

| instrument | function | instrument(self) |  |  |  |  |

| ProcIdentity | struct |  |  |  |  |  |

| Topology | struct |  |  |  |  |  |

| KeepAwake | trait |  |  |  |  |  |

| Platform | trait |  |  |  |  |  |


### crucible/crates/crucible-core/src/sched/budget.rs

| Admission | enum |  |  |  |  |  |

| Denial | enum |  |  |  |  |  |

| admit | function | admit(&self, level: Level, demand: Demand, declared: bool) |  |  |  |  |

| capacity | function | capacity(&self, level: Level) |  |  |  |  |

| contains | function | contains(&self, board_id: &str) |  |  |  |  |

| cores | function | cores(self) |  |  |  |  |

| errors | function | errors(lines: &[String]) |  |  |  |  |

| ids | function | ids(&self) |  |  |  |  |

| is_admitted | function | is_admitted(&self) |  |  |  |  |

| new | function | new(jobs: u32, threads: u32) |  |  |  |  |

| none | function | none() |  |  |  |  |

| of | function | of(b: &BoardSpec, d: &Defaults) |  |  |  |  |

| validate | function | validate(&self, m: &Manifest, declared: &Oversubscribed) |  |  |  |  |

| with_reserve | function | with_reserve(topology: Topology, reserve_p_cores: u32) |  |  |  |  |

| Accountant | struct |  |  |  |  |  |

| Demand | struct |  |  |  |  |  |

| Oversubscribed | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/sched/mod.rs

| Event | enum |  |  |  |  |  |

| Next | enum |  |  |  |  |  |

| backoff | function | backoff(&self, consecutive: u32) |  |  |  |  |

| order_boards | function | order_boards(boards: &[BoardState], pass: u32) |  |  |  |  |

| run | function | run(r: &mut dyn Runner, cfg: &LoopConfig) |  |  |  |  |

| Attempt | struct |  |  |  |  |  |

| BoardState | struct |  |  |  |  |  |

| LoopConfig | struct |  |  |  |  |  |

| Outcome | struct |  |  |  |  |  |

| Runner | trait |  |  |  |  |  |


### crucible/crates/crucible-core/src/sched/quiet.rs

| Admission | enum |  |  |  |  |  |

| Denied | enum |  |  |  |  |  |

| Rule | enum |  |  |  |  |  |

| config | function | config(&self) |  |  |  |  |

| is_admitted | function | is_admitted(&self) |  |  |  |  |

| new | function | new(cfg: Config) |  |  |  |  |

| poll | function | poll(&mut self, level: Level, sample: &Sample, now: Instant) |  |  |  |  |

| reset | function | reset(&mut self) |  |  |  |  |

| Config | struct |  |  |  |  |  |

| Gate | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/sched/referee.rs

| Bank | enum |  |  |  |  |  |

| Owe | enum |  |  |  |  |  |

| Verdict | enum |  |  |  |  |  |

| as_str | function | as_str(self) |  |  |  |  |

| banked | function | banked(self) |  |  |  |  |

| box_fault | function | box_fault(self) |  |  |  |  |

| judge | function | judge(rule: &Rule, f: &Facts) |  |  |  |  |

| rho | function | rho(&self) |  |  |  |  |

| rho_floor_ms | function | rho_floor_ms(&self) |  |  |  |  |

| timing | function | timing(f: &Facts) |  |  |  |  |

| Facts | struct |  |  |  |  |  |

| Rule | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/sched/resume.rs

| Disabled | enum |  |  |  |  |  |

| InstanceKey | enum |  |  |  |  |  |

| Reject | enum |  |  |  |  |  |

| disabled | function | disabled(&self) |  |  |  |  |

| from_document | function | from_document(doc: &crate::artifact::conditions::Conditions) |  |  |  |  |

| get | function | get(&self, key: &RowKey) |  |  |  |  |

| has_timeline | function | has_timeline(&self) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| judge | function | judge(row: &RawRow, cond: &Conditions, want: &RunParams) |  |  |  |  |

| judge_lines | function | judge_lines(text: &str, cond: &Conditions, want: &RunParams) |  |  |  |  |

| kind | function | kind(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| load | function | load(path: &Path) |  |  |  |  |

| mode_str | function | mode_str(&self) |  |  |  |  |

| none | function | none() |  |  |  |  |

| of | function | of(r: &RawRow) |  |  |  |  |

| parse | function | parse(text: &str) |  |  |  |  |

| reject_counts | function | reject_counts(&self) |  |  |  |  |

| rejected | function | rejected(&self) |  |  |  |  |

| Conditions | struct |  |  |  |  |  |

| Rejected | struct |  |  |  |  |  |

| Resume | struct |  |  |  |  |  |

| RowKey | struct |  |  |  |  |  |

| RunParams | struct |  |  |  |  |  |

| TimelineSample | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/sched/tier.rs

| Tier | enum |  |  |  |  |  |

| census | function | census(plan: &[Scheduled]) |  |  |  |  |

| classify | function | classify(prior: Option<Prior>, t: &Thresholds) |  |  |  |  |

| eta | function | eta(plan: &[Scheduled], jobs: u32) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| order | function | order(
    instances: &[RowKey],
    h: &dyn History,
    t: &Thresholds,
    budget_secs: f64,
) |  |  |  |  |

| NoHistory | struct |  |  |  |  |  |

| Prior | struct |  |  |  |  |  |

| Scheduled | struct |  |  |  |  |  |

| Thresholds | struct |  |  |  |  |  |

| History | trait |  |  |  |  |  |


### crucible/crates/crucible-core/src/sweep.rs

| argv | function | argv(cfg: &BoardCfg, domain: &Path, problem: &Path) |  |  |  |  |

| BoardCfg | struct |  |  |  |  |  |

| Engine | struct |  |  |  |  |  |

| Measured | struct |  |  |  |  |  |


### crucible/crates/crucible-core/src/validate/mod.rs

| Unavailable | enum |  |  |  |  |  |

| Verdict | enum |  |  |  |  |  |

| as_json | function | as_json(self) |  |  |  |  |

| find | function | find(repo: &Path, configured: Option<&Path>) |  |  |  |  |

| judge | function | judge(rc: Option<i32>, signal: Option<i32>, stdout: &str, stderr: &str) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| reason | function | reason(self) |  |  |  |  |

| render_plan | function | render_plan(steps: &[Step], temporal: bool) |  |  |  |  |

| validate | function | validate(
    val: Option<&Path>,
    domain: &Path,
    problem: &Path,
    steps: &[Step],
    temporal: bool,
    plan_path: &Path,
) |  |  |  |  |

| Step | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/archive.rs

| ArchiveError | enum |  |  |  |  |  |

| arch_key | function | arch_key(variant: &str, instance: u64) |  |  |  |  |

| arch_track | function | arch_track(variant: &str) |  |  |  |  |

| best_length | function | best_length(&self, k: &ArchKey) |  |  |  |  |

| best_makespan | function | best_makespan(&self, k: &ArchKey) |  |  |  |  |

| count_action_lines | function | count_action_lines(body: &str) |  |  |  |  |

| has_lengths | function | has_lengths(&self) |  |  |  |  |

| has_makespans | function | has_makespans(&self) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| iter | function | iter(&self) |  |  |  |  |

| lengths | function | lengths(&self, k: &ArchKey) |  |  |  |  |

| lengths_map | function | lengths_map(&self) |  |  |  |  |

| makespan_of | function | makespan_of(body: &str) |  |  |  |  |

| makespans | function | makespans(&self, k: &ArchKey) |  |  |  |  |

| makespans_map | function | makespans_map(&self) |  |  |  |  |

| open | function | open(path: &Path) |  |  |  |  |

| warnings | function | warnings(&self) |  |  |  |  |

| ArchiveWarnings | struct |  |  |  |  |  |

| Ipc5Archive | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/bounds.rs

| best | function | best(&self, year_key: &str, domain: &str, instance: u64) |  |  |  |  |

| from_sources | function | from_sources(bounds_2023: Option<&str>, cost_bounds_2018: Option<&str>) |  |  |  |  |

| get | function | get(&self, k: &BoundKey) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| load | function | load(corpus_root: &Path) |  |  |  |  |

| problems | function | problems(&self) |  |  |  |  |

| BestKnownBounds | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/class.rs

| Class | enum |  |  |  |  |  |

| failure_classes | function | failure_classes(&self) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| Coverage | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/compare.rs

| Cleanliness | enum |  |  |  |  |  |

| Clock | enum |  |  |  |  |  |

| Cost | enum |  |  |  |  |  |

| Mode | enum |  |  |  |  |  |

| a | function | a(&self) |  |  |  |  |

| a_solved | function | a_solved(&self) |  |  |  |  |

| b | function | b(&self) |  |  |  |  |

| b_solved | function | b_solved(&self) |  |  |  |  |

| cheaper_a | function | cheaper_a(&self) |  |  |  |  |

| cheaper_b | function | cheaper_b(&self) |  |  |  |  |

| clean | function | clean(&self) |  |  |  |  |

| cleanliness | function | cleanliness(&self, r: &RawRow) |  |  |  |  |

| clock | function | clock(&self) |  |  |  |  |

| common | function | common(&self) |  |  |  |  |

| coverage | function | coverage(&self) |  |  |  |  |

| delta | function | delta(&self) |  |  |  |  |

| dirty | function | dirty(&self) |  |  |  |  |

| equal | function | equal(&self) |  |  |  |  |

| from_json | function | from_json(src: &str) |  |  |  |  |

| from_jsonl | function | from_jsonl(
        name: impl Into<String>,
        budget: f64,
        src: &str,
        path: &str,
    ) |  |  |  |  |

| from_rows | function | from_rows(name: impl Into<String>, budget: f64, rows: Vec<RawRow>) |  |  |  |  |

| gained | function | gained(&self) |  |  |  |  |

| get | function | get(&self, k: &InstanceKey) |  |  |  |  |

| has_timeline | function | has_timeline(&self) |  |  |  |  |

| interval | function | interval(&self) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| is_regression | function | is_regression(&self) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| load | function | load(path: &Path) |  |  |  |  |

| lost | function | lost(&self) |  |  |  |  |

| mean_a | function | mean_a(&self) |  |  |  |  |

| mean_b | function | mean_b(&self) |  |  |  |  |

| name | function | name(self) |  |  |  |  |

| new | function | new(referee: &'r Referee, a: &'r RunRef, b: &'r RunRef) |  |  |  |  |

| none | function | none() |  |  |  |  |

| of | function | of(r: &RawRow) |  |  |  |  |

| percentiles | function | percentiles(&self) |  |  |  |  |

| qualified | function | qualified(&self) |  |  |  |  |

| quality | function | quality(&self) |  |  |  |  |

| referee | function | referee(&self) |  |  |  |  |

| render | function | render(&self, mode: Mode) |  |  |  |  |

| render_trend | function | render_trend(t: &Trend) |  |  |  |  |

| scored | function | scored(&self) |  |  |  |  |

| solved | function | solved(&self, referee: &Referee) |  |  |  |  |

| table | function | table(&self) |  |  |  |  |

| timing | function | timing(&self, cond_a: &Conditions, cond_b: &Conditions) |  |  |  |  |

| to_json | function | to_json(&self) |  |  |  |  |

| total_a | function | total_a(&self) |  |  |  |  |

| total_b | function | total_b(&self) |  |  |  |  |

| unstamped | function | unstamped(&self) |  |  |  |  |

| value | function | value(self) |  |  |  |  |

| variants | function | variants(&self) |  |  |  |  |

| with_trend | function | with_trend(mut self, t: Trend) |  |  |  |  |

| worse | function | worse(self, other: Cleanliness) |  |  |  |  |

| Conditions | struct |  |  |  |  |  |

| CoverageDiff | struct |  |  |  |  |  |

| Diff | struct |  |  |  |  |  |

| Gained | struct |  |  |  |  |  |

| InstanceKey | struct |  |  |  |  |  |

| Loaded | struct |  |  |  |  |  |

| Lost | struct |  |  |  |  |  |

| Percentiles | struct |  |  |  |  |  |

| QualityDiff | struct |  |  |  |  |  |

| RunRef | struct |  |  |  |  |  |

| TimingDiff | struct |  |  |  |  |  |

| VariantRow | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/field.rs

| cell | function | cell(
        &self,
        label: &str,
        rows: &[RawRow],
        referee: &Referee,
        solved: usize,
        total: usize,
    ) |  |  |  |  |

| cohort | function | cohort(&self, label: &str) |  |  |  |  |

| load | function | load(benchmarks_dir: &Path) |  |  |  |  |

| placement | function | placement(&self, s: usize, n: usize) |  |  |  |  |

| unmatched_splits | function | unmatched_splits(&self, label: &str, rows: &[RawRow]) |  |  |  |  |

| warnings | function | warnings(&self) |  |  |  |  |

| Cohort | struct |  |  |  |  |  |

| Entrant | struct |  |  |  |  |  |

| FieldBook | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/fmt.rs

| bar | function | bar(pct: f64, width: usize) |  |  |  |  |

| fmt_f | function | fmt_f(x: f64, places: usize) |  |  |  |  |

| ordinal | function | ordinal(n: usize) |  |  |  |  |

| ordinal_suffix | function | ordinal_suffix(n: usize) |  |  |  |  |

| pct | function | pct(solved: usize, total: usize) |  |  |  |  |

| py_round | function | py_round(x: f64, ndigits: i32) |  |  |  |  |

| py_round_i | function | py_round_i(x: f64) |  |  |  |  |

| thousands | function | thousands(n: u64) |  |  |  |  |


### crucible/crates/crucible-publish/src/history.rs

| HistoryError | enum |  |  |  |  |  |

| as_str | function | as_str(&self) |  |  |  |  |

| comparable_predecessor | function | comparable_predecessor(
        &self,
        box_: &BoxId,
        cur: &VersionKey,
    ) |  |  |  |  |

| current_version | function | current_version(root: &Path) |  |  |  |  |

| delta | function | delta(&self, label: &str, solved: usize, total: usize) |  |  |  |  |

| delta_cell | function | delta_cell(
    prev: Option<&ComparablePredecessor<'_>>,
    label: &str,
    solved: usize,
    total: usize,
) |  |  |  |  |

| from_json | function | from_json(src: &str) |  |  |  |  |

| load | function | load(path: &Path) |  |  |  |  |

| new | function | new(s: impl Into<String>) |  |  |  |  |

| parse | function | parse(s: &str) |  |  |  |  |

| parts | function | parts(&self) |  |  |  |  |

| snapshots | function | snapshots(&self) |  |  |  |  |

| to_json | function | to_json(&self) |  |  |  |  |

| track | function | track(&self, label: &str) |  |  |  |  |

| trend | function | trend(&self, label: &str, box_: &BoxId) |  |  |  |  |

| try_load | function | try_load(path: &Path) |  |  |  |  |

| upsert | function | upsert(&mut self, s: Snapshot) |  |  |  |  |

| version | function | version(&self) |  |  |  |  |

| version_key | function | version_key(&self) |  |  |  |  |

| BoxId | struct |  |  |  |  |  |

| ComparablePredecessor | struct |  |  |  |  |  |

| History | struct |  |  |  |  |  |

| MeasuredAt | struct |  |  |  |  |  |

| Snapshot | struct |  |  |  |  |  |

| Trend | struct |  |  |  |  |  |

| VersionKey | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/lib.rs

| parse_rows | function | parse_rows(src: &str, path: &str) |  |  |  |  |


### crucible/crates/crucible-publish/src/manifest.rs

| ManifestError | enum |  |  |  |  |  |

| PatternError | enum |  |  |  |  |  |

| board | function | board(&self, id: &str) |  |  |  |  |

| board_by_label | function | board_by_label(&self, label: &str) |  |  |  |  |

| board_by_raw | function | board_by_raw(&self, raw: &str) |  |  |  |  |

| errors | function | errors(&self) |  |  |  |  |

| is_match | function | is_match(&self, variant: &str) |  |  |  |  |

| is_proof_track | function | is_proof_track(&self, label: &str) |  |  |  |  |

| load | function | load(path: &Path) |  |  |  |  |

| parse | function | parse(src: &str) |  |  |  |  |

| rebaselined_on | function | rebaselined_on(&self, label: &str, box_: &str) |  |  |  |  |

| search | function | search(&self, hay: &str) |  |  |  |  |

| selector | function | selector(&self) |  |  |  |  |

| selects | function | selects(&self, variant: &str) |  |  |  |  |

| set | function | set(&self, name: &str) |  |  |  |  |

| track | function | track(&self, name: &str) |  |  |  |  |

| validate | function | validate(&self) |  |  |  |  |

| warnings | function | warnings(&self) |  |  |  |  |

| BoardSpec | struct |  |  |  |  |  |

| CorpusSpec | struct |  |  |  |  |  |

| Defaults | struct |  |  |  |  |  |

| Manifest | struct |  |  |  |  |  |

| Pattern | struct |  |  |  |  |  |

| Selector | struct |  |  |  |  |  |

| SetSpec | struct |  |  |  |  |  |

| TrackSpec | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/promote.rs

| Gate | enum |  |  |  |  |  |

| PromoteError | enum |  |  |  |  |  |

| TierMovePolicy | enum |  |  |  |  |  |

| accepts | function | accepts(&self, board: &str) |  |  |  |  |

| against | function | against(&self) |  |  |  |  |

| apply | function | apply(&self) |  |  |  |  |

| board | function | board(&self) |  |  |  |  |

| boards | function | boards(&self) |  |  |  |  |

| bytes | function | bytes(&self) |  |  |  |  |

| changes | function | changes(&self) |  |  |  |  |

| compute | function | compute(
        box_: &str,
        prev: Option<&ComparablePredecessor<'_>>,
        live: &[LiveBoard],
    ) |  |  |  |  |

| denominator_grew | function | denominator_grew(&self) |  |  |  |  |

| dst | function | dst(&self) |  |  |  |  |

| entries | function | entries(&self) |  |  |  |  |

| failures | function | failures(&self) |  |  |  |  |

| full_table | function | full_table(&self) |  |  |  |  |

| like_for_like | function | like_for_like(&self) |  |  |  |  |

| pct | function | pct(&self) |  |  |  |  |

| pct_dropped_on_entry_day | function | pct_dropped_on_entry_day(&self) |  |  |  |  |

| placements | function | placements(&self) |  |  |  |  |

| plan | function | plan(
    root: &Path,
    manifest: &Manifest,
    sets: &[&str],
    policy: &TierMovePolicy,
) |  |  |  |  |

| promoted_lines | function | promoted_lines(&self) |  |  |  |  |

| refusal | function | refusal(&self) |  |  |  |  |

| sentence | function | sentence(&self, box_: &str) |  |  |  |  |

| solved | function | solved(&self) |  |  |  |  |

| src | function | src(&self) |  |  |  |  |

| tier_moves | function | tier_moves(&self) |  |  |  |  |

| total | function | total(&self) |  |  |  |  |

| Change | struct |  |  |  |  |  |

| GateReport | struct |  |  |  |  |  |

| Headline | struct |  |  |  |  |  |

| LiveBoard | struct |  |  |  |  |  |

| Placement | struct |  |  |  |  |  |

| Promotion | struct |  |  |  |  |  |

| TierMove | struct |  |  |  |  |  |

| TwoHeadlines | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/pyjson.rs

| write_indent1 | function | write_indent1(v: &serde_json::Value, out: &mut String) |  |  |  |  |

| write_str | function | write_str(s: &str, out: &mut String) |  |  |  |  |

| write_value | function | write_value(v: &serde_json::Value, out: &mut String) |  |  |  |  |


### crucible/crates/crucible-publish/src/quality.rs

| Currency | enum |  |  |  |  |  |

| QualityNote | enum |  |  |  |  |  |

| bounds_wtl | function | bounds_wtl(
    rows: &[RawRow],
    referee: &Referee,
    bounds: &BestKnownBounds,
    year_key: &str,
    variant_suffix: &str,
) |  |  |  |  |

| currency | function | currency(&self) |  |  |  |  |

| l | function | l(&self) |  |  |  |  |

| length_wtl | function | length_wtl(rows: &[RawRow], referee: &Referee, arch: &Ipc5Archive) |  |  |  |  |

| makespan_wtl | function | makespan_wtl(rows: &[RawRow], referee: &Referee, arch: &Ipc5Archive) |  |  |  |  |

| mean | function | mean(&self) |  |  |  |  |

| n | function | n(&self) |  |  |  |  |

| new | function | new(scored: Option<Wtl>, fallback: impl Into<String>) |  |  |  |  |

| prefix | function | prefix(self) |  |  |  |  |

| render | function | render(&self) |  |  |  |  |

| t | function | t(&self) |  |  |  |  |

| w | function | w(&self) |  |  |  |  |

| Wtl | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/raw.rs

| Instance | enum |  |  |  |  |  |

| Notes | enum |  |  |  |  |  |

| as_num | function | as_num(&self) |  |  |  |  |

| current | function | current(solved: bool) |  |  |  |  |

| domain_key | function | domain_key(&self) |  |  |  |  |

| note_text | function | note_text(&self) |  |  |  |  |

| of | function | of(o: &serde_json::Map<String, serde_json::Value>) |  |  |  |  |

| text | function | text(&self) |  |  |  |  |

| time_secs | function | time_secs(&self) |  |  |  |  |

| write_row | function | write_row(r: &RawRow, out: &mut String) |  |  |  |  |

| Present | struct |  |  |  |  |  |

| RawRow | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/referee.rs

| budget_for | function | budget_for(&self, r: &RawRow, registry: f64) |  |  |  |  |

| classify | function | classify(&self, r: &RawRow, registry_budget: f64) |  |  |  |  |

| contains | function | contains(&self, r: &RawRow) |  |  |  |  |

| coverage | function | coverage(&self, rows: &[RawRow], registry_budget: f64) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| is_solved | function | is_solved(&self, r: &RawRow) |  |  |  |  |

| new | function | new(val_unavailable: ValUnavailable) |  |  |  |  |

| Referee | struct |  |  |  |  |  |

| ValUnavailable | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/render/detail.rs

| render | function | render(ctx: &RenderCtx) |  |  |  |  |


### crucible/crates/crucible-publish/src/render/mod.rs

| CtxError | enum |  |  |  |  |  |

| absent_cell | function | absent_cell(&self, label: &str) |  |  |  |  |

| board | function | board(&self, label: &str) |  |  |  |  |

| coverage_of | function | coverage_of(&self, rows: &[RawRow], budget: f64) |  |  |  |  |

| data | function | data(&self, label: &str) |  |  |  |  |

| load | function | load(root: &Path, box_id: BoxId) |  |  |  |  |

| new | function | new(
        manifest: Manifest,
        boards: Vec<BoardRows>,
        referee: Referee,
        archive: Ipc5Archive,
        bounds: BestKnownBounds,
        field: FieldBook,
        history: History,
        version: Option<String>,
        box_id: BoxId,
    ) |  |  |  |  |

| predecessor | function | predecessor(&self) |  |  |  |  |

| proof_mark | function | proof_mark(&self, label: &str) |  |  |  |  |

| split | function | split(&self, label: &str, ipc: &str) |  |  |  |  |

| standings | function | standings(&self) |  |  |  |  |

| BoardRows | struct |  |  |  |  |  |

| LiveBoard | struct |  |  |  |  |  |

| RenderCtx | struct |  |  |  |  |  |

| Standings | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/src/render/readme.rs

| block | function | block(ctx: &RenderCtx) |  |  |  |  |

| patch | function | patch(readme_text: &str, block: &str) |  |  |  |  |


### crucible/crates/crucible-publish/src/render/summary.rs

| render | function | render(ctx: &RenderCtx) |  |  |  |  |


### crucible/crates/crucible-publish/src/snapshot.rs

| SnapshotError | enum |  |  |  |  |  |

| Source | enum |  |  |  |  |  |

| bank | function | bank(
    root: &Path,
    manifest: &Manifest,
    referee: &Referee,
    args: &Args,
) |  |  |  |  |

| parse_args | function | parse_args(argv: &[String], env_box: Option<&str>) |  |  |  |  |

| tracks | function | tracks(
    root: &Path,
    manifest: &Manifest,
    referee: &Referee,
    source: &Source,
) |  |  |  |  |

| write | function | write(&self) |  |  |  |  |

| Args | struct |  |  |  |  |  |

| Banked | struct |  |  |  |  |  |


### crucible/crates/crucible-publish/tests/common/mod.rs

| incident | function | incident(name: &str) |  |  |  |  |

| real_val_map | function | real_val_map() |  |  |  |  |


### crucible/crates/crucible/src/backfill.rs

| run | function | run(repo: &Path, cfg: &crate::config::Config, o: Opts<'_>) |  |  |  |  |

| stage_for | function | stage_for(ver: &str) |  |  |  |  |

| Opts | struct |  |  |  |  |  |


### crucible/crates/crucible/src/config.rs

| in_quiet_hours | function | in_quiet_hours(&self, minutes_past_midnight: u32) |  |  |  |  |

| load | function | load(path: &std::path::Path) |  |  |  |  |

| path | function | path() |  |  |  |  |

| Config | struct |  |  |  |  |  |

| Contention | struct |  |  |  |  |  |

| Db | struct |  |  |  |  |  |

| QuietHours | struct |  |  |  |  |  |

| Referee | struct |  |  |  |  |  |

| Repo | struct |  |  |  |  |  |

| Scheduler | struct |  |  |  |  |  |

| Sweep | struct |  |  |  |  |  |

| Ui | struct |  |  |  |  |  |


### crucible/crates/crucible/src/monitor.rs

| compare | function | compare(
    repo: &Path,
    cfg: &crate::config::Config,
    set_name: &str,
    a: &str,
    b: &str,
    select: &crate::select::Select,
    lost: Option<&Path>,
) |  |  |  |  |

| frame | function | frame(repo: &Path, cfg: &crate::config::Config, set_name: &str) |  |  |  |  |

| run | function | run(repo: &Path, cfg: &crate::config::Config, set_name: &str) |  |  |  |  |

| status | function | status(
    repo: &Path,
    cfg: &crate::config::Config,
    set_name: &str,
    json: bool,
) |  |  |  |  |


### crucible/crates/crucible/src/out.rs

| flush_to_stdout | function | flush_to_stdout() |  |  |  |  |

| is_quiet | function | is_quiet() |  |  |  |  |

| quiet | function | quiet(on: bool) |  |  |  |  |

| recent | function | recent(n: usize) |  |  |  |  |

| say | function | say(line: String) |  |  |  |  |

| tz_offset_secs | function | tz_offset_secs() |  |  |  |  |


### crucible/crates/crucible/src/repo.rs

| RepoError | enum |  |  |  |  |  |

| build_planner | function | build_planner(dir: &Path) |  |  |  |  |

| candidate_path | function | candidate_path(repo: &Path) |  |  |  |  |

| probe | function | probe(path: &Path) |  |  |  |  |

| require_version | function | require_version(&self, want: &str) |  |  |  |  |

| short_hash | function | short_hash(&self) |  |  |  |  |

| supports_mode | function | supports_mode(&self, mode: &str) |  |  |  |  |

| worktree_for | function | worktree_for(worktree_dir: &Path, tag: &str) |  |  |  |  |

| Engine | struct |  |  |  |  |  |


### crucible/crates/crucible/src/resident.rs

| run | function | run(repo: &Path, cfg: &Config, o: Opts<'_>) |  |  |  |  |

| tags | function | tags(repo: &Path) |  |  |  |  |

| Opts | struct |  |  |  |  |  |


### crucible/crates/crucible/src/select.rs

| Prior | enum |  |  |  |  |  |

| admits | function | admits(
        &self,
        variant: &str,
        label: &str,
        prior: &BTreeMap<String, (bool, Option<f64>) |  |  |  |  |

| describe | function | describe(&self) |  |  |  |  |

| from_args | function | from_args(a: &SelectArgs) |  |  |  |  |

| is_subset | function | is_subset(&self) |  |  |  |  |

| needs_prior | function | needs_prior(&self) |  |  |  |  |

| parse_rows | function | parse_rows(src: &str) |  |  |  |  |

| stage | function | stage(&self, repo: &Path, set: &str, engine_ver: &str, engine_hash: &str) |  |  |  |  |

| wants_board | function | wants_board(&self, id: &str) |  |  |  |  |

| Select | struct |  |  |  |  |  |

| SelectArgs | struct |  |  |  |  |  |


### crucible/crates/crucible/src/sweep.rs

| attach | function | attach(&self, tx: mpsc::Sender<Ctl>) |  |  |  |  |

| attached | function | attached(&self) |  |  |  |  |

| calibrate | function | calibrate(
        &mut self,
        prior: Option<f64>,
        record: &dyn Fn(f64) |  |  |  |  |

| canary | function | canary(&self) |  |  |  |  |

| ctl_for | function | ctl_for(from: Level, to: Level, demote_ok: bool) |  |  |  |  |

| demote_ok | function | demote_ok(&self) |  |  |  |  |

| detach | function | detach(&self, id: u64) |  |  |  |  |

| from_config | function | from_config(c: &crate::config::Scheduler) |  |  |  |  |

| held | function | held(&self) |  |  |  |  |

| hold | function | hold(&self, on: bool) |  |  |  |  |

| label | function | label(&self) |  |  |  |  |

| level | function | level(&self) |  |  |  |  |

| new | function | new(manifest: &'a Manifest, setup: Setup<'_>) |  |  |  |  |

| policy_mem_budget | function | policy_mem_budget(mem_bytes: u64, at_the_box: bool, pack: &Pack) |  |  |  |  |

| policy_width | function | policy_width(
    level: Level,
    quiet_hours: bool,
    user_idle_secs: Option<f64>,
    foreign_pcpu: f64,
    pack: &Pack,
) |  |  |  |  |

| read | function | read(&mut self, on_spawn: Option<&dyn Fn(Pid, f64) |  |  |  |  |

| reason | function | reason(&self) |  |  |  |  |

| resolve | function | resolve(
        repo: &Path,
        engine: &Path,
        engine_hash: &str,
        r: &crate::config::Referee,
    ) |  |  |  |  |

| run | function | run(repo: &Path, cfg: &crate::config::Config, o: Opts<'_>) |  |  |  |  |

| run_engine | function | run_engine(
    repo: &Path,
    cfg: &crate::config::Config,
    o: Opts<'_>,
    manifest: &Manifest,
    engine: crate::repo::Engine,
    stage: Option<PathBuf>,
) |  |  |  |  |

| send | function | send(&self, c: Ctl) |  |  |  |  |

| set_canary | function | set_canary(&self, factor: f64) |  |  |  |  |

| set_demote_ok | function | set_demote_ok(&self, on: bool) |  |  |  |  |

| set_level | function | set_level(&self, level: Level, reason: Option<String>) |  |  |  |  |

| set_width | function | set_width(&self, w: usize) |  |  |  |  |

| solo | function | solo() |  |  |  |  |

| throttle_config | function | throttle_config(c: &crate::config::Contention) |  |  |  |  |

| total_instances | function | total_instances(&self) |  |  |  |  |

| width | function | width(&self) |  |  |  |  |

| Canary | struct |  |  |  |  |  |

| DbCtx | struct |  |  |  |  |  |

| Opts | struct |  |  |  |  |  |

| Pack | struct |  |  |  |  |  |

| Progress | struct |  |  |  |  |  |

| Running | struct |  |  |  |  |  |

| Setup | struct |  |  |  |  |  |

| Shared | struct |  |  |  |  |  |

| SweepRunner | struct |  |  |  |  |  |


### crucible/crates/crucible/src/tui/app.rs

| Cell | enum |  |  |  |  |  |

| Level | enum |  |  |  |  |  |

| LogKind | enum |  |  |  |  |  |

| Sort | enum |  |  |  |  |  |

| View | enum |  |  |  |  |  |

| back | function | back(&mut self) |  |  |  |  |

| banked | function | banked(self) |  |  |  |  |

| board | function | board(&self) |  |  |  |  |

| cycle_sort | function | cycle_sort(&mut self) |  |  |  |  |

| delta_secs | function | delta_secs(&self) |  |  |  |  |

| dismiss_toasts | function | dismiss_toasts(&mut self) |  |  |  |  |

| done | function | done(&self) |  |  |  |  |

| enter | function | enter(&mut self) |  |  |  |  |

| expire_toasts | function | expire_toasts(&mut self, dwell: Duration) |  |  |  |  |

| frac | function | frac(&self) |  |  |  |  |

| gain | function | gain(&self) |  |  |  |  |

| gains | function | gains(&self) |  |  |  |  |

| jump | function | jump(&mut self, to_end: bool) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| move_selection | function | move_selection(&mut self, delta: isize) |  |  |  |  |

| near_wall | function | near_wall(&self) |  |  |  |  |

| next | function | next(self) |  |  |  |  |

| owed | function | owed(&self) |  |  |  |  |

| prev_solved | function | prev_solved(&self) |  |  |  |  |

| rank | function | rank(self) |  |  |  |  |

| regression | function | regression(&self) |  |  |  |  |

| regressions | function | regressions(&self) |  |  |  |  |

| rho | function | rho(&self) |  |  |  |  |

| running | function | running(&self) |  |  |  |  |

| selected_instance | function | selected_instance(&self) |  |  |  |  |

| solve_secs | function | solve_secs(&self) |  |  |  |  |

| solved | function | solved(&self) |  |  |  |  |

| sorted_instances | function | sorted_instances(&self) |  |  |  |  |

| strip | function | strip(cells: &[InstanceCell], width: usize) |  |  |  |  |

| tally | function | tally(&mut self) |  |  |  |  |

| toggle_timeline | function | toggle_timeline(&mut self) |  |  |  |  |

| total | function | total(&self) |  |  |  |  |

| AttemptRow | struct |  |  |  |  |  |

| BoardRow | struct |  |  |  |  |  |

| InstanceCell | struct |  |  |  |  |  |

| InstanceDetail | struct |  |  |  |  |  |

| LevelState | struct |  |  |  |  |  |

| LogLine | struct |  |  |  |  |  |

| Slot | struct |  |  |  |  |  |

| SlotRun | struct |  |  |  |  |  |

| Snapshot | struct |  |  |  |  |  |

| StripCol | struct |  |  |  |  |  |

| SweepProgress | struct |  |  |  |  |  |

| Timeline | struct |  |  |  |  |  |

| TimelinePoint | struct |  |  |  |  |  |

| Toast | struct |  |  |  |  |  |


### crucible/crates/crucible/src/tui/banner.rs

| compact | function | compact(text: &str) |  |  |  |  |

| render | function | render(text: &str, max_width: usize) |  |  |  |  |


### crucible/crates/crucible/src/tui/demo.rs

| detail | function | detail() |  |  |  |  |

| snapshot | function | snapshot(t: f64) |  |  |  |  |


### crucible/crates/crucible/src/tui/draw.rs

| draw | function | draw(f: &mut Frame, s: &Snapshot, th: &Theme, banner_text: &str) |  |  |  |  |


### crucible/crates/crucible/src/tui/feed.rs

| new | function | new(progress: Arc<Mutex<Progress>>, shared: Arc<Shared>) |  |  |  |  |

| next | function | next(&mut self, prev: &Snapshot) |  |  |  |  |

| Feed | struct |  |  |  |  |  |


### crucible/crates/crucible/src/tui/run.rs

| Action | enum |  |  |  |  |  |

| action_for | function | action_for(k: KeyEvent, s: &mut Snapshot) |  |  |  |  |

| enter | function | enter() |  |  |  |  |

| TerminalGuard | struct |  |  |  |  |  |


### crucible/crates/crucible/src/tui/theme.rs

| Depth | enum |  |  |  |  |  |

| bar_cells | function | bar_cells(&self) |  |  |  |  |

| detect | function | detect() |  |  |  |  |

| forge | function | forge() |  |  |  |  |

| glyph | function | glyph(&self, unicode: &'static str, ascii: &'static str) |  |  |  |  |

| spark_cells | function | spark_cells(&self) |  |  |  |  |

| unicode_ok | function | unicode_ok() |  |  |  |  |

| Theme | struct |  |  |  |  |  |


### crucible/crates/crucible/src/tui/widget.rs

| bar | function | bar(theme: &Theme, frac: f64, width: usize) |  |  |  |  |

| duration | function | duration(secs: u64) |  |  |  |  |

| ellipsize | function | ellipsize(s: &str, width: usize) |  |  |  |  |

| spark | function | spark(theme: &Theme, values: &[f64], width: usize) |  |  |  |  |

| until | function | until(secs: u64) |  |  |  |  |



<!-- AGENT-FORBIDDEN-END -->

## Signature/type/default/errors table

<!-- RIGID table: header order is fixed; rows come only from the query. -->

| Item | Type | Signature | Params | Defaults | Errors | Invariants |
|------|------|-----------|--------|----------|--------|------------|

| advance | function | advance(time: Res<Time>, mut plan: ResMut<Plan>) |  |  |  |  |

| animate | function | animate(
    plan: Res<Plan>,
    scene: Res<Scene>,
    nodes: Query<(&NodeObj, &Transform) |  |  |  |  |

| controls | function | controls(
    keys: Res<ButtonInput<KeyCode>>,
    scene: Res<Scene>,
    editor: Res<crate::blocks::Editor>,
    mut plan: ResMut<Plan>,
    mut job: ResMut<SolveJob>,
) |  |  |  |  |

| frac | function | frac(&self) |  |  |  |  |

| poll_solve | function | poll_solve(mut job: ResMut<SolveJob>, mut plan: ResMut<Plan>) |  |  |  |  |

| span | function | span(&self) |  |  |  |  |

| start_frac | function | start_frac(&self, step: &Step, idx: usize) |  |  |  |  |

| Plan | struct |  |  |  |  |  |

| SolveJob | struct |  |  |  |  |  |

| Act | enum |  |  |  |  |  |

| DragKind | enum |  |  |  |  |  |

| Focus | enum |  |  |  |  |  |

| LitLoc | enum |  |  |  |  |  |

| Mode | enum |  |  |  |  |  |

| Zone | enum |  |  |  |  |  |

| editor_drag | function | editor_drag(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut drag: ResMut<Drag>,
    mut editor: ResMut<Editor>,
    grips: Query<(&DragKind, &RelativeCursorPosition) |  |  |  |  |

| handle_clicks | function | handle_clicks(
    interactions: Query<(&Interaction, &Act) |  |  |  |  |

| rebuild | function | rebuild(
    mut commands: Commands,
    mut editor: ResMut<Editor>,
    roots: Query<Entity, With<EditorRoot>>,
) |  |  |  |  |

| scroll_editor | function | scroll_editor(
    mut wheel: MessageReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    editor: Res<Editor>,
    mut q: Query<&mut ScrollPosition, With<EditorRoot>>,
) |  |  |  |  |

| text_input | function | text_input(mut evr: MessageReader<KeyboardInput>, mut editor: ResMut<Editor>) |  |  |  |  |

| toggle_editor | function | toggle_editor(
    keys: Res<ButtonInput<KeyCode>>,
    scene: Res<Scene>,
    mut editor: ResMut<Editor>,
) |  |  |  |  |

| Drag | struct |  |  |  |  |  |

| Editor | struct |  |  |  |  |  |

| EditorRoot | struct |  |  |  |  |  |

| Ghost | struct |  |  |  |  |  |

| gantt_now | function | gantt_now(plan: Res<Plan>, mut now: Query<&mut Node, With<GanttNow>>) |  |  |  |  |

| gantt_visibility | function | gantt_visibility(
    plan: Res<Plan>,
    state: Res<GanttState>,
    editor: Res<crate::blocks::Editor>,
    mut panel: Query<&mut Visibility, With<GanttPanel>>,
) |  |  |  |  |

| rebuild_gantt | function | rebuild_gantt(
    mut commands: Commands,
    plan: Res<Plan>,
    mut state: ResMut<GanttState>,
    track: Query<Entity, With<GanttTrack>>,
    bars: Query<Entity, With<GanttBar>>,
) |  |  |  |  |

| setup_gantt | function | setup_gantt(mut commands: Commands) |  |  |  |  |

| toggle_gantt | function | toggle_gantt(
    keys: Res<ButtonInput<KeyCode>>,
    editor: Res<crate::blocks::Editor>,
    mut state: ResMut<GanttState>,
) |  |  |  |  |

| GanttBar | struct |  |  |  |  |  |

| GanttNow | struct |  |  |  |  |  |

| GanttPanel | struct |  |  |  |  |  |

| GanttState | struct |  |  |  |  |  |

| GanttTrack | struct |  |  |  |  |  |

| IconShape | enum |  |  |  |  |  |

| color_for | function | color_for(ty: &str) |  |  |  |  |

| mat_handle | function | mat_handle(
    materials: &mut Assets<ColorMaterial>,
    cache: &mut MatCache,
    color: Color,
) |  |  |  |  |

| mesh_handle | function | mesh_handle(
    meshes: &mut Assets<Mesh>,
    cache: &mut MeshCache,
    shape: IconShape,
    size: f32,
) |  |  |  |  |

| shape_for | function | shape_for(ty: &str) |  |  |  |  |

| draw_selection | function | draw_selection(
    mut gizmos: Gizmos,
    selected: Res<Selected>,
    nodes: Query<(&NodeObj, &Transform) |  |  |  |  |

| interact | function | interact(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    editor: Res<crate::blocks::Editor>,
    cam_q: Query<(&Camera, &GlobalTransform) |  |  |  |  |

| DragState | struct |  |  |  |  |  |

| Selected | struct |  |  |  |  |  |

| camera_nav | function | camera_nav(
    mouse: Res<ButtonInput<MouseButton>>,
    editor: Res<crate::blocks::Editor>,
    mut motion: MessageReader<bevy::input::mouse::MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    mut cam: Query<(&mut Transform, &mut Projection) |  |  |  |  |

| draw_edges | function | draw_edges(
    mut gizmos: Gizmos,
    scene: Res<Scene>,
    plan: Res<crate::anim::Plan>,
    nodes: Query<(&NodeObj, &Transform) |  |  |  |  |

| handle_drops | function | handle_drops(mut drops: MessageReader<FileDragAndDrop>, mut scene: ResMut<Scene>) |  |  |  |  |

| load_src | function | load_src(&mut self, src: &str) |  |  |  |  |

| respawn_graph | function | respawn_graph(
    mut commands: Commands,
    mut scene: ResMut<Scene>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing: Query<Entity, With<GraphItem>>,
) |  |  |  |  |

| setup | function | setup(mut commands: Commands) |  |  |  |  |

| FanOffset | struct |  |  |  |  |  |

| GraphItem | struct |  |  |  |  |  |

| MainCamera | struct |  |  |  |  |  |

| MobileObj | struct |  |  |  |  |  |

| NodeObj | struct |  |  |  |  |  |

| Scene | struct |  |  |  |  |  |

| rebuild_notches | function | rebuild_notches(
    mut commands: Commands,
    plan: Res<Plan>,
    mut state: ResMut<Transport>,
    track: Query<Entity, With<ScrubTrack>>,
    notches: Query<Entity, With<StepNotch>>,
) |  |  |  |  |

| setup_transport | function | setup_transport(mut commands: Commands) |  |  |  |  |

| transport_input | function | transport_input(
    mouse: Res<ButtonInput<MouseButton>>,
    mut transport: ResMut<Transport>,
    mut plan: ResMut<Plan>,
    play_btn: Query<&Interaction, (With<PlayButton>, Changed<Interaction>) |  |  |  |  |

| transport_sync | function | transport_sync(
    plan: Res<Plan>,
    mut fill: Query<&mut Node, (With<ScrubFill>, Without<Playhead>) |  |  |  |  |

| transport_visibility | function | transport_visibility(plan: Res<Plan>, mut bar: Query<&mut Visibility, With<TransportBar>>) |  |  |  |  |

| PlayButton | struct |  |  |  |  |  |

| PlayIcon | struct |  |  |  |  |  |

| Playhead | struct |  |  |  |  |  |

| ScrubFill | struct |  |  |  |  |  |

| ScrubTrack | struct |  |  |  |  |  |

| StepNotch | struct |  |  |  |  |  |

| Transport | struct |  |  |  |  |  |

| TransportBar | struct |  |  |  |  |  |

| TransportLabel | struct |  |  |  |  |  |

| setup_ui | function | setup_ui(mut commands: Commands) |  |  |  |  |

| update_info | function | update_info(
    scene: Res<Scene>,
    selected: Res<Selected>,
    plan: Res<Plan>,
    mut q: Query<&mut Text, With<InfoText>>,
) |  |  |  |  |

| InfoText | struct |  |  |  |  |  |

| ModeArg | enum |  |  |  |  |  |

| ObjectiveArg | enum |  |  |  |  |  |

| SearchArg | enum |  |  |  |  |  |

| compile_pack | function | compile_pack(pack: &ObservationPack, output_dir: &Path) |  |  |  |  |

| replay_pack | function | replay_pack(
    pack: &ObservationPack,
    expected: &HarvestReceipt,
    output_dir: &Path,
) |  |  |  |  |

| extract_operators | function | extract_operators(report: &AdmissionReport) |  |  |  |  |

| collect_with_gh | function | collect_with_gh(
    repositories: &[String],
    window: ObservationWindow,
    max_pages: usize,
) |  |  |  |  |

| admit | function | admit(pack: &ObservationPack) |  |  |  |  |

| digest_bytes | function | digest_bytes(bytes: &[u8]) |  |  |  |  |

| load_observation_pack | function | load_observation_pack(path: &Path) |  |  |  |  |

| load_receipt | function | load_receipt(path: &Path) |  |  |  |  |

| receipt_exit_code | function | receipt_exit_code(receipt: &HarvestReceipt) |  |  |  |  |

| save_observation_pack | function | save_observation_pack(path: &Path, pack: &ObservationPack) |  |  |  |  |

| validate_pack | function | validate_pack(pack: &ObservationPack) |  |  |  |  |

| validate_window | function | validate_window(window: &ObservationWindow) |  |  |  |  |

| ActuationClass | enum |  |  |  |  |  |

| AdmissionLevel | enum |  |  |  |  |  |

| ExecutionResult | enum |  |  |  |  |  |

| FinalState | enum |  |  |  |  |  |

| GallCheckpoint | enum |  |  |  |  |  |

| RefusalCode | enum |  |  |  |  |  |

| ReplayState | enum |  |  |  |  |  |

| AdmissionReport | struct |  |  |  |  |  |

| AdmittedWork | struct |  |  |  |  |  |

| ArtifactEvidence | struct |  |  |  |  |  |

| EvidenceRef | struct |  |  |  |  |  |

| ExcludedWork | struct |  |  |  |  |  |

| ExecutionEvidence | struct |  |  |  |  |  |

| HarvestReceipt | struct |  |  |  |  |  |

| MethodCatalog | struct |  |  |  |  |  |

| ObservationPack | struct |  |  |  |  |  |

| ObservationWindow | struct |  |  |  |  |  |

| ObservedOutcome | struct |  |  |  |  |  |

| ObservedWorkItem | struct |  |  |  |  |  |

| OperatorOutcome | struct |  |  |  |  |  |

| OutputArtifact | struct |  |  |  |  |  |

| PlanningOperator | struct |  |  |  |  |  |

| SourceRevision | struct |  |  |  |  |  |

| TransportFailure | struct |  |  |  |  |  |

| ValidationRecord | struct |  |  |  |  |  |

| ValidationSummary | struct |  |  |  |  |  |

| validate_pair | function | validate_pair(domain_src: &str, problem_src: &str) |  |  |  |  |

| validate_paths | function | validate_paths(domain_path: &str, problem_path: &str) |  |  |  |  |

| Summary | struct |  |  |  |  |  |

| Effect | enum |  |  |  |  |  |

| GoalDesc | enum |  |  |  |  |  |

| Literal | enum |  |  |  |  |  |

| NumericValue | enum |  |  |  |  |  |

| Term | enum |  |  |  |  |  |

| ActionDef | struct |  |  |  |  |  |

| AtomicFormula | struct |  |  |  |  |  |

| ConstraintDef | struct |  |  |  |  |  |

| Domain | struct |  |  |  |  |  |

| MethodDef | struct |  |  |  |  |  |

| NumericFluentDecl | struct |  |  |  |  |  |

| OrderEdge | struct |  |  |  |  |  |

| PredicateDef | struct |  |  |  |  |  |

| Problem | struct |  |  |  |  |  |

| Subtask | struct |  |  |  |  |  |

| TaskCall | struct |  |  |  |  |  |

| TaskDef | struct |  |  |  |  |  |

| TaskNetwork | struct |  |  |  |  |  |

| TypeDef | struct |  |  |  |  |  |

| TypedObject | struct |  |  |  |  |  |

| TypedParam | struct |  |  |  |  |  |

| GroundError | enum |  |  |  |  |  |

| GroundGoal | enum |  |  |  |  |  |

| action_applicable | function | action_applicable(
    action: &GroundAction,
    facts: &BTreeSet<String>,
) |  |  |  |  |

| build_type_closure | function | build_type_closure(
    domain: &Domain,
) |  |  |  |  |

| compute_reachability | function | compute_reachability(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    initial_facts: &BTreeSet<String>,
    limits: &GroundingLimits,
) |  |  |  |  |

| compute_task_relevance | function | compute_task_relevance(
    domain: &Domain,
    problem: &Problem,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
) |  |  |  |  |

| evaluate_ground_goal | function | evaluate_ground_goal(
    goal: &GroundGoal,
    facts: &BTreeSet<String>,
) |  |  |  |  |

| evaluate_ground_goal_with_budget | function | evaluate_ground_goal_with_budget(
    goal: &GroundGoal,
    facts: &BTreeSet<String>,
    budget: usize,
) |  |  |  |  |

| ground | function | ground(
    domain: &Domain,
    problem: &Problem,
    limits: &GroundingLimits,
) |  |  |  |  |

| ground_actions | function | ground_actions(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
) |  |  |  |  |

| ground_actions_reachable | function | ground_actions_reachable(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    reachability: &ReachabilityInfo,
) |  |  |  |  |

| ground_actions_relevant | function | ground_actions_relevant(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    relevance: &TaskRelevanceInfo,
    reachability: Option<&ReachabilityInfo>,
) |  |  |  |  |

| ground_initial_facts | function | ground_initial_facts(problem: &Problem) |  |  |  |  |

| ground_methods | function | ground_methods(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
) |  |  |  |  |

| ground_methods_reachable | function | ground_methods_reachable(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    reachability: &ReachabilityInfo,
) |  |  |  |  |

| ground_methods_relevant | function | ground_methods_relevant(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    relevance: &TaskRelevanceInfo,
    reachability: Option<&ReachabilityInfo>,
) |  |  |  |  |

| ground_root_network | function | ground_root_network(
    problem: &Problem,
    objects_by_type: &BTreeMap<String, Vec<String>>,
) |  |  |  |  |

| index_objects_by_type | function | index_objects_by_type(
    domain: &Domain,
    problem: &Problem,
    closure: &BTreeMap<String, BTreeSet<String>>,
) |  |  |  |  |

| GroundAction | struct |  |  |  |  |  |

| GroundConditional | struct |  |  |  |  |  |

| GroundEffectBranch | struct |  |  |  |  |  |

| GroundMethod | struct |  |  |  |  |  |

| GroundRootNetwork | struct |  |  |  |  |  |

| GroundSubtask | struct |  |  |  |  |  |

| GroundedIR | struct |  |  |  |  |  |

| GroundingLimits | struct |  |  |  |  |  |

| ReachabilityInfo | struct |  |  |  |  |  |

| TaskRelevanceInfo | struct |  |  |  |  |  |

| ParseError | enum |  |  |  |  |  |

| parse_domain | function | parse_domain(src: &str) |  |  |  |  |

| parse_domain_with_budget | function | parse_domain_with_budget(src: &str, max_depth: usize) |  |  |  |  |

| parse_problem | function | parse_problem(src: &str) |  |  |  |  |

| parse_problem_with_budget | function | parse_problem_with_budget(src: &str, max_depth: usize) |  |  |  |  |

| has_probabilistic | function | has_probabilistic(src: &str) |  |  |  |  |

| preprocess | function | preprocess(src: &str) |  |  |  |  |

| TranslateError | enum |  |  |  |  |  |

| translate | function | translate(
    ir: &GroundedIR,
    limits: &TranslateLimits,
) |  |  |  |  |

| Goal | struct |  |  |  |  |  |

| Method | struct |  |  |  |  |  |

| PlanningProblem | struct |  |  |  |  |  |

| State | struct |  |  |  |  |  |

| Task | struct |  |  |  |  |  |

| Transition | struct |  |  |  |  |  |

| TranslateLimits | struct |  |  |  |  |  |

| DuplicateKind | enum |  |  |  |  |  |

| ValidationError | enum |  |  |  |  |  |

| ValidationWarning | enum |  |  |  |  |  |

| validate_domain | function | validate_domain(domain: &Domain) |  |  |  |  |

| validate_domain_with_warnings | function | validate_domain_with_warnings(
    domain: &Domain,
) |  |  |  |  |

| validate_problem | function | validate_problem(domain: &Domain, problem: &Problem) |  |  |  |  |

| validate_problem_with_warnings | function | validate_problem_with_warnings(
    domain: &Domain,
    problem: &Problem,
) |  |  |  |  |

| call | function | call(&mut self, tool: &str, args: Value) |  |  |  |  |

| call_json | function | call_json(&mut self, tool: &str, args: Value) |  |  |  |  |

| call_text | function | call_text(&mut self, tool: &str, args: Value) |  |  |  |  |

| finish | function | finish(mut self) |  |  |  |  |

| notify | function | notify(&mut self, method: &str) |  |  |  |  |

| request | function | request(&mut self, method: &str, params: Value) |  |  |  |  |

| start | function | start() |  |  |  |  |

| Client | struct |  |  |  |  |  |

| Authority | enum |  |  |  |  |  |

| permits | function | permits(g: Authority, n: Authority) |  |  |  |  |

| exponential | function | exponential(base: u64, attempt: u32, cap: u64) |  |  |  |  |

| new | function | new(n: u32) |  |  |  |  |

| remaining | function | remaining(&self) |  |  |  |  |

| take | function | take(&mut self) |  |  |  |  |

| Budget | struct |  |  |  |  |  |

| compatible | function | compatible(available: &[Capability], required: &str) |  |  |  |  |

| Capability | struct |  |  |  |  |  |

| open | function | open(&self) |  |  |  |  |

| record_failure | function | record_failure(&mut self) |  |  |  |  |

| Circuit | struct |  |  |  |  |  |

| fail | function | fail(&mut self, id: &str) |  |  |  |  |

| Coordinator | struct |  |  |  |  |  |

| after | function | after(d: Duration) |  |  |  |  |

| expired | function | expired(&self) |  |  |  |  |

| Deadline | struct |  |  |  |  |  |

| exclude | function | exclude(&mut self) |  |  |  |  |

| Edge | struct |  |  |  |  |  |

| next | function | next(self) |  |  |  |  |

| Epoch | struct |  |  |  |  |  |

| admitted | function | admitted(xs: &[Observation]) |  |  |  |  |

| admitted | function | admitted(&self) |  |  |  |  |

| ExactSubject | struct |  |  |  |  |  |

| FailureClass | enum |  |  |  |  |  |

| exclude | function | exclude(&mut self, id: &str) |  |  |  |  |

| lawful | function | lawful(&self) |  |  |  |  |

| Graph | struct |  |  |  |  |  |

| Task | struct |  |  |  |  |  |

| Health | enum |  |  |  |  |  |

| eligible | function | eligible(h: Health) |  |  |  |  |

| key | function | key(subject: &str, epoch: u64, provider: &str) |  |  |  |  |

| valid_for | function | valid_for(&self, o: &str, e: u64) |  |  |  |  |

| Lease | struct |  |  |  |  |  |

| Observation | struct |  |  |  |  |  |

| bound | function | bound(&self) |  |  |  |  |

| Event | struct |  |  |  |  |  |

| Outcome | enum |  |  |  |  |  |

| is_success | function | is_success(&self) |  |  |  |  |

| valid | function | valid(&self) |  |  |  |  |

| Plan | struct |  |  |  |  |  |

| exact | function | exact(&self) |  |  |  |  |

| EvidenceSet | struct |  |  |  |  |  |

| ranked | function | ranked(mut ps: Vec<Plan>) |  |  |  |  |

| acyclic | function | acyclic(e: &[Order]) |  |  |  |  |

| Order | struct |  |  |  |  |  |

| Provider | trait |  |  |  |  |  |

| changed | function | changed(a: &EpochArtifact, b: &EpochArtifact) |  |  |  |  |

| EpochArtifact | struct |  |  |  |  |  |

| exact | function | exact(&self) |  |  |  |  |

| Receipt | struct |  |  |  |  |  |

| reconcile | function | reconcile(g: &mut Graph, s: &[(String, Health) |  |  |  |  |

| exclude_failed | function | exclude_failed(g: &mut Graph, edge: &str) |  |  |  |  |

| contains | function | contains(&self, id: &str) |  |  |  |  |

| register | function | register(&mut self, id: impl Into<String>) |  |  |  |  |

| Registry | struct |  |  |  |  |  |

| same_decision | function | same_decision(a: &Receipt, b: &Receipt) |  |  |  |  |

| next_edge | function | next_edge(&self) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| pop_next | function | pop_next(&mut self) |  |  |  |  |

| push | function | push(&mut self, x: T) |  |  |  |  |

| Scheduler | struct |  |  |  |  |  |

| State | enum |  |  |  |  |  |

| allowed | function | allowed(a: State, b: State) |  |  |  |  |

| Restart | enum |  |  |  |  |  |

| should_restart | function | should_restart(r: Restart, abnormal: bool) |  |  |  |  |

| success_rate | function | success_rate(&self) |  |  |  |  |

| Counters | struct |  |  |  |  |  |

| actionable | function | actionable(&self) |  |  |  |  |

| Counterexample | struct |  |  |  |  |  |

| ModelRole | struct |  |  |  |  |  |

| add | function | add(&mut self, level: usize) |  |  |  |  |

| analyze_conflict | function | analyze_conflict(ctx: &mut Context, conflict: Conflict) |  |  |  |  |

| clause | function | clause(&self) |  |  |  |  |

| involved | function | involved(&self) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| test | function | test(&self, level: usize) |  |  |  |  |

| AnalyzeConflict | struct |  |  |  |  |  |

| EnqueueAssumption | enum |  |  |  |  |  |

| assumption_levels | function | assumption_levels(&self) |  |  |  |  |

| enqueue_assumption | function | enqueue_assumption(ctx: &mut Context) |  |  |  |  |

| full_restart | function | full_restart(&mut self) |  |  |  |  |

| set_assumptions | function | set_assumptions(ctx: &mut Context, user_assumptions: &[Lit]) |  |  |  |  |

| user_failed_core | function | user_failed_core(&self) |  |  |  |  |

| Assumptions | struct |  |  |  |  |  |

| add_binary_clause | function | add_binary_clause(&mut self, lits: [Lit; 2]) |  |  |  |  |

| count | function | count(&self) |  |  |  |  |

| implied | function | implied(&self, lit: Lit) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| simplify_binary | function | simplify_binary(ctx: &mut Context) |  |  |  |  |

| BinaryClauses | struct |  |  |  |  |  |

| conflict_step | function | conflict_step(ctx: &mut Context) |  |  |  |  |

| header | function | header(&self) |  |  |  |  |

| header_mut | function | header_mut(&mut self) |  |  |  |  |

| lits | function | lits(&self) |  |  |  |  |

| lits_mut | function | lits_mut(&mut self) |  |  |  |  |

| Clause | struct |  |  |  |  |  |

| bump_clause_activity | function | bump_clause_activity(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| decay_clause_activities | function | decay_clause_activities(ctx: &mut Context) |  |  |  |  |

| ClauseActivity | struct |  |  |  |  |  |

| add_clause | function | add_clause(&mut self, mut header: ClauseHeader, lits: &[Lit]) |  |  |  |  |

| buffer_size | function | buffer_size(&self) |  |  |  |  |

| check_bounds | function | check_bounds(&self, cref: ClauseRef, len: usize) |  |  |  |  |

| clause | function | clause(&self, cref: ClauseRef) |  |  |  |  |

| clause_mut | function | clause_mut(&mut self, cref: ClauseRef) |  |  |  |  |

| header | function | header(&self, cref: ClauseRef) |  |  |  |  |

| header_mut | function | header_mut(&mut self, cref: ClauseRef) |  |  |  |  |

| header_unchecked_mut | function | header_unchecked_mut(&mut self, cref: ClauseRef) |  |  |  |  |

| lits_ptr_mut_unchecked | function | lits_ptr_mut_unchecked(&mut self, cref: ClauseRef) |  |  |  |  |

| with_capacity | function | with_capacity(capacity: usize) |  |  |  |  |

| ClauseAlloc | struct |  |  |  |  |  |

| ClauseRef | struct |  |  |  |  |  |

| assess_learned_clause | function | assess_learned_clause(ctx: &mut Context, lits: &[Lit]) |  |  |  |  |

| bump_clause | function | bump_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| Tier | enum |  |  |  |  |  |

| add_clause | function | add_clause(ctx: &mut Context, header: ClauseHeader, lits: &[Lit]) |  |  |  |  |

| count | function | count() |  |  |  |  |

| count_by_tier | function | count_by_tier(&self, tier: Tier) |  |  |  |  |

| delete_clause | function | delete_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| from_index | function | from_index(index: usize) |  |  |  |  |

| set_clause_tier | function | set_clause_tier(ctx: &mut Context, cref: ClauseRef, tier: Tier) |  |  |  |  |

| try_delete_clause | function | try_delete_clause(ctx: &mut Context, cref: ClauseRef) |  |  |  |  |

| ClauseDb | struct |  |  |  |  |  |

| collect_garbage | function | collect_garbage(ctx: &mut Context) |  |  |  |  |

| active | function | active(&self) |  |  |  |  |

| activity | function | activity(&self) |  |  |  |  |

| deleted | function | deleted(&self) |  |  |  |  |

| glue | function | glue(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| mark | function | mark(&self) |  |  |  |  |

| new | function | new() |  |  |  |  |

| set_active | function | set_active(&mut self, active: bool) |  |  |  |  |

| set_activity | function | set_activity(&mut self, activity: f32) |  |  |  |  |

| set_deleted | function | set_deleted(&mut self, deleted: bool) |  |  |  |  |

| set_glue | function | set_glue(&mut self, glue: usize) |  |  |  |  |

| set_len | function | set_len(&mut self, length: usize) |  |  |  |  |

| set_mark | function | set_mark(&mut self, mark: bool) |  |  |  |  |

| set_tier | function | set_tier(&mut self, tier: Tier) |  |  |  |  |

| tier | function | tier(&self) |  |  |  |  |

| ClauseHeader | struct |  |  |  |  |  |

| dedup_and_mark_by_tier | function | dedup_and_mark_by_tier(ctx: &mut Context, tier: Tier) |  |  |  |  |

| reduce_locals | function | reduce_locals(ctx: &mut Context) |  |  |  |  |

| reduce_mids | function | reduce_mids(ctx: &mut Context) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| iter | function | iter(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| new | function | new() |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| var_count | function | var_count(&self) |  |  |  |  |

| CnfFormula | struct |  |  |  |  |  |

| ExtendFormula | trait |  |  |  |  |  |

| SolverConfig | struct |  |  |  |  |  |

| set_var_count | function | set_var_count(ctx: &mut Context, count: usize) |  |  |  |  |

| Context | struct |  |  |  |  |  |

| make_decision | function | make_decision(ctx: &mut Context) |  |  |  |  |

| bump | function | bump(&mut self, var: Var) |  |  |  |  |

| decay | function | decay(&mut self) |  |  |  |  |

| make_available | function | make_available(&mut self, var: Var) |  |  |  |  |

| make_unavailable | function | make_unavailable(&mut self, var: Var) |  |  |  |  |

| reset | function | reset(&mut self, var: Var) |  |  |  |  |

| seed | function | seed(&mut self, var: Var, activity: f32) |  |  |  |  |

| set_decay | function | set_decay(&mut self, decay: f32) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| var_count | function | var_count(&self) |  |  |  |  |

| Vsids | struct |  |  |  |  |  |

| DimacsError | enum |  |  |  |  |  |

| parse_dimacs | function | parse_dimacs(input: impl io::BufRead) |  |  |  |  |

| parse_dimacs_str | function | parse_dimacs_str(input: &str) |  |  |  |  |

| write_dimacs | function | write_dimacs(target: &mut impl io::Write, formula: &CnfFormula) |  |  |  |  |

| compute_glue | function | compute_glue(tmp_flags: &mut TmpFlags, impl_graph: &ImplGraph, lits: &[Lit]) |  |  |  |  |

| code | function | code(self) |  |  |  |  |

| from_code | function | from_code(code: usize) |  |  |  |  |

| from_dimacs | function | from_dimacs(number: isize) |  |  |  |  |

| from_index | function | from_index(index: usize) |  |  |  |  |

| from_var | function | from_var(var: Var, polarity: bool) |  |  |  |  |

| index | function | index(self) |  |  |  |  |

| is_negative | function | is_negative(self) |  |  |  |  |

| is_positive | function | is_positive(self) |  |  |  |  |

| lit | function | lit(self, polarity: bool) |  |  |  |  |

| map_var | function | map_var(self, f: impl FnOnce(Var) |  |  |  |  |

| max_count | function | max_count() |  |  |  |  |

| max_var | function | max_var() |  |  |  |  |

| negative | function | negative(self) |  |  |  |  |

| positive | function | positive(self) |  |  |  |  |

| to_dimacs | function | to_dimacs(self) |  |  |  |  |

| var | function | var(self) |  |  |  |  |

| Lit | struct |  |  |  |  |  |

| Var | struct |  |  |  |  |  |

| load_clause | function | load_clause(ctx: &mut Context, user_lits: &[Lit]) |  |  |  |  |

| assignment | function | assignment(&self) |  |  |  |  |

| reconstruct_global_model | function | reconstruct_global_model(ctx: &mut Context) |  |  |  |  |

| Model | struct |  |  |  |  |  |

| propagate | function | propagate(ctx: &mut Context) |  |  |  |  |

| assign_lit | function | assign_lit(&mut self, lit: Lit) |  |  |  |  |

| assignment | function | assignment(&self) |  |  |  |  |

| backtrack | function | backtrack(ctx: &mut Context, level: usize) |  |  |  |  |

| clear | function | clear(&mut self) |  |  |  |  |

| current_level | function | current_level(&self) |  |  |  |  |

| enqueue_assignment | function | enqueue_assignment(
    assignment: &mut Assignment,
    impl_graph: &mut ImplGraph,
    trail: &mut Trail,
    lit: Lit,
    reason: Reason,
) |  |  |  |  |

| fast_option_eq | function | fast_option_eq(a: Option<bool>, b: Option<bool>) |  |  |  |  |

| full_restart | function | full_restart(ctx: &mut Context) |  |  |  |  |

| last_var_value | function | last_var_value(&self, var: Var) |  |  |  |  |

| lit_is_false | function | lit_is_false(&self, lit: Lit) |  |  |  |  |

| lit_is_true | function | lit_is_true(&self, lit: Lit) |  |  |  |  |

| lit_is_unk | function | lit_is_unk(&self, lit: Lit) |  |  |  |  |

| lit_value | function | lit_value(&self, lit: Lit) |  |  |  |  |

| new_decision_level | function | new_decision_level(&mut self) |  |  |  |  |

| pop_queue | function | pop_queue(&mut self) |  |  |  |  |

| queue_head | function | queue_head(&self) |  |  |  |  |

| restart | function | restart(ctx: &mut Context) |  |  |  |  |

| set_var | function | set_var(&mut self, var: Var, assignment: Option<bool>) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| trail | function | trail(&self) |  |  |  |  |

| unassign_var | function | unassign_var(&mut self, var: Var) |  |  |  |  |

| var_value | function | var_value(&self, var: Var) |  |  |  |  |

| Assignment | struct |  |  |  |  |  |

| Trail | struct |  |  |  |  |  |

| propagate_binary | function | propagate_binary(ctx: &mut Context, lit: Lit) |  |  |  |  |

| Conflict | enum |  |  |  |  |  |

| Reason | enum |  |  |  |  |  |

| is_removed_unit | function | is_removed_unit(&self, var: Var) |  |  |  |  |

| is_unit | function | is_unit(&self) |  |  |  |  |

| level | function | level(&self, var: Var) |  |  |  |  |

| reason | function | reason(&self, var: Var) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| update_reason | function | update_reason(&mut self, var: Var, reason: Reason) |  |  |  |  |

| update_removed_unit | function | update_removed_unit(&mut self, var: Var) |  |  |  |  |

| ImplGraph | struct |  |  |  |  |  |

| ImplNode | struct |  |  |  |  |  |

| propagate_long | function | propagate_long(ctx: &mut Context, lit: Lit) |  |  |  |  |

| add_watch | function | add_watch(&mut self, lit: Lit, watch: Watch) |  |  |  |  |

| disable | function | disable(&mut self) |  |  |  |  |

| enable_watchlists | function | enable_watchlists(ctx: &mut Context) |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| watch_clause | function | watch_clause(&mut self, cref: ClauseRef, lits: [Lit; 2]) |  |  |  |  |

| watched_by_mut | function | watched_by_mut(&mut self, lit: Lit) |  |  |  |  |

| Watch | struct |  |  |  |  |  |

| Watchlists | struct |  |  |  |  |  |

| schedule_step | function | schedule_step(ctx: &mut Context) |  |  |  |  |

| Schedule | struct |  |  |  |  |  |

| advance | function | advance(&mut self) |  |  |  |  |

| LubySequence | struct |  |  |  |  |  |

| SolverError | enum |  |  |  |  |  |

| add_formula | function | add_formula(&mut self, formula: &CnfFormula) |  |  |  |  |

| assume | function | assume(&mut self, assumptions: &[Lit]) |  |  |  |  |

| failed_core | function | failed_core(&self) |  |  |  |  |

| is_recoverable | function | is_recoverable(&self) |  |  |  |  |

| model | function | model(&self) |  |  |  |  |

| new | function | new() |  |  |  |  |

| seed_activity | function | seed_activity(&mut self, seeds: &[(usize, f32) |  |  |  |  |

| set_conflict_limit | function | set_conflict_limit(&mut self, limit: Option<u64>) |  |  |  |  |

| solve | function | solve(&mut self) |  |  |  |  |

| Solver | struct |  |  |  |  |  |

| SatState | enum |  |  |  |  |  |

| SolverState | struct |  |  |  |  |  |

| set_var_count | function | set_var_count(&mut self, count: usize) |  |  |  |  |

| TmpData | struct |  |  |  |  |  |

| TmpFlags | struct |  |  |  |  |  |

| prove_units | function | prove_units(ctx: &mut Context) |  |  |  |  |

| resurrect_unit | function | resurrect_unit(ctx: &mut Context, lit: Lit) |  |  |  |  |

| unit_simplify | function | unit_simplify(ctx: &mut Context) |  |  |  |  |

| existing_user_from_solver | function | existing_user_from_solver(&self, solver: Var) |  |  |  |  |

| global_from_solver | function | global_from_solver(&self) |  |  |  |  |

| global_from_solver_mut | function | global_from_solver_mut(&mut self) |  |  |  |  |

| global_from_user | function | global_from_user(&self) |  |  |  |  |

| global_from_user_mut | function | global_from_user_mut(&mut self) |  |  |  |  |

| global_var_iter | function | global_var_iter(&self) |  |  |  |  |

| global_watermark | function | global_watermark(&self) |  |  |  |  |

| initialize_solver_var | function | initialize_solver_var(ctx: &mut Context, solver: Var, global: Var) |  |  |  |  |

| new_user_var | function | new_user_var(ctx: &mut Context) |  |  |  |  |

| next_unmapped_solver | function | next_unmapped_solver(&self) |  |  |  |  |

| next_unmapped_user | function | next_unmapped_user(&self) |  |  |  |  |

| remove_solver_var | function | remove_solver_var(ctx: &mut Context, solver: Var) |  |  |  |  |

| solver_from_global | function | solver_from_global(&self) |  |  |  |  |

| solver_from_global_mut | function | solver_from_global_mut(&mut self) |  |  |  |  |

| solver_from_user | function | solver_from_user(ctx: &mut Context, user: Var) |  |  |  |  |

| solver_from_user_lits | function | solver_from_user_lits(ctx: &mut Context, solver_lits: &mut Vec<Lit>, user_lits: &[Lit]) |  |  |  |  |

| solver_var_present | function | solver_var_present(&self, solver: Var) |  |  |  |  |

| solver_watermark | function | solver_watermark(&self) |  |  |  |  |

| user_from_global | function | user_from_global(&self) |  |  |  |  |

| user_var_iter | function | user_var_iter(&self) |  |  |  |  |

| user_watermark | function | user_watermark(&self) |  |  |  |  |

| var_data_global | function | var_data_global(&self, global: Var) |  |  |  |  |

| var_data_global_mut | function | var_data_global_mut(&mut self, global: Var) |  |  |  |  |

| var_data_solver_mut | function | var_data_solver_mut(&mut self, solver: Var) |  |  |  |  |

| Variables | struct |  |  |  |  |  |

| user_default | function | user_default() |  |  |  |  |

| VarData | struct |  |  |  |  |  |

| bwd | function | bwd(&self) |  |  |  |  |

| bwd_mut | function | bwd_mut(&mut self) |  |  |  |  |

| fwd | function | fwd(&self) |  |  |  |  |

| fwd_mut | function | fwd_mut(&mut self) |  |  |  |  |

| get | function | get(&self, from: Var) |  |  |  |  |

| insert | function | insert(&mut self, into: Var, from: Var) |  |  |  |  |

| remove | function | remove(&mut self, from: Var) |  |  |  |  |

| watermark | function | watermark(&self) |  |  |  |  |

| VarBiMap | struct |  |  |  |  |  |

| VarBiMapMut | struct |  |  |  |  |  |

| VarMap | struct |  |  |  |  |  |

| admit_candidates | function | admit_candidates(
    candidates: &[ProbeCandidate],
    goal_limit: usize,
    surface: &str,
) |  |  |  |  |

| corridor_problem | function | corridor_problem(n: usize) |  |  |  |  |

| probe_all | function | probe_all(
    parent: &Session,
    candidates: &[ProbeCandidate],
    evals: usize,
    mem_mb: usize,
) |  |  |  |  |

| probe_candidate | function | probe_candidate(
    parent: &Session,
    candidate: &ProbeCandidate,
    evals: usize,
    mem_mb: usize,
) |  |  |  |  |

| repair | function | repair(
    inner: &Session,
    plan: &mut Option<Plan>,
    cursor: &mut usize,
    evals: usize,
    mem_mb: usize,
) |  |  |  |  |

| ProbeCandidate | struct |  |  |  |  |  |

| advance | function | advance(&mut self) |  |  |  |  |

| apply_start | function | apply_start(&mut self, name: &str) |  |  |  |  |

| drop_plan | function | drop_plan(&mut self) |  |  |  |  |

| elapse | function | elapse(&mut self, dt: f64) |  |  |  |  |

| explain | function | explain(domain: &str, problem: &str, plan_json: &str) |  |  |  |  |

| fact | function | fact(&self, name: &str) |  |  |  |  |

| fluent | function | fluent(&self, name: &str) |  |  |  |  |

| fond_validate | function | fond_validate(problem_json: &str, plan_json: &str) |  |  |  |  |

| fork | function | fork(&self) |  |  |  |  |

| goal_met | function | goal_met(&self) |  |  |  |  |

| has_plan | function | has_plan(&self) |  |  |  |  |

| mind_bytes | function | mind_bytes(&self) |  |  |  |  |

| new | function | new(domain: &str, problem: &str) |  |  |  |  |

| observe | function | observe(&mut self, sight_json: &str) |  |  |  |  |

| plan | function | plan(
        domain: &str,
        problem: &str,
        mode: Option<String>,
        flags: Option<String>,
        search: Option<String>,
    ) |  |  |  |  |

| plan_production | function | plan_production(
        domain: &str,
        problem: &str,
        mode: Option<String>,
        search: Option<String>,
        max_evaluated: Option<usize>,
        max_plan_steps: Option<usize>,
        max_output_bytes: Option<usize>,
        request_id: Option<String>,
    ) |  |  |  |  |

| plan_valid_json | function | plan_valid_json(&self, plan_json: &str, from: usize) |  |  |  |  |

| probe_json | function | probe_json(&self, candidates_json: &str, evals: usize, mem_mb: usize) |  |  |  |  |

| readiness | function | readiness() |  |  |  |  |

| repair | function | repair(&mut self, evals: usize, mem_mb: usize) |  |  |  |  |

| replan_following | function | replan_following(&mut self, evals: usize, mem_mb: usize) |  |  |  |  |

| restrict_contains | function | restrict_contains(&mut self, filter: String) |  |  |  |  |

| restrict_prefix_claims | function | restrict_prefix_claims(&mut self, prefix: String, claimed: String) |  |  |  |  |

| set_fact | function | set_fact(&mut self, name: &str, value: bool) |  |  |  |  |

| set_fluent | function | set_fluent(&mut self, name: &str, value: f64) |  |  |  |  |

| set_goal | function | set_goal(&mut self, goal: &str) |  |  |  |  |

| set_timed_fact | function | set_timed_fact(&mut self, dt: f64, name: &str, value: bool) |  |  |  |  |

| step_json | function | step_json(&self) |  |  |  |  |

| suffix_json | function | suffix_json(&self) |  |  |  |  |

| think | function | think(&mut self, evals: usize, mem_mb: usize) |  |  |  |  |

| valid | function | valid(&self) |  |  |  |  |

| version | function | version() |  |  |  |  |

| world_bytes | function | world_bytes(&self) |  |  |  |  |

| WasmSession | struct |  |  |  |  |  |

| contradictory_fact | function | contradictory_fact(sight: &[(String, bool) |  |  |  |  |

| fact_key | function | fact_key(fact: &str) |  |  |  |  |

| Mode | enum |  |  |  |  |  |

| Search | enum |  |  |  |  |  |

| SolveError | enum |  |  |  |  |  |

| decompose | function | decompose(
    domain_src: &str,
    problem_src: &str,
    opts: &Options,
) |  |  |  |  |

| parse | function | parse(src: &str) |  |  |  |  |

| solve | function | solve(domain_src: &str, problem_src: &str, opts: &Options) |  |  |  |  |

| Contract | struct |  |  |  |  |  |

| Decomposition | struct |  |  |  |  |  |

| DomainSummary | struct |  |  |  |  |  |

| Options | struct |  |  |  |  |  |

| ParseReport | struct |  |  |  |  |  |

| Plan | struct |  |  |  |  |  |

| ProblemSummary | struct |  |  |  |  |  |

| Solution | struct |  |  |  |  |  |

| Statistics | struct |  |  |  |  |  |

| Step | struct |  |  |  |  |  |

| clear | function | clear(w: &mut [u64], i: usize) |  |  |  |  |

| count | function | count(w: &[u64]) |  |  |  |  |

| set | function | set(w: &mut [u64], i: usize) |  |  |  |  |

| test | function | test(w: &[u64], i: usize) |  |  |  |  |

| words_for | function | words_for(n_bits: usize) |  |  |  |  |

| elapsed_ms | function | elapsed_ms(&self) |  |  |  |  |

| elapsed_secs | function | elapsed_secs(&self) |  |  |  |  |

| elapsed_us | function | elapsed_us(&self) |  |  |  |  |

| now | function | now() |  |  |  |  |

| Clock | struct |  |  |  |  |  |

| Traj | enum |  |  |  |  |  |

| accepted | function | accepted(&self) |  |  |  |  |

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |

| expand | function | expand(domain: &Domain, problem: &Problem) |  |  |  |  |

| gate | function | gate(domain: &Domain, problem: &Problem) |  |  |  |  |

| hard_only_gated | function | hard_only_gated(
    domain: &Domain,
    problem: &Problem,
) |  |  |  |  |

| new | function | new(traj: &'a Traj) |  |  |  |  |

| op_name | function | op_name(&self) |  |  |  |  |

| step | function | step(&mut self, holds: &mut dyn FnMut(&Formula) |  |  |  |  |

| step_at | function | step_at(&mut self, time: f64, holds: &mut dyn FnMut(&Formula) |  |  |  |  |

| Expanded | struct |  |  |  |  |  |

| Fold | struct |  |  |  |  |  |

| improve | function | improve(
    task: &PackedTask,
    cf: usize,
    ops: Vec<usize>,
    first_cost: f64,
    threads: usize,
    base: SearchCfg,
    spent: usize,
    
    
    
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| improve_length | function | improve_length(
    task: &PackedTask,
    ops: Vec<usize>,
    threads: usize,
    base: SearchCfg,
    spent: usize,
) |  |  |  |  |

| metric_fluent | function | metric_fluent(problem: &Problem) |  |  |  |  |

| optimize_text | function | optimize_text(
    problem: &Problem,
    task: &PackedTask,
    optimize: bool,
    threads: usize,
    cfg: SearchCfg,
    ops: &mut Vec<usize>,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| plan_cost | function | plan_cost(task: &PackedTask, cf: usize, ops: &[usize]) |  |  |  |  |

| CostOutcome | struct |  |  |  |  |  |

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |

| espc_optimize | function | espc_optimize(
    task: &PackedTask,
    cost_fluent: usize,
    sat: &mut SatGuidance,
    seed: Option<(Vec<usize>, f64) |  |  |  |  |

| EspcPartition | struct |  |  |  |  |  |

| EspcResult | struct |  |  |  |  |  |

| EveError | enum |  |  |  |  |  |

| EveStage | enum |  |  |  |  |  |

| PlanningRegime | enum |  |  |  |  |  |

| enter | function | enter(request: EveRequest) |  |  |  |  |

| Activator | struct |  |  |  |  |  |

| CapabilityTarget | struct |  |  |  |  |  |

| Eve | struct |  |  |  |  |  |

| EveHandoff | struct |  |  |  |  |  |

| EveRequest | struct |  |  |  |  |  |

| GenesisProjection | struct |  |  |  |  |  |

| GenesisWorld | struct |  |  |  |  |  |

| GgenManufacturingRequest | struct |  |  |  |  |  |

| GroundedGoal | struct |  |  |  |  |  |

| HddlDecompositionRequest | struct |  |  |  |  |  |

| HddlSurface | struct |  |  |  |  |  |

| HumanPurpose | struct |  |  |  |  |  |

| ManufactureTarget | struct |  |  |  |  |  |

| McpPlusHandoff | struct |  |  |  |  |  |

| PpddlPolicyRequest | struct |  |  |  |  |  |

| PpddlSurface | struct |  |  |  |  |  |

| SplitDirective | struct |  |  |  |  |  |

| TruexContinuation | struct |  |  |  |  |  |

| DemandMode | enum |  |  |  |  |  |

| clear_overrides | function | clear_overrides() |  |  |  |  |

| demand_mode | function | demand_mode() |  |  |  |  |

| escalate | function | escalate() |  |  |  |  |

| espc | function | espc() |  |  |  |  |

| set_escalate_override | function | set_escalate_override(on: bool) |  |  |  |  |

| set_espc_override | function | set_espc_override(on: bool) |  |  |  |  |

| set_overrides | function | set_overrides(tdemand: bool, tdecomp: bool, tconc: bool) |  |  |  |  |

| tconc | function | tconc() |  |  |  |  |

| tdecomp | function | tdecomp() |  |  |  |  |

| tdemand | function | tdemand() |  |  |  |  |

| Outcome | enum |  |  |  |  |  |

| ground | function | ground(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| ground_fixpoint | function | ground_fixpoint(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| ground_stratified | function | ground_stratified(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| ground_stratified_walled | function | ground_stratified_walled(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| ground_task | function | ground_task(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| initial_state | function | initial_state(t: &PackedTask) |  |  |  |  |

| objects_by_type | function | objects_by_type(domain: &Domain, problem: &Problem) |  |  |  |  |

| FxHasher | struct |  |  |  |  |  |

| HddlError | enum |  |  |  |  |  |

| adapt_problem | function | adapt_problem(p: ferroplan_hddl::translate::PlanningProblem) |  |  |  |  |

| solve_hddl | function | solve_hddl(
    domain_src: &str,
    problem_src: &str,
    limits: &PlannerLimits,
) |  |  |  |  |

| solve_hddl_from_eve | function | solve_hddl_from_eve(
    handoff: &EveHandoff,
    limits: &PlannerLimits,
) |  |  |  |  |

| extraction_need_facts | function | extraction_need_facts(sc: &Scratch) |  |  |  |  |

| helpful_needed_adders | function | helpful_needed_adders(
    task: &PackedTask,
    sc: &Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
) |  |  |  |  |

| new | function | new(task: &PackedTask) |  |  |  |  |

| reachability_layers | function | reachability_layers(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
) |  |  |  |  |

| relaxed | function | relaxed(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
) |  |  |  |  |

| relaxed_costed | function | relaxed_costed(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
) |  |  |  |  |

| relaxed_helpful | function | relaxed_helpful(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
) |  |  |  |  |

| relaxed_plan_cost | function | relaxed_plan_cost(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
) |  |  |  |  |

| relaxed_to | function | relaxed_to(
    task: &PackedTask,
    sc: &mut Scratch,
    bits: &[u64],
    fv: &[f64],
    def: &[bool],
    goal_pos: &[u32],
    goal_num: &[NumPre],
) |  |  |  |  |

| Scratch | struct |  |  |  |  |  |

| TrpgInfo | struct |  |  |  |  |  |

| TrpgWindow | struct |  |  |  |  |  |

| explain | function | explain(domain_src: &str, problem_src: &str, plan: &Plan) |  |  |  |  |

| CausalLink | struct |  |  |  |  |  |

| Explanation | struct |  |  |  |  |  |

| InvariantSpan | struct |  |  |  |  |  |

| PrefReport | struct |  |  |  |  |  |

| synthesize | function | synthesize(domain: &Domain, task: &PackedTask) |  |  |  |  |

| search | function | search(
    task: &PackedTask,
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |

| search_subgoal | function | search_subgoal(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[crate::types::NumPre],
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
    len_anytime: bool,
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |

| goal_landmarks | function | goal_landmarks(task: &PackedTask) |  |  |  |  |

| landmarks_for | function | landmarks_for(
    task: &PackedTask,
    start: &crate::packed::State,
    goal_pos: &[u32],
) |  |  |  |  |

| Tok | enum |  |  |  |  |  |

| lex | function | lex(input: &str) |  |  |  |  |

| arm | function | arm() |  |  |  |  |

| armed | function | armed(&self) |  |  |  |  |

| declared_budget_bytes | function | declared_budget_bytes() |  |  |  |  |

| hit | function | hit(&self) |  |  |  |  |

| latch | function | latch() |  |  |  |  |

| latched | function | latched() |  |  |  |  |

| peak_resident_bytes | function | peak_resident_bytes() |  |  |  |  |

| resident_bytes | function | resident_bytes() |  |  |  |  |

| unarmed | function | unarmed() |  |  |  |  |

| MemWall | struct |  |  |  |  |  |

| from_env | function | from_env() |  |  |  |  |

| r_partition_facts | function | r_partition_facts(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[crate::types::NumPre],
    r_cap: usize,
) |  |  |  |  |

| search | function | search(
    task: &PackedTask,
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |

| search_driver | function | search_driver(
    task: &PackedTask,
    max_eval: usize,
    forbidden: &[bool],
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |

| search_light | function | search_light(
    task: &PackedTask,
    max_eval: usize,
    forbidden: &[bool],
) |  |  |  |  |

| search_subgoal | function | search_subgoal(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[crate::types::NumPre],
    threads: usize,
    max_eval: usize,
    forbidden: &[bool],
    slice: Option<(crate::clock::Clock, f64) |  |  |  |  |

| DriverCfg | struct |  |  |  |  |  |

| OperatorCompileError | enum |  |  |  |  |  |

| compile_operator | function | compile_operator(
    spec: &OperatorSpec,
    states: &[State],
) |  |  |  |  |

| CompiledOperator | struct |  |  |  |  |  |

| OperatorEffects | struct |  |  |  |  |  |

| OperatorSpec | struct |  |  |  |  |  |

| solve | function | solve(
    task: &PackedTask,
    cf: Option<usize>,
    max_nodes: usize,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| OptOutcome | struct |  |  |  |  |  |

| canonical_key | function | canonical_key(
        &self,
        task: &PackedTask,
        state: &State,
        agenda: &[(i64, usize) |  |  |  |  |

| canonical_skey | function | canonical_skey(
        &self,
        task: &PackedTask,
        state: &State,
        cost_fluent: Option<usize>,
    ) |  |  |  |  |

| canonical_skey_hash | function | canonical_skey_hash(
        &self,
        task: &PackedTask,
        state: &State,
        cost_fluent: Option<usize>,
    ) |  |  |  |  |

| detect | function | detect(domain: &Domain, problem: &Problem, task: &PackedTask) |  |  |  |  |

| detect_classical | function | detect_classical(domain: &Domain, problem: &Problem, task: &PackedTask) |  |  |  |  |

| detect_classical_iso | function | detect_classical_iso(
    domain: &Domain,
    problem: &Problem,
    task: &PackedTask,
) |  |  |  |  |

| detect_iso | function | detect_iso(domain: &Domain, problem: &Problem, task: &PackedTask) |  |  |  |  |

| gen_key | function | gen_key(&self, op: usize, classes: &[Vec<u16>]) |  |  |  |  |

| goal_free_view | function | goal_free_view(&self) |  |  |  |  |

| iso_active | function | iso_active(&self) |  |  |  |  |

| iso_goal_witness | function | iso_goal_witness(
        &self,
        task: &PackedTask,
        state: &State,
        goal_pos: &[u32],
        goal_num: &[NumPre],
    ) |  |  |  |  |

| iso_remap_op | function | iso_remap_op(&self, sigma: &[Vec<u16>], op: usize) |  |  |  |  |

| iso_untouched_goal | function | iso_untouched_goal(&self) |  |  |  |  |

| stabilizer_classes | function | stabilizer_classes(&self, state: &State, agenda: &[(f64, usize) |  |  |  |  |

| IsoGoal | struct |  |  |  |  |  |

| Orbit | struct |  |  |  |  |  |

| OrbitMap | struct |  |  |  |  |  |

| preamble | function | preamble(threads: usize) |  |  |  |  |

| render | function | render(task: &PackedTask, result: &PlanResult, threads: usize) |  |  |  |  |

| applicable_ops | function | applicable_ops(&self, s: &State, out: &mut Vec<u32>) |  |  |  |  |

| apply | function | apply(&self, oi: usize, s: &State) |  |  |  |  |

| build_succ | function | build_succ(pre_pos: &Csr<u32>, n_facts: usize, n_ops: usize) |  |  |  |  |

| cond_effs | function | cond_effs(&self, oi: usize) |  |  |  |  |

| fact_id | function | fact_id(&self, disp: &str) |  |  |  |  |

| finish | function | finish(self) |  |  |  |  |

| fluent_id | function | fluent_id(&self, disp: &str) |  |  |  |  |

| goal_met | function | goal_met(&self, s: &State) |  |  |  |  |

| goal_met_with | function | goal_met_with(&self, s: &State, goal_pos: &[u32], goal_num: &[NumPre]) |  |  |  |  |

| initial | function | initial(&self) |  |  |  |  |

| n_cond_effs | function | n_cond_effs(&self, oi: usize) |  |  |  |  |

| new | function | new() |  |  |  |  |

| op_applicable | function | op_applicable(&self, oi: usize, s: &State) |  |  |  |  |

| push_row | function | push_row(&mut self, items: impl IntoIterator<Item = T>) |  |  |  |  |

| slice | function | slice(&self, i: usize) |  |  |  |  |

| state_key | function | state_key(&self, s: &State) |  |  |  |  |

| state_key_eq | function | state_key_eq(&self, a: &State, b: &State, cost_fluent: Option<usize>) |  |  |  |  |

| state_key_hash | function | state_key_hash(&self, s: &State, cost_fluent: Option<usize>) |  |  |  |  |

| state_key_with_cost | function | state_key_with_cost(&self, s: &State, cost_fluent: Option<usize>) |  |  |  |  |

| static_fluent | function | static_fluent(&self, disp: &str) |  |  |  |  |

| CondEff | struct |  |  |  |  |  |

| Csr | struct |  |  |  |  |  |

| CsrBuilder | struct |  |  |  |  |  |

| PackedTask | struct |  |  |  |  |  |

| State | struct |  |  |  |  |  |

| StateKey | struct |  |  |  |  |  |

| num_threads | function | num_threads() |  |  |  |  |

| parse_domain | function | parse_domain(src: &str) |  |  |  |  |

| parse_problem | function | parse_problem(src: &str) |  |  |  |  |

| interaction_partition | function | interaction_partition(task: &PackedTask, groups: &[Vec<u32>]) |  |  |  |  |

| interaction_partition_of | function | interaction_partition_of(
    task: &PackedTask,
    groups: &[Vec<u32>],
    goals: &[u32],
    excluded_vars: &FxHashSet<usize>,
) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| merge_at | function | merge_at(groups: &mut Vec<Subgoal>, i: usize, j: usize) |  |  |  |  |

| merge_with_neighbor | function | merge_with_neighbor(groups: &mut Vec<Subgoal>, i: usize) |  |  |  |  |

| partition | function | partition(task: &PackedTask) |  |  |  |  |

| Subgoal | struct |  |  |  |  |  |

| close_seed | function | close_seed(
    task: &PackedTask,
    cost_fluent: usize,
    forgos: &[(usize, f64) |  |  |  |  |

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |

| display_metric | function | display_metric(&self, optimized: f64) |  |  |  |  |

| hard_goal_plan | function | hard_goal_plan(
    domain: &Domain,
    problem: &Problem,
    threads: usize,
    cfg: SearchCfg,
) |  |  |  |  |

| hard_goal_seed | function | hard_goal_seed(
    domain: &Domain,
    problem: &Problem,
    compiled: &PackedTask,
    threads: usize,
    cfg: SearchCfg,
) |  |  |  |  |

| has_preferences | function | has_preferences(problem: &Problem) |  |  |  |  |

| is_pddl3 | function | is_pddl3(problem: &Problem) |  |  |  |  |

| lift_seed | function | lift_seed(compiled: &PackedTask, names: &[String]) |  |  |  |  |

| metric_optimize | function | metric_optimize(
    task: &PackedTask,
    cost_fluent: usize,
    forgos: &[(usize, f64) |  |  |  |  |

| metric_optimize_seeded | function | metric_optimize_seeded(
    task: &PackedTask,
    cost_fluent: usize,
    forgos: &[(usize, f64) |  |  |  |  |

| pref_weights | function | pref_weights(domain: &Domain, problem: &Problem) |  |  |  |  |

| preferences | function | preferences(goal: &Formula, objs: &HashMap<Sym, Vec<Sym>>) |  |  |  |  |

| Compiled | struct |  |  |  |  |  |

| MetricResult | struct |  |  |  |  |  |

| PhaseTail | struct |  |  |  |  |  |

| SeededResult | struct |  |  |  |  |  |

| Validity | enum |  |  |  |  |  |

| parse_classical | function | parse_classical(src: &str) |  |  |  |  |

| parse_timed | function | parse_timed(src: &str) |  |  |  |  |

| validate_plan | function | validate_plan(
    domain_src: &str,
    problem_src: &str,
    plan_src: &str,
) |  |  |  |  |

| run_ff | function | run_ff(domain_src: &str, problem_src: &str, opts: &crate::Options) |  |  |  |  |

| run_planner | function | run_planner(
    domain_src: &str,
    problem_src: &str,
    opts: &crate::Options,
    ipc: bool,
) |  |  |  |  |

| PlannerError | enum |  |  |  |  |  |

| solve_planning_type | function | solve_planning_type(
    request: &UniversalPlanningRequest,
) |  |  |  |  |

| Agent | struct |  |  |  |  |  |

| Goal | struct |  |  |  |  |  |

| Method | struct |  |  |  |  |  |

| PlanStep | struct |  |  |  |  |  |

| PlannerLimits | struct |  |  |  |  |  |

| PlanningProblem | struct |  |  |  |  |  |

| PolicyEntry | struct |  |  |  |  |  |

| PolicyOutcome | struct |  |  |  |  |  |

| QueueState | struct |  |  |  |  |  |

| RdfTriple | struct |  |  |  |  |  |

| State | struct |  |  |  |  |  |

| Task | struct |  |  |  |  |  |

| Tool | struct |  |  |  |  |  |

| Transition | struct |  |  |  |  |  |

| UniversalPlan | struct |  |  |  |  |  |

| UniversalPlanningRequest | struct |  |  |  |  |  |

| WorkflowEdge | struct |  |  |  |  |  |

| PlanningCapability | enum |  |  |  |  |  |

| PlanningRail | enum |  |  |  |  |  |

| PlanningRouteError | enum |  |  |  |  |  |

| PlanningType | enum |  |  |  |  |  |

| rail | function | rail(self) |  |  |  |  |

| required_capabilities | function | required_capabilities(self) |  |  |  |  |

| route_planning_request | function | route_planning_request(
    request: &PlanningRequest,
) |  |  |  |  |

| token | function | token(self) |  |  |  |  |

| PlanningRequest | struct |  |  |  |  |  |

| PlanningRoute | struct |  |  |  |  |  |

| PolicyGuarantee | enum |  |  |  |  |  |

| PolicyIssue | enum |  |  |  |  |  |

| validate_fond_policy | function | validate_fond_policy(
    problem: &PlanningProblem,
    plan: &UniversalPlan,
) |  |  |  |  |

| PolicyValidationReport | struct |  |  |  |  |  |

| solve | function | solve(task: &PackedTask, threads: usize, cfg: SearchCfg) |  |  |  |  |

| Outcome | struct |  |  |  |  |  |

| PpddlError | enum |  |  |  |  |  |

| ProbabilisticObjective | enum |  |  |  |  |  |

| InitialStateProbability | struct |  |  |  |  |  |

| PolicyDecision | struct |  |  |  |  |  |

| PolicyOutcome | struct |  |  |  |  |  |

| PolicyValidation | struct |  |  |  |  |  |

| PpddlParseReport | struct |  |  |  |  |  |

| ProbabilisticOptions | struct |  |  |  |  |  |

| ProbabilisticSolution | struct |  |  |  |  |  |

| ProbabilisticState | struct |  |  |  |  |  |

| ProbabilisticStatistics | struct |  |  |  |  |  |

| SimulationReport | struct |  |  |  |  |  |

| parse_ppddl | function | parse_ppddl(domain_src: &str, problem_src: &str) |  |  |  |  |

| solve_ppddl | function | solve_ppddl(
    domain_src: &str,
    problem_src: &str,
    options: &ProbabilisticOptions,
) |  |  |  |  |

| validate_ppddl_policy | function | validate_ppddl_policy(
    domain_src: &str,
    problem_src: &str,
    options: &ProbabilisticOptions,
    solution: &ProbabilisticSolution,
) |  |  |  |  |

| simulate_ppddl | function | simulate_ppddl(
    domain_src: &str,
    problem_src: &str,
    options: &ProbabilisticOptions,
    episodes: usize,
    seed: u64,
) |  |  |  |  |

| decompose_production | function | decompose_production(
    domain: &str,
    problem: &str,
    options: &Options,
    limits: &ProductionLimits,
    request_id: Option<&str>,
) |  |  |  |  |

| goal_met | function | goal_met(&self) |  |  |  |  |

| mind_bytes | function | mind_bytes(&self) |  |  |  |  |

| new | function | new(
        domain: &str,
        problem: &str,
        options: &Options,
        limits: ProductionLimits,
    ) |  |  |  |  |

| parse_production | function | parse_production(
    source: &str,
    max_input_bytes: usize,
    request_id: Option<&str>,
) |  |  |  |  |

| replan | function | replan(
        &self,
        max_evaluated: usize,
        memory_mb: Option<usize>,
        request_id: Option<&str>,
    ) |  |  |  |  |

| solve_ppddl_production | function | solve_ppddl_production(
    domain: &str,
    problem: &str,
    options: &ProbabilisticOptions,
    max_input_bytes: usize,
    max_output_bytes: usize,
    request_id: Option<&str>,
) |  |  |  |  |

| trace_production | function | trace_production(
    domain: &str,
    problem: &str,
    plan: &[(String, Vec<String>) |  |  |  |  |

| validate_plan_production | function | validate_plan_production(
    domain: &str,
    problem: &str,
    plan: &str,
    max_input_bytes: usize,
    max_plan_bytes: usize,
    request_id: Option<&str>,
) |  |  |  |  |

| world_bytes | function | world_bytes(&self) |  |  |  |  |

| PlanValidationEvidence | struct |  |  |  |  |  |

| ProductionSession | struct |  |  |  |  |  |

| decompose_production | function | decompose_production(
    domain: &str,
    problem: &str,
    options: &Options,
    limits: &ProductionLimits,
    request_id: Option<&str>,
) |  |  |  |  |

| explain_production | function | explain_production(
    domain: &str,
    problem: &str,
    plan: &Plan,
    limits: &ProductionLimits,
    request_id: Option<&str>,
) |  |  |  |  |

| depth_reached | function | depth_reached(&self) |  |  |  |  |

| from_predecessors | function | from_predecessors(
        predecessors: &[Vec<u32>],
        prohibited: &[u32],
        max_depth: u32,
    ) |  |  |  |  |

| from_successors | function | from_successors(
        successors: &[Vec<u32>],
        prohibited: &[u32],
        max_depth: u32,
    ) |  |  |  |  |

| is_safe | function | is_safe(&self, state: u32) |  |  |  |  |

| saturated | function | saturated(&self) |  |  |  |  |

| unsafe_count | function | unsafe_count(&self) |  |  |  |  |

| BackwardSafeSet | struct |  |  |  |  |  |

| AuthorityClass | enum |  |  |  |  |  |

| CompatibilityClass | enum |  |  |  |  |  |

| DeterminismClass | enum |  |  |  |  |  |

| InterfaceKind | enum |  |  |  |  |  |

| ManifestError | enum |  |  |  |  |  |

| OutcomeClass | enum |  |  |  |  |  |

| ReadinessState | enum |  |  |  |  |  |

| ReplayClass | enum |  |  |  |  |  |

| SecurityClass | enum |  |  |  |  |  |

| ValidationStatus | enum |  |  |  |  |  |

| capability_manifest | function | capability_manifest() |  |  |  |  |

| fingerprint | function | fingerprint(&self) |  |  |  |  |

| new | function | new(code: impl Into<String>, message: impl Into<String>, retryable: bool) |  |  |  |  |

| production_input_fingerprint | function | production_input_fingerprint(domain: &str, problem: &str, options: &Options) |  |  |  |  |

| solve_production | function | solve_production(
    domain: &str,
    problem: &str,
    options: &Options,
    limits: &ProductionLimits,
    request_id: Option<&str>,
) |  |  |  |  |

| validate | function | validate(&self) |  |  |  |  |

| BuildIdentity | struct |  |  |  |  |  |

| CapabilityContract | struct |  |  |  |  |  |

| CapabilityEvaluation | struct |  |  |  |  |  |

| CapabilityManifest | struct |  |  |  |  |  |

| OperationEnvelope | struct |  |  |  |  |  |

| ProductionLimits | struct |  |  |  |  |  |

| PublicError | struct |  |  |  |  |  |

| ReadinessReport | struct |  |  |  |  |  |

| ff_plan | function | ff_plan(task: &PackedTask, ops: &[usize]) |  |  |  |  |

| ipc_plan | function | ipc_plan(task: &PackedTask, ops: &[usize], metric: Option<f64>) |  |  |  |  |

| metric_footer | function | metric_footer(
    cost: f64,
    iterations: usize,
    n_prefs: usize,
    threads: usize,
    warn_other: bool,
) |  |  |  |  |

| preamble | function | preamble(threads: usize) |  |  |  |  |

| timing | function | timing(stats: &Stats, threads: usize) |  |  |  |  |

| Solved | enum |  |  |  |  |  |

| solve | function | solve(
    task: &PackedTask,
    threads: usize,
    cfg: crate::search::SearchCfg,
    mutex_groups: &[Vec<u32>],
    
    
    
    
    
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| Stats | struct |  |  |  |  |  |

| detect_resources | function | detect_resources(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) |  |  |  |  |

| occupancy | function | occupancy(&self, bits: &[u64]) |  |  |  |  |

| trip_bound | function | trip_bound(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) |  |  |  |  |

| trips | function | trips(&self, bits: &[u64]) |  |  |  |  |

| ResourceVar | struct |  |  |  |  |  |

| TripBound | struct |  |  |  |  |  |

| from_env | function | from_env() |  |  |  |  |

| requires_concurrency | function | requires_concurrency(domain: &Domain, problem: &Problem) |  |  |  |  |

| solve_classical | function | solve_classical(
    task: &PackedTask,
    groups: &[Vec<u32>],
    cfg: &SatCfg,
) |  |  |  |  |

| solve_temporal | function | solve_temporal(
    domain: &Domain,
    problem: &Problem,
    threads: usize,
    cfg: &SatCfg,
) |  |  |  |  |

| solve_temporal_within | function | solve_temporal_within(
    domain: &Domain,
    problem: &Problem,
    threads: usize,
    cfg: &SatCfg,
    budget_secs: Option<f64>,
) |  |  |  |  |

| SatCfg | struct |  |  |  |  |  |

| SatOutcome | struct |  |  |  |  |  |

| PlanResult | enum |  |  |  |  |  |

| arm_wall_limit | function | arm_wall_limit() |  |  |  |  |

| cancelled | function | cancelled(&self) |  |  |  |  |

| cost | function | cost(&self, s: &State) |  |  |  |  |

| from_weights | function | from_weights(weight_g: f64, weight_h: f64, max_eval: Option<usize>) |  |  |  |  |

| holds | function | holds(&self, s: &State) |  |  |  |  |

| plan | function | plan(
    task: &PackedTask,
    threads: usize,
    cfg: SearchCfg,
    ehc_first: bool,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| plan_avoiding | function | plan_avoiding(
    task: &PackedTask,
    threads: usize,
    cfg: SearchCfg,
    ehc_first: bool,
    forbidden: &[bool],
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| search | function | search(task: &PackedTask, threads: usize, cfg: SearchCfg) |  |  |  |  |

| search_from | function | search_from(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: Option<usize>,
    cost_bound: f64,
    threads: usize,
    cfg: SearchCfg,
    forbidden: &[bool],
    sat: Option<&SatGuidance>,
    closure: Option<&ClosureCost>,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| solve_closure_bounded | function | solve_closure_bounded(
    task: &PackedTask,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
    bound: f64,
    closure: &ClosureCost,
    forbidden: &[bool],
    threads: usize,
    cfg: SearchCfg,
    sat: Option<&SatGuidance>,
) |  |  |  |  |

| solve_subgoal | function | solve_subgoal(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    threads: usize,
    cfg: SearchCfg,
    orbit: Option<&crate::orbits::OrbitMap>,
) |  |  |  |  |

| solve_subgoal_avoiding | function | solve_subgoal_avoiding(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    forbidden: &[bool],
    threads: usize,
    cfg: SearchCfg,
) |  |  |  |  |

| solve_subgoal_bounded | function | solve_subgoal_bounded(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    cost_fluent: usize,
    bound: f64,
    threads: usize,
    cfg: SearchCfg,
    sat: Option<&SatGuidance>,
) |  |  |  |  |

| solve_subgoal_guided | function | solve_subgoal_guided(
    task: &PackedTask,
    start: &State,
    goal_pos: &[u32],
    goal_num: &[NumPre],
    forbidden: &[bool],
    threads: usize,
    cfg: SearchCfg,
    sat: Option<&SatGuidance>,
) |  |  |  |  |

| with_cost_h | function | with_cost_h(mut self, cost_fluent: usize) |  |  |  |  |

| with_cost_weight | function | with_cost_weight(mut self, w_c: f64) |  |  |  |  |

| ClosureCost | struct |  |  |  |  |  |

| PlanOutcome | struct |  |  |  |  |  |

| PrefPhi | struct |  |  |  |  |  |

| SatGuidance | struct |  |  |  |  |  |

| SearchCfg | struct |  |  |  |  |  |

| select | function | select(
    task: &PackedTask,
    groups: &[Vec<u32>],
    weights: &[f64],
    dnf: &FxHashMap<usize, Vec<Vec<u32>>>,
    banned: &crate::hash::FxHashSet<u32>,
) |  |  |  |  |

| Selection | struct |  |  |  |  |  |

| ThinkVerdict | enum |  |  |  |  |  |

| apply_start | function | apply_start(&mut self, name: &str) |  |  |  |  |

| elapse | function | elapse(&mut self, dt: f64) |  |  |  |  |

| fact | function | fact(&self, name: &str) |  |  |  |  |

| fluent | function | fluent(&self, name: &str) |  |  |  |  |

| fork | function | fork(&self) |  |  |  |  |

| goal_met | function | goal_met(&self) |  |  |  |  |

| mind_bytes | function | mind_bytes(&self) |  |  |  |  |

| new | function | new(domain_src: &str, problem_src: &str, opts: &Options) |  |  |  |  |

| observe | function | observe(&mut self, sight: &[(&str, bool) |  |  |  |  |

| plan_still_valid | function | plan_still_valid(&self, plan: &Plan, from_step: usize) |  |  |  |  |

| replan | function | replan(&self) |  |  |  |  |

| replan_budgeted | function | replan_budgeted(&self, max_evaluated: usize, memory_mb: Option<usize>) |  |  |  |  |

| replan_following | function | replan_following(
        &self,
        prior: &Plan,
        from_step: usize,
        max_evaluated: usize,
        memory_mb: Option<usize>,
    ) |  |  |  |  |

| restrict_ops | function | restrict_ops(&mut self, mut keep: impl FnMut(&str) |  |  |  |  |

| set_fact | function | set_fact(&mut self, name: &str, value: bool) |  |  |  |  |

| set_fluent | function | set_fluent(&mut self, name: &str, value: f64) |  |  |  |  |

| set_goal | function | set_goal(&mut self, goal: &str) |  |  |  |  |

| set_timed_fact | function | set_timed_fact(&mut self, dt: f64, name: &str, value: bool) |  |  |  |  |

| state_fingerprint | function | state_fingerprint(&self) |  |  |  |  |

| think | function | think(&self, budget: &ThinkBudget) |  |  |  |  |

| think_following | function | think_following(&self, prior: &Plan, from_step: usize, budget: &ThinkBudget) |  |  |  |  |

| world_bytes | function | world_bytes(&self) |  |  |  |  |

| Session | struct |  |  |  |  |  |

| Think | struct |  |  |  |  |  |

| ThinkBudget | struct |  |  |  |  |  |

| Bet | enum |  |  |  |  |  |

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |

| declines | function | declines(domain: &Domain, problem: &Problem) |  |  |  |  |

| lay_out | function | lay_out(
    domain: &Domain,
    task: &PackedTask,
    ops: &[usize],
    shift: bool,
) |  |  |  |  |

| solve | function | solve(domain: &Domain, problem: &Problem, threads: usize, bet: Bet) |  |  |  |  |

| compile | function | compile(domain: &Domain, problem: &Problem) |  |  |  |  |

| is_temporal | function | is_temporal(domain: &Domain) |  |  |  |  |

| prepare | function | prepare(domain: &'a Domain, problem: &'a Problem) |  |  |  |  |

| score | function | score(&self, plan: &TimedPlan) |  |  |  |  |

| score_soft | function | score_soft(domain: &Domain, problem: &Problem, plan: &TimedPlan) |  |  |  |  |

| solve | function | solve(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| solve_scored | function | solve_scored(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| to_ipc | function | to_ipc(&self) |  |  |  |  |

| validate | function | validate(domain: &Domain, problem: &Problem, plan: &TimedPlan) |  |  |  |  |

| ScoredPlan | struct |  |  |  |  |  |

| SnapInfo | struct |  |  |  |  |  |

| SoftScore | struct |  |  |  |  |  |

| SoftScorer | struct |  |  |  |  |  |

| TemporalCompiled | struct |  |  |  |  |  |

| TimedPlan | struct |  |  |  |  |  |

| TimedStep | struct |  |  |  |  |  |

| trace | function | trace(
    domain_src: &str,
    problem_src: &str,
    plan: &[(String, Vec<String>) |  |  |  |  |

| StateSnapshot | struct |  |  |  |  |  |

| solve | function | solve(domain: &Domain, problem: &Problem, threads: usize) |  |  |  |  |

| n_actors | function | n_actors(domain: &Domain, problem: &Problem) |  |  |  |  |

| reschedule | function | reschedule(domain: &Domain, problem: &Problem, plan: &TimedPlan) |  |  |  |  |

| single_actor_problem | function | single_actor_problem(domain: &Domain, problem: &Problem) |  |  |  |  |

| AssignOp | enum |  |  |  |  |  |

| CompOp | enum |  |  |  |  |  |

| Constraint | enum |  |  |  |  |  |

| Effect | enum |  |  |  |  |  |

| Expr | enum |  |  |  |  |  |

| Formula | enum |  |  |  |  |  |

| MetricDir | enum |  |  |  |  |  |

| NExpr | enum |  |  |  |  |  |

| Term | enum |  |  |  |  |  |

| TimeSpec | enum |  |  |  |  |  |

| chosen | function | chosen(&self) |  |  |  |  |

| collect_fluents | function | collect_fluents(&self, out: &mut Vec<u32>) |  |  |  |  |

| eval | function | eval(&self, fv: &[f64], def: &[bool]) |  |  |  |  |

| eval_numpre | function | eval_numpre(np: &NumPre, fv: &[f64], def: &[bool]) |  |  |  |  |

| fixed | function | fixed(e: Expr) |  |  |  |  |

| new | function | new(line: u32, message: impl Into<String>) |  |  |  |  |

| Action | struct |  |  |  |  |  |

| DerivedRule | struct |  |  |  |  |  |

| Domain | struct |  |  |  |  |  |

| Duration | struct |  |  |  |  |  |

| DurativeAction | struct |  |  |  |  |  |

| NumEff | struct |  |  |  |  |  |

| NumPre | struct |  |  |  |  |  |

| ParseError | struct |  |  |  |  |  |

| Problem | struct |  |  |  |  |  |

| TimedLiteral | struct |  |  |  |  |  |

| verify | function | verify(
    domain_src: &str,
    problem_src: &str,
    plan: &[(String, Vec<String>) |  |  |  |  |

| Verified | struct |  |  |  |  |  |

| PredKind | enum |  |  |  |  |  |

| build | function | build(domain: &Domain, problem: &Problem) |  |  |  |  |

| domain_to_pddl | function | domain_to_pddl(
    name: &str,
    requirements: &str,
    types: &[(String, String) |  |  |  |  |

| dynamic_predicates | function | dynamic_predicates(domain: &Domain) |  |  |  |  |

| goal_facts | function | goal_facts(problem: &Problem) |  |  |  |  |

| positions_at | function | positions_at(&self, facts: &[String]) |  |  |  |  |

| to_pddl | function | to_pddl(
    name: &str,
    domain_name: &str,
    objects: &[(String, String) |  |  |  |  |

| VizEdge | struct |  |  |  |  |  |

| VizGraph | struct |  |  |  |  |  |

| VizMobile | struct |  |  |  |  |  |

| VizNode | struct |  |  |  |  |  |

| corpus_dir | function | corpus_dir() |  |  |  |  |

| corpus_ipc_dir | function | corpus_ipc_dir() |  |  |  |  |

| differential_run_dir | function | differential_run_dir() |  |  |  |  |

| harness_present | function | harness_present(what: &str, path: &Path) |  |  |  |  |

| oracle_dir | function | oracle_dir() |  |  |  |  |

| oracle_runner | function | oracle_runner() |  |  |  |  |

| base_sizes | function | base_sizes(rng: &mut Rng) |  |  |  |  |

| below | function | below(&mut self, n: u64) |  |  |  |  |

| chance | function | chance(&mut self, percent: u64) |  |  |  |  |

| draw_for | function | draw_for(seed: u64, sizes: Sizes) |  |  |  |  |

| draw_valid | function | draw_valid(seed: u64, sizes: Sizes) |  |  |  |  |

| generate | function | generate(seed: u64, sizes: Sizes) |  |  |  |  |

| generate_valid | function | generate_valid(seed: u64, sizes: Sizes) |  |  |  |  |

| halve_sizes | function | halve_sizes(s: Sizes) |  |  |  |  |

| mutation_of | function | mutation_of(seed: u64) |  |  |  |  |

| new | function | new(seed: u64) |  |  |  |  |

| next_u64 | function | next_u64(&mut self) |  |  |  |  |

| pick_idx | function | pick_idx(&mut self, len: usize) |  |  |  |  |

| range | function | range(&mut self, lo: u64, hi: u64) |  |  |  |  |

| range_usize | function | range_usize(&mut self, lo: usize, hi: usize) |  |  |  |  |

| render | function | render(model: &Model, problem_name: &str) |  |  |  |  |

| render_with_provenance | function | render_with_provenance(
    model: &Model,
    problem_name: &str,
    header: &[&str],
) |  |  |  |  |

| sizes_for | function | sizes_for(seed: u64) |  |  |  |  |

| Model | struct |  |  |  |  |  |

| Rng | struct |  |  |  |  |  |

| Sizes | struct |  |  |  |  |  |

| from_rows | function | from_rows(rows: &[RawRow]) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| render | function | render(
    header: &BoardHeader,
    summary: &[VariantSummary],
    score_against: Option<&str>,
) |  |  |  |  |

| summarize_variants | function | summarize_variants(rows: &[RawRow], reference: Option<&Reference>) |  |  |  |  |

| BoardHeader | struct |  |  |  |  |  |

| Reference | struct |  |  |  |  |  |

| VariantSummary | struct |  |  |  |  |  |

| ConditionsError | enum |  |  |  |  |  |

| Slot | enum |  |  |  |  |  |

| as_option | function | as_option(&self) |  |  |  |  |

| at | function | at(&self) |  |  |  |  |

| competitors_total | function | competitors_total(&self) |  |  |  |  |

| idle_pct | function | idle_pct(&self) |  |  |  |  |

| is_present | function | is_present(&self) |  |  |  |  |

| new | function | new(
        at: Option<Number>,
        idle_pct: Option<Number>,
        competitors_total: Option<Number>,
    ) |  |  |  |  |

| observe | function | observe(&mut self, r: &Reading<'_>) |  |  |  |  |

| of | function | of(self_exclusion: Vec<String>) |  |  |  |  |

| parse | function | parse(text: &str, path: &str) |  |  |  |  |

| percentile | function | percentile(v: &[f64], p: f64) |  |  |  |  |

| rollup_from_timeline | function | rollup_from_timeline(timeline: &[TimelineEntry]) |  |  |  |  |

| statistics_median | function | statistics_median(v: &[f64]) |  |  |  |  |

| summarize | function | summarize(r: &Rollup, ended: &str, provenance: Option<&Provenance>) |  |  |  |  |

| to_json | function | to_json(&self) |  |  |  |  |

| Competitors | struct |  |  |  |  |  |

| Conditions | struct |  |  |  |  |  |

| CpuSpeedLimit | struct |  |  |  |  |  |

| IdlePct | struct |  |  |  |  |  |

| LoadAvg | struct |  |  |  |  |  |

| Provenance | struct |  |  |  |  |  |

| Reading | struct |  |  |  |  |  |

| Rollup | struct |  |  |  |  |  |

| SwapMb | struct |  |  |  |  |  |

| TimelineEntry | struct |  |  |  |  |  |

| TimelineRollup | struct |  |  |  |  |  |

| instances | function | instances(v: &Variant, max: usize, warnings: &mut Vec<String>) |  |  |  |  |

| variants | function | variants(corpus: &Path, ipcs: &[String], selects: &dyn Fn(&str) |  |  |  |  |

| Instance | struct |  |  |  |  |  |

| Variant | struct |  |  |  |  |  |

| Walk | struct |  |  |  |  |  |

| LockError | enum |  |  |  |  |  |

| acquire | function | acquire(dir: &Path) |  |  |  |  |

| path | function | path(&self) |  |  |  |  |

| DirLock | struct |  |  |  |  |  |

| DbError | enum |  |  |  |  |  |

| open | function | open(dir: &Path) |  |  |  |  |

| path | function | path(&self) |  |  |  |  |

| reader | function | reader(&self) |  |  |  |  |

| writer | function | writer(&self) |  |  |  |  |

| Db | struct |  |  |  |  |  |

| Cleanliness | enum |  |  |  |  |  |

| PassVerdict | enum |  |  |  |  |  |

| RunState | enum |  |  |  |  |  |

| TimingQuality | enum |  |  |  |  |  |

| ValReason | enum |  |  |  |  |  |

| as_str | function | as_str(self) |  |  |  |  |

| identity | function | identity(&self) |  |  |  |  |

| of | function | of(i: &Instance) |  |  |  |  |

| parse | function | parse(s: &str) |  |  |  |  |

| sort_key | function | sort_key(label: &str) |  |  |  |  |

| to_instance | function | to_instance(&self) |  |  |  |  |

| AttemptRec | struct |  |  |  |  |  |

| BoardFacts | struct |  |  |  |  |  |

| BoardKey | struct |  |  |  |  |  |

| BoardPassRec | struct |  |  |  |  |  |

| EngineFacts | struct |  |  |  |  |  |

| EngineKey | struct |  |  |  |  |  |

| EventRec | struct |  |  |  |  |  |

| InstanceKey | struct |  |  |  |  |  |

| LiveChild | struct |  |  |  |  |  |

| Measured | struct |  |  |  |  |  |

| RunRecord | struct |  |  |  |  |  |

| SamplePoint | struct |  |  |  |  |  |

| SampleRec | struct |  |  |  |  |  |

| ThrottleWindowRec | struct |  |  |  |  |  |

| VariantKey | struct |  |  |  |  |  |

| attempts_for | function | attempts_for(
        &self,
        board_id: i64,
        engine_id: i64,
        variant: &str,
        label: &str,
    ) |  |  |  |  |

| banked_instances | function | banked_instances(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| boards_named | function | boards_named(&self, name: &str) |  |  |  |  |

| canary_baseline | function | canary_baseline(
        &self,
        label: &str,
        window: usize,
        pct: f64,
    ) |  |  |  |  |

| canary_max_between | function | canary_max_between(&self, start_ts: f64, end_ts: f64) |  |  |  |  |

| clean_instances | function | clean_instances(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| competitors_between | function | competitors_between(
        &self,
        start_ts: f64,
        end_ts: f64,
    ) |  |  |  |  |

| conn | function | conn(&self) |  |  |  |  |

| engine_by_hash | function | engine_by_hash(&self, blake3: &str) |  |  |  |  |

| engines_for_board | function | engines_for_board(&self, board_id: i64) |  |  |  |  |

| engines_matching | function | engines_matching(&self, needle: &str) |  |  |  |  |

| export_rows | function | export_rows(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| live_children | function | live_children(&self) |  |  |  |  |

| next_attempt | function | next_attempt(
        &self,
        board_id: i64,
        engine_id: i64,
        ipc: Option<&str>,
        variant: &str,
        label: &str,
    ) |  |  |  |  |

| open | function | open(path: &Path) |  |  |  |  |

| pass_verdict | function | pass_verdict(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| prior_peak_rss | function | prior_peak_rss(&self, variant: &str, label: &str) |  |  |  |  |

| run_census | function | run_census(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| runs_between | function | runs_between(
        &self,
        engine_id: i64,
        start_ts: f64,
        end_ts: f64,
    ) |  |  |  |  |

| sample_count | function | sample_count(&self, pass: Option<i64>) |  |  |  |  |

| samples_between | function | samples_between(&self, start_ts: f64, end_ts: f64) |  |  |  |  |

| solo_attempts | function | solo_attempts(
        &self,
        board_id: i64,
        engine_id: i64,
        variant: &str,
        label: &str,
    ) |  |  |  |  |

| swap_growth_between | function | swap_growth_between(&self, start_ts: f64, end_ts: f64) |  |  |  |  |

| throttle_windows_between | function | throttle_windows_between(
        &self,
        start_ts: f64,
        end_ts: f64,
    ) |  |  |  |  |

| timing_census | function | timing_census(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| val_ok | function | val_ok(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| val_rejected | function | val_rejected(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| val_unavailable | function | val_unavailable(&self, board_id: i64, engine_id: i64) |  |  |  |  |

| verdicts_for | function | verdicts_for(
        &self,
        board_id: i64,
        engine_id: i64,
    ) |  |  |  |  |

| window_gate | function | window_gate(
        &self,
        start_ts: f64,
        end_ts: f64,
        interval: f64,
        pass: Option<i64>,
    ) |  |  |  |  |

| Reader | struct |  |  |  |  |  |

| board_facts | function | board_facts(spec: &BoardSpec, m: &Manifest, first: Option<&RawRow>) |  |  |  |  |

| board_key_from_manifest | function | board_key_from_manifest(m: &Manifest, spec: &BoardSpec) |  |  |  |  |

| export | function | export(reader: &Reader, board_id: i64, engine_id: i64) |  |  |  |  |

| export_to | function | export_to(
    reader: &Reader,
    board_id: i64,
    engine_id: i64,
    path: &Path,
) |  |  |  |  |

| rebuild_from_artifacts | function | rebuild_from_artifacts(
    writer: &WriterHandle,
    manifest: &Manifest,
    dir: &Path,
    val_unavailable: Option<&ValUnavailable>,
) |  |  |  |  |

| RebuiltBoard | struct |  |  |  |  |  |

| MigrateError | enum |  |  |  |  |  |

| migrate | function | migrate(conn: &Connection) |  |  |  |  |

| board_pass | function | board_pass(&self, p: BoardPassRec) |  |  |  |  |

| canary | function | canary(&self, at: f64, label: String, secs: f64, solo: bool) |  |  |  |  |

| child_gone | function | child_gone(&self, pid: i32) |  |  |  |  |

| child_spawned | function | child_spawned(&self, c: LiveChild) |  |  |  |  |

| child_stopped | function | child_stopped(&self, pid: i32, stopped: bool) |  |  |  |  |

| event | function | event(&self, e: EventRec) |  |  |  |  |

| flush | function | flush(&self) |  |  |  |  |

| handle | function | handle(&self) |  |  |  |  |

| resolve | function | resolve(
        &self,
        board: BoardKey,
        board_facts: BoardFacts,
        engine: EngineKey,
        engine_facts: EngineFacts,
    ) |  |  |  |  |

| run | function | run(&self, rec: RunRecord) |  |  |  |  |

| sample | function | sample(&self, s: SampleRec) |  |  |  |  |

| start | function | start(conn: Connection) |  |  |  |  |

| take_error | function | take_error(&self) |  |  |  |  |

| throttle_close | function | throttle_close(&self, id: i64, ended_at: f64) |  |  |  |  |

| throttle_open | function | throttle_open(&self, w: ThrottleWindowRec) |  |  |  |  |

| Writer | struct |  |  |  |  |  |

| WriterHandle | struct |  |  |  |  |  |

| build | function | build(
    timeout_secs: u64,
    mem_gb: f64,
    board_env: &BTreeMap<String, String>,
) |  |  |  |  |

| validate | function | validate(
    timeout_secs: u64,
    mem_gb: f64,
    board_env: &BTreeMap<String, String>,
) |  |  |  |  |

| Ctl | enum |  |  |  |  |  |

| ExecError | enum |  |  |  |  |  |

| Killed | enum |  |  |  |  |  |

| install_interrupt_handler | function | install_interrupt_handler() |  |  |  |  |

| interrupted | function | interrupted() |  |  |  |  |

| set_interrupted | function | set_interrupted(on: bool) |  |  |  |  |

| RunOutcome | struct |  |  |  |  |  |

| RunRequest | struct |  |  |  |  |  |

| Reaped | enum |  |  |  |  |  |

| armed | function | armed(&self) |  |  |  |  |

| disarm | function | disarm(&mut self) |  |  |  |  |

| install_panic_reaper | function | install_panic_reaper() |  |  |  |  |

| new | function | new(pgid: Pid) |  |  |  |  |

| pgid | function | pgid(&self) |  |  |  |  |

| pid | function | pid(&self) |  |  |  |  |

| reap_registered | function | reap_registered() |  |  |  |  |

| record | function | record(pid: Pid, run_id: Option<i64>, id: &ProcIdentity, spawned_at: f64) |  |  |  |  |

| signalled | function | signalled(&self) |  |  |  |  |

| GroupGuard | struct |  |  |  |  |  |

| LiveChild | struct |  |  |  |  |  |

| busiest | function | busiest(&self, procs: &[Proc]) |  |  |  |  |

| game_pids | function | game_pids(&self, procs: &[Proc]) |  |  |  |  |

| GameRules | struct |  |  |  |  |  |

| Proc | struct |  |  |  |  |  |

| attribute | function | attribute(ps_output: &str, exclude: &dyn Fn(&str) |  |  |  |  |

| is_clean | function | is_clean(&self) |  |  |  |  |

| Sample | struct |  |  |  |  |  |

| Level | enum |  |  |  |  |  |

| Reason | enum |  |  |  |  |  |

| level | function | level(&self) |  |  |  |  |

| new | function | new(cfg: Config) |  |  |  |  |

| on_sample | function | on_sample(&mut self, s: &Sample, g: &GameState, now: Instant) |  |  |  |  |

| set_manual_hold | function | set_manual_hold(&mut self, on: bool) |  |  |  |  |

| Config | struct |  |  |  |  |  |

| GameState | struct |  |  |  |  |  |

| Throttle | struct |  |  |  |  |  |

| Transition | struct |  |  |  |  |  |

| Generic | struct |  |  |  |  |  |

| mach_ticks_to_ns | function | mach_ticks_to_ns(ticks: u64) |  |  |  |  |

| MacOs | struct |  |  |  |  |  |

| MemCap | enum |  |  |  |  |  |

| bytes | function | bytes(self) |  |  |  |  |

| host | function | host() |  |  |  |  |

| instrument | function | instrument(self) |  |  |  |  |

| ProcIdentity | struct |  |  |  |  |  |

| Topology | struct |  |  |  |  |  |

| KeepAwake | trait |  |  |  |  |  |

| Platform | trait |  |  |  |  |  |

| Admission | enum |  |  |  |  |  |

| Denial | enum |  |  |  |  |  |

| admit | function | admit(&self, level: Level, demand: Demand, declared: bool) |  |  |  |  |

| capacity | function | capacity(&self, level: Level) |  |  |  |  |

| contains | function | contains(&self, board_id: &str) |  |  |  |  |

| cores | function | cores(self) |  |  |  |  |

| errors | function | errors(lines: &[String]) |  |  |  |  |

| ids | function | ids(&self) |  |  |  |  |

| is_admitted | function | is_admitted(&self) |  |  |  |  |

| new | function | new(jobs: u32, threads: u32) |  |  |  |  |

| none | function | none() |  |  |  |  |

| of | function | of(b: &BoardSpec, d: &Defaults) |  |  |  |  |

| validate | function | validate(&self, m: &Manifest, declared: &Oversubscribed) |  |  |  |  |

| with_reserve | function | with_reserve(topology: Topology, reserve_p_cores: u32) |  |  |  |  |

| Accountant | struct |  |  |  |  |  |

| Demand | struct |  |  |  |  |  |

| Oversubscribed | struct |  |  |  |  |  |

| Event | enum |  |  |  |  |  |

| Next | enum |  |  |  |  |  |

| backoff | function | backoff(&self, consecutive: u32) |  |  |  |  |

| order_boards | function | order_boards(boards: &[BoardState], pass: u32) |  |  |  |  |

| run | function | run(r: &mut dyn Runner, cfg: &LoopConfig) |  |  |  |  |

| Attempt | struct |  |  |  |  |  |

| BoardState | struct |  |  |  |  |  |

| LoopConfig | struct |  |  |  |  |  |

| Outcome | struct |  |  |  |  |  |

| Runner | trait |  |  |  |  |  |

| Admission | enum |  |  |  |  |  |

| Denied | enum |  |  |  |  |  |

| Rule | enum |  |  |  |  |  |

| config | function | config(&self) |  |  |  |  |

| is_admitted | function | is_admitted(&self) |  |  |  |  |

| new | function | new(cfg: Config) |  |  |  |  |

| poll | function | poll(&mut self, level: Level, sample: &Sample, now: Instant) |  |  |  |  |

| reset | function | reset(&mut self) |  |  |  |  |

| Config | struct |  |  |  |  |  |

| Gate | struct |  |  |  |  |  |

| Bank | enum |  |  |  |  |  |

| Owe | enum |  |  |  |  |  |

| Verdict | enum |  |  |  |  |  |

| as_str | function | as_str(self) |  |  |  |  |

| banked | function | banked(self) |  |  |  |  |

| box_fault | function | box_fault(self) |  |  |  |  |

| judge | function | judge(rule: &Rule, f: &Facts) |  |  |  |  |

| rho | function | rho(&self) |  |  |  |  |

| rho_floor_ms | function | rho_floor_ms(&self) |  |  |  |  |

| timing | function | timing(f: &Facts) |  |  |  |  |

| Facts | struct |  |  |  |  |  |

| Rule | struct |  |  |  |  |  |

| Disabled | enum |  |  |  |  |  |

| InstanceKey | enum |  |  |  |  |  |

| Reject | enum |  |  |  |  |  |

| disabled | function | disabled(&self) |  |  |  |  |

| from_document | function | from_document(doc: &crate::artifact::conditions::Conditions) |  |  |  |  |

| get | function | get(&self, key: &RowKey) |  |  |  |  |

| has_timeline | function | has_timeline(&self) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| judge | function | judge(row: &RawRow, cond: &Conditions, want: &RunParams) |  |  |  |  |

| judge_lines | function | judge_lines(text: &str, cond: &Conditions, want: &RunParams) |  |  |  |  |

| kind | function | kind(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| load | function | load(path: &Path) |  |  |  |  |

| mode_str | function | mode_str(&self) |  |  |  |  |

| none | function | none() |  |  |  |  |

| of | function | of(r: &RawRow) |  |  |  |  |

| parse | function | parse(text: &str) |  |  |  |  |

| reject_counts | function | reject_counts(&self) |  |  |  |  |

| rejected | function | rejected(&self) |  |  |  |  |

| Conditions | struct |  |  |  |  |  |

| Rejected | struct |  |  |  |  |  |

| Resume | struct |  |  |  |  |  |

| RowKey | struct |  |  |  |  |  |

| RunParams | struct |  |  |  |  |  |

| TimelineSample | struct |  |  |  |  |  |

| Tier | enum |  |  |  |  |  |

| census | function | census(plan: &[Scheduled]) |  |  |  |  |

| classify | function | classify(prior: Option<Prior>, t: &Thresholds) |  |  |  |  |

| eta | function | eta(plan: &[Scheduled], jobs: u32) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| order | function | order(
    instances: &[RowKey],
    h: &dyn History,
    t: &Thresholds,
    budget_secs: f64,
) |  |  |  |  |

| NoHistory | struct |  |  |  |  |  |

| Prior | struct |  |  |  |  |  |

| Scheduled | struct |  |  |  |  |  |

| Thresholds | struct |  |  |  |  |  |

| History | trait |  |  |  |  |  |

| argv | function | argv(cfg: &BoardCfg, domain: &Path, problem: &Path) |  |  |  |  |

| BoardCfg | struct |  |  |  |  |  |

| Engine | struct |  |  |  |  |  |

| Measured | struct |  |  |  |  |  |

| Unavailable | enum |  |  |  |  |  |

| Verdict | enum |  |  |  |  |  |

| as_json | function | as_json(self) |  |  |  |  |

| find | function | find(repo: &Path, configured: Option<&Path>) |  |  |  |  |

| judge | function | judge(rc: Option<i32>, signal: Option<i32>, stdout: &str, stderr: &str) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| reason | function | reason(self) |  |  |  |  |

| render_plan | function | render_plan(steps: &[Step], temporal: bool) |  |  |  |  |

| validate | function | validate(
    val: Option<&Path>,
    domain: &Path,
    problem: &Path,
    steps: &[Step],
    temporal: bool,
    plan_path: &Path,
) |  |  |  |  |

| Step | struct |  |  |  |  |  |

| ArchiveError | enum |  |  |  |  |  |

| arch_key | function | arch_key(variant: &str, instance: u64) |  |  |  |  |

| arch_track | function | arch_track(variant: &str) |  |  |  |  |

| best_length | function | best_length(&self, k: &ArchKey) |  |  |  |  |

| best_makespan | function | best_makespan(&self, k: &ArchKey) |  |  |  |  |

| count_action_lines | function | count_action_lines(body: &str) |  |  |  |  |

| has_lengths | function | has_lengths(&self) |  |  |  |  |

| has_makespans | function | has_makespans(&self) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| iter | function | iter(&self) |  |  |  |  |

| lengths | function | lengths(&self, k: &ArchKey) |  |  |  |  |

| lengths_map | function | lengths_map(&self) |  |  |  |  |

| makespan_of | function | makespan_of(body: &str) |  |  |  |  |

| makespans | function | makespans(&self, k: &ArchKey) |  |  |  |  |

| makespans_map | function | makespans_map(&self) |  |  |  |  |

| open | function | open(path: &Path) |  |  |  |  |

| warnings | function | warnings(&self) |  |  |  |  |

| ArchiveWarnings | struct |  |  |  |  |  |

| Ipc5Archive | struct |  |  |  |  |  |

| best | function | best(&self, year_key: &str, domain: &str, instance: u64) |  |  |  |  |

| from_sources | function | from_sources(bounds_2023: Option<&str>, cost_bounds_2018: Option<&str>) |  |  |  |  |

| get | function | get(&self, k: &BoundKey) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| load | function | load(corpus_root: &Path) |  |  |  |  |

| problems | function | problems(&self) |  |  |  |  |

| BestKnownBounds | struct |  |  |  |  |  |

| Class | enum |  |  |  |  |  |

| failure_classes | function | failure_classes(&self) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| Coverage | struct |  |  |  |  |  |

| Cleanliness | enum |  |  |  |  |  |

| Clock | enum |  |  |  |  |  |

| Cost | enum |  |  |  |  |  |

| Mode | enum |  |  |  |  |  |

| a | function | a(&self) |  |  |  |  |

| a_solved | function | a_solved(&self) |  |  |  |  |

| b | function | b(&self) |  |  |  |  |

| b_solved | function | b_solved(&self) |  |  |  |  |

| cheaper_a | function | cheaper_a(&self) |  |  |  |  |

| cheaper_b | function | cheaper_b(&self) |  |  |  |  |

| clean | function | clean(&self) |  |  |  |  |

| cleanliness | function | cleanliness(&self, r: &RawRow) |  |  |  |  |

| clock | function | clock(&self) |  |  |  |  |

| common | function | common(&self) |  |  |  |  |

| coverage | function | coverage(&self) |  |  |  |  |

| delta | function | delta(&self) |  |  |  |  |

| dirty | function | dirty(&self) |  |  |  |  |

| equal | function | equal(&self) |  |  |  |  |

| from_json | function | from_json(src: &str) |  |  |  |  |

| from_jsonl | function | from_jsonl(
        name: impl Into<String>,
        budget: f64,
        src: &str,
        path: &str,
    ) |  |  |  |  |

| from_rows | function | from_rows(name: impl Into<String>, budget: f64, rows: Vec<RawRow>) |  |  |  |  |

| gained | function | gained(&self) |  |  |  |  |

| get | function | get(&self, k: &InstanceKey) |  |  |  |  |

| has_timeline | function | has_timeline(&self) |  |  |  |  |

| interval | function | interval(&self) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| is_regression | function | is_regression(&self) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| len | function | len(&self) |  |  |  |  |

| load | function | load(path: &Path) |  |  |  |  |

| lost | function | lost(&self) |  |  |  |  |

| mean_a | function | mean_a(&self) |  |  |  |  |

| mean_b | function | mean_b(&self) |  |  |  |  |

| name | function | name(self) |  |  |  |  |

| new | function | new(referee: &'r Referee, a: &'r RunRef, b: &'r RunRef) |  |  |  |  |

| none | function | none() |  |  |  |  |

| of | function | of(r: &RawRow) |  |  |  |  |

| percentiles | function | percentiles(&self) |  |  |  |  |

| qualified | function | qualified(&self) |  |  |  |  |

| quality | function | quality(&self) |  |  |  |  |

| referee | function | referee(&self) |  |  |  |  |

| render | function | render(&self, mode: Mode) |  |  |  |  |

| render_trend | function | render_trend(t: &Trend) |  |  |  |  |

| scored | function | scored(&self) |  |  |  |  |

| solved | function | solved(&self, referee: &Referee) |  |  |  |  |

| table | function | table(&self) |  |  |  |  |

| timing | function | timing(&self, cond_a: &Conditions, cond_b: &Conditions) |  |  |  |  |

| to_json | function | to_json(&self) |  |  |  |  |

| total_a | function | total_a(&self) |  |  |  |  |

| total_b | function | total_b(&self) |  |  |  |  |

| unstamped | function | unstamped(&self) |  |  |  |  |

| value | function | value(self) |  |  |  |  |

| variants | function | variants(&self) |  |  |  |  |

| with_trend | function | with_trend(mut self, t: Trend) |  |  |  |  |

| worse | function | worse(self, other: Cleanliness) |  |  |  |  |

| Conditions | struct |  |  |  |  |  |

| CoverageDiff | struct |  |  |  |  |  |

| Diff | struct |  |  |  |  |  |

| Gained | struct |  |  |  |  |  |

| InstanceKey | struct |  |  |  |  |  |

| Loaded | struct |  |  |  |  |  |

| Lost | struct |  |  |  |  |  |

| Percentiles | struct |  |  |  |  |  |

| QualityDiff | struct |  |  |  |  |  |

| RunRef | struct |  |  |  |  |  |

| TimingDiff | struct |  |  |  |  |  |

| VariantRow | struct |  |  |  |  |  |

| cell | function | cell(
        &self,
        label: &str,
        rows: &[RawRow],
        referee: &Referee,
        solved: usize,
        total: usize,
    ) |  |  |  |  |

| cohort | function | cohort(&self, label: &str) |  |  |  |  |

| load | function | load(benchmarks_dir: &Path) |  |  |  |  |

| placement | function | placement(&self, s: usize, n: usize) |  |  |  |  |

| unmatched_splits | function | unmatched_splits(&self, label: &str, rows: &[RawRow]) |  |  |  |  |

| warnings | function | warnings(&self) |  |  |  |  |

| Cohort | struct |  |  |  |  |  |

| Entrant | struct |  |  |  |  |  |

| FieldBook | struct |  |  |  |  |  |

| bar | function | bar(pct: f64, width: usize) |  |  |  |  |

| fmt_f | function | fmt_f(x: f64, places: usize) |  |  |  |  |

| ordinal | function | ordinal(n: usize) |  |  |  |  |

| ordinal_suffix | function | ordinal_suffix(n: usize) |  |  |  |  |

| pct | function | pct(solved: usize, total: usize) |  |  |  |  |

| py_round | function | py_round(x: f64, ndigits: i32) |  |  |  |  |

| py_round_i | function | py_round_i(x: f64) |  |  |  |  |

| thousands | function | thousands(n: u64) |  |  |  |  |

| HistoryError | enum |  |  |  |  |  |

| as_str | function | as_str(&self) |  |  |  |  |

| comparable_predecessor | function | comparable_predecessor(
        &self,
        box_: &BoxId,
        cur: &VersionKey,
    ) |  |  |  |  |

| current_version | function | current_version(root: &Path) |  |  |  |  |

| delta | function | delta(&self, label: &str, solved: usize, total: usize) |  |  |  |  |

| delta_cell | function | delta_cell(
    prev: Option<&ComparablePredecessor<'_>>,
    label: &str,
    solved: usize,
    total: usize,
) |  |  |  |  |

| from_json | function | from_json(src: &str) |  |  |  |  |

| load | function | load(path: &Path) |  |  |  |  |

| new | function | new(s: impl Into<String>) |  |  |  |  |

| parse | function | parse(s: &str) |  |  |  |  |

| parts | function | parts(&self) |  |  |  |  |

| snapshots | function | snapshots(&self) |  |  |  |  |

| to_json | function | to_json(&self) |  |  |  |  |

| track | function | track(&self, label: &str) |  |  |  |  |

| trend | function | trend(&self, label: &str, box_: &BoxId) |  |  |  |  |

| try_load | function | try_load(path: &Path) |  |  |  |  |

| upsert | function | upsert(&mut self, s: Snapshot) |  |  |  |  |

| version | function | version(&self) |  |  |  |  |

| version_key | function | version_key(&self) |  |  |  |  |

| BoxId | struct |  |  |  |  |  |

| ComparablePredecessor | struct |  |  |  |  |  |

| History | struct |  |  |  |  |  |

| MeasuredAt | struct |  |  |  |  |  |

| Snapshot | struct |  |  |  |  |  |

| Trend | struct |  |  |  |  |  |

| VersionKey | struct |  |  |  |  |  |

| parse_rows | function | parse_rows(src: &str, path: &str) |  |  |  |  |

| ManifestError | enum |  |  |  |  |  |

| PatternError | enum |  |  |  |  |  |

| board | function | board(&self, id: &str) |  |  |  |  |

| board_by_label | function | board_by_label(&self, label: &str) |  |  |  |  |

| board_by_raw | function | board_by_raw(&self, raw: &str) |  |  |  |  |

| errors | function | errors(&self) |  |  |  |  |

| is_match | function | is_match(&self, variant: &str) |  |  |  |  |

| is_proof_track | function | is_proof_track(&self, label: &str) |  |  |  |  |

| load | function | load(path: &Path) |  |  |  |  |

| parse | function | parse(src: &str) |  |  |  |  |

| rebaselined_on | function | rebaselined_on(&self, label: &str, box_: &str) |  |  |  |  |

| search | function | search(&self, hay: &str) |  |  |  |  |

| selector | function | selector(&self) |  |  |  |  |

| selects | function | selects(&self, variant: &str) |  |  |  |  |

| set | function | set(&self, name: &str) |  |  |  |  |

| track | function | track(&self, name: &str) |  |  |  |  |

| validate | function | validate(&self) |  |  |  |  |

| warnings | function | warnings(&self) |  |  |  |  |

| BoardSpec | struct |  |  |  |  |  |

| CorpusSpec | struct |  |  |  |  |  |

| Defaults | struct |  |  |  |  |  |

| Manifest | struct |  |  |  |  |  |

| Pattern | struct |  |  |  |  |  |

| Selector | struct |  |  |  |  |  |

| SetSpec | struct |  |  |  |  |  |

| TrackSpec | struct |  |  |  |  |  |

| Gate | enum |  |  |  |  |  |

| PromoteError | enum |  |  |  |  |  |

| TierMovePolicy | enum |  |  |  |  |  |

| accepts | function | accepts(&self, board: &str) |  |  |  |  |

| against | function | against(&self) |  |  |  |  |

| apply | function | apply(&self) |  |  |  |  |

| board | function | board(&self) |  |  |  |  |

| boards | function | boards(&self) |  |  |  |  |

| bytes | function | bytes(&self) |  |  |  |  |

| changes | function | changes(&self) |  |  |  |  |

| compute | function | compute(
        box_: &str,
        prev: Option<&ComparablePredecessor<'_>>,
        live: &[LiveBoard],
    ) |  |  |  |  |

| denominator_grew | function | denominator_grew(&self) |  |  |  |  |

| dst | function | dst(&self) |  |  |  |  |

| entries | function | entries(&self) |  |  |  |  |

| failures | function | failures(&self) |  |  |  |  |

| full_table | function | full_table(&self) |  |  |  |  |

| like_for_like | function | like_for_like(&self) |  |  |  |  |

| pct | function | pct(&self) |  |  |  |  |

| pct_dropped_on_entry_day | function | pct_dropped_on_entry_day(&self) |  |  |  |  |

| placements | function | placements(&self) |  |  |  |  |

| plan | function | plan(
    root: &Path,
    manifest: &Manifest,
    sets: &[&str],
    policy: &TierMovePolicy,
) |  |  |  |  |

| promoted_lines | function | promoted_lines(&self) |  |  |  |  |

| refusal | function | refusal(&self) |  |  |  |  |

| sentence | function | sentence(&self, box_: &str) |  |  |  |  |

| solved | function | solved(&self) |  |  |  |  |

| src | function | src(&self) |  |  |  |  |

| tier_moves | function | tier_moves(&self) |  |  |  |  |

| total | function | total(&self) |  |  |  |  |

| Change | struct |  |  |  |  |  |

| GateReport | struct |  |  |  |  |  |

| Headline | struct |  |  |  |  |  |

| LiveBoard | struct |  |  |  |  |  |

| Placement | struct |  |  |  |  |  |

| Promotion | struct |  |  |  |  |  |

| TierMove | struct |  |  |  |  |  |

| TwoHeadlines | struct |  |  |  |  |  |

| write_indent1 | function | write_indent1(v: &serde_json::Value, out: &mut String) |  |  |  |  |

| write_str | function | write_str(s: &str, out: &mut String) |  |  |  |  |

| write_value | function | write_value(v: &serde_json::Value, out: &mut String) |  |  |  |  |

| Currency | enum |  |  |  |  |  |

| QualityNote | enum |  |  |  |  |  |

| bounds_wtl | function | bounds_wtl(
    rows: &[RawRow],
    referee: &Referee,
    bounds: &BestKnownBounds,
    year_key: &str,
    variant_suffix: &str,
) |  |  |  |  |

| currency | function | currency(&self) |  |  |  |  |

| l | function | l(&self) |  |  |  |  |

| length_wtl | function | length_wtl(rows: &[RawRow], referee: &Referee, arch: &Ipc5Archive) |  |  |  |  |

| makespan_wtl | function | makespan_wtl(rows: &[RawRow], referee: &Referee, arch: &Ipc5Archive) |  |  |  |  |

| mean | function | mean(&self) |  |  |  |  |

| n | function | n(&self) |  |  |  |  |

| new | function | new(scored: Option<Wtl>, fallback: impl Into<String>) |  |  |  |  |

| prefix | function | prefix(self) |  |  |  |  |

| render | function | render(&self) |  |  |  |  |

| t | function | t(&self) |  |  |  |  |

| w | function | w(&self) |  |  |  |  |

| Wtl | struct |  |  |  |  |  |

| Instance | enum |  |  |  |  |  |

| Notes | enum |  |  |  |  |  |

| as_num | function | as_num(&self) |  |  |  |  |

| current | function | current(solved: bool) |  |  |  |  |

| domain_key | function | domain_key(&self) |  |  |  |  |

| note_text | function | note_text(&self) |  |  |  |  |

| of | function | of(o: &serde_json::Map<String, serde_json::Value>) |  |  |  |  |

| text | function | text(&self) |  |  |  |  |

| time_secs | function | time_secs(&self) |  |  |  |  |

| write_row | function | write_row(r: &RawRow, out: &mut String) |  |  |  |  |

| Present | struct |  |  |  |  |  |

| RawRow | struct |  |  |  |  |  |

| budget_for | function | budget_for(&self, r: &RawRow, registry: f64) |  |  |  |  |

| classify | function | classify(&self, r: &RawRow, registry_budget: f64) |  |  |  |  |

| contains | function | contains(&self, r: &RawRow) |  |  |  |  |

| coverage | function | coverage(&self, rows: &[RawRow], registry_budget: f64) |  |  |  |  |

| is_empty | function | is_empty(&self) |  |  |  |  |

| is_solved | function | is_solved(&self, r: &RawRow) |  |  |  |  |

| new | function | new(val_unavailable: ValUnavailable) |  |  |  |  |

| Referee | struct |  |  |  |  |  |

| ValUnavailable | struct |  |  |  |  |  |

| render | function | render(ctx: &RenderCtx) |  |  |  |  |

| CtxError | enum |  |  |  |  |  |

| absent_cell | function | absent_cell(&self, label: &str) |  |  |  |  |

| board | function | board(&self, label: &str) |  |  |  |  |

| coverage_of | function | coverage_of(&self, rows: &[RawRow], budget: f64) |  |  |  |  |

| data | function | data(&self, label: &str) |  |  |  |  |

| load | function | load(root: &Path, box_id: BoxId) |  |  |  |  |

| new | function | new(
        manifest: Manifest,
        boards: Vec<BoardRows>,
        referee: Referee,
        archive: Ipc5Archive,
        bounds: BestKnownBounds,
        field: FieldBook,
        history: History,
        version: Option<String>,
        box_id: BoxId,
    ) |  |  |  |  |

| predecessor | function | predecessor(&self) |  |  |  |  |

| proof_mark | function | proof_mark(&self, label: &str) |  |  |  |  |

| split | function | split(&self, label: &str, ipc: &str) |  |  |  |  |

| standings | function | standings(&self) |  |  |  |  |

| BoardRows | struct |  |  |  |  |  |

| LiveBoard | struct |  |  |  |  |  |

| RenderCtx | struct |  |  |  |  |  |

| Standings | struct |  |  |  |  |  |

| block | function | block(ctx: &RenderCtx) |  |  |  |  |

| patch | function | patch(readme_text: &str, block: &str) |  |  |  |  |

| render | function | render(ctx: &RenderCtx) |  |  |  |  |

| SnapshotError | enum |  |  |  |  |  |

| Source | enum |  |  |  |  |  |

| bank | function | bank(
    root: &Path,
    manifest: &Manifest,
    referee: &Referee,
    args: &Args,
) |  |  |  |  |

| parse_args | function | parse_args(argv: &[String], env_box: Option<&str>) |  |  |  |  |

| tracks | function | tracks(
    root: &Path,
    manifest: &Manifest,
    referee: &Referee,
    source: &Source,
) |  |  |  |  |

| write | function | write(&self) |  |  |  |  |

| Args | struct |  |  |  |  |  |

| Banked | struct |  |  |  |  |  |

| incident | function | incident(name: &str) |  |  |  |  |

| real_val_map | function | real_val_map() |  |  |  |  |

| run | function | run(repo: &Path, cfg: &crate::config::Config, o: Opts<'_>) |  |  |  |  |

| stage_for | function | stage_for(ver: &str) |  |  |  |  |

| Opts | struct |  |  |  |  |  |

| in_quiet_hours | function | in_quiet_hours(&self, minutes_past_midnight: u32) |  |  |  |  |

| load | function | load(path: &std::path::Path) |  |  |  |  |

| path | function | path() |  |  |  |  |

| Config | struct |  |  |  |  |  |

| Contention | struct |  |  |  |  |  |

| Db | struct |  |  |  |  |  |

| QuietHours | struct |  |  |  |  |  |

| Referee | struct |  |  |  |  |  |

| Repo | struct |  |  |  |  |  |

| Scheduler | struct |  |  |  |  |  |

| Sweep | struct |  |  |  |  |  |

| Ui | struct |  |  |  |  |  |

| compare | function | compare(
    repo: &Path,
    cfg: &crate::config::Config,
    set_name: &str,
    a: &str,
    b: &str,
    select: &crate::select::Select,
    lost: Option<&Path>,
) |  |  |  |  |

| frame | function | frame(repo: &Path, cfg: &crate::config::Config, set_name: &str) |  |  |  |  |

| run | function | run(repo: &Path, cfg: &crate::config::Config, set_name: &str) |  |  |  |  |

| status | function | status(
    repo: &Path,
    cfg: &crate::config::Config,
    set_name: &str,
    json: bool,
) |  |  |  |  |

| flush_to_stdout | function | flush_to_stdout() |  |  |  |  |

| is_quiet | function | is_quiet() |  |  |  |  |

| quiet | function | quiet(on: bool) |  |  |  |  |

| recent | function | recent(n: usize) |  |  |  |  |

| say | function | say(line: String) |  |  |  |  |

| tz_offset_secs | function | tz_offset_secs() |  |  |  |  |

| RepoError | enum |  |  |  |  |  |

| build_planner | function | build_planner(dir: &Path) |  |  |  |  |

| candidate_path | function | candidate_path(repo: &Path) |  |  |  |  |

| probe | function | probe(path: &Path) |  |  |  |  |

| require_version | function | require_version(&self, want: &str) |  |  |  |  |

| short_hash | function | short_hash(&self) |  |  |  |  |

| supports_mode | function | supports_mode(&self, mode: &str) |  |  |  |  |

| worktree_for | function | worktree_for(worktree_dir: &Path, tag: &str) |  |  |  |  |

| Engine | struct |  |  |  |  |  |

| run | function | run(repo: &Path, cfg: &Config, o: Opts<'_>) |  |  |  |  |

| tags | function | tags(repo: &Path) |  |  |  |  |

| Opts | struct |  |  |  |  |  |

| Prior | enum |  |  |  |  |  |

| admits | function | admits(
        &self,
        variant: &str,
        label: &str,
        prior: &BTreeMap<String, (bool, Option<f64>) |  |  |  |  |

| describe | function | describe(&self) |  |  |  |  |

| from_args | function | from_args(a: &SelectArgs) |  |  |  |  |

| is_subset | function | is_subset(&self) |  |  |  |  |

| needs_prior | function | needs_prior(&self) |  |  |  |  |

| parse_rows | function | parse_rows(src: &str) |  |  |  |  |

| stage | function | stage(&self, repo: &Path, set: &str, engine_ver: &str, engine_hash: &str) |  |  |  |  |

| wants_board | function | wants_board(&self, id: &str) |  |  |  |  |

| Select | struct |  |  |  |  |  |

| SelectArgs | struct |  |  |  |  |  |

| attach | function | attach(&self, tx: mpsc::Sender<Ctl>) |  |  |  |  |

| attached | function | attached(&self) |  |  |  |  |

| calibrate | function | calibrate(
        &mut self,
        prior: Option<f64>,
        record: &dyn Fn(f64) |  |  |  |  |

| canary | function | canary(&self) |  |  |  |  |

| ctl_for | function | ctl_for(from: Level, to: Level, demote_ok: bool) |  |  |  |  |

| demote_ok | function | demote_ok(&self) |  |  |  |  |

| detach | function | detach(&self, id: u64) |  |  |  |  |

| from_config | function | from_config(c: &crate::config::Scheduler) |  |  |  |  |

| held | function | held(&self) |  |  |  |  |

| hold | function | hold(&self, on: bool) |  |  |  |  |

| label | function | label(&self) |  |  |  |  |

| level | function | level(&self) |  |  |  |  |

| new | function | new(manifest: &'a Manifest, setup: Setup<'_>) |  |  |  |  |

| policy_mem_budget | function | policy_mem_budget(mem_bytes: u64, at_the_box: bool, pack: &Pack) |  |  |  |  |

| policy_width | function | policy_width(
    level: Level,
    quiet_hours: bool,
    user_idle_secs: Option<f64>,
    foreign_pcpu: f64,
    pack: &Pack,
) |  |  |  |  |

| read | function | read(&mut self, on_spawn: Option<&dyn Fn(Pid, f64) |  |  |  |  |

| reason | function | reason(&self) |  |  |  |  |

| resolve | function | resolve(
        repo: &Path,
        engine: &Path,
        engine_hash: &str,
        r: &crate::config::Referee,
    ) |  |  |  |  |

| run | function | run(repo: &Path, cfg: &crate::config::Config, o: Opts<'_>) |  |  |  |  |

| run_engine | function | run_engine(
    repo: &Path,
    cfg: &crate::config::Config,
    o: Opts<'_>,
    manifest: &Manifest,
    engine: crate::repo::Engine,
    stage: Option<PathBuf>,
) |  |  |  |  |

| send | function | send(&self, c: Ctl) |  |  |  |  |

| set_canary | function | set_canary(&self, factor: f64) |  |  |  |  |

| set_demote_ok | function | set_demote_ok(&self, on: bool) |  |  |  |  |

| set_level | function | set_level(&self, level: Level, reason: Option<String>) |  |  |  |  |

| set_width | function | set_width(&self, w: usize) |  |  |  |  |

| solo | function | solo() |  |  |  |  |

| throttle_config | function | throttle_config(c: &crate::config::Contention) |  |  |  |  |

| total_instances | function | total_instances(&self) |  |  |  |  |

| width | function | width(&self) |  |  |  |  |

| Canary | struct |  |  |  |  |  |

| DbCtx | struct |  |  |  |  |  |

| Opts | struct |  |  |  |  |  |

| Pack | struct |  |  |  |  |  |

| Progress | struct |  |  |  |  |  |

| Running | struct |  |  |  |  |  |

| Setup | struct |  |  |  |  |  |

| Shared | struct |  |  |  |  |  |

| SweepRunner | struct |  |  |  |  |  |

| Cell | enum |  |  |  |  |  |

| Level | enum |  |  |  |  |  |

| LogKind | enum |  |  |  |  |  |

| Sort | enum |  |  |  |  |  |

| View | enum |  |  |  |  |  |

| back | function | back(&mut self) |  |  |  |  |

| banked | function | banked(self) |  |  |  |  |

| board | function | board(&self) |  |  |  |  |

| cycle_sort | function | cycle_sort(&mut self) |  |  |  |  |

| delta_secs | function | delta_secs(&self) |  |  |  |  |

| dismiss_toasts | function | dismiss_toasts(&mut self) |  |  |  |  |

| done | function | done(&self) |  |  |  |  |

| enter | function | enter(&mut self) |  |  |  |  |

| expire_toasts | function | expire_toasts(&mut self, dwell: Duration) |  |  |  |  |

| frac | function | frac(&self) |  |  |  |  |

| gain | function | gain(&self) |  |  |  |  |

| gains | function | gains(&self) |  |  |  |  |

| jump | function | jump(&mut self, to_end: bool) |  |  |  |  |

| label | function | label(self) |  |  |  |  |

| move_selection | function | move_selection(&mut self, delta: isize) |  |  |  |  |

| near_wall | function | near_wall(&self) |  |  |  |  |

| next | function | next(self) |  |  |  |  |

| owed | function | owed(&self) |  |  |  |  |

| prev_solved | function | prev_solved(&self) |  |  |  |  |

| rank | function | rank(self) |  |  |  |  |

| regression | function | regression(&self) |  |  |  |  |

| regressions | function | regressions(&self) |  |  |  |  |

| rho | function | rho(&self) |  |  |  |  |

| running | function | running(&self) |  |  |  |  |

| selected_instance | function | selected_instance(&self) |  |  |  |  |

| solve_secs | function | solve_secs(&self) |  |  |  |  |

| solved | function | solved(&self) |  |  |  |  |

| sorted_instances | function | sorted_instances(&self) |  |  |  |  |

| strip | function | strip(cells: &[InstanceCell], width: usize) |  |  |  |  |

| tally | function | tally(&mut self) |  |  |  |  |

| toggle_timeline | function | toggle_timeline(&mut self) |  |  |  |  |

| total | function | total(&self) |  |  |  |  |

| AttemptRow | struct |  |  |  |  |  |

| BoardRow | struct |  |  |  |  |  |

| InstanceCell | struct |  |  |  |  |  |

| InstanceDetail | struct |  |  |  |  |  |

| LevelState | struct |  |  |  |  |  |

| LogLine | struct |  |  |  |  |  |

| Slot | struct |  |  |  |  |  |

| SlotRun | struct |  |  |  |  |  |

| Snapshot | struct |  |  |  |  |  |

| StripCol | struct |  |  |  |  |  |

| SweepProgress | struct |  |  |  |  |  |

| Timeline | struct |  |  |  |  |  |

| TimelinePoint | struct |  |  |  |  |  |

| Toast | struct |  |  |  |  |  |

| compact | function | compact(text: &str) |  |  |  |  |

| render | function | render(text: &str, max_width: usize) |  |  |  |  |

| detail | function | detail() |  |  |  |  |

| snapshot | function | snapshot(t: f64) |  |  |  |  |

| draw | function | draw(f: &mut Frame, s: &Snapshot, th: &Theme, banner_text: &str) |  |  |  |  |

| new | function | new(progress: Arc<Mutex<Progress>>, shared: Arc<Shared>) |  |  |  |  |

| next | function | next(&mut self, prev: &Snapshot) |  |  |  |  |

| Feed | struct |  |  |  |  |  |

| Action | enum |  |  |  |  |  |

| action_for | function | action_for(k: KeyEvent, s: &mut Snapshot) |  |  |  |  |

| enter | function | enter() |  |  |  |  |

| TerminalGuard | struct |  |  |  |  |  |

| Depth | enum |  |  |  |  |  |

| bar_cells | function | bar_cells(&self) |  |  |  |  |

| detect | function | detect() |  |  |  |  |

| forge | function | forge() |  |  |  |  |

| glyph | function | glyph(&self, unicode: &'static str, ascii: &'static str) |  |  |  |  |

| spark_cells | function | spark_cells(&self) |  |  |  |  |

| unicode_ok | function | unicode_ok() |  |  |  |  |

| Theme | struct |  |  |  |  |  |

| bar | function | bar(theme: &Theme, frac: f64, width: usize) |  |  |  |  |

| duration | function | duration(secs: u64) |  |  |  |  |

| ellipsize | function | ellipsize(s: &str, width: usize) |  |  |  |  |

| spark | function | spark(theme: &Theme, values: &[f64], width: usize) |  |  |  |  |

| until | function | until(secs: u64) |  |  |  |  |


<!-- ============================================================= -->
<!-- AGENT-FORBIDDEN-END: nothing below this line may describe     -->
<!-- code behavior.                                                -->
<!-- ============================================================= -->
