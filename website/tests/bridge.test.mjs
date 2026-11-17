import { readFile } from 'node:fs/promises';
import assert from 'node:assert/strict';
import { test } from 'node:test';
import init, * as raw from '../public/wasm/decimal.js';
import { createBridge } from '../src/bridge/index.js';
await init({module_or_path: await readFile(new URL('../public/wasm/decimal_bg.wasm', import.meta.url))});
const { Decimal, NullDecimal, ByteBuffer, RoundingMode, snapshot } = createBridge(raw);

test('full i64 values and decimal source text stay exact', () => {
  for (const n of [-(1n<<63n), (1n<<63n)-1n, 9007199254740993n]) {
    const d = Decimal.new(n); assert.equal(d.coefficient(), n); d.free();
  }
  for (const n of [1, '1', 1n<<63n, -(1n<<63n)-1n]) assert.throws(() => Decimal.new(n), {category:'parameter'});
  assert.throws(() => Decimal.new_from_string(1), {category:'parameter'});
});

test('rounding library errors retain full categories and inputs', () => {
  const d = Decimal.new_from_string('1.245');
  assert.throws(() => d.rescale(2, RoundingMode.Unnecessary), e => e.name === 'DecimalError' && e.category === 'library' && e.kinds.includes('RoundUnnecessary'));
  assert.equal(d.string(), '1.245');
  assert.throws(() => d.rescale(-1, RoundingMode.HalfEven), e => e.kinds.includes('InvalidPrecision'));
  assert.throws(() => d.rescale(2, 7), e => e.kinds.includes('InvalidArgument'));
  for (const p of ['2', NaN, Infinity, 1.5, 2147483648]) assert.throws(() => d.rescale(p, 5), {category:'parameter'});
  d.free();
});

test('consuming methods invalidate values on library success and failure', () => {
  const d = Decimal.new_from_string('1.245');
  const result = d.into_rescaled(2, 5); assert.equal(result.string(), '1.24');
  assert.throws(() => d.string(), {category:'moved'});
  const failing = result.clone();
  assert.throws(() => failing.into_rescaled(-1, 5), e => e.kinds.includes('InvalidPrecision'));
  assert.throws(() => failing.clone(), {category:'moved'});
  const invalid = result.clone();
  assert.throws(() => invalid.into_rescaled('bad', 5), {category:'parameter'});
  assert.equal(invalid.string(), '1.24'); // no Rust call/move for an invalid JS type
  invalid.free(); result.free(); d.free(); failing.free();
});

test('null validity is independent and inner access is explicitly copied', () => {
  const d = Decimal.default(); const n = NullDecimal.new(d);
  assert.equal(n.valid, true); assert.throws(() => d.is_nil(), {category:'moved'});
  const before = snapshot(n); n.valid = false;
  assert.equal(before.valid, true); assert.equal(snapshot(n).decimal.initialized, false);
  const inner = n.get_decimal_clone(); assert.equal(inner.is_nil(), true);
  const value = Decimal.new_from_string('12.00'); n.set_decimal(value);
  assert.throws(() => value.string(), {category:'moved'});
  assert.equal(n.valid, false); assert.equal(snapshot(n).decimal.scale, '2');
  assert.equal(inner.is_nil(), true); assert.throws(() => { n.valid = 1; }, {category:'parameter'});
  inner.free(); n.free();
});

test('buffers copy bytes and foreign-run wrappers are rejected', () => {
  const input = new Uint8Array([0,128,255]); const buffer = ByteBuffer.new(input);
  input[0] = 3; const output = buffer.to_uint8_array(); assert.deepEqual([...output], [0,128,255]);
  output[0] = 9; assert.equal(buffer.to_uint8_array()[0],0);
  const other = createBridge(raw).Decimal.new(2n); const ours = Decimal.new(3n);
  assert.throws(() => ours.mul_exact(other), {category:'parameter'});
  buffer.free(); other.free(); ours.free();
});
