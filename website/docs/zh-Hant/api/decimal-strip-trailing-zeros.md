---
locale: zh-Hant
source_revision: a66a15086c7943a9a541c1451a0fe544a096af2a9b938eb44bd20730eac8975d
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

去掉多餘的小數末尾零，不改變數值。

```rust
fn strip_trailing_zeros (& self) -> Self
```

## 參數、回傳值及常規用法

無參數，回傳獨立的 Decimal。僅在 scale 為正時移除十因子，因此不會去掉 1200 這樣的整數末尾零。任何數值零都會變為已初始化且 scale 為零的零值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
