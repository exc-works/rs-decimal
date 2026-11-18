import { readFile } from 'node:fs/promises';
import assert from 'node:assert/strict';
import test from 'node:test';
import init, * as raw from '../public/wasm/decimal.js';
import { createBridge } from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const {Decimal,RoundingMode,snapshot}=createBridge(raw);
test('all seven modes preserve signed tie rules and original input',()=>{
  const names=['Down','Up','Ceiling','HalfUp','HalfDown','HalfEven','Unnecessary'];
  for(const [source,answers] of [['2.5',['2','3','3','3','2','2']],['-2.5',['-2','-3','-2','-3','-2','-2']]]){
    const input=Decimal.new_from_string(source),before=snapshot(input);
    names.forEach((name,index)=>{
      assert.equal(RoundingMode[name],index);assert.equal(RoundingMode.from_raw(index),index);
      if(name==='Unnecessary') assert.throws(()=>input.rescale(0,RoundingMode[name]),e=>e.category==='library'&&e.kinds.includes('RoundUnnecessary'));
      else {const result=input.rescale(0,RoundingMode[name]);assert.equal(result.string(),answers[index]);result.free();}
    });
    assert.deepEqual(snapshot(input),before);input.free();
  }
  for(const source of ['3.5','-3.5']) {const input=Decimal.new_from_string(source),result=input.round();assert.equal(result.string(),source[0]==='-'?'-4':'4');input.free();result.free();}
});
test('directed methods and precision variants keep expected scale',()=>{
  const input=Decimal.new_from_string('-1.235');
  for(const [method,expected] of [['ceil','-1'],['floor','-2'],['truncate','-1'],['round','-1']]){const out=input[method]();assert.equal(out.string(),expected);assert.equal(out.scale(),0);out.free();}
  for(const [method,expected] of [['ceil_with_prec','-1.23'],['floor_with_prec','-1.24'],['truncate_with_prec','-1.23'],['rescale_down','-1.23'],['round_with_prec','-1.24']]){
    const out=input[method](2),expanded=input[method](4);assert.equal(out.string_with_trailing_zeros(),expected);assert.equal(expanded.string_with_trailing_zeros(),'-1.2350');
    assert.throws(()=>input[method](-1),e=>e.kinds.includes('InvalidPrecision'));assert.throws(()=>input[method](.5),{category:'parameter'});assert.throws(()=>input[method](1001),{category:'resource'});
    out.free();expanded.free();
  }
  input.free();
});
test('exact rounding, initialized zero and redundant zeros stay distinct',()=>{
  const exact=Decimal.new_from_string('1.20'),out=exact.rescale(1,RoundingMode.Unnecessary);assert.equal(out.string_with_trailing_zeros(),'1.2');exact.free();out.free();
  const empty=Decimal.default();
  for(const method of ['ceil','floor','round','truncate','strip_trailing_zeros']){const value=empty[method]();assert.equal(value.is_nil(),false);value.free();}
  const shifted=empty.shift(0),significant=empty.significant_figures(3,RoundingMode.HalfEven);assert.equal(shifted.is_nil(),false);assert.equal(significant.is_nil(),false);assert.equal(empty.is_nil(),true);
  const value=Decimal.new_from_string('12.3400'),stripped=value.strip_trailing_zeros();assert.equal(stripped.string_with_trailing_zeros(),'12.34');assert.equal(value.scale(),4);
  for(const d of [empty,shifted,significant,value,stripped])d.free();
});
test('shifts change value while significant figures never add precision',()=>{
  const input=Decimal.new_from_string('12.340'),right=input.shift(2),left=input.shift(-2);assert.equal(right.string_with_trailing_zeros(),'1234.0');assert.equal(left.string_with_trailing_zeros(),'0.12340');
  const large=Decimal.new_from_string('1234.5'),rounded=large.significant_figures(3,RoundingMode.HalfEven),copy=large.significant_figures(2147483647,RoundingMode.HalfEven);assert.equal(rounded.string(),'1230');assert.deepEqual(snapshot(copy),snapshot(large));
  assert.throws(()=>large.significant_figures(0,RoundingMode.HalfEven),e=>e.kinds.includes('InvalidArgument'));
  assert.throws(()=>large.significant_figures(3,RoundingMode.Unnecessary),e=>e.kinds.includes('RoundUnnecessary'));
  for(const d of [input,right,left,large,rounded,copy])d.free();
});
test('raw shift/precision allocation and significant carry stay bounded',()=>{
  const input=Decimal.new(1n);
  assert.throws(()=>input.shift(2147483647),{category:'resource'});assert.throws(()=>input.shift(-1001),{category:'resource'});
  assert.throws(()=>input.shift(-2147483648),e=>e.category==='library'&&e.kinds.includes('Overflow'));
  const rawValue=raw.Decimal.new_i64_text('1');assert.throws(()=>rawValue.shift(2147483647),{category:'resource'});assert.throws(()=>rawValue.floor_with_prec(1001),{category:'resource'});rawValue.free();
  const zero=Decimal.new(0n),shifted=zero.shift(2147483647);assert.equal(shifted.coefficient(),0n);
  const power=Decimal.new_from_big_int(1n<<8191n),large=power.mul_exact(power);assert.throws(()=>large.significant_figures(1,RoundingMode.HalfEven),{category:'resource'});assert.equal(large.coefficient(),1n<<16382n);
  for(const d of [input,zero,shifted,power,large])d.free();
});
test('invalid mode codes preserve library errors and reject JS narrowing',()=>{
  assert.throws(()=>RoundingMode.from_raw(7),e=>e.category==='library'&&e.kinds.includes('InvalidArgument'));
  for(const value of [1.5,NaN,Infinity,2147483648,'5'])assert.throws(()=>RoundingMode.from_raw(value),{category:'parameter'});
});
