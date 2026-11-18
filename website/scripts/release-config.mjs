export function releaseConfig(env = process.env) {
  const local = env.RELEASE_ALLOW_LOCALHOST === '1';
  const docs = new URL(env.DOCS_ORIGIN);
  const runner = new URL(env.DOCS_RUNNER_URL);
  for (const url of [docs, runner]) {
    if (url.username || url.password || url.search || url.hash || url.pathname !== '/' ||
        (url.protocol !== 'https:' && !(local && url.protocol === 'http:' && ['localhost', '127.0.0.1', '[::1]'].includes(url.hostname)))) {
      throw Error('Release requires HTTPS origins with no credentials, path, query or fragment');
    }
  }
  if (env.DOCS_ORIGIN !== docs.origin || env.DOCS_RUNNER_URL !== runner.origin + '/' || docs.origin === runner.origin) {
    throw Error('Use a canonical DOCS_ORIGIN and separate root DOCS_RUNNER_URL ending in /');
  }
  const base = env.DOCS_BASE || '/';
  if (!/^\/(?:[A-Za-z0-9_-]+\/)*$/.test(base)) throw Error('DOCS_BASE must be / or slash-terminated path segments');
  return {docsOrigin: docs.origin, runnerUrl: runner.href, base, local};
}

export function checkBuilds(config, commit, docs, runner, wasm) {
  if (docs.schema !== 1 || runner.schema !== 1 || docs.dirty ||
      [docs, runner, wasm].some(build => build.commit !== commit) ||
      wasm.wasm_memory_limit_bytes !== 67108864 || docs.base !== config.base ||
      docs.runnerUrl !== config.runnerUrl ||
      JSON.stringify(runner.parentOrigins) !== JSON.stringify([config.docsOrigin])) {
    throw Error('Release build identity, origins, base path or Wasm bounds mismatch');
  }
}
