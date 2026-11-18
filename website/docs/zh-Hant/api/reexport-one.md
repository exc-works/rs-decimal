---
locale: zh-Hant
source_revision: f5ea64d7e7a6135acc2f55f3a40411c6d9222e20ac49981dab203cb401fa6651
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::one
---
# reexport::one

從 rs_decimal crate 根路徑匯入 one。

```rust
pub use decimal::one as one
```

## 常規用法及語義

使用 rs_decimal::one；這是同一條目的公開匯入路徑，不是額外的數值實現。JavaScript 沒有 Rust use 語法，對應行為見 bridge 文件。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
