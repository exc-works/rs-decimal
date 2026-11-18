import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const {Decimal:D,snapshot}=createBridge(raw);
const kind=k=>e=>e.category==='library'&&e.kinds.includes(k);
test('roots retain default precision, initialized zero and borrowed state',()=>{
 const d=D.new(2n),before=snapshot(d),root=d.sqrt();assert.equal(root.string_with_trailing_zeros(),'1.41421356237309504880');assert.equal(root.scale(),20);assert.deepEqual(snapshot(d),before);
 const high=D.new_with_prec(4000000000000000000n,18).rescale(25,6),out=high.sqrt();assert.equal(out.scale(),25);assert.equal(out.string(),'2');
 const empty=D.default(),z=empty.sqrt();assert.equal(z.is_nil(),false);assert.equal(z.scale(),20);assert.equal(empty.is_nil(),true);
 for(const v of [d,root,high,out,empty,z])v.free();
});
test('exact midpoint roots use half-even with signed odd roots',()=>{
 for(const [source,degree,want] of [['2.25',2n,'2'],['6.25',2n,'2'],['-3.375',3n,'-2'],['-15.625',3n,'-2']]){const d=D.new_from_string(source),r=d.approx_root_with_prec(degree,0);assert.equal(r.string(),want);d.free();r.free();}
 const negative=D.new(-8n),cube=negative.approx_root(3n);assert.equal(cube.string(),'-2');assert.equal(cube.scale(),20);negative.free();cube.free();
});
test('domain and precision errors retain core precedence and do not consume',()=>{
 const d=D.new(-2n);assert.throws(()=>d.sqrt(),kind('NegativeRoot'));assert.throws(()=>d.sqrt_with_prec(-1),kind('InvalidPrecision'));assert.throws(()=>d.sqrt_with_prec(1001),kind('NegativeRoot'));
 assert.throws(()=>d.approx_root_with_prec(0n,1001),kind('InvalidRoot'));assert.throws(()=>d.approx_root_with_prec(-1n,-1),kind('InvalidPrecision'));
 assert.throws(()=>d.approx_root(2),{category:'parameter'});assert.throws(()=>d.sqrt_with_prec(.5),{category:'parameter'});assert.equal(d.string(),'-2');d.free();
});
test('root intermediate budgets prevent large powers but allow zero/unit fast paths',()=>{
 const d=D.new(2n);assert.throws(()=>d.approx_root(129n),{category:'resource'});assert.throws(()=>d.approx_root_with_prec(128n,1000),{category:'resource'});assert.throws(()=>d.sqrt_with_prec(1001),{category:'resource'});
 const exact=d.approx_root_with_prec(1n,2);assert.equal(exact.string_with_trailing_zeros(),'2.00');exact.free();
 const zero=D.default(),unit=D.new(-1n),z=zero.approx_root_with_prec((1n<<63n)-1n,2),u=unit.approx_root_with_prec((1n<<63n)-1n,2);assert.equal(z.string_with_trailing_zeros(),'0.00');assert.equal(u.string_with_trailing_zeros(),'-1.00');
 const rawValue=raw.Decimal.new_i64_text('2');assert.throws(()=>rawValue.approx_root_with_prec('128',1000),{category:'resource'});rawValue.free();
 for(const v of [d,zero,unit,z,u])v.free();
});
