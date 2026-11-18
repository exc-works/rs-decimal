---
locale: zh-Hant
source_revision: a9093dcd1b9fe39261e7248debd207a7c4986a0a263f3e970359020a9ba293a8
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::between
---
# reexport::between

從 rs_decimal crate 根路徑匯入 between。

```rust
pub use decimal::between as between
```

## 常規用法及語義

使用 rs_decimal::between；這是同一條目的公開匯入路徑，不是額外的數值實現。JavaScript 沒有 Rust use 語法，對應行為見 bridge 文件。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-between" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
