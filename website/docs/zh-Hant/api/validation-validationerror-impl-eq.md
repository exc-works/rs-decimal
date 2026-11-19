---
locale: zh-Hant
source_revision: cfe29d83921c9d9fef3ddeed3842336ccc12799d1b0ed231fac999e4d87de7f7
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError::impl Eq"
---
# validation::ValidationError::impl Eq

使用 Rust Eq 實現。

```rust
impl Eq for validation::ValidationError
```

## 參數、回傳值及常規用法

ValidationError 相等同時比較 field、tag 和原始 parameter，不是只比較翻譯後的訊息。這是原生 Rust trait 語法，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-eq" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
