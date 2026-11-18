import { readFile } from 'node:fs/promises';
import assert from 'node:assert/strict';
import test from 'node:test';
import init, * as raw from '../public/wasm/decimal.js';
import { createBridge } from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const bridge=createBridge(raw), {Decimal,snapshot}=bridge;
test('numeric equality ignores representation while min/max retain the first tie',()=>{
  const left=Decimal.new_from_string('12.00'),right=Decimal.new_from_string('12.0');
  const before=[snapshot(left),snapshot(right)];
  assert.equal(left.cmp(right),0);assert.equal(left.equal(right),true);assert.equal(left.not_equal(right),false);
  assert.equal(left.gt(right),false);assert.equal(left.gte(right),true);assert.equal(left.lt(right),false);assert.equal(left.lte(right),true);
  for(const method of ['min','max']){
    const result=left[method](right),reverse=right[method](left);
    assert.deepEqual(snapshot(result),before[0]);assert.deepEqual(snapshot(reverse),before[1]);
    result.free();reverse.free();
  }
  assert.deepEqual([snapshot(left),snapshot(right)],before);left.free();right.free();
});
test('uninitialized zero, initialized zero and unary results have distinct state',()=>{
  const empty=Decimal.default(),zero=Decimal.new_with_prec(0n,3);
  assert.equal(empty.equal(zero),true);assert.equal(empty.sign(),0);assert.equal(empty.is_zero(),true);
  assert.equal(empty.is_not_zero(),false);assert.equal(empty.is_positive(),false);assert.equal(empty.is_negative(),false);
  assert.equal(empty.is_nil(),true);assert.equal(zero.is_nil(),false);
  for(const method of ['abs','neg']){const value=empty[method]();assert.equal(value.is_nil(),false);value.free();}
  for(const method of ['min','max']){const value=empty[method](zero);assert.equal(value.is_nil(),true);value.free();}
  assert.equal(empty.is_nil(),true);empty.free();zero.free();
});
test('signed cross-scale comparisons and copied coefficients remain exact',()=>{
  const left=Decimal.new_from_string('-12.340'),right=Decimal.new_from_string('-12.35');
  assert.equal(left.cmp(right),1);assert.equal(right.cmp(left),-1);
  assert.equal(left.gt(right),true);assert.equal(left.gte(right),true);assert.equal(right.lt(left),true);assert.equal(right.lte(left),true);
  assert.equal(left.big_int(),-12340n);assert.equal(left.coefficient(),-12340n);assert.equal(left.precision(),3);assert.equal(left.scale(),3);
  const absolute=left.abs(),negated=left.neg();assert.equal(absolute.string_with_trailing_zeros(),'12.340');assert.deepEqual(snapshot(absolute),snapshot(negated));
  const positive=Decimal.new(1n);assert.equal(positive.is_positive(),true);assert.equal(left.is_negative(),true);assert.equal(positive.cmp(left),1);
  const huge=Decimal.new_from_string('1e1000'),tiny=Decimal.new_from_string('1e-1000');assert.equal(huge.cmp(tiny),1);
  const min=left.min(right),max=left.max(right);assert.equal(min.string_with_trailing_zeros(),'-12.35');assert.equal(max.string_with_trailing_zeros(),'-12.340');
  for(const value of [left,right,absolute,negated,positive,huge,tiny,min,max])value.free();
});
test('borrowed comparisons reject moved or foreign wrappers without consuming valid input',()=>{
  const value=Decimal.new(1n),gone=Decimal.new(2n),foreign=createBridge(raw).Decimal.new(1n);gone.free();
  assert.throws(()=>value.cmp(gone),{category:'moved'});assert.throws(()=>value.equal(foreign),{category:'parameter'});
  assert.equal(value.coefficient(),1n);value.free();foreign.free();
});
