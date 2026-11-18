---
locale: zh-Hant
source_revision: 9cb35d36232b48dfa841cf9ea23e2fcf6d9b36b6ef45c2f68538158901813e1b
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

向最接近值捨入，正中間值取偶數，保留指定的小數位數。

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## 參數

`prec: isize` — 保留的小數位數。bridge 接受 JavaScript 整數 number。

## 回傳值

回傳已初始化的 Decimal，包括預設零輸入。方法借用輸入，提高精度會附加尾零。

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). 使用 [`rescale`](./decimal-rescale) 可指定其他捨入模式。

## 錯誤

負精度回傳 `InvalidPrecision`. bridge 會拋出對應錯誤。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
