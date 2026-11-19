---
locale: zh-Hant
source_revision: 53cafbe90f3ab017866bcf2afb140d94d6b1a293f7aa6ca2a32a775110c8f82c
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::ceil
---
# Decimal::ceil

向正無窮方向，得到整數。

```rust
fn ceil (& self) -> Self
```

## 參數、回傳值及常規用法

無參數，回傳小數位數為零的新 Decimal。輸入僅被借用，保持不變。結果已初始化，包括預設零輸入。提高精度會追加零，不改變數值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
