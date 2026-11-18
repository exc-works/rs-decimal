---
locale: zh-Hant
source_revision: 099c3533a52ede06adc92af14955d2d2a20dadf66c185c385b9eda728b52b6ba
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::zero
---
# reexport::zero

從 rs_decimal crate 根路徑匯入 zero。

```rust
pub use decimal::zero as zero
```

## 常規用法及語義

使用 rs_decimal::zero；這是同一條目的公開匯入路徑，不是額外的數值實現。JavaScript 沒有 Rust use 語法，對應行為見 bridge 文件。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
