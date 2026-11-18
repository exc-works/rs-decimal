---
locale: zh-Hant
source_revision: 6d83b155d9084cec9f42deea0ac618a5403b16dd0003e0789794c0428f3a019a
translation_provenance: zh-Hans-terminology-reviewed
api_id: NullDecimal::new
---
# NullDecimal::new

把 Decimal 移入一個有效的可空值。

```rust
fn new (decimal : Decimal) -> Self
```

## 常規用法及語義

decimal: Decimal 會被消耗；結果始終 valid=true，即使 decimal 是未初始化的零。移動後的 JavaScript 輸入不能繼續使用。需要保留原值時應傳入副本。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-new" locale="zh-Hant" />

這份共用範例同時在原生 Rust 和瀏覽器中執行。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
