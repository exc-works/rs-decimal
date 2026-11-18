import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { apiPath, summary, symbols, symbolById } from '../scripts/reference-index.mjs';
import { referencePage } from '../scripts/reference-page.mjs';
import { searchReference } from '../scripts/reference-search.mjs';
import { referenceLabels } from '../reference-labels.mjs';
import { locales } from '../locales.mjs';

test('every Rust symbol keeps its translated page, summary and group', () => {
  const inventory = JSON.parse(readFileSync(new URL('../api/coverage.json', import.meta.url)));
  assert.equal(symbolById.size, inventory.length);
  for (const locale of Object.keys(locales)) {
    for (const { id, doc_id } of inventory) {
      const symbol = symbolById.get(id);
      assert.equal(apiPath(locale, symbol), '/' + locale + '/api/' + doc_id + '.html');
      assert.ok(summary(locale, symbol).trim(), id);
      assert.ok(referenceLabels[locale][symbol.group], id);
    }
  }
  assert.equal(symbolById.get('NullDecimal.valid').parent, 'NullDecimal');
  assert.equal(symbolById.get('config::ConfigValue::Null').parent, 'config::ConfigValue');
});

test('task and symbol searches lead to the corresponding localized reference', () => {
  const page = { relativePath: 'zh-Hans/api/index.md', frontmatter: {} };
  referencePage(page);
  for (const [term, id] of [['Decimal::round_with_prec', 'Decimal::round_with_prec'],
    ['保留两位小数', 'Decimal::round_with_prec'], ['尾零', 'Decimal::string_with_trailing_zeros']]) {
    const hits = searchReference(page.reference.directory, term, referenceLabels['zh-Hans']);
    assert.equal(hits[0].id, id);
  }
  assert.equal(symbols.find(symbol => symbol.id === 'Decimal::round_with_prec').group, 'precision');
});
