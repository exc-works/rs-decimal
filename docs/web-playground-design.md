# Multilingual documentation and browser playground

Status: implementation design; browser support is not yet verified. See the
[delivery plan](web-playground-pr-plan.md) for independently reviewed changes.
This extends the existing Rust documentation without changing the numerical
contract, Go compatibility baseline, or core crate's Rust 1.85 MSRV.

## Goals and scope

Publish usage-oriented documentation in Simplified Chinese (`zh-Hans`),
Traditional Chinese (`zh-Hant`), US English (`en-US`), British English (`en-GB`),
French (`fr`), Korean (`ko`), Japanese (`ja`), and German (`de`). Every public
Rust entry point must have a documented disposition. Provide editable scripts
that call the actual Rust library compiled to WebAssembly, without a computation
server. JavaScript supplies variables, loops, functions and exception handling;
Rust supplies decimal arithmetic, conversions, formatting and codecs.

Arbitrary Rust compilation, arbitrary npm installation, database connections,
accounts, and server-side snippet storage are outside this release. Sharing uses
versioned URL fragments for bounded scripts and file export for larger scripts.
Shared scripts never execute automatically. A preview is not a claim of complete
API or translation coverage.

## Architecture

```text
VitePress documentation / Monaco editor
    -> run controller and structured messages
    -> isolated runner origin, fresh Web Worker for each run
    -> JavaScript bridge with Rust method names
    -> wasm-bindgen wrapper crate
    -> existing rs-decimal crate
```

Use VitePress/Vue/TypeScript for static pages and interaction, Monaco for the
full editor, and wasm-bindgen with `wasm32-unknown-unknown` for the library.
Load the editor and Wasm only when needed, after client mounting, never during
static-site rendering. Pin dependencies and toolchains. Build the wrapper with
an explicit feature list excluding `postgres`, not `--all-features`. Keep Wasm
dependencies out of the core crate. New workspace dependencies must preserve
existing MSRV jobs or be isolated in a separately locked tools workspace with a
documented toolchain; silently increasing the core MSRV is not allowed.

```text
crates/playground-bridge/        Rust wrappers and Wasm exports
website/docs/<locale>/          translated guides and method explanations
website/api/                    Rust API inventory and bridge dispositions
website/examples/               paired Rust/JS examples and expected states
website/src/bridge/             JS facade, declarations, error conversion
website/src/runner/             run lifecycle and snapshot protocol
website/src/components/         editor, method forms, result panels
website/.vitepress/             site, locales, navigation and search
tools/check-web-api/            coverage and consistency checks
```

The user interface is an object bridge, not a generic JSON operation dispatcher.
Page/Worker messages carry run identity, source, output snapshots and status.
Messages use a versioned discriminated union and reject unexpected shapes,
oversized payloads, stale run IDs and untrusted senders. Version metadata includes
crate version, commit, bridge schema, target, enabled features and applied limits.

## Bridge contract

Preserve Rust method names (including snake_case), parameter order, parameter
meaning and required arguments. Associated functions become static methods and
methods become instance methods. Do not introduce implicit rounding, precision,
numeric coercion or operator overloads. Bridge-only helpers live under
`playground`, separate from the library API. Display both signatures together.

```rust
use rs_decimal::Decimal;
let price = Decimal::new_from_string("12.350")?;
let total = price.mul_exact(&Decimal::new(3))?;
assert_eq!(total.string_with_trailing_zeros(), "37.050");
# Ok::<(), rs_decimal::DecimalError>(())
```

```javascript
const price = Decimal.new_from_string("12.350");
const total = price.mul_exact(Decimal.new(3n));
playground.show(total);
playground.assert(total.string_with_trailing_zeros() === "37.050");
```

The JavaScript example specifies the planned API, not an existing export.

| Rust contract | Browser mapping |
| --- | --- |
| Decimal / NullDecimal | Wrapper objects backed by real Rust values |
| `&Decimal` | Existing object, retained after the call |
| `&mut self` | Mutates receiver; decoder failure atomicity is retained |
| consuming `self` | Invalidates the input wrapper, including returned-error paths after consumption |
| i64/u64/i128/u128, BigInt | JS bigint with explicit Rust range checks; no narrowing or wrapping |
| wasm32 isize/usize, small integers | JS number checked for integerness and target range before Wasm ABI conversion |
| f32/f64 | JS number; f32 conversion and NaN/Infinity behavior documented |
| strings / char | JS string; char requires exactly one Unicode scalar value |
| immutable bytes | Uint8Array |
| mutable Vec of bytes | Explicit mutable buffer wrapper, preserving mutation visibility |
| BigRational | Exact numerator/denominator bigint representation, rejecting zero denominator |
| tuples | Ordered JS tuples, including exactness flags |
| Result | T on success; structured exception on returned error |
| Option | null or value, independently of Decimal initialization state |
| enums / argument structs | Named constants / field objects with strict validation |

Keep all Decimal input text intact. In particular raw JSON number examples must
pass original text or bytes into Rust, not round-trip through JS JSON numeric
values. Snapshot transport uses tagged strings for arbitrary integers, explicit
float bit representations where needed, and byte arrays/hex; it cannot rely on
plain JSON.stringify for bigint, NaN, Infinity or negative zero.

Returned DecimalError exceptions expose the complete stable kind set and the
original diagnostic. Localized explanations are additional UI text, not changes
to library messages. Bridge type/range errors, use-after-move, sandbox limits,
timeouts and Wasm traps are separate categories. Ordinary returned errors do not
become panics. `must_*` methods retain failure semantics; a trap ends that run and
discards the instance. Do not depend on native catch_unwind in default Wasm.

Borrowed coefficient access returns an immutable bigint snapshot in JS, not a
live Rust reference. max_ref/min_ref must select the corresponding original
wrapper, not silently clone it. Rust borrow checking, generic trait resolution,
and lifetimes remain documented Rust-only semantics. Alias combinations rejected
by Rust borrow rules must be detected/rejected consistently by the bridge rather
than causing hidden copies or panics. Mutable public fields, especially
NullDecimal.decimal, require an explicit access policy in WEB-03: a cloned getter
must not masquerade as a mutable reference. A disposition may require a named
adapter instead of misleading direct access.

## API inventory and ownership

The existing api-inventory.json describes the pinned Go baseline; it is not the
Rust API inventory. Add a source-backed Rust inventory including associated
functions, methods, free functions, public types/fields/constants, conditional
features, macro-generated conversions and trait implementations. Track explicit
re-exports without claiming all third-party BigInt APIs as rs-decimal methods.
Inventory extraction must not depend on an unpinned nightly rustdoc JSON schema.
Generated wrappers/declarations can derive from checked metadata, but the source
inventory is independently compared to prevent jointly missing a method.

Each entry records Rust ID/signature, feature, receiver ownership, bridge
signature, mapping differences, documentation ID, examples, tests, delivery PR,
and status: direct, named adapter, or Rust-only with a reason. Planned coverage
is not implemented coverage. Read the existing conformance fixtures as evidence,
retaining their documented Go-to-Rust error migrations; do not relabel panics as
successful numerical outcomes. Avoid exposing the trusted CLI's unwrap/expect
parameter handling to browser inputs.

| Family | Delivery |
| --- | --- |
| construction, comparison, arithmetic, rounding, formatting | Executable bridge and examples |
| roots, logarithms, exponentials, conversions | Executable bridge with resource preflight |
| Text/Param/JSON/Binary/YAML/XML/BSON | Exact raw inputs and before/after states |
| validation and driver-neutral SQL | Executable named APIs |
| config | Value conversion APIs and fixed configuration examples |
| generic Serde field adapters | Fixed concrete Rust scenarios, labeled as adapters |
| PostgreSQL SQLx | Native examples and integration tests; no browser connection |
| traits, borrowing, lifetimes | Rust examples/compile checks where relevant; explicit mapping limits |

wasm32 has 32-bit isize/usize. i64/u64 retain their full ranges. Test common-domain
native/Wasm equivalence and separate target-specific fixtures; never promise
all 64-bit native boundary behavior in the browser.

## Runner, resources and deployment

Create a fresh Worker and Wasm instance per run. Cache downloads/compiled assets
where supported, not mutable objects from past runs. The main page controls
deadlines and termination; a Promise timeout or worker-local timer cannot stop
synchronous Wasm. Invalidate the run ID before termination, discard late output,
and ensure stop/restart works after both infinite JS loops and costly Wasm calls.

Limits must cover source/input bytes, coefficient digits, scale, power/root
degree, formatting width, operation count, output count/bytes and elapsed time.
Enforce allocation-sensitive limits inside Rust wrappers before calling the core,
as JS checks alone are bypassable. Validate expanded exponent/scale and estimated
result size, not just source string length. Bounded objects must remain bounded
across sequences of calls. Snapshotting and formatting must also be bounded.
Publish concrete defaults with measurements in WEB-04 and each operation PR.
These are playground restrictions, reported separately from core library errors.
A Worker does not provide a hard process-memory quota; preflight bounds and
termination reduce risk but must not be advertised as unlimited safe execution.

The current deployment target is a single GitHub Pages project site. This replaces
WEB-04b/WEB-16's initial separate-origin deployment following the user's explicit
single-site requirement. The parent fetches a versioned trusted bootstrap bundle
from the same site and creates an iframe with `sandbox="allow-scripts"` only.
Its opaque origin isolates document and origin storage. User source crosses a
private MessageChannel; it is never interpolated into srcdoc or evaluated in the
parent/frame. A fresh Blob Worker embeds the bridge and Wasm for each run.

The iframe uses a meta CSP denying network requests and external imports; Blob
Workers inherit that policy. Worker/SharedWorker constructors are locked before
user code so it cannot spawn descendants. Runtime probes fail closed when the
network policy is absent. Three-engine tests cover opaque origin/storage behavior,
network/import denial, constructor restrictions and a missing-policy control.
No shared snippet auto-runs; output remains bounded text/structured data.

All files deploy together through the GitHub Pages Actions workflow, including
`runner/sandbox.json`. No separate static host, custom response headers or origin
allowlist is required. Base paths remain configurable. Docs, bootstrap and Wasm
must identify the same commit, and mismatches prevent execution until refreshed.
See `website/DEPLOYMENT.md` for publishing and rollback checks.

## Documentation, localization and interaction

Use en-US as the technical reference text; author/review zh-Hans alongside each
API group. All eight languages have the same API IDs and example data. Translate
explanations, navigation, search labels, controls and contextual diagnostics;
do not translate method names, exact input syntax or stable error identifiers.
Switching language preserves method and script. Search current-language prose
and original API names. Provide localized fallback explanations in previews,
but require complete, current translations for release.

Method pages lead with purpose, normal Rust usage and expected output, then an
equivalent runnable bridge example, parameters/returns, common pitfalls, errors
and exceptional boundaries, and related APIs. Explain relevant scale and null
behavior next to the example, not buried in implementation history. Distinguish
decimal places from significant figures and rounding away from zero from rounding
toward positive infinity. Keep compatibility/test history in linked maintainer
documents rather than the main usage narrative.

Use a shared glossary and source-revision/hash tracking per translated unit.
Traditional Chinese requires terminology review, not only character conversion.
British English may reuse reviewed US English units at build time with explicit
regional overrides and coverage tracking; missing translations cannot silently
count as complete. Changes to behavioral prose invalidate translations, while
shared code/expected results update once for every locale.

Method forms and scripts share bridge calls and example definitions. Forms can
open their generated calls in the full editor. The editor supports completion,
parameter help, variables/loops/functions, try/catch, assertions, multiple output
snapshots, stop/reset, draft recovery and file import/export. Helpers capture
state immediately at show() time so later mutation or movement cannot alter old
output. Decimal snapshots show compact/trailing-zero text, coefficient, scale and
initialization. Nullable snapshots include validity; decoder examples include
before/after state even on failure. Preserve bounded partial output on errors.

Share payloads contain schema, source, crate/bridge version and limits profile;
validate size before decoding/decompressing. Never silently run a versioned
snippet against a different library. Unknown versions offer an explicit migration
or download action. Do not promise automatic conversion of arbitrary JS to Rust;
maintain executable paired Rust/JS examples instead.

## Verification and release gates

Each API PR includes behavior tests, paired examples, en-US/zh-Hans explanation,
inventory status, and bridge declarations. Compare native and browser results as
complete states, not formatted numbers alone. Test bigint boundaries, float bit
cases, tuple exactness flags, mutation, moved wrappers, aliases, invalid enum and
argument shapes, and receiver preservation on failure. Maintain current native
Go differential/oracle gates without conflating them with Wasm evidence.

Run wasm-bindgen browser tests and page/Worker integration tests. Release checks
cover Chromium, Firefox and WebKit, failed loading, timeout/restart, isolation,
version mismatch, localization navigation, search, links and deployed base paths.
Check all public Rust IDs have dispositions and every runnable entry has an
example and test; Rust-only items still need usage documentation. Track generated
artifacts independently from hand-written changes. Keep existing fmt, clippy,
MSRV and packaging checks passing. Verify the core crate package does not acquire
path dependencies on unpublished browser tools.

Report measured compressed Wasm/JS sizes, download/initialization and representative
run times rather than promising unmeasured targets. Release requires all eight
languages complete/current, no unresolved correctness review findings, successful
checks at the final commit, and the review/merge process in the delivery plan.

## References

- [Rust wasm32 target and panic behavior](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html)
- [wasm-bindgen exported types and moves](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/exported-rust-types.html)
- [wasm-bindgen Result mapping](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/result.html)
- [VitePress internationalization](https://vitepress.dev/guide/i18n)
- [VitePress SSR compatibility](https://vitepress.dev/guide/ssr-compat)
- [Web Workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers)
- [Existing resource boundaries](rust-api.md#resource-boundaries)
- [Existing conformance protocol](conformance-protocol.md)
