---
locale: zh-Hant
source_revision: 7f1c2832917badced6e78a9c287520234452b297109e8406a028c3703581c859
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError::impl PartialEq"
---
# validation::ValidationError::impl PartialEq

使用 Rust PartialEq 實現。

```rust
impl PartialEq for validation::ValidationError
```

## 參數、回傳值及常規用法

ValidationError 相等同時比較 field、tag 和原始 parameter，不是只比較翻譯後的訊息。這是原生 Rust trait 語法，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
