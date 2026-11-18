import { readFile } from 'node:fs/promises';
import assert from 'node:assert/strict';
import test from 'node:test';
import init, * as raw from '../public/wasm/decimal.js';
import { createBridge } from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const { Decimal, snapshot }=createBridge(raw);
test('coefficient precision and appended zeros are different constructions',()=>{
  const scaled=Decimal.new_with_prec(12n,2), appended=Decimal.new_with_append_prec(12n,2);
  assert.equal(scaled.string_with_trailing_zeros(),'0.12');
  assert.equal(appended.string_with_trailing_zeros(),'12.00');
  const zero=Decimal.new_with_prec(0n,2), parsed=Decimal.new_from_string('0.00');
  assert.equal(zero.scale(),2);assert.equal(parsed.scale(),0);
  const original=Decimal.default(), copied=Decimal.new_from_decimal(original);
  assert.deepEqual(snapshot(copied),snapshot(original));assert.equal(copied.is_nil(),true);
  for(const value of [scaled,appended,zero,parsed,original,copied])value.free();
});
test('exact integer transports preserve boundaries and reject narrowing',()=>{
  for(const method of ['new_with_prec','new_from_int64']) {
    const value=Decimal[method](9223372036854775807n,2);assert.equal(value.coefficient(),9223372036854775807n);value.free();
    assert.throws(()=>Decimal[method](1n<<63n,2),{category:'parameter'});
    assert.throws(()=>Decimal[method](1,2),{category:'parameter'});
  }
  for(const method of ['new_from_uint64','new_from_uint_with_append_prec']) {
    const value=Decimal[method]((1n<<64n)-1n,0);assert.equal(value.coefficient(),(1n<<64n)-1n);value.free();
    assert.throws(()=>Decimal[method](-1n,0),{category:'parameter'});
    assert.throws(()=>Decimal[method](1n<<64n,0),{category:'parameter'});
  }
  const native=Decimal.new_from_int(2147483647);assert.equal(native.coefficient(),2147483647n);native.free();
  assert.throws(()=>Decimal.new_from_int(2147483648),{category:'parameter'});
  const big=Decimal.new_from_big_int(900719925474099312345n);assert.equal(big.coefficient(),900719925474099312345n);big.free();
});
test('constructor preflight keeps library errors distinct and bounds raw allocation',()=>{
  for(const method of ['new_with_prec','new_from_int64','new_from_uint64','new_from_big_int_with_prec','new_with_append_prec','new_from_uint_with_append_prec']) {
    assert.throws(()=>Decimal[method](12n,-1),e=>e.category==='library'&&e.kinds.includes('InvalidPrecision'));
    assert.throws(()=>Decimal[method](12n,1.5),{category:'parameter'});
    assert.throws(()=>Decimal[method](12n,2147483647),{category:'resource'});
  }
  assert.throws(()=>Decimal.new_from_big_int(10n**4096n),{category:'resource'});
  assert.throws(()=>raw.Decimal.new_from_big_int('9'.repeat(4097)),{category:'resource'});
  assert.throws(()=>raw.Decimal.new_with_append_prec('1',2147483647),{category:'resource'});
});
