"""Reject incomplete promised API entries; explicitly track the PR 15 platform mappings."""
import json
from pathlib import Path
root = Path(__file__).resolve().parent.parent
inventory = {row["id"] for row in json.loads((root / "docs/api-inventory.json").read_text())}
rows = [line.strip("| ").split(" | ") for line in (root / "docs/api-compatibility.md").read_text().splitlines() if line.startswith("| `")]
assert {r[0].strip("`") for r in rows} == inventory
pending = []
for row in rows:
    assert len(row) == 9, row
    if row[3] == "15" and row[8] == "延后":
        pending.append(row[0])
        continue
    assert row[8] == "已验证", row
    assert all("待设计" not in value and "待实现" not in value and "未开始" not in value for value in row), row
    assert row[1].strip() and row[5].strip() and row[6].strip() and row[7].strip(), row
print(f"{len(rows)-len(pending)} verified entries; {len(pending)} explicitly assigned PR 15 ecosystem mappings")
