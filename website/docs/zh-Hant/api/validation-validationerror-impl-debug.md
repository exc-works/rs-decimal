---
locale: zh-Hant
source_revision: 0da87c25bd0ca11269721d2b8d815d7bac612f73d7110e87ce497372ebbaf01e
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError::impl Debug"
---
# validation::ValidationError::impl Debug

使用 Rust Debug 實現。

```rust
impl Debug for validation::ValidationError
```

## 參數、回傳值及常規用法

Debug 產生診斷文字，不是穩定的驗證序列化格式。這是原生 Rust trait 語法，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-debug" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
