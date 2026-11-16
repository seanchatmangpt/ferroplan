#!/usr/bin/env bash
# run_ignored_with_census.sh — run `cargo test -- --ignored` under a corpus
# census so a green ignored job cannot silently exercise nothing.
#
# Corpus-dependent #[ignore]d tests print NAMED census markers:
#   CORPUS-CENSUS|SKIP|<test>|<why>
#   CORPUS-CENSUS|RUN|<test>|<n> cases
# (see crates/ferroplan/tests/common/external.rs). This wrapper tees the
# run, counts markers, and fails with CORPUS_GATE_VACUOUS when more than
# half of the marker-emitting ignored corpus tests skipped (or none ran).
#
# Usage: scripts/run_ignored_with_census.sh cargo test ... -- --ignored
# Requires --nocapture (auto-appended after `--` when missing) so the
# markers survive libtest's output capture.
set -euo pipefail

if [[ $# -eq 0 ]]; then
  echo "usage: $0 <cargo test ... -- --ignored [args...]>" >&2
  exit 64
fi

args=("$@")
has_nocapture=0
for a in "${args[@]}"; do
  [[ "$a" == "--nocapture" ]] && has_nocapture=1
done

if [[ $has_nocapture -eq 0 ]]; then
  # insert --nocapture after the libtest separator `--`
  fixed=()
  inserted=0
  for a in "${args[@]}"; do
    fixed+=("$a")
    if [[ $inserted -eq 0 && "$a" == "--" ]]; then
      fixed+=("--nocapture")
      inserted=1
    fi
  done
  if [[ $inserted -eq 0 ]]; then
    fixed+=("--" "--nocapture")
  fi
  args=("${fixed[@]}")
fi

log=$(mktemp /tmp/ferroplan-ignored-census.XXXXXX.log)
trap 'rm -f "$log"' EXIT

set +e
"${args[@]}" 2>&1 | tee "$log"
status=${PIPESTATUS[0]}
set -e

skipped=$(grep -c '^CORPUS-CENSUS|SKIP|' "$log" || true)
ran=$(grep -c '^CORPUS-CENSUS|RUN|' "$log" || true)
echo "corpus census: ${skipped} skipped, ${ran} ran"

if [[ ${skipped} -eq 0 && ${ran} -eq 0 ]]; then
  echo "CORPUS_GATE_VACUOUS: no CORPUS-CENSUS markers in ignored-test output — \
the corpus tests either did not run or their markers were captured; \
refusing to count this as evidence." >&2
  exit 91
fi

half=$(( skipped * 2 ))
if [[ ${half} -gt ${ran} ]]; then
  echo "CORPUS_GATE_VACUOUS: ${skipped} of $(( skipped + ran )) marker-emitting ignored \
corpus tests skipped (>50%) — the ignored job exercised nothing; populate \
FERROPLAN_CORPUS_DIR / FERROPLAN_ORACLE_DIR (default ~/.cache/ferroplan) \
or accept the typed gate failure." >&2
  exit 92
fi

exit "$status"
