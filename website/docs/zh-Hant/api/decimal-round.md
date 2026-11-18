---
locale: zh-Hant
source_revision: dfbccd4ef90aecedf2d24565d11016e8068251bd103af1f82ea5d3b25f25cee0
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

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
