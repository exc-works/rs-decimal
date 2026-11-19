---
locale: zh-Hant
source_revision: 262ffddf12b7585e3d9a91337def5d4995343306864bd291c3fadc2686ec5701
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationError::impl Clone"
---
# validation::ValidationError::impl Clone

使用 Rust Clone 實現。

```rust
impl Clone for validation::ValidationError
```

## 參數、回傳值及常規用法

Clone 複製完整規則、模板映射或失敗欄位，副本獨立持有。這是原生 Rust trait 語法，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-clone" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
