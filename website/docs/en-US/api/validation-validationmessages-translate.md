---
locale: en-US
source_revision: d21e2ccb9059c66b32b65cffe1a9b447ab9a7fc0664cf5fa020b0d334fe294fd
api_id: "validation::ValidationMessages::translate"
---
# validation::ValidationMessages::translate

Translate validation failures with owned templates.

```rust
fn translate (& self , errors : & [ValidationError]) -> Vec < String >
```

## Parameters, return and normal usage

errors: &[ValidationError]/array; returns a string array in the same order. Locale matching is case-insensitive, accepts hyphens/underscores and language-region suffixes; zh-Hant/zh-TW select traditional Chinese. en-US/en-GB use English; unknown locales fall back to English. {0} inserts the field and {1} the original parameter in a single pass; inserted braces are never expanded again. Returned strings are plain text; render them as text, not HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-translate" locale="en-US" />

## Related methods

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
