"""MuStar Signatures — Domain-Agnostic Two-Pass Semantic Planning.

Ported from `~/chatmangpt/ostar/src/ostar/process/mu_star_signatures.py`
near-verbatim (zero deps beyond `dspy`) -- this is the actual intellectual
content of MuStar: one signature call produces three concurrent
representations of a solution strategy (flat build order, formal POWL
control-flow model, sequence diagram) across arbitrary problem domains,
something ferroplan's PDDL-only planner has no equivalent of.

Pass 1 (MuStarPlanSignature):
  problem_statement + domain + constraints -> build_order, powl_model, sequence_diagram

Pass 2 (MuStarExecuteSignature):
  problem_statement + build_order + powl_model + sequence_diagram -> artifact

Pass 3 (MuStarRefineSignature):
  original_build_order + failure_feedback + domain + constraints -> refined_build_order

Domain coverage:
  ALGORITHM — Pure functions, standard library only
  BACKEND_API — REST/RPC services, databases, caching
  DATA_PIPELINE — Streaming, batch processing, transformations
  DATABASE_DESIGN — Schema, transactions, indexing
  FRONTEND_COMPONENT — UI, state management, forms
  ONTOLOGY — RDF/OWL classes, properties, constraints
  WORKFLOW — BPMN processes, state machines, orchestration
  MANUFACTURING — CodeManufactory operators, proof gates, receipts
  SYSTEM_DESIGN — Architecture, fault tolerance, distributed coordination
  SECURITY_REVIEW — Threat modeling, vulnerability analysis, hardening

The build order format is universal:
  -> (explore: [10 activities], plan: [10 activities], write: [10 activities])
"""

import dspy


class MuStarPlanSignature(dspy.Signature):
    """Generate a semantic build order for any problem domain.

    This is the planning phase (Z-network) of MuStar: read problem + domain + constraints,
    output THREE concurrent representations of the solution strategy:
    1. Build order (flat ordered activities)
    2. POWL model (formal control flow with operators)
    3. Sequence diagram (visual execution flow)

    All three describe the same strategy but serve different purposes:
    - Build order: what to do (activity enumeration, domain-specific)
    - POWL: how to do it (control flow with loops, branches, sequences)
    - Sequence diagram: when/where (execution timeline and state transitions)

    Domain-specific precision:
    - For CODE domains: exact function/module names, blocking order by constraints
    - For ONTOLOGY: exact class names, property names, constraint types
    - For WORKFLOW: exact task names, state machine transitions, swimlanes
    - For MANUFACTURING: exact operator names, proof gate sequences, receipt chains

    The executor (Pass 2) receives all three and implements activities in strict order,
    using POWL as formal spec and diagram as visual reference.
    Problem constraints drive timing and ordering decisions.
    """

    problem_statement: str = dspy.InputField(
        desc="Natural language problem statement. May include requirements, constraints, examples, acceptance criteria."
    )
    domain: str = dspy.InputField(
        desc="Problem domain determines build order strategy: ALGORITHM (pure functions), BACKEND_API (REST/RPC), DATA_PIPELINE (streaming/batch), DATABASE_DESIGN (schema), FRONTEND_COMPONENT (UI/state), ONTOLOGY (RDF/OWL), WORKFLOW (BPMN), MANUFACTURING (CodeManufactory), SYSTEM_DESIGN (architecture), SECURITY_REVIEW (threat model)"
    )
    constraints: str = dspy.InputField(
        desc="Technical constraints that drive activity ordering: latency (cache/async), scale (throughput), consistency (idempotency), persistence, compliance, security"
    )

    build_order: str = dspy.OutputField(
        desc=(
            "Flat ordered list of SPECIFIC activities by phase (explore → plan → write). "
            "FORMAT (required): -> (explore: [activity1, activity2, ...], plan: [activity_a, activity_b, ...], write: [activity_x, activity_y, ...]) "
            "RULES: (1) EXACTLY 10 activities per phase: explore=10, plan=10, write=10. Total: 30 activities. "
            "(2) Exact, specific names (not generic descriptions). For CODE: function/module names. For ONTOLOGY: class/property URIs. For WORKFLOW: task names. "
            "(3) Order matters — most blocking/critical constraint first. (4) No annotations — no brackets, no metadata embedded. "
            "EXAMPLE (ALGORITHM — closest pair): -> (explore: [range_validation, sorted_collection, distance_metric, pairs_enumeration, early_termination, numerical_stability, edge_cases_empty, edge_cases_single, floating_point_precision, performance_baseline], "
            "plan: [sorted_array_data_structure, min_distance_tracker, pair_tuple_representation, loop_bounds_calculation, comparison_operator_definition, distance_calculation_formula, termination_condition_check, collection_iteration_pattern, result_representation_choice, performance_measure_mechanism], "
            "write: [validate_input_range, sort_collection, initialize_min_distance, outer_loop_pairs, inner_loop_increment, distance_compute_call, comparison_and_update, early_exit_check, return_result_tuple, performance_tracking_wrapper])"
        )
    )

    powl_model: str = dspy.OutputField(
        desc=(
            "Formal POWL (Partially Ordered Workflow Language) with control-flow operators. "
            "OPERATORS: SEQ() for strict sequence, PO{} for partial order, LOOP() for iteration, IF() for conditional branches. "
            "RULES: (1) Capture loops explicitly (LOOP for 'repeat until' or 'for each'). "
            "(2) Capture branches explicitly (IF for 'only when condition'). "
            "(3) Use SEQ() for sequential steps. (4) Use PO{} for tasks that can overlap. (5) Nest operators to show structure. "
            "EXAMPLE (ALGORITHM): LOOP(read_pair, IF(distance_less, update_min), next_pair) "
            "EXAMPLE (ONTOLOGY): SEQ(define_classes, LOOP(add_property, IF(is_object_property, add_domain_range)), add_constraints) "
            "EXAMPLE (WORKFLOW): SEQ(receive_order, IF(valid, PO{validate_payment, check_inventory}), IF(approved, pack_ship, notify_rejection)) "
            "The POWL describes WHEN/WHERE loops and branches happen. Activities in build_order list WHAT to do."
        )
    )

    sequence_diagram: str = dspy.OutputField(
        desc=(
            "Mermaid diagram showing execution flow. Choose the diagram type that best expresses the problem structure. "
            "DIAGRAM TYPES: "
            "(a) flowchart TD — for algorithmic control flow with branches/loops; "
            "(b) sequenceDiagram — for request/response or message-passing between services; "
            "(c) stateDiagram-v2 — for object lifecycle or state machine transitions; "
            "(d) classDiagram — for data model with relationships; "
            "(e) erDiagram — for database schema and entity relationships; "
            "(f) gantt — for phase scheduling and parallel execution timelines. "
            "RULES: (1) Show all write: activities from build_order. (2) Show control flow (loops, branches, decisions). "
            "(3) Match diagram type to domain: CODE (algorithm/API)→flowchart/sequence, ONTOLOGY→classDiagram, WORKFLOW→stateDiagram, DATABASE→erDiagram. "
            "(4) Complex problems: use TWO diagram types (e.g., sequenceDiagram for API interactions + stateDiagram-v2 for artifact state). "
            "EXAMPLE (ALGORITHM): flowchart TD; Start --> Validate{valid input?}; Validate -->|No| ReturnEmpty; "
            "Validate -->|Yes| Sort[sort array]; Sort --> Loop[for each pair]; Loop --> Compute{distance < min?}; "
            "Compute -->|Yes| Update[update min]; Update --> Loop; Compute -->|No| Loop; Loop --> Return[return pair]. "
            "EXAMPLE (ONTOLOGY): classDiagram; Disease --|> MedicalEntity; Symptom --|> MedicalEntity; "
            "Treatment --|> Intervention; Disease -->|hasSympom| Symptom; Disease -->|hasUseful| Treatment. "
            "The diagram makes POWL structure visible and catches missing loops/branches."
        )
    )


class MuStarExecuteSignature(dspy.Signature):
    """Execute a semantic build order to produce a domain-specific artifact.

    This is the execution phase (action head) of MuStar: read problem + strategy,
    output complete implementation of all write: phase activities.

    Strategy is provided three ways (all describe the same plan):
    1. Build order (flat ordered activities)
    2. POWL model (formal control flow with operators)
    3. Sequence diagram (visual execution flow)

    Use all three to understand the strategy, then implement it in the target format.

    Execution rules:
    - Explore activities tell you what gaps were discovered
    - Plan activities tell you the architecture and data structures
    - Write activities list the EXACT ACTIVITIES YOU MUST IMPLEMENT
    - POWL model shows where loops and branches happen
    - Sequence diagram shows the execution order visually
    - Every activity in write: must appear in your artifact
    - Problem statement is the ground truth — constraints drive implementation choices

    Artifact format depends on domain:
    - CODE (ALGORITHM, BACKEND_API, FRONTEND_COMPONENT, DATA_PIPELINE): Python/Go/Rust source code
    - ONTOLOGY: RDF N-Triples or SPARQL CONSTRUCT
    - WORKFLOW: BPMN 2.0 XML or YAML state machine
    - MANUFACTURING: YAML manufacturing configuration with operators, gates, receipts
    - DATABASE_DESIGN: SQL DDL with schema, indexes, constraints
    - SYSTEM_DESIGN: Architecture specification or deployment config
    - SECURITY_REVIEW: Threat model or vulnerability report (structured)

    DO NOT INCLUDE:
    - Test code, test fixtures, mock data
    - Comments that repeat what code does (names should be self-documenting)
    - Incomplete implementations or TODOs
    - Placeholder or stub functions
    """

    problem_statement: str = dspy.InputField(
        desc="Original problem statement. Constraints in this statement drive implementation choices."
    )
    build_order: str = dspy.InputField(
        desc="Flat ordered list of specific activities from planning phase. Every write: activity must be implemented."
    )
    powl_model: str = dspy.InputField(
        desc="POWL control-flow model showing where loops and branches occur in the implementation."
    )
    sequence_diagram: str = dspy.InputField(
        desc="Visual execution flow diagram. Use as reference for control flow and state transitions."
    )

    artifact: str = dspy.OutputField(
        desc=(
            "The generated artifact in domain-specific format. "
            "For CODE domains: Complete, runnable source code. For ONTOLOGY: RDF N-Triples or SPARQL CONSTRUCT. "
            "For WORKFLOW: BPMN 2.0 XML. For MANUFACTURING: YAML config. For DATABASE: SQL DDL. "
            "For SYSTEM_DESIGN: Architecture spec. For SECURITY_REVIEW: Structured threat model. "
            "Must implement all write: activities from build_order. No stubs, no TODOs, no incomplete implementations."
        )
    )
    artifact_type: str = dspy.OutputField(
        desc=(
            "Type tag categorizing the artifact. "
            "Valid values: python_code, go_code, rust_code, rdf_ntriples, sparql_construct, bpmn_xml, yaml_config, sql_ddl, "
            "architecture_spec, threat_model, or domain-appropriate tag."
        )
    )
    operator_notation: str = dspy.OutputField(
        desc=(
            "A short label for the kind of transformation this artifact represents "
            "(e.g. breed-ontology, compile-artifact, run-validation) — freeform, "
            "domain-appropriate; no fixed external scheme is assumed here."
        )
    )
    build_order_adhered: bool = dspy.OutputField(
        desc="Whether execution followed the build_order (true) or deviated (false). If false, note deviations in implementation."
    )
    implementation_complete: bool = dspy.OutputField(
        desc="Whether the artifact fully addresses problem_statement and implements all write: activities (true) or is incomplete (false)."
    )


class MuStarRefineSignature(dspy.Signature):
    """Refine a failed build order based on execution feedback.

    This is the refinement loop: when Pass 2 (MuStarExecute) fails to generate a
    valid artifact, MuStarRefineSignature takes the failed attempt and problem context,
    then produces an improved build order for retry.

    Refinement triggers:
    - artifact_type mismatch (claimed ONTOLOGY but generated CODE)
    - build_order_adhered=false (implementation diverged from plan)
    - implementation_complete=false (artifact doesn't solve the problem)
    - conformance check failed (artifact violates domain constraints)

    Refinement strategy:
    1. Analyze what went wrong in the failed attempt
    2. Identify which activities were skipped, misdirected, or oversimplified
    3. Reorder activities to fix the blocking constraint
    4. Add specificity where activities were too generic
    5. Rebalance explore/plan/write phases if one was overloaded

    The refined build_order is retried immediately in Pass 2 (MuStarExecute).
    A loop of up to 3-5 refinement attempts is typical before escalation.
    """

    original_build_order: str = dspy.InputField(
        desc="The failed build order from planning phase that needs refinement."
    )
    failure_feedback: str = dspy.InputField(
        desc="Detailed feedback on why execution failed: artifact_type mismatch, activities skipped, constraints violated, incomplete implementation, examples of what went wrong."
    )
    domain: str = dspy.InputField(
        desc="Problem domain (same as original). Helps interpret what activities were misdirected."
    )
    constraints: str = dspy.InputField(
        desc="Problem constraints (same as original). Helps identify which constraint was violated."
    )

    refined_build_order: str = dspy.OutputField(
        desc=(
            "Improved build order addressing the failure. "
            "FORMAT: -> (explore: [10 activities], plan: [10 activities], write: [10 activities]) "
            "CHANGES: (1) Reorder activities to fix the blocking constraint that caused failure. "
            "(2) Add specificity to activities that were too generic. "
            "(3) Balance phases if one was overloaded. "
            "(4) Ensure all 10 activities per phase are present. "
            "The refined_build_order is retried in Pass 2 immediately."
        )
    )
    confidence: float = dspy.OutputField(
        desc="Confidence that refined_build_order will succeed (0.0-1.0). Used to decide whether to retry or escalate."
    )


__all__ = ["MuStarExecuteSignature", "MuStarPlanSignature", "MuStarRefineSignature"]
