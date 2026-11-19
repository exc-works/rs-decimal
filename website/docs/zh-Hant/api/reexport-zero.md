---
locale: zh-Hant
source_revision: d9e6c46654b05ae244bd56f7035b17e55d6f30ce2bee3665fad9be992491dd6d
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::zero
---
# reexport::zero

從 rs_decimal crate 根路徑匯入 zero。

```rust
pub use decimal::zero as zero
```

## 常規用法及語義

使用 rs_decimal::zero；這是同一條目的公開匯入路徑，不是額外的數值實現。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
