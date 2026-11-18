"""Exercise the shipped editor UI, including real file import and completion."""
import base64
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import threading
import time
import urllib.error
import urllib.request

ROOT=Path(__file__).resolve().parents[1]
HEADERS=json.loads((ROOT/'runner-dist/headers.json').read_text())
class Handler(SimpleHTTPRequestHandler):
    def log_message(self,*args):pass
    def do_GET(self):
        try:super().do_GET()
        except (BrokenPipeError,ConnectionResetError):pass
    def translate_path(self,path):
        if self.server.server_port==4210 and path.startswith('/rs-decimal/'):
            path=path[len('/rs-decimal'):]
        return super().translate_path(path)
    def end_headers(self):
        if self.server.server_port==4211:
            for k,v in {**HEADERS['/*'],**HEADERS.get(self.path.split('?')[0],{})}.items():self.send_header(k,v)
        super().end_headers()
servers=[];driver=None;session=None
base='http://127.0.0.1:9520'
def call(path,payload=None,method=None):
    req=urllib.request.Request(base+path,data=None if payload is None else json.dumps(payload).encode(),headers={'Content-Type':'application/json'},method=method)
    try:
        with urllib.request.urlopen(req,timeout=20) as r:return json.load(r)['value']
    except urllib.error.HTTPError as error:raise RuntimeError(error.read().decode()) from error

def until(check):
    for _ in range(200):
        value=check()
        if value:return value
        time.sleep(.1)
    print(js('return {suggest:document.querySelector(".suggest-widget")?.textContent,code:document.querySelector(".view-lines")?.textContent,focus:document.activeElement?.outerHTML};'))
    print(call(f'/session/{session}/log',{'type':'browser'}))
    (ROOT.parent/'.cache/editor-failure.png').write_bytes(base64.b64decode(call(f'/session/{session}/screenshot')))
    raise AssertionError('UI condition timed out')

with tempfile.TemporaryDirectory(prefix='decimal-editor-') as temp:
    try:
        for port,directory in [(4210,ROOT/'docs/.vitepress/dist'),(4211,ROOT/'runner-dist')]:
            server=ThreadingHTTPServer(('127.0.0.1',port),partial(Handler,directory=str(directory)));servers.append(server);threading.Thread(target=server.serve_forever,daemon=True).start()
        driver=subprocess.Popen([os.environ['CHROMEDRIVER'],'--port=9520'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        for _ in range(100):
            try:call('/status');break
            except OSError:time.sleep(.1)
        session=call('/session',{'capabilities':{'alwaysMatch':{'browserName':'chrome','goog:loggingPrefs':{'browser':'ALL'},'pageLoadStrategy':'eager','goog:chromeOptions':{'binary':os.environ['TEST_CHROME_BINARY'],'args':['--headless=new','--window-size=1440,1100']+(['--no-sandbox'] if sys.platform.startswith('linux') else []),'prefs':{'download.default_directory':temp}}}}})['sessionId']
        def js(script):return call(f'/session/{session}/execute/sync',{'script':script,'args':[]})
        def navigate(locale):call(f'/session/{session}/url',{'url':f'http://127.0.0.1:4210/rs-decimal/{locale}/playground.html'})
        def ready():return js('return document.querySelector(".playground")?.dataset.editorReady === "true"')
        def click(action):js(f'document.querySelector("[data-action={action}]").click()')
        def output():return js('return document.querySelector(".pg-output").textContent')
        def import_source(source):
            path=Path(temp)/'input.js';path.write_text(source)
            el=call(f'/session/{session}/element',{'using':'css selector','value':'input[type=file]'})['element-6066-11e4-a52e-4f735466cecf']
            call(f'/session/{session}/element/{el}/value',{'text':str(path)})
            until(lambda:js('return document.querySelector(".pg-status").textContent.includes("Ready")'))
        def finished():return js('return document.querySelector("[data-action=stop]").disabled')
        navigate('en-US');until(ready)
        assert not output().strip(),'no auto-run'
        click('run');until(lambda:'37050' in output());until(finished)
        assert '"scale": "3"' in output() and '"string": "37"' in output(),output()
        print('Exact snapshots and explicit execution passed',flush=True)
        import_source('show("<img src=x onerror=alert(1)>");')
        assert not output().strip(),'import must not run'
        click('run');until(lambda:'<img' in output());until(finished)
        assert js('return document.querySelectorAll(".pg-output img").length')==0
        import_source('show(Decimal.new(1n)); Decimal.new(1n).rescale(-1, RoundingMode.HalfEven);')
        click('run');until(lambda:js('return document.querySelector(".pg-error")?.textContent.includes("InvalidPrecision")'))
        assert '"coefficient": "1"' in output()
        print('Text-only output and partial output on library errors passed',flush=True)
        import_source('while(true){}');click('run');time.sleep(.1);click('stop');until(finished)
        assert 'Stopped' in js('return document.querySelector(".pg-status").textContent')
        click('reset');click('run');until(lambda:'37050' in output());until(finished)
        print('Stop, reset and recovery passed',flush=True)
        # Completion lists are virtualized; filter before asserting a row is rendered.
        import_source('Decimal.new_from_s')
        js('(document.querySelector(".monaco-editor .native-edit-context")||document.querySelector(".monaco-editor textarea.inputarea")).focus()')
        keys=[{'type':'keyDown','value':'\ue010'},{'type':'keyUp','value':'\ue010'},{'type':'keyDown','value':'\ue009'},{'type':'keyDown','value':' '},{'type':'keyUp','value':' '},{'type':'keyUp','value':'\ue009'}]
        call(f'/session/{session}/actions',{'actions':[{'type':'key','id':'keyboard','actions':keys}]})
        until(lambda:js('return document.querySelector(".suggest-widget")?.textContent.includes("new_from_string")'))
        print('Rust-named completion from bridge declarations passed',flush=True)
        import_source('betw')
        js('(document.querySelector(".monaco-editor .native-edit-context")||document.querySelector(".monaco-editor textarea.inputarea")).focus()')
        call(f'/session/{session}/actions',{'actions':[{'type':'key','id':'keyboard','actions':keys}]})
        until(lambda:js('return document.querySelector(".suggest-widget")?.textContent.includes("between")'))
        print('Free helper completion passed',flush=True)
        import_source('most_signif')
        js('(document.querySelector(".monaco-editor .native-edit-context")||document.querySelector(".monaco-editor textarea.inputarea")).focus()')
        call(f'/session/{session}/actions',{'actions':[{'type':'key','id':'keyboard','actions':keys}]})
        until(lambda:js('return document.querySelector(".suggest-widget")?.textContent.includes("most_significant_bit")'))
        print('Conversion helper completion passed',flush=True)
        for prefix, expected in [('ScanValue.Int6','Int64'),('ErrorKind.Unm','Unmarshal'),('DecimalError.ne','new')]:
            import_source(prefix)
            js('(document.querySelector(".monaco-editor .native-edit-context")||document.querySelector(".monaco-editor textarea.inputarea")).focus()')
            call(f'/session/{session}/actions',{'actions':[{'type':'key','id':'keyboard','actions':keys}]})
            until(lambda:js('return document.querySelector(".suggest-widget")?.textContent.includes('+json.dumps(expected)+')'))
        print('SQL and error type completion passed',flush=True)
        import_source('Decimal.new_from_string(')
        js('(document.querySelector(".monaco-editor .native-edit-context")||document.querySelector(".monaco-editor textarea.inputarea")).focus()')
        modifier='\ue03d' if sys.platform=='darwin' else '\ue009'
        call(f'/session/{session}/actions',{'actions':[{'type':'key','id':'keyboard','actions':[{'type':'keyDown','value':modifier},{'type':'keyDown','value':'\ue008'},{'type':'keyDown','value':' '},{'type':'keyUp','value':' '},{'type':'keyUp','value':'\ue008'},{'type':'keyUp','value':modifier}]}]})
        until(lambda:js('return document.querySelector(".parameter-hints-widget")?.textContent.includes("text: string")'))
        print('Parameter help passed',flush=True)
        import_source('assert(false, "checked assertion");');click('run')
        until(lambda:js('return document.querySelector(".pg-error")?.textContent.includes("checked assertion")'))

        import_source('show(Decimal.new_from_string("9.00"));')
        click('run');until(lambda:'900' in output());until(finished)
        # Hold only the draft debounce pending, then perform real editor input and refresh.
        # This deterministically covers navigation before the 200 ms save fires.
        js('const original = window.setTimeout; window.setTimeout = (fn, delay, ...args) => original(fn, delay === 200 ? 60000 : delay, ...args); (document.querySelector(".monaco-editor .native-edit-context")||document.querySelector(".monaco-editor textarea.inputarea")).focus()')
        suffix=' 123'
        call(f'/session/{session}/actions',{'actions':[{'type':'key','id':'keyboard','actions':[action for char in suffix for action in [{'type':'keyDown','value':char},{'type':'keyUp','value':char}]]}]})
        call(f'/session/{session}/refresh',{})
        until(ready)
        click('export');until(lambda:(Path(temp)/'decimal-playground.js').exists())
        assert (Path(temp)/'decimal-playground.js').read_text()=='show(Decimal.new_from_string("9.00"));'+suffix,repr((Path(temp)/'decimal-playground.js').read_text())
        print('Pending draft survives immediate browser refresh',flush=True)
        js('document.querySelector(".VPNavBarTranslations button").click();Array.from(document.querySelectorAll(".VPNavBarTranslations a")).find(x=>x.href.includes("/ja/")).click()')
        until(lambda:js('return document.documentElement.lang === "ja"') and ready())
        assert '900' in output(),'locale change preserves result'
        assert js('return document.querySelector("[data-action=run]").textContent')=='実行'
        call(f'/session/{session}/refresh',{})
        until(ready)
        assert '900' in output(),'reload restores output without execution'
        click('run');until(lambda:'900' in output());until(finished)
        print('File export, locale switch and draft/result restoration passed',flush=True)
        screenshots=ROOT.parent/'.cache';screenshots.mkdir(exist_ok=True)
        (screenshots/'editor-desktop.png').write_bytes(base64.b64decode(call(f'/session/{session}/screenshot')))
        call(f'/session/{session}/goog/cdp/execute',{'cmd':'Emulation.setDeviceMetricsOverride','params':{'width':390,'height':844,'deviceScaleFactor':1,'mobile':True}})
        time.sleep(.3)
        assert js('return document.documentElement.scrollWidth<=innerWidth')
        (screenshots/'editor-mobile.png').write_bytes(base64.b64decode(call(f'/session/{session}/screenshot')))
        print('Editor browser UI checks passed.')
    finally:
        if session:
            try:call(f'/session/{session}',method='DELETE')
            except (OSError,RuntimeError):pass
        if driver:driver.terminate();driver.wait(timeout=10)
        for server in servers:server.shutdown();server.server_close()
