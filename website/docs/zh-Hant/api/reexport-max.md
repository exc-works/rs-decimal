---
locale: zh-Hant
source_revision: 0db8fa19358a4f2f8842ab2f8da65e8e108f7914cbdecf8f41f654ca5171b70c
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::max
---
# reexport::max

從 rs_decimal crate 根路徑匯入 max。

```rust
pub use decimal::max as max
```

## 常規用法及語義

使用 rs_decimal::max；這是同一條目的公開匯入路徑，不是額外的數值實現。JavaScript 沒有 Rust use 語法，對應行為見 bridge 文件。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
