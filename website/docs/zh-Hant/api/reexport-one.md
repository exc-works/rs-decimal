---
locale: zh-Hant
source_revision: 29e91f626fd685fb0b5987fc104c1ba0e2752897194773305e3d64dcc1b16f1a
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::one
---
# reexport::one

從 rs_decimal crate 根路徑匯入 one。

```rust
pub use decimal::one as one
```

## 常規用法及語義

使用 rs_decimal::one；這是同一條目的公開匯入路徑，不是額外的數值實現。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
