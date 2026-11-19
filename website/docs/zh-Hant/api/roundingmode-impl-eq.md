---
locale: zh-Hant
source_revision: d588c3aa025efc49b40f1c2c9f6f76c0ff1df249830b32d566fb6a6a5794cdc5
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::impl Eq
---
# RoundingMode::impl Eq

使用捨入模式的 Rust Eq 實現。

```rust
impl Eq for RoundingMode
```

## 參數、回傳值及常規用法

RoundingMode 是小型 Copy 列舉，複製不會改變所選規則。相等比較列舉項，Debug 輸出名稱。此範例驗證原生 Rust trait 語法；瀏覽器模式是不可變的數字值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-eq" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
