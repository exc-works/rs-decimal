import { execFileSync } from 'node:child_process';
import { cp, mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { checkBuilds, releaseConfig } from './release-config.mjs';

const root = fileURLToPath(new URL('../', import.meta.url));
const config = releaseConfig();
const git = (...args) => execFileSync('git', args, {cwd: root, encoding: 'utf8'}).trim();
const clean = () => { if (git('status', '--porcelain')) throw Error('Commit source changes before building a release'); };
clean();
const commit = git('rev-parse', 'HEAD');
const env = {...process.env, DOCS_BASE: config.base};
const run = (command, args) => execFileSync(command, args, {cwd: root, env, stdio: 'inherit'});
run('python3', ['../tools/check-web-api/check.py', '--release']);
run(process.execPath, ['scripts/check-i18n.mjs', '--release']);
run('python3', ['../tools/build-web-wasm.py']);
run(process.execPath, ['scripts/build-runner.mjs']);
run(process.execPath, ['scripts/build-docs.mjs']);
const json = async path => JSON.parse(await readFile(new URL('../' + path, import.meta.url), 'utf8'));
const docs = await json('docs/.vitepress/dist/build.json');
const runner = await json('public/runner/manifest.json');
const wasm = await json('public/wasm/build.json');
checkBuilds(config, commit, docs, runner, wasm);
clean();
if (git('rev-parse', 'HEAD') !== commit) throw Error('Git HEAD changed during release build');
const out = new URL('../release-dist/', import.meta.url);
await rm(out, {recursive: true, force: true});
await mkdir(out);
await cp(new URL('../docs/.vitepress/dist/', import.meta.url), new URL('docs/', out), {recursive: true});
await cp(new URL('../DEPLOYMENT.md', import.meta.url), new URL('DEPLOYMENT.md', out));
const files = {};
async function inventory(dir, prefix = '') {
  for (const entry of (await readdir(dir, {withFileTypes: true})).sort((a,b) => a.name.localeCompare(b.name))) {
    const path = prefix + entry.name;
    if (entry.isDirectory()) await inventory(new URL(entry.name + '/', dir), path + '/');
    else {
      const bytes = await readFile(new URL(entry.name, dir));
      files[path] = {bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex')};
    }
  }
}
await inventory(out);
await writeFile(new URL('release.json', out), JSON.stringify({schema: 2, commit, ...config,
  locales: ['zh-Hans','zh-Hant','en-US','en-GB','fr','ko','ja','de'], apiEntries: 355,
  wasm, files,
}, null, 2) + '\n');
console.log(`Packaged ${Object.keys(files).length} static files at ${commit}; base=${config.base}. No deployment performed.`);
