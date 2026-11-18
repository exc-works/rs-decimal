import { build } from 'esbuild';
import { readFile, mkdir, writeFile, rm } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
const root = fileURLToPath(new URL('../', import.meta.url));
const commit = execFileSync('git', ['rev-parse', 'HEAD'], {cwd: root, encoding: 'utf8'}).trim();
const wasm = JSON.parse(await readFile(new URL('../public/wasm/build.json', import.meta.url), 'utf8'));
if (wasm.commit !== commit || wasm.wasm_memory_limit_bytes !== 67108864)
  throw Error('Rebuild bounded Wasm at the current commit before packaging runner');
const config = {commit};
const bundle = async (entry, define = {}) => (await build({
  absWorkingDir: root, entryPoints: [entry], bundle: true, write: false,
  format: 'iife', platform: 'browser', target: 'es2022', loader: {'.wasm': 'binary'},
  define: {'import.meta.url': JSON.stringify('about:blank'), __RUNNER_CONFIG__: JSON.stringify(config), ...define},
})).outputFiles[0].text;
const worker = await bundle('src/runner/worker.js');
const frame = await bundle('src/runner/frame.js', {__WORKER_SOURCE__: JSON.stringify(worker)});
// Only trusted build output enters srcdoc. User source is sent through a private
// MessagePort after bootstrap; it never becomes HTML or a script element.
const policy = "default-src 'none'; script-src 'unsafe-inline' 'unsafe-eval'; worker-src blob:; connect-src 'none'; base-uri 'none'; form-action 'none'; object-src 'none'";
const html = `<!doctype html><meta charset="utf-8"><meta http-equiv="Content-Security-Policy" content="${policy}"><script>${frame.replace(/<\/script/gi, '<\\/script')}</script>`;
const out = new URL('../public/runner/', import.meta.url);
await rm(out, {recursive: true, force: true}); await mkdir(out, {recursive: true});
await writeFile(new URL('sandbox.json', out), JSON.stringify({schema: 2, commit, html}));
await writeFile(new URL('manifest.json', out), JSON.stringify({schema: 2, commit, isolation: 'opaque-iframe-blob-worker'}, null, 2)+'\n');
await writeFile(new URL('probe.js', out), 'export const probe = true;\n');
console.log(`Built single-site sandbox at ${commit}; no response headers or separate origin required.`);
