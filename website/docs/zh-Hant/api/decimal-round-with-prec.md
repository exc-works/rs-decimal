---
locale: zh-Hant
source_revision: d6f697a3d962962b5c45eb345607fbc6629bac0f2d7f0f5c403da5649a7d1d4a
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

向最接近值捨入，正中間值取偶數，保留指定的小數位數。

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## 參數

`prec: isize` — 保留的小數位數，取整數。

## 回傳值

回傳已初始化的 Decimal，包括預設零輸入。方法借用輸入，提高精度會附加尾零。

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). 使用 [`rescale`](./decimal-rescale) 可指定其他捨入模式。

## 錯誤

負精度回傳 `InvalidPrecision`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
