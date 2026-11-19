---
locale: zh-Hant
source_revision: db004727284a336c6f0e549dbc589384cfceb8780f95c6036a8917d02c730891
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::floor
---
# Decimal::floor

向負無窮方向，得到整數。

```rust
fn floor (& self) -> Self
```

## 參數、回傳值及常規用法

無參數，回傳小數位數為零的新 Decimal。負數截斷與向下取整不同：-1.5 截斷為 -1，floor 為 -2。輸入僅被借用，保持不變。結果已初始化，包括預設零輸入。提高精度會追加零，不改變數值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
