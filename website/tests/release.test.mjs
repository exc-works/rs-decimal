import test from 'node:test';
import assert from 'node:assert/strict';
import {checkBuilds, releaseConfig} from '../scripts/release-config.mjs';
test('single-site configuration supports root/project paths and rejects traversal or old origins', () => {
  assert.equal(releaseConfig({}).runnerUrl, '/runner/');
  assert.equal(releaseConfig({DOCS_BASE:'/rs-decimal/'}).runnerUrl, '/rs-decimal/runner/');
  for (const base of ['/../','/./','//evil/','/x?y/','/x#y/','/x%2fy/','https://evil/','/no-trailing'])
    assert.throws(() => releaseConfig({DOCS_BASE:base}));
  assert.throws(() => releaseConfig({DOCS_RUNNER_URL:'https://runner.example/'}));
});
test('mixed versions, stale paths, dirty docs and wrong sandbox metadata fail closed', () => {
  const config=releaseConfig({DOCS_BASE:'/rs-decimal/'}),commit='a'.repeat(40);
  const docs={schema:2,commit,dirty:false,...config};
  const runner={schema:2,commit,isolation:'opaque-iframe-blob-worker'};
  const wasm={commit,wasm_memory_limit_bytes:67108864};
  checkBuilds(config,commit,docs,runner,wasm);
  for(const change of [{commit:'b'.repeat(40)},{dirty:true},{base:'/'},{runnerUrl:'/wrong/'}])
    assert.throws(()=>checkBuilds(config,commit,{...docs,...change},runner,wasm));
  assert.throws(()=>checkBuilds(config,commit,docs,{...runner,schema:1},wasm));
  assert.throws(()=>checkBuilds(config,commit,docs,runner,{...wasm,wasm_memory_limit_bytes:0}));
});
