# Static isolated runner (WEB-04b)

The documentation page never evaluates user source. `RunnerClient` embeds a
sandboxed iframe on a **separate dedicated static origin**, then transfers a
MessageChannel to that trusted frame. Each run gets a fresh Worker, private
channel and Wasm instance. The worker bundle contains the actual Rust Wasm and
bridge; it fetches no initialization dependency at runtime.

Build the Wasm first, then set `RUNNER_PARENT_ORIGINS` to a JSON array of exact
HTTPS documentation origins and run `npm run build:runner` from `website`.
The package in `runner-dist/` must be served at the runner origin root, including
its response headers. `_headers` describes a static-host configuration;
`headers.json` contains the equivalent map for other hosts. This slice does not
provision hosting. Local tests use `RUNNER_ALLOW_LOCALHOST=1` and an explicit
`http://127.0.0.1:4210` parent, never a wildcard. Do not enable this override in a
production build.

The iframe document permits only local trusted scripts/workers and restricts
`frame-ancestors` to configured parents. The worker response allows evaluation
for user JS/Wasm but denies network connections, script imports (including data
URLs) and nested workers. Frame and worker CSP paths are disjoint because some
static hosts append overlapping headers. Deploy headers as HTTP response headers,
not only a meta tag. Startup probes must fail before source can run; the package
provides successful probe endpoints so missing CSP is detected. Browser tests
also verify that no probe request reaches the protected server and that the
same package without CSP rejects execution. These probes are a configuration
check, not a substitute for the tested header policy.

Both sides validate schema, commit, run identity and payload bounds. The parent
allowlist is compiled into the frame. The documentation's runner URL and commit
come from trusted deployment configuration, never from shared user snippets.
The runner refuses a Wasm/build commit mismatch. The parent rejects same-origin
execution and handshake failures; no shared script runs automatically.

The default execution deadline is 3 seconds, enforced outside user JS by the
parent and independently by the frame. Stop/replacement invalidates the old run
before terminating its Worker. Late results are ignored. A script that loops,
traps, throws or closes its Worker cannot prevent the parent deadline. A fresh
run works after cancellation or failure. The parent can choose a shorter deadline
for tests. No mutable Decimal survives across runs.

Source is limited to 64 KiB UTF-8. `print(value)` snapshots Decimal/NullDecimal and
serializes bounded data: 100 outputs / 256 KiB total; depth 4, 100 properties per
object, 1000 visited nodes, strings truncated to 8192 characters. Accessors are
shown without invoking them; proxies can still execute code, so the external
deadline remains necessary. Callers receive JSON text and must render via
`textContent` or escaped framework interpolation. Never assign it to `innerHTML`.
Raw global Worker messages are ignored; only the private channel carries output.

These limits supplement [Rust preflight](../docs/en-US/resource-policy.md). A
Worker and the 64 MiB Wasm maximum do **not** impose a hard JavaScript/process
memory quota; arbitrary JavaScript can allocate memory before termination. Use a
credential-free runner origin with no other application or valuable storage.
This is a static execution design, not a server-side Rust compilation service.

`tests/browser-runner.py` launches three local static servers: docs, protected
runner and a missing-CSP control. Set matching `CHROMEDRIVER` and
`TEST_CHROME_BINARY`; run it from the repo after packaging with the local parent.
The development harness lives under `tests/` and is not in the deployed package.
The Monaco editor/UI is WEB-06; this PR supplies and tests the execution client.
