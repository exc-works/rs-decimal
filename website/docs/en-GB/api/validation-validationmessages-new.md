---
locale: en-GB
source_revision: d38eb31907db7ef2944305de43af506a13542551be2fcba604fea1ed0a54e78f
translation_provenance: en-US-reviewed-reuse
api_id: "validation::ValidationMessages::new"
---
# validation::ValidationMessages::new

Translate validation failures with owned templates.

```rust
fn new (locale : & str) -> Self
```

## Parameters, return and normal usage

locale: &str/string; returns fresh owned message templates. Locale matching is case-insensitive, accepts hyphens/underscores and language-region suffixes; zh-Hant/zh-TW select traditional Chinese. en-US/en-GB use English; unknown locales fall back to English. {0} inserts the field and {1} the original parameter in a single pass; inserted braces are never expanded again. Returned strings are plain text; render them as text, not HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-new" locale="en-GB" />

## Browser boundaries

The validation feature is enabled. Each string is capped at 8192 UTF-8 bytes; rule/error lists and message maps at 64 entries; JSON transport at 65536 bytes. Decimal rule parameters follow the [resource policy](../resource-policy). Translation preflights a conservative total substituted-text bound of 65536 bytes and may reject a just-fitting result. Custom tags and fields are data, never executed code. Validation failures, configuration errors and bridge resource errors remain distinct.

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
