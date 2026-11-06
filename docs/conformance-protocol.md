# Conformance protocol v1

The Go reference is pinned to `a8791208e2d15483cf652cd2814b74b0c1a84187` by its Go pseudo-version and checksum. Neither the reference nor CI resolves `main`. The Rust executor is a separate workspace package; consumers of the library do not depend on its serialization packages.

Each UTF-8 JSONL line contains one request. stdout contains only responses, in request order. Diagnostics and panic hooks use stderr. A malformed request terminates the process; the driver reports this as a transport failure, never as a numerical error.

```json
{"schema_version":1,"case_id":"state-1","operation":"state","inputs":[{"coefficient":"123","scale":"2","initialized":true}],"params":{}}
```

Decimal states carry a signed integer coefficient string, a nonnegative native-range scale string, and an initialized boolean. An uninitialized state must have coefficient and scale zero. Corpus inputs use canonical integer spelling. Integer operation parameters also use strings; floats use a bit width and hexadecimal bits, bytes use hex. Later operation-specific codecs define their params without changing these representations.

Responses contain `schema_version`, `case_id`, `outcome`, `inputs_after`, and `error_kinds`. Successful responses additionally contain `result`. Decoder operations add `receiver_before` and `receiver_after`, including `valid` for nullable receivers. Error categories are compared as sets; complete states are compared separately from numerical equality.

`outcome` is `ok`, `error`, `panic`, or `unsupported`; malformed protocol can report `protocol_error`. Only the explicitly expected outcome passes, so two identical unexpected errors still fail. The driver rejects unsupported operations, non-protocol stdout, missing/extra responses, changed IDs, abnormal exit and timeout. Corpus-only `expected_outcome` is removed before sending requests. An actual mismatch is saved in `.cache/conformance-failures/latest.json`; a minimized confirmed regression must be committed under testdata before fixing it.

PR 02 implements `state` to exercise initialized zero, uninitialized zero, large integer coefficients and retained scale. Numerical operations are enabled only with the corresponding library implementation. State round-trip is protocol coverage, not a numerical compatibility claim.

The independent oracle accepts the same request identity and numeric inputs plus `candidate`, and responds with `status=verified/mismatch/uncertified/unsupported` and `detail`. Initially it certifies exact rational addition and checks representation. It imports only Go's standard library. Roots and transcendental test logic will be extracted from the fixed source in PR 07/09 with provenance and checksums; no such extraction is claimed in PR 02. Only `verified` passes an oracle-selected case.

Run the infrastructure checks with:

```sh
cargo build --locked -p decimal-conformance
(cd tools/go-reference && go build -o ../../.cache/go-reference .)
(cd tools/go-oracle && go test ./... && go build -o ../../.cache/go-oracle .)
python3 tools/conformance/test_driver.py
python3 tools/conformance/run.py --go .cache/go-reference --rust target/debug/decimal-conformance --cases testdata/conformance/state.jsonl
```
