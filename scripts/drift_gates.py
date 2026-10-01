#!/usr/bin/env python3
"""Drift gates (plan R3 minus the contract/template-copy gate).

Each gate returns a list of typed findings "GATE_CODE: message". Empty list = pass.
Usage: drift_gates.py [--root DIR] [--gate NAME ...]   exit 0 pass, 1 findings.
"""
import argparse, re, sys
from pathlib import Path


def read(root, rel):
    return (root / rel).read_text(encoding="utf-8")


def workspace_version(root):
    m = re.search(r'\[workspace\.package\][^\[]*?^version\s*=\s*"([^"]+)"', read(root, "Cargo.toml"), re.S | re.M)
    return m.group(1) if m else None


def gate_version_pins(root):
    out, ws = [], workspace_version(root)
    if not ws:
        return ["VERSION_PIN_NO_WORKSPACE_VERSION: Cargo.toml has no [workspace.package] version"]
    # path deps on ferroplan that carry an explicit version pin
    for toml in sorted((root / "crates").glob("*/Cargo.toml")):
        txt = toml.read_text(encoding="utf-8")
        rel = toml.relative_to(root)
        for m in re.finditer(r'^\s*(?:ferroplan|ferroplan_core)\s*=\s*\{[^}]*\}', txt, re.M):
            dep = m.group(0)
            if "path" not in dep:
                continue
            v = re.search(r'\bversion\s*=\s*"([^"]+)"', dep)
            if v and v.group(1) != ws:
                out.append(f"VERSION_PIN_DEP_MISMATCH: {rel} pins ferroplan version {v.group(1)} != workspace {ws}")
        if "ferroplan-py" in str(rel):
            pm = re.search(r'^\[package\][^\[]*?^version\s*=\s*"([^"]+)"', txt, re.S | re.M)
            if not pm:
                out.append(f"VERSION_PIN_PY_MISSING: {rel} has no [package] version")
            elif pm.group(1) != ws:
                out.append(f"VERSION_PIN_PY_MISMATCH: {rel} version {pm.group(1)} != workspace {ws}")
    return out


def wasm_bindgen_pin(root):
    out = []
    m = re.search(r'^wasm-bindgen\s*=\s*"=([^"]+)"', read(root, "crates/ferroplan-wasm/Cargo.toml"), re.M)
    if not m:
        return ["WASM_BINDGEN_PIN_MISSING: crates/ferroplan-wasm/Cargo.toml has no exact (=) wasm-bindgen pin"]
    pin = m.group(1)
    copies = 0
    for wf in sorted((root / ".github/workflows").glob("*.y*ml")):
        for n, line in enumerate(wf.read_text(encoding="utf-8").splitlines(), 1):
            c = re.search(r'wasm-bindgen-cli\s+--version\s+(\S+)', line)
            if c:
                copies += 1
                if c.group(1) != pin:
                    out.append(f"WASM_BINDGEN_PIN_MISMATCH: {wf.relative_to(root)}:{n} installs {c.group(1)} != Cargo pin {pin}")
    if copies == 0:
        out.append("WASM_BINDGEN_PIN_NO_WORKFLOW_COPIES: no workflow installs wasm-bindgen-cli")
    lock = root / "Cargo.lock"
    if lock.exists():
        lm = re.search(r'name = "wasm-bindgen"\nversion = "([^"]+)"', lock.read_text(encoding="utf-8"))
        if lm and lm.group(1) != pin:
            out.append(f"WASM_BINDGEN_PIN_LOCK_MISMATCH: Cargo.lock resolves {lm.group(1)} != pin {pin}")
    return out


def gate_sw_shell(root):
    web = root / "crates/ferroplan-wasm/web"
    sw = (web / "sw.js").read_text(encoding="utf-8")
    block = re.search(r'const SHELL = \[(.*?)\];', sw, re.S)
    if not block:
        return ["SW_SHELL_UNPARSEABLE: no `const SHELL = [...]` in sw.js"]
    entries = re.findall(r"'([^']+)'", block.group(1))
    out, listed = [], set()
    for e in entries:
        p = e[2:] if e.startswith("./") else e
        if p == "" or p.startswith("pkg/"):
            continue  # directory root and the wasm-pack bundle are build outputs
        listed.add(p)
        if not (web / p).is_file():
            out.append(f"SW_SHELL_MISSING_FILE: sw.js SHELL lists ./{p} but web/{p} does not exist")
    for f in sorted(x for x in web.iterdir() if x.is_file()):
        if f.name == "sw.js":
            continue
        if f.name not in listed:
            out.append(f"SW_SHELL_UNLISTED_FILE: web/{f.name} exists but is not in sw.js SHELL")
    return out


def canonical_tempo_total(root):
    txt = read(root, "benchmarks/ipc-standings.md")
    rows = re.findall(r'^\|\s*tempo-sat\s*\|\s*yes\s*\|\s*(\d+)/(\d+)\s*\|', txt, re.M)
    # IPC-6 (2008, /390) + IPC-7 (2011, /240); the 2014 row is /200 and not part of 630
    solved = sum(int(a) for a, b in rows if b in ("390", "240"))
    total = sum(int(b) for a, b in rows if b in ("390", "240"))
    return solved, total


DATED = re.compile(r'\d+\.\d+|\bat \d+ ?s\b|recon|baseline|\bcut\b|\bwas\b|→|->', re.I)
DELTA = re.compile(r'[^|]*\(vs [\d.]+\)')  # generated delta column is not a date for the count
COUNT_FILES = ["README.md", "STATUS.md", "STANDINGS.md", "WAVE-RECEIPT.md"]


def gate_counts(root):
    solved, total = canonical_tempo_total(root)
    if total != 630:
        return [f"COUNT_CANON_UNPARSEABLE: ipc-standings.md tempo-sat rows sum to {solved}/{total}, expected denominator 630"]
    out = []
    for rel in COUNT_FILES:
        p = root / rel
        if not p.exists():
            continue
        for n, line in enumerate(p.read_text(encoding="utf-8").splitlines(), 1):
            for m in re.finditer(r'(\d+)/630', line):
                if DATED.search(DELTA.sub('', line)):
                    continue  # dated/historical mention (version, tier, recon, delta)
                if int(m.group(1)) != solved:
                    out.append(f"COUNT_DRIFT: {rel}:{n} says {m.group(0)} but benchmarks/ipc-standings.md derives {solved}/630")
    return out


def gate_validate_all_code(root):
    p = root / ".github/workflows/validate-all-code.yml"
    if not p.exists():
        return []
    ws, out = workspace_version(root), []
    for n, line in enumerate(p.read_text(encoding="utf-8").splitlines(), 1):
        m = re.search(r'ferroplan\.version\(\)\s*==\s*"([^"]+)"', line)
        if m and m.group(1) != ws:
            out.append(f"VALIDATE_ALL_STALE_VERSION: validate-all-code.yml:{n} hardcodes {m.group(1)} != Cargo {ws}; derive from Cargo.toml (workspace version)")
    return out


GATES = {
    "version-pins": gate_version_pins,
    "wasm-bindgen-pin": wasm_bindgen_pin,
    "sw-shell": gate_sw_shell,
    "counts": gate_counts,
    "validate-all-code": gate_validate_all_code,
}


def run(root, names=None):
    res = {}
    for name in names or GATES:
        res[name] = GATES[name](Path(root))
    return res


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=str(Path(__file__).resolve().parent.parent))
    ap.add_argument("--gate", action="append", choices=list(GATES))
    a = ap.parse_args()
    bad = 0
    for name, fs in run(a.root, a.gate).items():
        print(f"[{'FAIL' if fs else 'PASS'}] {name}")
        for f in fs:
            print(f"  {f}")
        bad += bool(fs)
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
