---
locale: zh-Hant
source_revision: 05a4d32c73f6c4d59c8e657272c093b1db00da738b0234c960526209d3e3196c
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::NullDecimal
---
# reexport::NullDecimal

從 rs_decimal crate 根路徑匯入 NullDecimal。

```rust
pub use decimal::NullDecimal as NullDecimal
```

## 常規用法及語義

使用 rs_decimal::NullDecimal；這是同一條目的公開匯入路徑，不是額外的數值實現。JavaScript 沒有 Rust use 語法，對應行為見 bridge 文件。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-nulldecimal" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
