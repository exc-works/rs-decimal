"""Cross-decode both encoders; check canonical bytes and independent expected state."""
import argparse
import importlib.util
import json
from pathlib import Path
spec=importlib.util.spec_from_file_location("driver",Path(__file__).with_name("run.py"))
driver=importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)
parser=argparse.ArgumentParser()
parser.add_argument("--go",required=True)
parser.add_argument("--rust",required=True)
parser.add_argument("--native-bits",type=int,choices=[32,64],default=64)
args=parser.parse_args()
cases=[json.loads(l) for l in Path("testdata/conformance/bson.jsonl").read_text().splitlines()]
cases=[c for c in cases if c["operation"]=="encode_bson" and c.pop("expected_outcome","ok")=="ok"]
for c in cases:
    if c["inputs"][0]["scale"]==str(2**63-1):c["inputs"][0]["scale"]=str(2**(args.native_bits-1)-1)
left=driver.run([args.go],cases)
right=driver.run([args.rust],cases)
driver.compare(cases,left,right)
def requests(results):
    return [{"schema_version":1,"case_id":r["case_id"],"operation":"decode","inputs":[{"coefficient":"9870","scale":"2","initialized":True}],"params":{"nullable":"false","valid":"false","format":"bson","type":r["result"]["type"],"text":r["result"]["data"]}} for r in results]
# Go-produced bytes go to Rust; Rust-produced bytes go to Go.
from_go=requests(left)
from_rust=requests(right)
rust_decoded=driver.run([args.rust],from_go)
go_decoded=driver.run([args.go],from_rust)
driver.compare(from_go,go_decoded,rust_decoded)
for case,result in zip(cases,rust_decoded):
    source=case["inputs"][0]
    coefficient,scale=int(source["coefficient"]),int(source["scale"])
    nullable=case["params"]["nullable"]=="true"
    valid=case["params"]["valid"]=="true"
    initialized = valid if nullable else source["initialized"]
    if not initialized: coefficient,scale=0,0
    elif coefficient==0: scale=0
    else:
        while scale and coefficient%10==0:coefficient//=10;scale-=1
    expected={"coefficient":str(coefficient),"scale":str(scale),"initialized":initialized}
    assert result["receiver_after"]=={"decimal":expected,"valid":None},case["case_id"]
print(f"{len(cases)} BSON encodings and both cross-decode directions verified")
