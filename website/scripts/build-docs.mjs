import { execFileSync } from 'node:child_process';
import { writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('../', import.meta.url));
const git = (...args) => execFileSync('git', args, {cwd: root, encoding: 'utf8'}).trim();
const commit = git('rev-parse', 'HEAD');
const dirty = !!git('status', '--porcelain');
execFileSync(process.execPath, ['node_modules/vitepress/bin/vitepress.js', 'build', 'docs'], {cwd: root, stdio: 'inherit'});
if (git('rev-parse', 'HEAD') !== commit) throw Error('Git HEAD changed during documentation build');
await writeFile(new URL('../docs/.vitepress/dist/build.json', import.meta.url), JSON.stringify({
  schema: 2, commit, dirty: dirty || !!git('status', '--porcelain'),
  base: process.env.DOCS_BASE || '/', runnerUrl: (process.env.DOCS_BASE || '/') + 'runner/',
}, null, 2) + '\n');
