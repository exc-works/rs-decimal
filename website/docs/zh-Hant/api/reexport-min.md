---
locale: zh-Hant
source_revision: 0d0d53ee31abc5089791404b98708465e5ae1271ee54c709858dd761b9a5ca22
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::min
---
# reexport::min

從 rs_decimal crate 根路徑匯入 min。

```rust
pub use decimal::min as min
```

## 常規用法及語義

使用 rs_decimal::min；這是同一條目的公開匯入路徑，不是額外的數值實現。JavaScript 沒有 Rust use 語法，對應行為見 bridge 文件。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-min" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
