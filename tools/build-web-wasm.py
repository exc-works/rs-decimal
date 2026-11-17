#!/usr/bin/env python3
"""Build the pinned WEB-02 feasibility artifact and record reproducible metadata."""
import gzip
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parent.parent
CLI = os.environ.get('WASM_BINDGEN', str(ROOT / '.cache/wasm-tools/bin/wasm-bindgen'))
if not Path(CLI).is_file():
    CLI = shutil.which('wasm-bindgen') or CLI
version = subprocess.check_output([CLI, '--version'], text=True).strip()
if version != 'wasm-bindgen 0.2.105':
    raise SystemExit(f'Expected wasm-bindgen 0.2.105, got {version}')
commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
env = dict(os.environ, DECIMAL_BUILD_COMMIT=commit)
subprocess.run(['cargo', 'build', '--locked', '--release', '-p', 'decimal-playground-bridge', '--target', 'wasm32-unknown-unknown'], cwd=ROOT, env=env, check=True)
out = ROOT / 'website/public/wasm'
out.mkdir(parents=True, exist_ok=True)
subprocess.run([CLI, '--target', 'web', '--out-dir', str(out), '--out-name', 'decimal', str(ROOT / 'target/wasm32-unknown-unknown/release/decimal_playground_bridge.wasm')], cwd=ROOT, check=True)
assets = {}
for file in sorted(out.iterdir()):
    if file.name == 'build.json':
        continue
    data = file.read_bytes()
    assets[file.name] = dict(bytes=len(data), gzip_bytes=len(gzip.compress(data, mtime=0)), sha256=hashlib.sha256(data).hexdigest())
report = dict(commit=commit, target='wasm32-unknown-unknown', wasm_bindgen=version, rustc=subprocess.check_output(['rustc','--version'],text=True).strip(), assets=assets)
(out / 'build.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
