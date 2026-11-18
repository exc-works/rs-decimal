# Web documentation and playground delivery plan

This implements the [technical design](web-playground-design.md). WEB identifiers
are independent of historical core PRs 01–15. This document records planned
scope, not completed functionality. The design PR must finish review and merge
before implementation branches are opened.

## Review and stacked PR workflow

1. Branch from the merged design for WEB-01. Each dependent branch starts from
   its parent branch and opens a GitHub PR against that parent, not against main.
   Independent work may target main once its dependencies are merged. Record
   parent, children, stack order and completed checks in each PR description.
2. Complete local scope checks, push the reviewable changes, and comment
   `@codex review` on every PR. Request another review after substantive fixes.
3. Inspect the actual bot review, inline threads and CI results. For every
   finding, reproduce/check it against the contract. Fix valid findings with
   relevant regression coverage, or reply with evidence explaining why no change
   is needed. Do not treat lack of response, a pending reaction, unavailable
   review quota or a failed service as a successful review.
4. Keep review discussions attributable to the reviewed head SHA. Resolve a
   thread only after recording its disposition; repeat until no actionable
   findings remain. If the bot reports no findings, explicitly record that the
   result was examined and no fix is needed, with validation evidence.
5. Merge bottom-up only after the intended base is main, dependency merges are
   complete, required checks pass, and review dispositions are recorded. Use
   merge commits where repository policy permits, preserving ancestry. Do not
   merge a child into its feature parent as a substitute for merging to main.
6. After the parent merge, retarget the next PR to main, update it with the merged
   base as needed, verify its diff contains only its own scope, and rerun checks.
   If rebasing/squashing is required, restack with explicit old parent boundaries
   and lease-protected pushes; never overwrite unrelated changes. Re-review
   changed behavior, conflicts or substantive changes in the final merge context.
7. Check the head SHA immediately before merge and use the API's expected-head
   guard. Do not bypass branch protection or merge a different revision from the
   one whose checks/review were assessed. Preserve stack branches until children
   have been retargeted and ancestry verified.

The user has authorized opening PRs, posting review requests/dispositions, fixing
findings, and merging after this process. Review-service failure is a blocker to
merge, not permission to omit the requested review.

## PR scopes and acceptance

| ID | Scope | Dependencies | Acceptance |
| --- | --- | --- | --- |
| WEB-01 | Rust API inventory, dispositions, document/example schemas and checks | merged design | Every public entry point classified, including cfg/macro/trait cases; source drift detected |
| WEB-02 | Minimal Wasm wrapper and reproducible build | 01 | Browser parse, mul_exact, rescale, state/error smoke tests; artifact measurements; core MSRV retained |
| WEB-03 | Bridge types, strict conversion, errors, ownership and alias policy | 02 | Bigint/range checks, tuple/enum mapping, consumption and mutable/null semantics verified |
| WEB-04a | Rust resource preflight and per-instance operation budget | 03 | Expanded inputs, chained operations, owned failure and Wasm memory maximum verified |
| WEB-04b | Isolated runner, lifecycle and message protocol | 04a | Stop/timeout/trap recovery, stale output rejection, bounded output, tested static CSP/bootstrap isolation |
| WEB-05 | VitePress site, eight locale shells, glossary, navigation/search and translation checks | 01 | Static build, locale routes and fallback/status behavior; preview clearly labeled |
| WEB-06 | Monaco editor and run controls | 03, 04b, 05 | Completion, parameter help, snapshots, stop/reset, drafts and file import/export |
| WEB-07a | Safe constructors and executable documentation examples | 03, 05 | Integer width, coefficient/append precision, input preservation and shared native/Wasm/browser examples |
| WEB-07b | Representation queries and borrowed comparison/unary methods | 07a | State distinctions, cross-scale comparison, equality tie representation and borrowed input preservation |
| WEB-07c | Free helpers and panic constructor | 07b | Inclusive bounds, helper completion, shared examples and native panic/Wasm trap separation |
| WEB-07d | Nullable/type/trait documentation and metadata | 07c | Native-only trait examples, helper re-exports, initialization and nullable state distinctions |
| WEB-08 | Rounding and scale operations | 07 | Seven modes, signed ties, Unnecessary, precision and platform checks |
| WEB-09a | Add/sub/mul/div, safe/raw variants | 08 | Result scale, zero divisors, errors and unchanged inputs match native |
| WEB-09b | Powers, remainders, quotient/remainder and consuming methods | 09a | Tuple results, limits, owned success/error consumption and alias handling |
| WEB-10a | Integer, rational and float conversions | 07 | Range/exactness flags, binary boundaries and snapshot semantics |
| WEB-10b | Formatting, separators and append buffers | 07 | Trailing zeros, format options, buffer mutation and output preflight |
| WEB-11a | Square/general roots | 08 | Default precision, domains, budgets and complete native/Wasm states |
| WEB-11b | Logarithms and exponential | 11a | Exact certified results/errors, resource checks and cancellation |
| WEB-12a | Text/Param/JSON and NullDecimal | 10b | Raw numeric token fidelity, null state transitions, failure atomicity |
| WEB-12b | Binary and BSON | 12a | Golden byte round trips and receiver preservation |
| WEB-12c | YAML and XML | 12a | Raw text/tag/empty-value rules and localized usage examples |
| WEB-13a-1 | Driver-neutral Scan/Value and error types | 12a | Exact typed scalar transport, atomic nullable state and error categories |
| WEB-13a-2 | Validation rules and localized messages | 13a-1 | Exact comparisons, rule errors, field data and message translation |
| WEB-13b-1 | Config scenarios and Serde adapters | 13a-2 | Typed configuration, exact raw JSON and generic Serde distinctions |
| WEB-13b-2 | SQLx and native trait usage documentation | 13b-1 | Native-only examples tested and labeled, feature-specific checks |
| WEB-14 | Method forms, open-in-playground and versioned sharing | 06, 07, 08 | Forms use shared examples; switching locale retains state; version/size checks |
| WEB-15-* | Remaining translations by language and method family | 05 and corresponding method PR | Glossary, current source hashes, links and shared examples validated |
| WEB-16 | Deployment package and full release verification | all applicable above | Eight complete/current locales, full coverage, three browser engines, static headers/base paths and consistent versions |

Every API group includes its declarations, native/browser checks, paired examples,
en-US/zh-Hans prose, and coverage status in the same PR. Additional translations
follow as single-language/single-family PRs (for example WEB-15-ja-rounding and
WEB-15-fr-conversion). Reviewed en-GB reuse is allowed with explicit provenance;
untranslated content does not satisfy a release gate. Shared code, signatures
and expected results are not copied into eight independent implementations.

Aim for one behavior family and roughly 300–800 hand-written implementation lines
per review. Treat this as a reviewability guideline, not a reason to omit tests.
Split further when a patch grows past a coherent review scope; generated metadata
and translations are reported separately. Do not bundle numerical refactoring
or optimization into browser bridge work.

## Milestones and final checks

- First executable proof: WEB-02 runs actual Rust in a browser, with measured
  startup/artifact size. It is not evidence for all optional features.
- Interactive preview: WEB-01–08 provide locale shells, editor, basic constructors
  and comparisons, minimal arithmetic and full rounding exploration. Incomplete
  API/translation coverage is explicitly shown.
- Complete behavior coverage: WEB-09–14 and all inventory dispositions have
  evidence; native-only entry points have tested Rust examples.
- Release: translations and WEB-16 pass at the final commit. No reviewer finding,
  target difference or translation gap is concealed behind a successful build.

Run checks appropriate to each change plus the existing required repository CI.
Documentation-only PRs validate links, referenced symbols and diff hygiene;
bridge PRs add meaningful boundary/state tests; runner PRs test lifecycle and
isolation; release checks exercise Chromium, Firefox and WebKit. Record review
outcome and CI URLs in each PR. Never claim a check passed when it was unavailable
or when only the native equivalent ran.
