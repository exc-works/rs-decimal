import test from 'node:test';
import assert from 'node:assert/strict';
import { readdirSync, readFileSync } from 'node:fs';
import { methodInputs } from '../scripts/method-inputs.mjs';
import { formSource } from '../src/editor/method-form.mjs';
import { encodeShare, decodeShare, encodeTransfer, decodeTransfer } from '../src/editor/transfer.mjs';
const commit = 'a'.repeat(40);
test('shared code is Unicode-exact and rejects another build, schema and malformed input', () => {
  const source = 'show("繁體中文 日本語 한국어 é ß 😀");\n';
  const share = encodeShare(source, commit);
  assert.equal(decodeShare(share, commit), source);
  assert.throws(() => decodeShare(share, 'b'.repeat(40)));
  for (const input of ['#share=%zz', '#share=' + 'a'.repeat(32769), '#share=__', share + '=']) assert.throws(() => decodeShare(input, commit));
  for (const value of [{schema:2,commit,source}, {schema:1,commit,source,extra:true}, {schema:1,commit,source:42}]) assert.throws(() => decodeTransfer(JSON.stringify(value), commit));
});
test('sharing and local handoffs enforce separate UTF-8 byte limits before large parsing', () => {
  assert.equal(decodeShare(encodeShare('😀'.repeat(1024), commit), commit), '😀'.repeat(1024));
  assert.throws(() => encodeShare('😀'.repeat(1025), commit));
  assert.equal(decodeTransfer(encodeTransfer('x'.repeat(65536), commit), commit).length, 65536);
  assert.throws(() => encodeTransfer('x'.repeat(65537), commit));
  assert.throws(() => decodeTransfer(' '.repeat(65536 * 6 + 129), commit));
});
test('form changes keep strings as data, exact bigint values, signs and argument order', () => {
  const source = 'const value = Decimal.new_from_string("1.20").rescale(-1, RoundingMode.HalfEven);';
  const fields = methodInputs(source);
  assert.deepEqual(fields.map(x => x.type), ['string', 'number']);
  const injected = '"); throw Error("injected"); // 😀\n';
  let received;
  new Function('Decimal','RoundingMode','show',formSource(source, fields, [injected, '2']))({new_from_string(text) { received = text; return {rescale(n) { assert.equal(n, 2); return null; }}; }}, {HalfEven: 1}, () => {});
  assert.equal(received, injected);
  const bigSource = 'const value = Decimal.new(-12n);';
  const result = formSource(bigSource, methodInputs(bigSource), ['9007199254740993']);
  assert.match(result, /9007199254740993n/);
  for (const input of ['01', '1n', '1);run()', 'NaN']) assert.throws(() => formSource(bigSource, methodInputs(bigSource), [input]));
});
test('every runnable shared fixture produces a valid default form without changing its arguments', () => {
  let count = 0;
  for (const name of readdirSync(new URL('../examples/', import.meta.url))) {
    if (!name.endsWith('.json') || name === 'index.json') continue;
    const example = JSON.parse(readFileSync(new URL('../examples/' + name, import.meta.url)));
    if (!example.javascript.trim()) continue;
    const fields = methodInputs(example.javascript);
    const result = formSource(example.javascript, fields, fields.map(x => x.value));
    assert.doesNotThrow(() => new Function(result), name);
    assert.ok(result.endsWith('\nshow(value);\n'), name);
    count++;
  }
  assert.equal(count, 232);
});
