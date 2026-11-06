"""Verify frozen upstream bytes and API inventory/matrix completeness."""
import hashlib
import json
from pathlib import Path
import re
import argparse
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument("--source", type=Path, help="checkout of the exact upstream revision")
args = parser.parse_args()
root = Path(__file__).resolve().parent.parent
upstream = root / "testdata/upstream"
manifest = json.loads((upstream / "manifest.json").read_text())
assert manifest["commit"] == "a8791208e2d15483cf652cd2814b74b0c1a84187"
if args.source:
    source_sha = subprocess.check_output(["git", "-C", str(args.source), "rev-parse", "HEAD"], text=True).strip()
    assert source_sha == manifest["commit"], "wrong upstream checkout revision"
required_files = {"LICENSE", "docs/numeric-contract.md", "docs/numeric-contract-migration.md", "docs/correctness-gates.md"}
required_files.update("testdata/" + name for name in (
    "generate_must_succeed.py", "generate_roots.py", "generate_transcendental.py",
    "must_succeed.json", "roots.json", "transcendental.json", "yaml_scalars.json"))
assert set(manifest["files"]) == required_files, "baseline file inventory drift"
for name, checksum in manifest["files"].items():
    frozen = (upstream / name).read_bytes()
    assert hashlib.sha256(frozen).hexdigest() == checksum, name
    if args.source:
        original = subprocess.check_output(["git", "-C", str(args.source), "show", manifest["commit"] + ":" + name])
        assert frozen == original, f"upstream bytes differ: {name}"
inventory = json.loads((root / "docs/api-inventory.json").read_text())
ids = [row["id"] for row in inventory]
rows = re.findall(r"^\| `([^`]+)` \|", (root / "docs/api-compatibility.md").read_text(), re.M)
assert len(ids) == len(set(ids)), "duplicate inventory IDs"
assert len(rows) == len(set(rows)), "duplicate matrix IDs"
assert set(ids) == set(rows), f"matrix mismatch: {set(ids) ^ set(rows)}"
corpus = json.loads((upstream / "testdata/must_succeed.json").read_text())
assert len(corpus["cases"]) == 45
assert len({case["id"] for case in corpus["cases"]}) == 45
print(f"Baseline checks passed: {len(manifest['files'])} files, {len(ids)} API entries, 45 required cases")
