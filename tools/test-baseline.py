"""Regression checks for synchronized manifest drift and inherited constants."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

parser = argparse.ArgumentParser()
parser.add_argument("--source", type=Path, required=True)
args = parser.parse_args()
root = Path(__file__).resolve().parent.parent
with tempfile.TemporaryDirectory() as temp:
    temp = Path(temp)
    for directory in ("tools", "docs", "testdata/upstream"):
        (temp / directory).mkdir(parents=True, exist_ok=True)
    shutil.copy(root / "tools/check-baseline.py", temp / "tools/check-baseline.py")
    for name in ("api-inventory.json", "api-compatibility.md"):
        shutil.copy(root / "docs" / name, temp / "docs" / name)
    shutil.copytree(root / "testdata/upstream", temp / "testdata/upstream", dirs_exist_ok=True)
    file = temp / "testdata/upstream/testdata/must_succeed.json"
    file.write_bytes(file.read_bytes() + b"\n")
    manifest_path = temp / "testdata/upstream/manifest.json"
    manifest = json.loads(manifest_path.read_text())
    manifest["files"]["testdata/must_succeed.json"] = hashlib.sha256(file.read_bytes()).hexdigest()
    manifest_path.write_text(json.dumps(manifest))
    result = subprocess.run(["python3", str(temp / "tools/check-baseline.py"), "--source", str(args.source.resolve())], capture_output=True, text=True)
    assert result.returncode != 0 and "upstream bytes differ" in result.stderr, result

    (temp / "constants.go").write_text("package decimal\ntype Mode int\nconst (\nA Mode = iota * 2\nB\nC = 9\nD\n)\n")
    result = subprocess.check_output(["go", "run", str(root / "tools/api-inventory/main.go"), str(temp)], text=True)
    entries = {e["id"]: e["signature"] for e in json.loads(result)}
    assert entries == {"Mode": "Mode int", "A": "A Mode = 0", "B": "B Mode = 2", "C": "C untyped int = 9", "D": "D untyped int = 9"}, entries
print("Baseline regression checks passed")
