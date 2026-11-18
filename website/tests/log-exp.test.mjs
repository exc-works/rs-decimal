import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const {Decimal:D,snapshot}=createBridge(raw);
const kind=k=>e=>e.category==='library'&&e.kinds.includes(k);
test('certified log/exp return complete expected decimals and leave inputs unchanged',()=>{
 for(const [method,source,expected] of [['ln','2','0.69314718055994530942'],['log2','3','1.58496250072115618145'],['log10','2','0.30102999566398119521'],['exp','1','2.71828182845904523536']]){
  const d=D.new_from_string(source),before=snapshot(d),out=d[method](),short=d[method+'_with_prec'](6);assert.equal(out.string_with_trailing_zeros(),expected);assert.equal(out.scale(),20);assert.equal(out.is_nil(),false);assert.deepEqual(snapshot(d),before);assert.equal(short.scale(),6);d.free();out.free();short.free();
 }
});
test('exact log powers, exp zero and signed values preserve precision',()=>{
 for(const [source,method,expected] of [['1','ln','0.0000'],['0.125','log2','-3.0000'],['1000','log10','3.0000'],['-1','exp','0.3679']]){const d=D.new_from_string(source),out=d[method+'_with_prec'](4);assert.equal(out.string_with_trailing_zeros(),expected);d.free();out.free();}
 const zero=D.default(),one=zero.exp();assert.equal(one.string(),'1');assert.equal(one.scale(),20);assert.equal(one.is_nil(),false);assert.equal(zero.is_nil(),true);zero.free();one.free();
 const high=D.new_from_string('2.'+'0'.repeat(30)),out=high.ln();assert.equal(out.scale(),30);high.free();out.free();
});
test('domain and negative precision errors retain Rust precedence',()=>{
 for(const source of ['0','-1'])for(const method of ['ln','log2','log10']){const d=D.new_from_string(source);assert.throws(()=>d[method](),kind('InvalidLog'));assert.throws(()=>d[method+'_with_prec'](129),kind('InvalidLog'));assert.throws(()=>d[method+'_with_prec'](-1),kind('InvalidPrecision'));assert.equal(d.string(),source);d.free();}
 const d=D.new(1n);assert.throws(()=>d.exp_with_prec(-1),kind('InvalidPrecision'));for(const p of [.5,NaN,2147483648])assert.throws(()=>d.ln_with_prec(p),{category:'parameter'});d.free();
});
test('Rust rejects excessive input/precision before refinement and preserves proven zero',()=>{
 const two=D.new(2n);assert.throws(()=>two.ln_with_prec(129),{category:'resource'});const edge=two.ln_with_prec(128);assert.equal(edge.scale(),128);edge.free();
 const large=D.new_from_big_int(1n<<4096n);assert.throws(()=>large.log2(),{category:'resource'});large.free();
 const tiny=D.new_from_string('1e-257');assert.throws(()=>tiny.ln_with_prec(20),{category:'resource'});tiny.free();
 const positive=D.new(65n),negative=D.new(-65n);assert.throws(()=>positive.exp(),{category:'resource'});assert.throws(()=>negative.exp_with_prec(128),{category:'resource'});const small=negative.exp();assert.equal(small.coefficient(),0n);assert.equal(small.scale(),20);
 const rawValue=raw.Decimal.new_i64_text('2');assert.throws(()=>rawValue.ln_with_prec(129),{category:'resource'});rawValue.free();
 for(const v of [two,positive,negative,small])v.free();
});
