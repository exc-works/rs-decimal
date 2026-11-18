export function releaseConfig(env = process.env) {
  const base = env.DOCS_BASE || '/';
  if (!/^\/(?:[A-Za-z0-9_.-]+\/)*$/.test(base) || base.split('/').some(x => x === '.' || x === '..'))
    throw Error('DOCS_BASE must be / or slash-terminated path segments');
  if (env.DOCS_RUNNER_URL || env.DOCS_ORIGIN || env.RUNNER_PARENT_ORIGINS)
    throw Error('Single-site builds only need DOCS_BASE; remove old origin settings');
  return {base, runnerUrl: base + 'runner/'};
}
export function checkBuilds(config, commit, docs, runner, wasm) {
  if (docs.schema !== 2 || runner.schema !== 2 || docs.dirty ||
      [docs, runner, wasm].some(build => build.commit !== commit) ||
      wasm.wasm_memory_limit_bytes !== 67108864 || docs.base !== config.base ||
      docs.runnerUrl !== config.runnerUrl || runner.isolation !== 'opaque-iframe-blob-worker')
    throw Error('Release build identity, base path or Wasm bounds mismatch');
}
