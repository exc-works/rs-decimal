---
locale: zh-Hant
source_revision: dce0eeed904ef266b7197b18fd411cf158683a2fe3f9074f19a11965111706e9
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::impl PartialEq
---
# RoundingMode::impl PartialEq

使用捨入模式的 Rust PartialEq 實現。

```rust
impl PartialEq for RoundingMode
```

## 參數、回傳值及常規用法

RoundingMode 是小型 Copy 列舉，複製不會改變所選規則。相等比較列舉項，Debug 輸出名稱。此範例驗證原生 Rust trait 語法；瀏覽器模式是不可變的數字值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-partialeq" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
