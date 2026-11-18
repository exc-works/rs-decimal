"""Exercise the built documentation with ChromeDriver (no external Python deps)."""
import json
import os
from pathlib import Path
import subprocess
import time
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
BASE = 'http://127.0.0.1:9518'
SITE = 'http://127.0.0.1:4175/rs-decimal/'

def call(path, payload=None):
    request = urllib.request.Request(BASE + path,
        data=None if payload is None else json.dumps(payload).encode(),
        headers={'Content-Type': 'application/json'})
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
    raise AssertionError('Browser condition timed out')

processes = []
try:
    processes.append(subprocess.Popen(['npm', 'run', 'preview', '--', '--host', '127.0.0.1', '--port', '4175'], cwd=ROOT,
        env=dict(os.environ, DOCS_BASE='/rs-decimal/'), stdout=subprocess.DEVNULL))
    processes.append(subprocess.Popen([os.environ['CHROMEDRIVER'], '--port=9518'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
    def ready():
        try:
            call('/status')
            with urllib.request.urlopen(SITE, timeout=1) as response:
                return response.status == 200
        except (OSError, RuntimeError):
            return False
    until(ready)
    session = call('/session', {'capabilities': {'alwaysMatch': {'browserName': 'chrome', 'goog:chromeOptions': {
        'binary': os.environ['TEST_CHROME_BINARY'], 'args': ['--headless=new', '--no-sandbox', '--window-size=1440,1000']}}}})['sessionId']
    def js(script):
        return call(f'/session/{session}/execute/sync', {'script': script, 'args': []})
    def navigate(path):
        call(f'/session/{session}/url', {'url': SITE + path})
    count = len(json.loads((ROOT / 'api/rust-api.json').read_text()))
    for locale in ['en-US', 'en-GB', 'zh-Hans', 'zh-Hant', 'fr', 'ko', 'ja', 'de']:
        navigate(f'{locale}/api/')
        until(lambda: js('return document.querySelectorAll("tbody tr").length') == count)
        assert js('return document.documentElement.lang') == locale
        js('const el=document.querySelector("#api-query"); el.value="mul_exact"; el.dispatchEvent(new Event("input",{bubbles:true}));')
        until(lambda: 0 < js('return document.querySelectorAll("tbody tr").length') < count)
        assert js('return [...document.querySelectorAll("tbody tr")].every(x=>x.textContent.includes("mul_exact"))')
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
    for process in reversed(processes):
        process.terminate()
        process.wait(timeout=10)
