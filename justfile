export PATH := env_var('HOME') / ".local/bin:" + env_var('PATH')
scripts := "plugins/chatman-ecosystem/scripts"

# List available recipes
default:
    @just --list

# Build the full Rust workspace
build:
    cargo build --workspace

# Run Rust + Python test suites
test:
    cargo test --workspace
    cd plugins/chatman-ecosystem && uv run pytest tests/

# Automated ALIVE/BLOCKED audit -- real commands, real exit codes, no
# standing reported from source presence alone (see doctor.py).
doctor:
    cd plugins/chatman-ecosystem && uv run python3 scripts/doctor.py

# Single-planner benchmark, real VAL scoring, N real corpus problems
bench N="5":
    cd plugins/chatman-ecosystem && uv run python3 scripts/planner_benchmark.py run \
        --sample-size {{N}} --ocel /tmp/bench-$(date +%s).ocel.json

# Launch the bounded overnight autonomics loop
overnight max_cycles="40" max_hours="8":
    cd {{scripts}} && python3 overnight_autonomics.py \
        --max-cycles {{max_cycles}} --max-hours {{max_hours}} --cycle-pause-seconds 300
