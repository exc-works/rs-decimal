"""Exercise the built documentation with ChromeDriver (no external Python deps)."""
import json
import os
from pathlib import Path
import subprocess
import sys
import socket
import time
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
def free_port():
    with socket.socket() as listener:
        listener.bind(('127.0.0.1', 0))
        return listener.getsockname()[1]

DRIVER_PORT, SITE_PORT = free_port(), free_port()
BASE = f'http://127.0.0.1:{DRIVER_PORT}'
SITE = f'http://127.0.0.1:{SITE_PORT}/rs-decimal/'

def call(path, payload=None, method=None):
    request = urllib.request.Request(BASE + path,
        data=None if payload is None else json.dumps(payload).encode(),
        headers={'Content-Type': 'application/json'}, method=method)
    try:
        with urllib.request.urlopen(request, timeout=20) as response:
            return json.load(response)['value']
    except urllib.error.HTTPError as error:
        raise RuntimeError(error.read().decode()) from error

def until(check):
    for _ in range(100):
        result = check()
        if result:
            return result
        time.sleep(.1)
    if 'session' in globals(): print(js('return {url:location.href, title:document.title, text:document.querySelector(".vp-doc")?.textContent};'))
    raise AssertionError('Browser condition timed out')

processes = []
try:
    processes.append(subprocess.Popen(['node', 'node_modules/vitepress/bin/vitepress.js', 'preview', 'docs', '--host', '127.0.0.1', '--port', str(SITE_PORT)], cwd=ROOT,
        env=dict(os.environ, DOCS_BASE='/rs-decimal/'), stdout=subprocess.DEVNULL))
    processes.append(subprocess.Popen([os.environ['CHROMEDRIVER'], f'--port={DRIVER_PORT}'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
    def ready():
        try:
            call('/status')
            with urllib.request.urlopen(SITE, timeout=1) as response:
                return response.status == 200
        except (OSError, RuntimeError):
            return False
    until(ready)
    session = call('/session', {'capabilities': {'alwaysMatch': {'browserName': 'chrome', 'pageLoadStrategy': 'eager', 'goog:chromeOptions': {
        'binary': os.environ['TEST_CHROME_BINARY'], 'args': ['--headless=new', '--window-size=1440,1000'] + (['--no-sandbox'] if sys.platform.startswith('linux') else [])}}}})['sessionId']
    def js(script):
        return call(f'/session/{session}/execute/sync', {'script': script, 'args': []})
    def navigate(path):
        call(f'/session/{session}/url', {'url': SITE + path})
    count = len(json.loads((ROOT / 'api/rust-api.json').read_text()))
    coverage = {entry['id']: entry for entry in json.loads((ROOT / 'api/coverage.json').read_text())}
    groups = json.loads((ROOT / 'translation-manifest.json').read_text()).get('method_locale_owners', {})
    for locale in ['en-US', 'en-GB', 'zh-Hans', 'zh-Hant', 'fr', 'ko', 'ja', 'de']:
        print('Checking', locale, flush=True)
        navigate(f'{locale}/api/')
        until(lambda: js('return document.querySelectorAll("tbody tr").length') == count)
        assert js('return document.documentElement.lang') == locale
        links = js('return [...document.querySelectorAll("tbody tr")].map(row=>[row.querySelector("code").textContent,row.querySelector("a")?.pathname])')
        for api_id, href in links:
            entry = coverage[api_id]
            available = locale in ['en-US','zh-Hans','en-GB'] or '*' in groups.get(locale,[]) or entry['owner'] in groups.get(locale,[])
            target = locale if available else 'en-US'
            assert href == f'/rs-decimal/{target}/api/{entry["doc_id"]}.html', (locale, api_id, href)
        js('const el=document.querySelector("#api-query"); el.value="mul_exact"; el.dispatchEvent(new Event("input",{bubbles:true}));')
        until(lambda: 0 < js('return document.querySelectorAll("tbody tr").length') < count)
        assert js('return [...document.querySelectorAll("tbody tr")].every(x=>x.textContent.includes("mul_exact"))')
    for locale in ['en-US', 'zh-Hans', 'en-GB', 'zh-Hant', 'fr']:
        navigate(f'{locale}/api/decimal-new-with-prec.html')
        until(lambda: js('return document.querySelectorAll(".method-example > pre").length') == 3)
        until(lambda: js('return !!document.querySelector(".method-form [data-parameter]")'))
        assert 'Decimal.new_with_prec(12n, 2)' in js('return document.querySelector(".method-example").textContent')
        js('document.querySelector(".VPNavBarTranslations button").click()')
        js('Array.from(document.querySelectorAll(".VPNavBarTranslations a")).find(x=>x.href.includes("/ja/")).click()')
        until(lambda: js('return document.documentElement.lang') == 'ja')
        until(lambda: js('return document.querySelector(".vp-doc")?.textContent.includes("準備中")'))
        assert js('return document.querySelector(".vp-doc a[href*=en-US]").pathname') == '/rs-decimal/en-US/api/decimal-new-with-prec.html'
    for locale in ['en-US', 'zh-Hans', 'en-GB', 'zh-Hant', 'fr']:
        navigate(f'{locale}/api/decimal-unmarshal-xml-attr.html')
        until(lambda: js('return document.querySelector(".vp-doc")?.textContent.includes("&#49;.23")'))
    navigate('en-US/api/decimal-impl-clone.html')
    until(lambda: js('return document.querySelectorAll(".method-example > pre").length') == 2)
    assert 'native test harness' in js('return document.querySelector(".method-example").textContent')
    navigate('en-US/api/nulldecimal-valid.html')
    until(lambda: js('return document.querySelectorAll(".method-example > pre").length') == 3)
    assert '"valid": false' in js('return document.querySelector(".method-example").textContent')
    navigate('zh-Hans/guide.html')
    until(lambda: js('return !!document.querySelector(".VPNavBarTranslations button")'))
    js('document.querySelector(".VPNavBarTranslations button").click()')
    links = js('return [...document.querySelectorAll(".VPNavBarTranslations a")].map(x=>x.getAttribute("href"))')
    assert len(links) == 7 and '/rs-decimal/ja/guide.html' in links, links
    js('Array.from(document.querySelectorAll(".VPNavBarTranslations a")).find(x=>x.href.includes("/ja/")).click()')
    until(lambda: js('return document.documentElement.lang') == 'ja')
    assert js('return location.pathname') == '/rs-decimal/ja/guide.html'
    js('document.querySelector(".DocSearch-Button").click()')
    until(lambda: js('return !!document.querySelector("#localsearch-input")'))
    js('const el=document.querySelector("#localsearch-input"); el.value="HalfEven"; el.dispatchEvent(new Event("input",{bubbles:true}));')
    results = until(lambda: js('return [...document.querySelectorAll(".VPLocalSearchBox a")].map(x=>x.getAttribute("href"))'))
    assert all('/ja/' in link for link in results), results
    # CDP device emulation avoids desktop Chrome's minimum window width.
    call(f'/session/{session}/goog/cdp/execute', {'cmd': 'Emulation.setDeviceMetricsOverride', 'params': {'width':390,'height':844,'deviceScaleFactor':1,'mobile':True}})
    navigate('zh-Hans/guide.html')
    until(lambda: js('return !!document.querySelector("h1")'))
    assert js('return document.documentElement.scrollWidth <= innerWidth')
    print('Eight locale routes, API filtering, language switching, locale search and mobile layout passed.')
finally:
    if 'session' in globals():
        try:
            call(f'/session/{session}', method='DELETE')
        except (OSError, RuntimeError):
            pass
    for process in reversed(processes):
        process.terminate()
        process.wait(timeout=10)
