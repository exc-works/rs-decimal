"""Verify frozen upstream bytes and API inventory/matrix completeness."""
import hashlib
import json
from pathlib import Path
import re

root = Path(__file__).resolve().parent.parent
upstream = root / "testdata/upstream"
manifest = json.loads((upstream / "manifest.json").read_text())
assert manifest["commit"] == "a8791208e2d15483cf652cd2814b74b0c1a84187"
for name, checksum in manifest["files"].items():
    assert hashlib.sha256((upstream / name).read_bytes()).hexdigest() == checksum, name
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
