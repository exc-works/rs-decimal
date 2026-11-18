"""Serve static assets with their real CSP and exercise two-origin execution."""
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import subprocess
import sys
import threading
import time
import urllib.request
import urllib.error

ROOT=Path(__file__).resolve().parents[1]
HEADERS=json.loads((ROOT/'runner-dist/headers.json').read_text())
PROBES=[]
class Handler(SimpleHTTPRequestHandler):
    def log_message(self, *args): pass
    def do_GET(self):
        if self.path=='/runner-manifest.json':
            data=(ROOT/'runner-dist/manifest.json').read_bytes()
            self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(data);return
        if self.path.startswith('/probe'):PROBES.append(self.server.server_port)
        super().do_GET()
    def end_headers(self):
        if self.server.server_port==4211:
            values={**HEADERS['/*'],**HEADERS.get(self.path.split('?')[0],{})}
            for key,value in values.items():self.send_header(key,value)
        super().end_headers()
servers=[]
driver=None
session=None
base='http://127.0.0.1:9519'
def call(path,payload=None,method=None):
    req=urllib.request.Request(base+path,data=None if payload is None else json.dumps(payload).encode(),headers={'Content-Type':'application/json'},method=method)
    try:
        with urllib.request.urlopen(req,timeout=20) as r:return json.load(r)['value']
    except urllib.error.HTTPError as error:
        raise RuntimeError(error.read().decode()) from error
try:
    for port,directory in [(4210,ROOT),(4211,ROOT/'runner-dist'),(4212,ROOT/'runner-dist')]:
        server=ThreadingHTTPServer(('127.0.0.1',port),partial(Handler,directory=str(directory)));servers.append(server);threading.Thread(target=server.serve_forever,daemon=True).start()
    driver=subprocess.Popen([os.environ['CHROMEDRIVER'],'--port=9519'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
    for _ in range(100):
        try:call('/status');break
        except OSError:time.sleep(.1)
    session=call('/session',{'capabilities':{'alwaysMatch':{'browserName':'chrome','pageLoadStrategy':'eager','goog:chromeOptions':{'binary':os.environ['TEST_CHROME_BINARY'],'args':['--headless=new'] + (['--no-sandbox'] if sys.platform.startswith('linux') else [])}}}})['sessionId']
    call(f'/session/{session}/url',{'url':'http://127.0.0.1:4210/tests/runner-harness.html'})
    for _ in range(200):
        result=call(f'/session/{session}/execute/sync',{'script':'return window.testResult || null','args':[]})
        if result:break
        time.sleep(.1)
    print(json.dumps(result,indent=2))
    assert result and result['ok'],result
    assert 4211 not in PROBES,PROBES
    assert 4212 in PROBES,'missing-CSP control did not reach the static server'
    print('Protected origin received no probe requests; missing-CSP control was rejected.')
finally:
    if session:
        try:call(f'/session/{session}',method='DELETE')
        except OSError:pass
    if driver:driver.terminate();driver.wait(timeout=10)
    for server in servers:server.shutdown();server.server_close()
