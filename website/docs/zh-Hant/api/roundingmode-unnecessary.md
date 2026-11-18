---
locale: zh-Hant
source_revision: 1d47fa6ae1a12d0a01d3c07d39948787bbb4209bfed6d0edc8d1a4933e938a92
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

按只允許精確結果規則捨入。

```rust
Unnecessary = 6
```

## 參數、回傳值及常規用法

整數判別值為 6，建議使用具名值。規則適用於正數和負數。丟棄零數字仍屬精確操作；丟棄任何非零數字都回傳 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
