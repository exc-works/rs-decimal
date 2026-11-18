import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
const bytes=await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url));
await init({module_or_path:bytes});
const {Decimal,RoundingMode:R,snapshot}=createBridge(raw);
const kind=name=>e=>e.category==='library'&&e.kinds.includes(name);
test('aligned arithmetic is exact and safe methods constrain only the result',()=>{
 const a=Decimal.new_from_string('-1.20'),b=Decimal.new_from_string('2.3'),before=snapshot(a);
 for(const [method,answer] of [['add','1.10'],['sub','-3.50'],['safe_add','1.10']]){const v=a[method](b);assert.equal(v.string_with_trailing_zeros(),answer);v.free();}
 assert.throws(()=>a.safe_sub(b),kind('InvalidArgument'));
 const result=b.safe_sub(a);assert.equal(result.string_with_trailing_zeros(),'3.50');result.free();
 const negative=Decimal.new(-10n);assert.throws(()=>a.safe_add(negative),kind('InvalidArgument'));negative.free();
 assert.deepEqual(snapshot(a),before);assert.equal(b.string(),'2.3');a.free();b.free();
});
test('raw arithmetic uses coefficient units and exact i64 inputs',()=>{
 const a=Decimal.new_from_string('1.20');
 for(const [method,answer] of [['add_raw','1.21'],['sub_raw','1.19']]){const v=a[method](1n);assert.equal(v.string_with_trailing_zeros(),answer);v.free();
  for(const bad of [1,1n<<63n,-(1n<<63n)-1n])assert.throws(()=>a[method](bad),{category:'parameter'});
 }
 const low=a.add_raw(-(1n<<63n)),high=a.sub_raw((1n<<63n)-1n);assert.equal(low.coefficient(),120n-(1n<<63n));assert.equal(high.coefficient(),121n-(1n<<63n));
 for(const v of [a,low,high])v.free();
});
test('multiplication distinguishes exact scale, explicit precision and signed truncation',()=>{
 const a=Decimal.new_from_string('-1.25'),b=Decimal.new_from_string('1.3');
 for(const [v,want] of [[a.mul(b,R.HalfEven),'-1.62'],[a.mul(b,R.HalfUp),'-1.63'],[a.mul_down(b),'-1.62'],[a.mul_exact(b),'-1.625'],[a.mul_with_prec(b,4,R.Unnecessary),'-1.6250']]){assert.equal(v.string_with_trailing_zeros(),want);v.free();}
 assert.throws(()=>a.mul(b,R.Unnecessary),kind('RoundUnnecessary'));assert.throws(()=>a.mul_with_prec(b,-1,R.Down),kind('InvalidPrecision'));
 for(const p of [.5,Infinity,2147483648])assert.throws(()=>a.mul_with_prec(b,p,R.Down),{category:'parameter'});
 assert.throws(()=>a.mul(b,7),kind('InvalidArgument'));
 // A rounded result may fit even when the exact product's scale does not.
 const tiny=Decimal.new_with_prec(1n,600),rounded=tiny.mul(tiny,R.Down);assert.equal(rounded.scale(),600);assert.equal(rounded.coefficient(),0n);assert.throws(()=>tiny.mul_exact(tiny),{category:'resource'});
 for(const v of [a,b,tiny,rounded])v.free();
});
test('division keeps input precision and core error precedence',()=>{
 const a=Decimal.new(-1n),b=Decimal.new(3n),zero=Decimal.default();
 for(const [v,want] of [[a.quo(b,R.HalfEven),'0'],[a.quo_down(b),'0'],[a.quo_with_prec(b,2,R.HalfEven),'-0.33'],[a.quo_with_prec(b,2,R.Up),'-0.34']]){assert.equal(v.string(),want);v.free();}
 assert.throws(()=>a.quo(b,R.Unnecessary),kind('RoundUnnecessary'));
 assert.throws(()=>a.quo_with_prec(zero,2147483647,R.Down),kind('DivideByZero'));
 assert.throws(()=>a.quo_with_prec(zero,-1,R.Down),kind('InvalidPrecision'));
 assert.throws(()=>a.quo_with_prec(b,1001,R.Down),{category:'resource'});
 const z=zero.quo_with_prec(b,4,R.Unnecessary);assert.equal(z.scale(),4);assert.equal(z.is_nil(),false);assert.equal(zero.is_nil(),true);
 for(const v of [a,b,zero,z])v.free();
});
test('fraction queries and integer extraction preserve exact signed values',()=>{
 for(const [s,integer,part] of [['12.00',true,12n],['-12.75',false,-12n],['9007199254740993.99',false,9007199254740993n]]){
  const a=Decimal.new_from_string(s);assert.equal(a.is_integer(),integer);assert.equal(a.has_fraction(),!integer);assert.equal(a.int_part(),part);a.free();
 }
 const empty=Decimal.default();assert.equal(empty.is_integer(),true);assert.equal(empty.has_fraction(),false);assert.equal(empty.int_part(),0n);assert.equal(empty.is_nil(),true);
 const copy=empty.must_non_negative();assert.equal(copy.is_nil(),false);copy.free();empty.free();
});
test('raw ABI rejects alignment, product and division growth before core allocation',()=>{
 const large=raw.Decimal.new_from_string('9'.repeat(4096)),tiny=raw.Decimal.new_with_prec('1',1000);
 for(const method of ['add','sub','safe_add','safe_sub'])assert.throws(()=>large[method](tiny),{category:'resource'});
 assert.throws(()=>large.mul(large,0),{category:'resource'});assert.throws(()=>large.mul_with_prec(large,0,0),{category:'resource'});
 assert.throws(()=>large.quo_with_prec(tiny,1000,0),{category:'resource'});assert.throws(()=>tiny.quo_with_prec(large,0,0),{category:'resource'});
 assert.equal(large.string(),'9'.repeat(4096));large.free();tiny.free();
});
test('negative requirement retains its panic in a dedicated Wasm instance',async()=>{
 const isolated=await import('../public/wasm/decimal.js?arithmetic-panic');await isolated.default({module_or_path:bytes});
 const {Decimal:D}=createBridge(isolated),negative=D.new(-1n);assert.throws(()=>negative.must_non_negative(),WebAssembly.RuntimeError);
});
