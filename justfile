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

# Regenerate wja registry (root) and beam-host capsule (crates/ferroplan-wasm) via ggen
wasm-gen:
    ggen sync run
    cp ontology/ferroplan-host-contract.ttl crates/ferroplan-wasm/ontology/contract.ttl
    cd crates/ferroplan-wasm && ggen sync run

# Drift gate: regenerate, fail if registry/generated/contract copy changed vs the pre-regen bytes
# (content hash, so it also holds while the paths are untracked) or differ from HEAD when tracked
wasm-gen-check:
    #!/usr/bin/env bash
    set -euo pipefail
    snap() { find crates/ferroplan-wasm/registry crates/ferroplan-wasm/generated crates/ferroplan-wasm/ontology/contract.ttl -type f | LC_ALL=C sort | xargs shasum -a 256; }
    before=$(snap)
    just wasm-gen >/dev/null
    after=$(snap)
    if [ "$before" != "$after" ]; then
        echo "DRIFT: regeneration changed committed projection" >&2
        diff <(echo "$before") <(echo "$after") >&2 || true
        exit 1
    fi
    git diff --exit-code -- crates/ferroplan-wasm/registry crates/ferroplan-wasm/generated crates/ferroplan-wasm/ontology/contract.ttl
    echo "gen-check OK"

# Build wasm32-wasip1 release; compare sha256+bytes against ontology wja:wasmSha256/wasmBytes
wasm-build-pin:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build -p ferroplan-wasm --release --target wasm32-wasip1
    f=target/wasm32-wasip1/release/ferroplan_wasm.wasm
    got_sha=$(shasum -a 256 "$f" | cut -d' ' -f1)
    got_bytes=$(wc -c < "$f" | tr -d ' ')
    want_sha=$(grep -o 'wja:wasmSha256 "[0-9a-f]*"' ontology/ferroplan-wasm.ttl | head -1 | cut -d'"' -f2)
    want_bytes=$(grep -o 'wja:wasmBytes [0-9]*' ontology/ferroplan-wasm.ttl | head -1 | cut -d' ' -f2)
    echo "pinned: sha256=$want_sha bytes=$want_bytes"
    echo "built:  sha256=$got_sha bytes=$got_bytes"
    if [ "$got_sha" != "$want_sha" ] || [ "$got_bytes" != "$want_bytes" ]; then
        echo "MISMATCH: built artifact differs from ontology pin" >&2
        exit 1
    fi
    echo "pin OK"
