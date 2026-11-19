---
locale: zh-Hant
source_revision: 823f73234f73cba4c9bfd0b5f1d418fa482625a8365337d1d27eb5d7306b4e6d
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::impl Debug
---
# RoundingMode::impl Debug

使用捨入模式的 Rust Debug 實現。

```rust
impl Debug for RoundingMode
```

## 參數、回傳值及常規用法

RoundingMode 是小型 Copy 列舉，複製不會改變所選規則。相等比較列舉項，Debug 輸出名稱。此範例驗證原生 Rust trait 語法；瀏覽器模式是不可變的數字值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-debug" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
