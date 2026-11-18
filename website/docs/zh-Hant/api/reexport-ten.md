---
locale: zh-Hant
source_revision: 76d9cb7e766229b955c894271a5c238caac503dc592850e7b5cbe4a6a3eb33d3
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::ten
---
# reexport::ten

從 rs_decimal crate 根路徑匯入 ten。

```rust
pub use decimal::ten as ten
```

## 常規用法及語義

使用 rs_decimal::ten；這是同一條目的公開匯入路徑，不是額外的數值實現。JavaScript 沒有 Rust use 語法，對應行為見 bridge 文件。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-ten" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
