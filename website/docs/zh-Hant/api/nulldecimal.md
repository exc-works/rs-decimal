---
locale: zh-Hant
source_revision: 8675e68949343d33893aa6f81cc6a57f32cdafd3aaf80161520178113f46a31d
translation_provenance: zh-Hans-terminology-reviewed
api_id: NullDecimal
---
# NullDecimal

用獨立的有效性旗標和儲存值表示可空 Decimal。

```rust
pub struct NullDecimal
```

## 常規用法及語義

是否存在值僅由 valid 決定。無效的可空值可以保留非零 decimal；有效的可空值也可以包含未初始化的零。Rust 的公開 decimal 欄位在 JavaScript 中透過明確的複製及移動適配器存取。預設值表示缺失。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal" locale="zh-Hant" />

這份共用範例同時在原生 Rust 和瀏覽器中執行。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
