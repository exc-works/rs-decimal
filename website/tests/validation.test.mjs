import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
import init,* as raw from '../public/wasm/decimal.js';
import {createBridge} from '../src/bridge/index.js';
await init({module_or_path:await readFile(new URL('../public/wasm/decimal_bg.wasm',import.meta.url))});
const bridge=createBridge(raw),{Decimal:D,validation:V,snapshot}=bridge,{ValidationRule:R,ValidationMessages:M}=V;
const invalid=e=>e.category==='library'&&e.kinds.includes('InvalidArgument');
test('validation comparisons stay exact and precision uses retained scale',()=>{
 const d=D.new_from_string('9007199254740993.00000000000000001'),before=snapshot(d);
 for(const [tag,param,want] of [['decimal_gt','9007199254740993',true],['decimal_eq','9007199254740993',false],['decimal_between','9007199254740993~9007199254740994',true],['decimal_max_precision','2',false]]){
  const r=R.parse(tag,param);assert.equal(r.matches(d),want);assert.deepEqual(snapshot(d),before);r.free();
 }
 const scaled=D.new_from_string('1.2300');assert.equal(R.parse('decimal_max_precision','2').matches(scaled),false);scaled.free();d.free();
});
test('required and signed rules retain Rust nil versus initialized zero semantics',()=>{
 const nil=D.default(),zero=D.new(0n),negative=D.new(-1n);
 for(const [tag,param,want] of [['decimal_required','',false],['decimal_eq','0',true],['decimal_between','0~0',false],['decimal_max_precision','0',true],['decimal_nonzero','',false]]){const r=R.parse(tag,param);assert.equal(r.matches(nil),want);r.free();}
 assert.equal(R.parse('decimal_required','').matches(zero),true);assert.equal(R.parse('decimal_negative','').matches(negative),true);assert.equal(R.parse('decimal_positive','').matches(zero),false);nil.free();zero.free();negative.free();
});
test('failed validation carries original field/tag/parameter and aggregates in caller order',()=>{
 const d=D.default(),r=R.parse('decimal_gt',' 0 '),required=R.parse('decimal_required','');
 assert.throws(()=>r.validate('amount{1}',d),e=>e.name==='ValidationError'&&e.category==='validation'&&e.field==='amount{1}'&&e.tag==='decimal_gt'&&e.parameter===' 0 ');
 assert.deepEqual(V.validate_decimal('amount',d,[r,required,r]),[{field:'amount',tag:'decimal_gt',parameter:' 0 '},{field:'amount',tag:'decimal_required',parameter:''},{field:'amount',tag:'decimal_gt',parameter:' 0 '}]);
 const one=D.new(1n);assert.equal(r.validate('amount',one),undefined);assert.deepEqual(V.validate_decimal('amount',one,[r,required]),[]);one.free();r.free();required.free();d.free();
});
test('all eight requested locales and unknown-locale fallback come from Rust templates',()=>{
 const expected={'zh-Hans':'amount为必填项','zh-Hant':'amount為必填項','en-US':'amount is required','en-GB':'amount is required',fr:'amount est obligatoire',ko:'amount은(는) 필수 항목입니다',ja:'amountは必須です',de:'amount ist erforderlich'};
 const errors=[{field:'amount',tag:'decimal_required',parameter:''}];
 for(const [locale,want] of Object.entries(expected)){const m=M.new(locale);assert.deepEqual(m.translate(errors),[want]);m.free();}
 assert.deepEqual(M.new('unknown').translate(errors),['amount is required']);const map=V.default_validation_messages('zh-Hant');map.decimal_required='changed';assert.equal(V.default_validation_messages('zh-Hant').decimal_required,'{0}為必填項');
});
test('custom templates use English defaults and single-pass literal substitution',()=>{
 const m=M.with_messages({decimal_required:'Required: {0}',decimal_gt:'{0}/{1}/{0}'});
 const errors=[{field:'amount{1}',tag:'decimal_required',parameter:''},{field:'amount{1}',tag:'decimal_gt',parameter:'{0}'},{field:'amount',tag:'decimal_ne',parameter:'0'}];
 assert.deepEqual(m.translate(errors),['Required: amount{1}','amount{1}/{0}/amount{1}','amount must not be equal to 0']);
 errors[0].field='edited';errors[0].tag='unknown';assert.deepEqual(m.translate([errors[0]]),['edited is invalid']);m.free();
 for(const template of ['','{2}','{{0}}','{1{0}}','{中}','text}'])assert.throws(()=>M.with_messages({decimal_gt:template}),invalid);
 assert.throws(()=>M.with_messages({unknown:'x'}),invalid);
});
test('rule syntax errors remain library failures while growth and list/message expansion are bounded',()=>{
 for(const [tag,param] of [['decimal_between','2~1'],['decimal_eq','bad'],['decimal_max_precision','-1'],['decimal_max_precision','2147483648'],['unknown','']])assert.throws(()=>R.parse(tag,param),invalid);
 for(const [tag,param] of [['decimal_eq','1e100000'],['decimal_between','0~1e100000']])assert.throws(()=>R.parse(tag,param),{category:'resource'});
 const d=D.new(1n),r=R.parse('decimal_gt','0');assert.throws(()=>V.validate_decimal('field',d,new Array(65).fill(r)),{category:'resource'});
 const m=M.with_messages({decimal_required:'{0}'.repeat(2000)}),error={field:'x'.repeat(8192),tag:'decimal_required',parameter:''};assert.throws(()=>m.translate([error]),{category:'resource'});assert.deepEqual(m.translate([]),[]);
 assert.throws(()=>M.new('en').translate([{field:'x'.repeat(8193),tag:'decimal_required',parameter:''}]),{category:'resource'});
 const direct=raw.ValidationMessages.with_messages(JSON.stringify({decimal_required:'{0}'.repeat(2000)}));assert.throws(()=>direct.translate(JSON.stringify([error])),{category:'resource'});direct.free();
 assert.throws(()=>raw.ValidationRule.parse('decimal_eq','1e100000'),{category:'resource'});m.free();r.free();d.free();
});
test('validation objects reject foreign or disposed handles and invalid parameter shapes',()=>{
 const d=D.new(1n),r=R.parse('decimal_gt','0'),other=createBridge(raw).validation.ValidationRule.parse('decimal_gt','0');
 assert.throws(()=>V.validate_decimal('field',d,[other]),{category:'parameter'});assert.throws(()=>M.new('en').translate([{field:'x',tag:'decimal_gt',parameter:0}]),{category:'parameter'});assert.throws(()=>M.with_messages({decimal_gt:1}),{category:'parameter'});
 r.free();assert.throws(()=>r.matches(d),{category:'moved'});d.into_coefficient();assert.throws(()=>other.matches(d),{category:'parameter'});other.free();assert.ok(bridge.metadata().features.includes('validation'));
});
