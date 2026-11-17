#!/usr/bin/env python3
"""Check web API dispositions and example/document evidence; --release closes gaps."""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LOCALES = ('zh-Hans', 'zh-Hant', 'en-US', 'en-GB', 'fr', 'ko', 'ja', 'de')


def check(root=ROOT, release=False):
    api = json.loads((root / 'website/api/rust-api.json').read_text())
    coverage = json.loads((root / 'website/api/coverage.json').read_text())
    ids = [entry['id'] for entry in api]
    mapped = [entry['id'] for entry in coverage]
    if len(ids) != len(set(ids)) or len(mapped) != len(set(mapped)):
        raise ValueError('duplicate API/coverage ID')
    if set(ids) != set(mapped):
        raise ValueError(f'coverage mismatch: missing={set(ids)-set(mapped)}, unknown={set(mapped)-set(ids)}')
    docs = set()
    for row in coverage:
        required = {'id', 'disposition', 'status', 'owner', 'doc_id', 'reason', 'bridge_signature', 'examples', 'tests'}
        if set(row) != required:
            raise ValueError(f"invalid coverage fields: {row['id']}")
        if row['disposition'] not in ('direct', 'adapter', 'rust-only'):
            raise ValueError(f"invalid disposition: {row['id']}")
        if row['status'] not in ('planned', 'implemented'):
            raise ValueError(f"invalid status: {row['id']}")
        if not row['owner'].startswith('WEB-') or not row['doc_id']:
            raise ValueError(f"missing owner/doc ID: {row['id']}")
        if row['doc_id'] in docs:
            raise ValueError(f"duplicate documentation ID: {row['doc_id']}")
        docs.add(row['doc_id'])
        if row['disposition'] != 'direct' and not row['reason'].strip():
            raise ValueError(f"missing mapping explanation: {row['id']}")
        if not isinstance(row['examples'], list) or not isinstance(row['tests'], list):
            raise ValueError('examples/tests must be lists')
        if release and row['status'] != 'implemented':
            raise ValueError(f"release contains planned API: {row['id']}")
        if row['status'] == 'implemented':
            if row['disposition'] != 'rust-only' and not row['bridge_signature']:
                raise ValueError(f"missing bridge signature: {row['id']}")
            if not row['examples'] or not row['tests']:
                raise ValueError(f"missing evidence: {row['id']}")
            for name in row['tests']:
                target = (root / name).resolve()
                if not target.is_relative_to(root.resolve()) or not target.is_file():
                    raise ValueError(f'invalid test evidence path: {name}')
            for example_id in row['examples']:
                example = root / 'website/examples' / f'{example_id}.json'
                data = json.loads(example.read_text())
                api_ids = data.get('api_ids')
                if not isinstance(api_ids, list) or not api_ids or any(not isinstance(x, str) or x not in ids for x in api_ids) or len(api_ids) != len(set(api_ids)):
                    raise ValueError(f'invalid example API IDs: {example_id}')
                if data['id'] != example_id or row['id'] not in api_ids:
                    raise ValueError(f'example does not cover API: {example_id}')
                if set(data) != {'id', 'api_ids', 'rust', 'javascript', 'expected', 'schema_version'} or data['schema_version'] != 1:
                    raise ValueError(f'invalid example envelope: {example_id}')
                if not data['rust'].strip() or (row['disposition'] != 'rust-only' and not data['javascript'].strip()):
                    raise ValueError(f'missing paired example: {example_id}')
                if not isinstance(data['expected'], dict) or not data['expected']:
                    raise ValueError(f'missing expected state: {example_id}')
            for locale in LOCALES if release else ('en-US', 'zh-Hans'):
                page = root / 'website/docs' / locale / 'api' / f"{row['doc_id']}.md"
                if not page.is_file() or not page.read_text().strip():
                    raise ValueError(f'missing method documentation: {page}')
    return len(api)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--release', action='store_true')
    args = parser.parse_args()
    print(f'{check(release=args.release)} API dispositions checked; release={args.release}')
