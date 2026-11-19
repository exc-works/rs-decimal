---
locale: zh-Hant
source_revision: 572356860c801a3a257544cc180e1dca7bec96afecfd793ab3dbbcce432242dd
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationMessages::impl Clone"
---
# validation::ValidationMessages::impl Clone

使用 Rust Clone 實現。

```rust
impl Clone for validation::ValidationMessages
```

## 參數、回傳值及常規用法

Clone 複製完整規則、模板映射或失敗欄位，副本獨立持有。這是原生 Rust trait 語法，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-impl-clone" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
