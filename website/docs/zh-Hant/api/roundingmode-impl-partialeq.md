---
locale: zh-Hant
source_revision: d358fea1722186e0df2e76e232b3793bff18d2e3a10cb6ef0ae0a724a40da40b
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

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
