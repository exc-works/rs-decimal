"""Negative coverage checks: omissions and invented implementation evidence fail."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

spec = importlib.util.spec_from_file_location('web_check', Path(__file__).with_name('check.py'))
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)


class CoverageTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / 'website/api').mkdir(parents=True)
        self.row = dict(id='Decimal::new', disposition='direct', status='planned', owner='WEB-07', doc_id='decimal-new', reason='', bridge_signature=None, examples=[], tests=[])
        (self.root / 'website/api/rust-api.json').write_text(json.dumps([{'id': self.row['id']}]))

    def write(self, rows):
        (self.root / 'website/api/coverage.json').write_text(json.dumps(rows))

    def test_missing_and_duplicate_entries_fail(self):
        for rows in ([], [self.row, self.row]):
            self.write(rows)
            with self.assertRaises(ValueError):
                checker.check(self.root)

    def test_planned_is_allowed_only_in_preview(self):
        self.write([self.row])
        self.assertEqual(checker.check(self.root), 1)
        with self.assertRaisesRegex(ValueError, 'planned API'):
            checker.check(self.root, release=True)

    def test_implementation_requires_signature_and_evidence(self):
        self.row['status'] = 'implemented'
        self.write([self.row])
        with self.assertRaisesRegex(ValueError, 'bridge signature'):
            checker.check(self.root)
        self.row['bridge_signature'] = 'Decimal.new(value: bigint): Decimal'
        self.write([self.row])
        with self.assertRaisesRegex(ValueError, 'missing evidence'):
            checker.check(self.root)

    def test_example_ids_are_exact_list_members_not_substrings(self):
        self.row.update(status='implemented', bridge_signature='Decimal.new(value: bigint): Decimal', examples=['example'], tests=['test.rs'])
        self.write([self.row])
        (self.root / 'test.rs').write_text('test evidence')
        (self.root / 'website/examples').mkdir()
        for locale in checker.LOCALES:
            folder = self.root / 'website/docs' / locale / 'api'
            folder.mkdir(parents=True)
            (folder / 'decimal-new.md').write_text('method documentation')
        example = dict(schema_version=1, id='example', api_ids='Decimal::new_from_string', rust='let value = Decimal::new(0);', javascript='const value = Decimal.new(0n);', expected={'coefficient': '0'})
        for bad in ['Decimal::new_from_string', ['Unknown'], [None], ['Decimal::new', 'Decimal::new']]:
            example['api_ids'] = bad
            (self.root / 'website/examples/example.json').write_text(json.dumps(example))
            with self.assertRaisesRegex(ValueError, 'invalid example API IDs'):
                checker.check(self.root, release=True)
        example['api_ids'] = ['Decimal::new']
        (self.root / 'website/examples/example.json').write_text(json.dumps(example))
        self.assertEqual(checker.check(self.root, release=True), 1)

    def test_adapter_requires_reason(self):
        self.row['disposition'] = 'adapter'
        self.write([self.row])
        with self.assertRaisesRegex(ValueError, 'mapping explanation'):
            checker.check(self.root)


if __name__ == '__main__':
    unittest.main()
