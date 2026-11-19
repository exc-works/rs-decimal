---
locale: zh-Hant
source_revision: 6e97ceecb692f3b30f694df6cf3812a7bc93e427ba0e4c690447e33c151380a9
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::round
---
# Decimal::round

向最接近值捨入，正中間值取偶數，得到整數。

```rust
fn round (& self) -> Self
```

## 參數、回傳值及常規用法

無參數，回傳小數位數為零的新 Decimal。這裡預設使用 HalfEven，與 JavaScript Math.round 的正中間值處理規則不同。輸入僅被借用，保持不變。結果已初始化，包括預設零輸入。提高精度會追加零，不改變數值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
