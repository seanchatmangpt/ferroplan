#!/usr/bin/env python3
"""Emit receipts/v26.9.22/<WO>.v2.json siblings from the immutable v1 receipts.

Resolves 7-char shas to 40-hex via `git rev-parse`; maps legacy fields into the
5-field law (identity/authority/consequence/replay/standing) plus ALOOP fields.
Originals are never modified. Usage: receipt_v2.py [repo_root]
"""
import json, re, subprocess, sys
from pathlib import Path

root = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent).resolve()

def rp(sha):
    out = subprocess.run(["git", "-C", str(root), "rev-parse", "--verify", sha + "^{commit}"],
                         capture_output=True, text=True)
    if out.returncode != 0:
        raise SystemExit(f"cannot resolve {sha}: {out.stderr.strip()}")
    return out.stdout.strip()

def shas(text):
    return [t for t in re.findall(r"\b[0-9a-f]{7,40}\b", text) if re.search(r"\d", t)]

for src in sorted((root / "receipts/v26.9.22").glob("FERROPLAN-26922-0?.json")):
    v1 = json.loads(src.read_text())
    # merge_sha may be free text (WO-08): the first hex token is the merge commit.
    merge = rp(shas(v1["merge_sha"])[0])
    commits = [rp(t) for t in shas(v1["commit"])]
    base = rp(v1["base_sha"])
    v2 = {
        "work_order_id": v1["wo"],
        "origin_authority": {"ceiling": "CONSTRUCT", "grant": "NONE", "actor": "claude-code"},
        "provider": {"name": "claude-code", "transport": "local-cli"},
        "provider_execution_id": f"{v1['wo']}:{v1['branch']}",
        "identity": {"subject": v1["wo"], "repo": str(root), "subject_sha": merge, "base_sha": base},
        "authority": {"ceiling": "CONSTRUCT", "grant": "NONE", "actor": "claude-code"},
        "consequence": {"commits": commits + [merge], "files_changed": [], "remote_effects": []},
        "replay": {
            "commands": [{"cmd": v1["command"], "cwd": str(root), "exit": v1["exit"],
                          "summary": v1["output_excerpt"]}],
            "durable_location": f"receipts/v26.9.22/{src.name}",
        },
        "standing": {"value": v1["standing"],
                     "derived_from": f"recorded replay command at merge {merge} (v1 receipt {src.name})"},
        "provider_ext.ferroplan": {"v1_receipt": src.name, "branch": v1["branch"],
                                   "v1_merge_sha": v1["merge_sha"], "v1_commit": v1["commit"]},
    }
    dst = src.with_name(src.stem + ".v2.json")
    dst.write_text(json.dumps(v2, indent=2) + "\n")
    print("wrote", dst.relative_to(root))
