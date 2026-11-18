---
locale: zh-Hant
source_revision: 340ee9c355dc0ac1e0c13e6d43aee222d595260389706ec4d9398cbe1a49147c
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

從 crate 根路徑匯入捨入模式。

```rust
pub use rounding::RoundingMode as RoundingMode
```

## 參數、回傳值及常規用法

使用 rs_decimal::RoundingMode，這是同一個列舉，不是另一份實現。瀏覽器直接提供具名的 RoundingMode 物件，不使用 Rust 匯入語法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
