# Method translation provenance

The source reference is en-US. Each completed method page records the SHA-256
revision of that reference body. Examples, signatures and expected states come
from the shared executable fixtures; localised prose does not fork numerical
behaviour.

## en-GB

All 355 method references reuse the reviewed en-US technical descriptions under
the reuse provision in `docs/web-playground-pr-plan.md`. This is an explicit
editorial choice: the APIs, precision rules, Rust terminology, error categories
and machine-readable decimal syntax have the same meaning for British readers.
US spelling retained in technical descriptions does not change their meaning.
The en-GB glossary, navigation and shared guides remain localised separately.

Each page records `translation_provenance: en-US-reviewed-reuse`, retains its
source revision and sets its shared example component to en-GB. Code identifiers
and decimal punctuation are unchanged. The ordinary translation check now
requires en-GB method pages to be present and current alongside en-US and
zh-Hans; this does not mark the remaining five method locales complete.

Later regional wording changes must preserve the shared examples and document
their source revision. A source update requires reviewing the affected en-GB
page again; stale pages fail the check.

## zh-Hant

The core methods (WEB-07a–d, WEB-08 and WEB-09a–b; 130 entries) are based on the
reviewed zh-Hans descriptions. Script conversion was followed by a terminology
review of the distinct descriptions, caveats and supplemental bridge/resource
pages. Terms follow the zh-Hant glossary: 係數, 捨入, 小數位數, 有效位數,
已初始化, 有效性旗標 and 精確. Prose also uses 回傳, 預設, 物件, 字串,
位元組, 記憶體 and 參照. Rust identifiers, numeric bounds, sign examples and
ownership/error ordering remain unchanged. Each completed page records
`translation_provenance: zh-Hans-terminology-reviewed` and the en-US revision.

`method_locale_owners` records the completed API groups. Ordinary checks enforce
their current source revisions; other groups retain their pending notices. The
full release gate still requires every method in every requested locale.

The conversion, formatting, root and log/exp groups (WEB-10a–b and WEB-11a–b;
47 more entries) have also been reviewed in zh-Hant. The review preserves exact
conversion flags, negative-zero/overflow transport, Unicode scalar separators,
rounding certification, domain/error precedence and resource budgets.

The text/JSON, binary/BSON and YAML/XML groups (WEB-12a–c; 33 more entries)
have been reviewed in zh-Hant, preserving each codec’s distinct null/empty
behaviour, state transitions, wire format and atomic decode failures.

SQL scalar/error and validation/message references (WEB-13a-1–2; 73 more
entries) have been reviewed in zh-Hant. The review retains borrowed byte-buffer
behaviour, error-kind membership, rule-specific missing-value handling, literal
message substitution and separate validation/library/resource errors.

The remaining config/Serde and native SQLx/trait groups (WEB-13b-1–2; 72
entries) complete all 355 zh-Hant references. Review covers generic Serde versus
raw JSON adapters, 128-bit input ranges, nullable state, native-only trait syntax,
PostgreSQL OIDs/wire limits and the separate live-database fixture profile. The
manifest now uses `*` so future API groups cannot bypass zh-Hant completeness.

## fr

The 130 core references (WEB-07a–d, WEB-08 and WEB-09a–b) and supplemental
bridge/resource guides are translated from en-US. The review follows the French
glossary (échelle, décimales, chiffres significatifs, arrondi, initialisation),
preserving sign/tie rules, state distinctions, borrowing/consumption, integer
widths and error/resource boundaries. Shared examples and Rust identifiers are
unchanged. Other French API groups remain pending and use explicit fallback.

The French conversion/formatting/root/log-exp references (WEB-10a–b,
WEB-11a–b; 47 additional entries) preserve exactness flags and IEEE edge cases,
Unicode scalar formatting, certified half-even results and resource/domain limits.

The French codec groups (WEB-12a–c; 33 additional entries) preserve distinct
null/empty semantics, canonical versus stored scale, binary/BSON wire types,
YAML/XML parsing caveats and atomic decode failures. XML entity literals remain
code-formatted so their visible meaning is unchanged.
