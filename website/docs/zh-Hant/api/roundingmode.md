---
locale: zh-Hant
source_revision: 24579cc5f98ad85ad1558995873e9949e2e4e6ae6e01cc4702b2ee7ca18a14ea
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode
---
# RoundingMode

選擇七種明確的捨入規則之一。

```rust
pub enum RoundingMode
```

## 參數、回傳值及常規用法

Rust 使用具名列舉項，bridge 使用 RoundingMode 欄位。Rust 判別值 0–6 與瀏覽器數字一致。Down 向零，Up 遠離零，Ceiling 向正無窮。HalfUp/HalfDown/HalfEven 對正中間值分別遠離零、向零、取偶數。Unnecessary 要求精確。不設定隱式的全域捨入方式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
