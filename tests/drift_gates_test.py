"""Mutation falsifiers for scripts/drift_gates.py. Real files, scratch copy, no mocks.
Run: python3 tests/drift_gates_test.py"""
import re, shutil, sys, tempfile, unittest
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO / "scripts"))
import drift_gates as dg

COPY = ["Cargo.toml", "Cargo.lock", "README.md", "STATUS.md", "STANDINGS.md", "benchmarks/ipc-standings.md",
        "crates/ferroplan-py/Cargo.toml", "crates/ferroplan-cli/Cargo.toml", "crates/ferroplan-mcp/Cargo.toml",
        "crates/ferroplan-wasm/Cargo.toml", ".github/workflows"]


def scratch():
    d = Path(tempfile.mkdtemp(prefix="driftgate-"))
    for rel in COPY:
        s, t = REPO / rel, d / rel
        t.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(s, t) if s.is_dir() else shutil.copy(s, t)
    shutil.copytree(REPO / "crates/ferroplan-wasm/web", d / "crates/ferroplan-wasm/web")
    return d


def current_version():
    """Read the CURRENT workspace version from the root Cargo.toml at test time."""
    t = (REPO / "Cargo.toml").read_text(encoding="utf-8")
    m = re.search(r'^version = "([^"]+)"', t, re.M)
    assert m, "workspace version not found in root Cargo.toml"
    return m.group(1)


def prev_patch_version(v):
    """CURRENT version with the patch component decremented by 1."""
    a, b, c = v.split(".")
    return f"{a}.{b}.{int(c) - 1}"


def sub(path, old, new):
    t = path.read_text(encoding="utf-8")
    assert old in t, (path, old)
    path.write_text(t.replace(old, new, 1), encoding="utf-8")


class Gates(unittest.TestCase):
    def setUp(self):
        self.d = scratch()
        self.addCleanup(shutil.rmtree, self.d, True)

    def codes(self, gate):
        return [f.split(":")[0] for f in dg.GATES[gate](self.d)]

    def test_pass_on_tree(self):
        for g in ("version-pins", "wasm-bindgen-pin", "sw-shell", "counts"):
            self.assertEqual(dg.GATES[g](REPO), [], g)

    def test_version_dep_mutation(self):
        cur, prev = current_version(), prev_patch_version(current_version())
        sub(self.d / "crates/ferroplan-cli/Cargo.toml", f'version = "{cur}"', f'version = "{prev}"')
        self.assertIn("VERSION_PIN_DEP_MISMATCH", self.codes("version-pins"))

    def test_version_py_mutation(self):
        cur, prev = current_version(), prev_patch_version(current_version())
        sub(self.d / "crates/ferroplan-py/Cargo.toml", f'version = "{cur}"', f'version = "{prev}"')
        self.assertIn("VERSION_PIN_PY_MISMATCH", self.codes("version-pins"))

    def test_wasm_bindgen_mutation(self):
        sub(self.d / ".github/workflows/pages.yml", "--version 0.2.126", "--version 0.2.125")
        self.assertIn("WASM_BINDGEN_PIN_MISMATCH", self.codes("wasm-bindgen-pin"))

    def test_sw_shell_mutations(self):
        w = self.d / "crates/ferroplan-wasm/web"
        (w / "extra.js").write_text("//x")
        self.assertIn("SW_SHELL_UNLISTED_FILE", self.codes("sw-shell"))
        (w / "extra.js").unlink()
        (w / "icon.svg").unlink()
        self.assertIn("SW_SHELL_MISSING_FILE", self.codes("sw-shell"))

    def test_count_mutation(self):
        sub(self.d / "STANDINGS.md", "| tempo-sat | 441/630 |", "| tempo-sat | 440/630 |")
        self.assertIn("COUNT_DRIFT", self.codes("counts"))

    def test_validate_all_code_mutation(self):
        p = self.d / ".github/workflows/validate-all-code.yml"
        if 'version() == "0.21.0"' in p.read_text():
            self.assertIn("VALIDATE_ALL_STALE_VERSION", self.codes("validate-all-code"))
        t = p.read_text()
        p.write_text(re.sub(r'ferroplan\.version\(\) == [^\n]+', f'ferroplan.version() == "{current_version()}"', t))
        self.assertEqual(self.codes("validate-all-code"), [])
        p.write_text(re.sub(r'ferroplan\.version\(\) == [^\n]+', 'ferroplan.version() == "0.1.0"', t))
        self.assertIn("VALIDATE_ALL_STALE_VERSION", self.codes("validate-all-code"))


if __name__ == "__main__":
    unittest.main()
