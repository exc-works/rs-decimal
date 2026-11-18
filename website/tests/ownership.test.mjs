import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const {Decimal:D,RoundingMode:R,snapshot}=createBridge(raw);
const kind=k=>e=>e.category==='library'&&e.kinds.includes(k);
test('owned arithmetic consumes only left and rejects aliases before moving',()=>{
 for(const [method,want] of [['add_owned','2.25'],['sub_owned','1.75']]){
  const a=D.new(2n),b=D.new_with_prec(25n,2);assert.throws(()=>a[method](a),{category:'parameter'});assert.equal(a.string(),'2');
  const dead=D.new(1n);dead.free();assert.throws(()=>a[method](dead),{category:'moved'});assert.equal(a.string(),'2');
  const foreign=createBridge(raw).Decimal.new(1n);assert.throws(()=>a[method](foreign),{category:'parameter'});foreign.free();
  const out=a[method](b);assert.equal(out.string_with_trailing_zeros(),want);assert.throws(()=>a.string(),{category:'moved'});assert.equal(b.string(),'0.25');a.free();b.free();out.free();
  const large=D.new_from_string('9'.repeat(4096)),tiny=D.new_with_prec(1n,1000);assert.throws(()=>large[method](tiny),{category:'resource'});assert.throws(()=>large.coefficient(),{category:'moved'});assert.equal(tiny.coefficient(),1n);large.free();tiny.free();
 }
});
test('into_rescaled preserves pre-call validation and consumes on Rust errors',()=>{
 for(const [p,m,category] of [[-1,R.Down,'library'],[1001,R.Down,'resource'],[0,R.Unnecessary,'library'],[1,7,'library']]){
  const a=D.new_from_string('1.25');assert.throws(()=>a.into_rescaled(p,m),e=>e.category===category);assert.throws(()=>a.scale(),{category:'moved'});a.free();
 }
 const a=D.new_from_string('1.25');assert.throws(()=>a.into_rescaled(.5,R.Down),{category:'parameter'});assert.equal(a.string(),'1.25');const out=a.into_rescaled(1,R.HalfEven);assert.equal(out.string(),'1.2');assert.throws(()=>a.string(),{category:'moved'});out.free();
});
test('coefficient ownership retains scale and exact bigint before extraction',()=>{
 const a=D.try_from_coefficient(900719925474099312300n,4);assert.equal(a.string_with_trailing_zeros(),'90071992547409931.2300');assert.equal(a.into_coefficient(),900719925474099312300n);assert.throws(()=>a.scale(),{category:'moved'});
 const empty=D.default();assert.equal(empty.into_coefficient(),0n);assert.throws(()=>empty.is_nil(),{category:'moved'});
 const z=D.try_from_coefficient(0n,3);assert.equal(z.is_nil(),false);assert.equal(z.scale(),3);z.free();
 for(const s of [-1,.5,4294967296])assert.throws(()=>D.try_from_coefficient(1n,s),{category:'parameter'});
 assert.throws(()=>D.try_from_coefficient(1n,2147483648),kind('Overflow'));assert.throws(()=>D.try_from_coefficient(1n,1001),{category:'resource'});
 assert.throws(()=>raw.Decimal.try_from_coefficient('1',1001),{category:'resource'});
});
test('reference selection preserves original identity, representation and alias lifetime',()=>{
 const a=D.new_from_string('1.00'),b=D.new_from_string('1.0'),c=D.new(2n);
 assert.equal(a.max_ref(b),a);assert.equal(a.min_ref(b),a);assert.equal(a.max_ref(c),c);assert.equal(c.min_ref(a),a);assert.equal(a.min_ref(a),a);
 const alias=a.max_ref(b);alias.into_coefficient();assert.throws(()=>a.string(),{category:'moved'});assert.equal(b.string(),'1');
 const empty=D.default(),zero=D.new(0n);assert.equal(empty.min_ref(zero),empty);assert.equal(empty.max_ref(zero).is_nil(),true);
 for(const d of [a,b,c,empty,zero])d.free();
});
test('quotient/remainder tuples preserve order, signs and independent handles',()=>{
 for(const [as,bs,qs,rs] of [['-5.50','2','-2','-1.50'],['5.50','-2','-2','1.50']]){
  const a=D.new_from_string(as),b=D.new_from_string(bs),before=snapshot(a),[q,r]=a.quo_rem(b);assert.equal(q.string(),qs);assert.equal(q.scale(),0);assert.equal(r.string_with_trailing_zeros(),rs);
  const product=q.mul_exact(b),sum=product.add(r);assert.equal(sum.equal(a),true);
  for(const method of ['mod','modulo']){const v=a[method](b);assert.deepEqual(snapshot(v),snapshot(r));v.free();}
  q.free();assert.equal(r.string_with_trailing_zeros(),rs);assert.deepEqual(snapshot(a),before);
  for(const v of [a,b,r,product,sum])v.free();
 }
 const a=D.new_from_string('-12.750');assert.deepEqual(a.remainder(),[-12n,-750n]);a.free();
 const large=D.new_from_string('9'.repeat(4096)),zero=D.new_with_prec(0n,1000);assert.throws(()=>large.quo_rem(zero),kind('DivideByZero'));
 const tiny=D.new_with_prec(1n,1000);assert.throws(()=>large.modulo(tiny),{category:'resource'});for(const v of [large,zero,tiny])v.free();
});
test('power preserves half-even, reciprocal, zero and unit fast paths with bounds',()=>{
 const a=D.new_from_string('2.00'),reciprocal=a.power(-3n);assert.equal(reciprocal.string_with_trailing_zeros(),'0.12');assert.equal(a.string_with_trailing_zeros(),'2.00');reciprocal.free();
 const zero=D.default(),one=zero.power(0n);assert.equal(one.string(),'1');assert.equal(one.is_nil(),false);assert.throws(()=>zero.power(-1n),kind('DivideByZero'));
 const unit=D.new_from_string('-1.00'),even=unit.power(-(1n<<63n)),odd=unit.power((1n<<63n)-1n);assert.equal(even.string_with_trailing_zeros(),'1.00');assert.equal(odd.string(),'-1');
 assert.throws(()=>a.power(10000n),{category:'resource'});assert.throws(()=>a.power((1n<<63n)-1n),kind('Overflow'));assert.throws(()=>a.power(2),{category:'parameter'});
 const tiny=D.new_from_string('0.1');assert.throws(()=>tiny.power(1001n),{category:'resource'});
 const rawValue=raw.Decimal.new_i64_text('2');assert.throws(()=>rawValue.power('10000'),{category:'resource'});rawValue.free();
 for(const v of [a,zero,one,unit,even,odd,tiny])v.free();
});
