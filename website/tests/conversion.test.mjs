import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
import {numberSnapshot} from '../src/runner/number-snapshot.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const {Decimal:D,RoundingMode:R,most_significant_bit}=createBridge(raw);
const kind=k=>e=>e.category==='library'&&e.kinds.includes(k);
function bits(value,width){const v=new DataView(new ArrayBuffer(8));if(width===32){v.setFloat32(0,value);return v.getUint32(0).toString(16).padStart(8,'0');}v.setFloat64(0,value);return v.getBigUint64(0).toString(16).padStart(16,'0');}
test('integer tuple flags reject fractions and overflow without losing bigint precision',()=>{
 for(const [s,signed,unsigned] of [['9223372036854775807',[9223372036854775807n,true],[9223372036854775807n,true]],['-9223372036854775808',[-9223372036854775808n,true],[0n,false]],['18446744073709551615',[0n,false],[18446744073709551615n,true]],['18446744073709551616',[0n,false],[0n,false]],['1.2',[0n,false],[0n,false]],['12.00',[12n,true],[12n,true]]]){
  const d=D.new_from_string(s);assert.deepEqual(d.int64(),signed);assert.deepEqual(d.uint64(),unsigned);d.free();
 }
 const empty=D.default();assert.deepEqual(empty.int64(),[0n,true]);assert.equal(empty.is_nil(),true);empty.free();
});
test('float constructors use shortest round trips with explicit binary32 narrowing',()=>{
 const f32=D.new_from_float32(.1),f64=D.new_from_float64(.1),narrow=D.new_from_float32(16777217),negzero=D.new_from_float64(-0);
 assert.equal(f32.string(),'0.1');assert.equal(f64.string(),'0.1');assert.equal(narrow.string(),'16777216');assert.equal(negzero.string(),'0');assert.equal(negzero.is_nil(),false);
 for(const method of ['new_from_float32','new_from_float64']){
  for(const v of [NaN,Infinity,-Infinity])assert.throws(()=>D[method](v),kind('InvalidFormat'));
  for(const v of ['1',1n,null])assert.throws(()=>D[method](v),{category:'parameter'});
 }
 assert.throws(()=>D.new_from_float32(Number.MAX_VALUE),kind('InvalidFormat'));
 for(const v of [f32,f64,narrow,negzero])v.free();
});
test('float conversion preserves exactness, signed zero, infinity and tie bits',()=>{
 for(const [s,width,expected,exact] of [['0.5',32,'3f000000',true],['0.1',32,'3dcccccd',false],['0.1',64,'3fb999999999999a',false],['16777217',32,'4b800000',false],['1.000000059604644775390625',32,'3f800000',false],['-1e-1000',32,'80000000',false],['-1e-1000',64,'8000000000000000',false],['1e1000',32,'7f800000',false],['-1e1000',64,'fff0000000000000',false],['1e-45',32,'00000001',false],['5e-324',64,'0000000000000001',false]]){
  const d=D.new_from_string(s),[value,flag]=d['float'+width]();assert.equal(bits(value,width),expected,s);assert.equal(flag,exact,s);d.free();
 }
});
test('rational constructors reduce exactly and reject nontermination or zero denominator',()=>{
 const a=D.new_from_big_rat({numerator:2n,denominator:-8n});assert.equal(a.string(),'-0.25');assert.deepEqual(a.big_rat(),{numerator:-1n,denominator:4n});assert.equal(Object.isFrozen(a.big_rat()),true);a.free();
 assert.throws(()=>D.new_from_big_rat({numerator:1n,denominator:3n}),kind('InvalidFormat'));
 assert.throws(()=>D.new_from_big_rat({numerator:1n,denominator:0n}),{category:'parameter'});assert.throws(()=>raw.Decimal.new_from_big_rat('1','0'),{category:'parameter'});
 assert.throws(()=>D.new_from_big_rat({numerator:1,denominator:2n}),{category:'parameter'});
 const z=D.new_from_big_rat({numerator:0n,denominator:-3n});assert.deepEqual(z.big_rat(),{numerator:0n,denominator:1n});z.free();
});
test('rational precision and scale/growth limits are checked in Rust',()=>{
 const r={numerator:1n,denominator:3n},d=D.new_from_big_rat_with_prec(r,2,R.HalfEven);assert.equal(d.string_with_trailing_zeros(),'0.33');d.free();
 assert.throws(()=>D.new_from_big_rat_with_prec(r,2,R.Unnecessary),kind('RoundUnnecessary'));assert.throws(()=>D.new_from_big_rat_with_prec(r,-1,R.Down),kind('InvalidPrecision'));
 assert.throws(()=>raw.Decimal.new_from_big_rat_with_prec('1','3',1001,0),{category:'resource'});
 assert.throws(()=>raw.Decimal.new_from_big_rat('1',(1n<<1001n).toString()),{category:'resource'});
 assert.throws(()=>raw.Decimal.new_from_big_rat('1',(5n**1001n).toString()),{category:'resource'});
 const boundary=D.new_from_big_rat({numerator:1n,denominator:1n<<1000n});assert.equal(boundary.scale(),1000);boundary.free();
 // Reduction must precede output-scale checks.
 const reduced=D.new_from_big_rat({numerator:1n<<1001n,denominator:1n<<1002n});assert.equal(reduced.string(),'0.5');reduced.free();
 assert.throws(()=>raw.Decimal.new_from_big_rat_with_prec('9'.repeat(4096),'3',1000,0),{category:'resource'});
});
test('bit queries use the unscaled coefficient and zero-based index',()=>{
 const d=D.new_from_string('-1.20');assert.equal(d.bit_len(),7);assert.equal(most_significant_bit(120n),6);assert.equal(most_significant_bit(0n),0);assert.equal(most_significant_bit(1n),0);assert.throws(()=>most_significant_bit(-1n),kind('InvalidArgument'));assert.throws(()=>most_significant_bit(1),{category:'parameter'});d.free();
});
test('output snapshots survive JSON for exceptional floats',()=>{
 for(const [value,label,expected] of [[Infinity,'Infinity','7ff0000000000000'],[-Infinity,'-Infinity','fff0000000000000'],[-0,'-0','8000000000000000']])assert.deepEqual(JSON.parse(JSON.stringify(numberSnapshot(value))),{type:'float64',value:label,bits:expected});
 assert.equal(numberSnapshot(NaN).value,'NaN');assert.equal(numberSnapshot(.5),.5);assert.equal(numberSnapshot(0),0);
});
