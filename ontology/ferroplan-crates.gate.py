#!/usr/bin/env python3
"""Crate-roster gate. UNSUPPORTED(generator-capability: marker-region splice) -- ggen
Merge mode uses git-conflict markers, so the README region splice lives here.

  ferroplan-crates.gate.py          check (exit 1 on drift)
  ferroplan-crates.gate.py --write  splice ontology/ferroplan-crates.readme.md into README

Checks: (1) every crates/*/Cargo.toml dir has a fpc:Crate individual and vice versa;
(2) README marker region == generated fragment; (3) book/src/crates.md lists exactly
the individuals; (4) the fragment lists exactly the individuals.
"""
import re, sys, pathlib

ROOT = pathlib.Path(__file__).resolve().parent.parent
BEGIN = "<!-- BEGIN GENERATED: crate-roster (ontology/ferroplan-crates.ttl) -->"
END = "<!-- END GENERATED: crate-roster -->"


def individuals(ttl):
    return set(re.findall(r'fpc:crateName "([^"]+)"', ttl))


def linked(md):
    return set(re.findall(r"\[`(ferroplan[\w-]*)`\]\(", md))


def main():
    write = "--write" in sys.argv
    ttl = (ROOT / "ontology/ferroplan-crates.ttl").read_text()
    ind = individuals(ttl)
    dirs = {p.parent.name for p in (ROOT / "crates").glob("*/Cargo.toml")}
    errs = []
    for n in sorted(dirs - ind):
        errs.append(f"crate dir without individual: {n}")
    for n in sorted(ind - dirs):
        errs.append(f"individual without crate dir: {n}")
    frag_p = ROOT / "ontology/ferroplan-crates.readme.md"
    frag = frag_p.read_text() if frag_p.exists() else ""
    if linked(frag) != ind:
        errs.append(f"fragment crates {sorted(linked(frag) ^ ind)} differ from ontology (run ggen sync)")
    book_p = ROOT / "book/src/crates.md"
    book = book_p.read_text() if book_p.exists() else ""
    if linked(book) != ind:
        errs.append(f"book/src/crates.md crates {sorted(linked(book) ^ ind)} differ from ontology (run ggen sync)")
    readme_p = ROOT / "README.md"
    readme = readme_p.read_text()
    m = re.search(re.escape(BEGIN) + r"\n(.*?)" + re.escape(END), readme, re.S)
    if not m:
        errs.append("README.md missing crate-roster marker region")
    else:
        want = frag.rstrip("\n") + "\n"
        if m.group(1) != want:
            if write and not [e for e in errs if "fragment" in e]:
                readme = readme[: m.start(1)] + want + readme[m.end(1):]
                readme_p.write_text(readme)
                print("README region spliced")
            else:
                errs.append("README crate-roster region differs from fragment (run --write)")
    for e in errs:
        print("GATE FAIL:", e)
    if not errs:
        print(f"crate-roster gate OK ({len(ind)} crates)")
    return 1 if errs else 0


sys.exit(main())
