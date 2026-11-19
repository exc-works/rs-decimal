---
locale: en-US
source_revision: 2b24eac398b765662bae8967979a2547332ee566c8d7e9c0bb785ba0d1f9e478
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

<MethodExample id="validation-validation-validationmessages-new" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
