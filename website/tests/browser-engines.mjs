// Real engines, shipped response headers, shared Rust fixtures and production UI.
import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { readFile, stat, mkdir, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { resolve, extname, sep } from 'node:path';
import { chromium, firefox, webkit } from 'playwright';

const root = resolve(fileURLToPath(new URL('../', import.meta.url)));
const manifest = JSON.parse(await readFile(resolve(root, 'public/runner/manifest.json'), 'utf8'));
const probes = [];
const servers = [];
const report = [];
const mime = {'.html':'text/html', '.js':'text/javascript', '.mjs':'text/javascript', '.json':'application/json', '.css':'text/css', '.wasm':'application/wasm', '.svg':'image/svg+xml', '.woff2':'font/woff2'};
async function serve(port) {
  const server = createServer(async (req, res) => {
    try {
      const path = decodeURIComponent(new URL(req.url, 'http://localhost').pathname);
      if (path.endsWith('/probe.js')) probes.push(path);
      if (path === '/insecure-runner/sandbox.json') {
        const bundle = JSON.parse(await readFile(resolve(root, 'public/runner/sandbox.json'), 'utf8'));
        bundle.html = bundle.html.replace(/<meta http-equiv="Content-Security-Policy"[^>]*>/, '');
        res.writeHead(200, {'Content-Type': 'application/json'}); res.end(JSON.stringify(bundle)); return;
      }
      let directory = root;
      let relative = path;
      if (port === 4210 && path.startsWith('/rs-decimal/')) {
        directory = resolve(root, 'docs/.vitepress/dist'); relative = path.slice('/rs-decimal'.length);
      } else if (port === 4210 && path === '/runner-manifest.json') relative = '/public/runner/manifest.json';
      if (path === '/insecure-runner/probe.js') relative = '/public/runner/probe.js';
      let file = resolve(directory, '.' + relative);
      if (file !== directory && !file.startsWith(directory + sep)) throw Error('Invalid path');
      if ((await stat(file)).isDirectory()) file = resolve(file, 'index.html');
      const data = await readFile(file);
      res.writeHead(200, {'Content-Type': mime[extname(file)] || 'application/octet-stream'});
      res.end(data);
    } catch { res.writeHead(404); res.end('Not found'); }
  });
  await new Promise((done, reject) => { server.once('error', reject); server.listen(port, '127.0.0.1', done); });
  servers.push(server);
}
const origin = 'http://127.0.0.1:4210';
try {
  for (const port of [4210]) await serve(port);
  for (const [name, engine] of Object.entries({chromium, firefox, webkit})) {
    console.log(`${name}: launching browser`);
    const browser = await engine.launch(name === 'chromium' ? {chromiumSandbox: process.platform !== 'linux'} : {});
    const context = await browser.newContext({viewport: {width: 1440, height: 1000}});
    const page = await context.newPage();
    page.setDefaultTimeout(30000);
    const failures = [];
    page.on('pageerror', error => { failures.push(error.message); console.error(`${name}: ${error.message}`); });
    try {
      const firstProbe = probes.length;
      const results = {};
      for (const harness of ['runner', 'examples']) {
        console.log(`${name}: opening ${harness} harness`);
        const response = await page.goto(`${origin}/tests/${harness}-harness.html`);
        assert.equal(response.status(), 200, `${harness} harness route`);
        await page.waitForFunction(() => window.testResult, null, {timeout: 120000});
        const result = await page.evaluate(() => window.testResult);
        assert.equal(result.ok, true, JSON.stringify(result));
        results[harness] = result.checks.length;
        console.log(`${name}: ${harness} ${result.checks.length} checks passed`);
      }
      assert(!probes.slice(firstProbe).includes('/public/runner/probe.js'), `${name}: protected runner received network probe`);
      assert(probes.slice(firstProbe).includes('/insecure-runner/probe.js'), `${name}: missing-header control was not exercised`);
      for (const locale of ['en-US','en-GB','zh-Hans','zh-Hant','fr','ko','ja','de']) {
        const response = await page.goto(`${origin}/rs-decimal/${locale}/api/decimal-new.html`);
        assert.equal(response.status(), 200);
        await page.locator('[data-parameter="0"]').waitFor();
        assert.equal(await page.locator('html').getAttribute('lang'), locale);
        assert.equal(await page.locator('.VPNavBar .playground-nav').getAttribute('href'), `/rs-decimal/${locale}/playground.html`);
        assert.equal(await page.locator('#VPSidebarNav a[href$="/playground.html"]').count(), 0);
      }
      await page.goto(`${origin}/rs-decimal/zh-Hans/`);
      await page.locator('.ref-tasks').getByRole('link', {name:'舍入并保留两位小数'}).click();
      await page.locator('h2#返回值').waitFor();
      await page.goto(`${origin}/rs-decimal/zh-Hans/api/`);
      await page.locator('.ref-types').getByRole('link', {name:'Decimal', exact:true}).click();
      await page.locator('.ref-jump a[href="#members-precision"]').click();
      await page.locator('#members-precision').getByRole('link', {name:'Decimal::round_with_prec', exact:true}).click();
      await page.locator('h2#返回值').waitFor();
      assert.ok(await page.locator('[data-example-code] .hljs-keyword').count(), 'highlight Rust');
      await page.getByRole('button', {name:'JavaScript', exact:true}).focus();
      await page.keyboard.press('Enter');
      assert.match(await page.locator('[data-example-code]').textContent(), /input\.round_with_prec/);
      assert.ok(await page.locator('[data-example-code] .hljs-keyword').count(), 'highlight JavaScript');
      const methodUrl = page.url();
      const places = page.locator('[data-parameter="1"]');
      await places.fill('1');
      await page.waitForFunction(() => document.querySelector('[data-example-output]')?.textContent === '-1.2');
      assert.equal(page.url(), methodUrl);
      await places.fill('-1');
      assert.equal(await page.locator('[data-example-output]').count(), 0, 'discard stale results on edit');
      await page.locator('.method-form [role=alert]').filter({hasText:'InvalidPrecision'}).waitFor();
      await places.fill('2');
      await page.waitForFunction(() => document.querySelector('[data-example-output]')?.textContent === '-1.24');
      await page.locator('[data-action=open-example]').click();
      await page.locator('[data-action=run]:enabled').click();
      await page.waitForFunction(() => document.querySelector('.pg-output')?.textContent.includes('-1.24'));

      await page.goto(`${origin}/rs-decimal/en-US/api/nulldecimal-valid.html`);
      await page.waitForFunction(() => document.querySelector('[data-example-output]')?.textContent.includes('"valid": false'));

      await page.goto(`${origin}/rs-decimal/zh-Hans/api/decimal-rescale.html`);
      await page.locator('.vp-doc p a[href$="/roundingmode.html"]').first().waitFor();
      assert.ok(await page.locator('.vp-doc p a[href$="/errorkind-invalidprecision.html"]').count());
      const mode = page.getByLabel('舍入模式');
      assert.equal(await mode.locator('option').count(), 7);
      await mode.selectOption('Down');
      await page.waitForFunction(() => document.querySelector('[data-example-output]')?.textContent === '-1.23');
      assert.equal(await page.locator('[data-example-code] mark').textContent(), 'RoundingMode::Down');
      await page.getByRole('button', {name:'JavaScript', exact:true}).click();
      assert.equal(await page.locator('[data-example-code] mark').textContent(), 'RoundingMode.Down');
      await mode.selectOption('Unnecessary');
      await page.locator('.method-form [role=alert]').filter({hasText:'RoundUnnecessary'}).waitFor();
      await mode.selectOption('HalfUp');
      await page.waitForFunction(() => document.querySelector('[data-example-output]')?.textContent === '-1.24');
      assert.equal(await page.locator('[data-example-code] mark').count(), 0);

      await page.goto(`${origin}/rs-decimal/en-US/api/decimal-new.html`);
      await page.locator('[data-parameter="0"]').fill('9007199254740993');
      await page.locator('[data-action=open-example]').click();
      const ready = () => page.waitForFunction(() => document.querySelector('.playground')?.dataset.editorReady === 'true');
      const output = page.locator('.pg-output');
      await ready();
      assert.equal((await output.textContent()).trim(), '', 'handoff must not run');
      await page.locator('[data-action=run]').click();
      await page.waitForFunction(() => document.querySelector('.pg-output')?.textContent.includes('9007199254740993'));
      await page.waitForFunction(() => document.querySelector('[data-action=stop]').disabled);
      const runnerFrame = page.locator('.playground iframe');
      assert.equal(await runnerFrame.count(), 1, 'execution frame is mounted');
      for (const dark of [false, true]) {
        await page.evaluate(value => document.documentElement.classList.toggle('dark', value), dark);
        assert.equal(await runnerFrame.isVisible(), false, 'execution frame must not appear in either theme');
        assert.equal(await runnerFrame.evaluate(frame => frame.getClientRects().length), 0, 'execution frame must not occupy layout space');
      }
      await page.locator('[data-action=share]').click();
      const link = await page.locator('[data-share-link]').inputValue();
      assert(link.includes('#share='));
      await page.goto(link);
      await ready();
      assert.equal((await output.textContent()).trim(), '', 'share must not run');
      assert.equal(new URL(page.url()).hash, '');
      await page.locator('[data-action=run]').click();
      await page.waitForFunction(() => document.querySelector('.pg-output')?.textContent.includes('9007199254740993'));
      await page.waitForFunction(() => document.querySelector('[data-action=stop]').disabled);
      const source = 'show("<img src=x onerror=alert(1)>");';
      await page.locator('input[type=file]').setInputFiles({name:'input.js',mimeType:'text/javascript',buffer:Buffer.from(source)});
      await page.waitForFunction(() => document.querySelector('.pg-status')?.textContent.includes('Ready'));
      assert.equal((await output.textContent()).trim(), '');
      await page.locator('[data-action=run]').click();
      await page.waitForFunction(() => document.querySelector('.pg-output')?.textContent.includes('<img'));
      assert.equal(await page.locator('.pg-output img').count(), 0);
      await page.waitForFunction(() => document.querySelector('[data-action=stop]').disabled);
      const downloadEvent = page.waitForEvent('download');
      await page.locator('[data-action=export]').click();
      const download = await downloadEvent;
      assert.equal(await readFile(await download.path(), 'utf8'), source);
      await page.reload(); await ready();
      assert((await output.textContent()).includes('<img'), 'restore saved output without running');
      await page.locator('.monaco-editor .view-line').first().click();
      assert(await page.locator('textarea.inputarea').evaluate(input => document.activeElement === input),
        'keyboard extensions should see an editable text control');
      // WebKit can present as macOS on a Linux runner. Match Monaco's browser platform.
      const modifier = await page.evaluate(() => navigator.userAgent.includes('Macintosh') ? 'Meta' : 'Control');
      await page.keyboard.press(modifier + '+a');
      await page.keyboard.type('console.log("hjkl");');
      await page.keyboard.press(modifier + '+Enter');
      await page.waitForFunction(() => document.querySelector('.pg-output')?.textContent.trim() === 'hjkl');
      await page.setViewportSize({width:390,height:844});
      await page.waitForFunction(() => document.documentElement.scrollWidth <= innerWidth);
      assert.deepEqual(failures, [], `${name}: uncaught page errors`);
      report.push({commit: manifest.commit, engine: name, version: browser.version(), ...results, locales: 8,
        editor: ['exact bigint form','no auto-run','share','import/export','escaped output','draft restoration','mobile']});
      console.log(`${name}: eight locales and production editor passed`);
    } finally { await context.close(); await browser.close(); }
  }
  await mkdir(resolve(root, '../.cache'), {recursive: true});
  await writeFile(resolve(root, '../.cache/browser-engines.json'), JSON.stringify(report, null, 2) + '\n');
} finally {
  for (const server of servers) {
    server.closeAllConnections();
    await new Promise(done => server.close(done));
  }
}
