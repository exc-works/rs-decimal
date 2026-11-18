import test from 'node:test';
import assert from 'node:assert/strict';
import { checkBuilds, releaseConfig } from '../scripts/release-config.mjs';

const input = {DOCS_ORIGIN: 'https://docs.example.org', DOCS_RUNNER_URL: 'https://runner.example.net/', DOCS_BASE: '/rs-decimal/'};
test('deployment rejects shared origins, unsafe base paths and development URLs by default', () => {
  assert.equal(releaseConfig(input).base, '/rs-decimal/');
  for (const change of [
    {DOCS_RUNNER_URL: input.DOCS_ORIGIN + '/'}, {DOCS_BASE: '/../'},
    {DOCS_BASE: '/x?y/'}, {DOCS_RUNNER_URL: 'https://runner.example.net/sub/'},
    {DOCS_RUNNER_URL: 'https://runner.example.net/#x'}, {DOCS_ORIGIN: 'https://u:p@docs.example.org'},
    {DOCS_ORIGIN: 'http://127.0.0.1:4210'},
  ]) assert.throws(() => releaseConfig({...input, ...change}));
  assert.equal(releaseConfig({...input, DOCS_ORIGIN: 'http://127.0.0.1:4210', RELEASE_ALLOW_LOCALHOST: '1'}).local, true);
});
test('mixed commits, stale URL configuration, dirty docs and unbounded Wasm cannot be packaged', () => {
  const config = releaseConfig(input), commit = 'a'.repeat(40);
  const docs = {schema: 1, commit, dirty: false, base: config.base, runnerUrl: config.runnerUrl};
  const runner = {schema: 1, commit, parentOrigins: [config.docsOrigin]};
  const wasm = {commit, wasm_memory_limit_bytes: 67108864};
  checkBuilds(config, commit, docs, runner, wasm);
  for (const change of [{commit: 'b'.repeat(40)}, {dirty: true}, {base: '/'}, {runnerUrl: 'https://other.example/'}])
    assert.throws(() => checkBuilds(config, commit, {...docs, ...change}, runner, wasm));
  assert.throws(() => checkBuilds(config, commit, docs, {...runner, parentOrigins: ['*']}, wasm));
  assert.throws(() => checkBuilds(config, commit, docs, runner, {...wasm, wasm_memory_limit_bytes: 0}));
});
