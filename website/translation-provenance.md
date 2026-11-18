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
