---
locale: zh-Hant
source_revision: b1d5b92283e0de37c46d70318c8e896d60bc04df8806e365913a4c12caa7a8d9
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

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
