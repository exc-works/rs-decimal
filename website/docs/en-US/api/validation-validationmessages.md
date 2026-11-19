---
locale: en-US
source_revision: 9937fd6977e8a466e16b6a35082706b1183e93c9c873b5eb8e6c66d5ab917b4f
api_id: "validation::ValidationMessages"
---
# validation::ValidationMessages

Translate validation failures with owned templates.

```rust
pub struct ValidationMessages
```

## Parameters, return and normal usage

Owns templates for translating ValidationError records. Use new(locale) or with_messages(map); changing another template map does not affect this handle. Locale matching is case-insensitive, accepts hyphens/underscores and language-region suffixes; zh-Hant/zh-TW select traditional Chinese. en-US/en-GB use English; unknown locales fall back to English. {0} inserts the field and {1} the original parameter in a single pass; inserted braces are never expanded again. Returned strings are plain text; render them as text, not HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
