---
locale: zh-Hant
source_revision: 69f5b453b56bcf2350f3a3cf2e3d29b1035a464f2105cefeff33c4c49c926524
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::shift
---
# Decimal::shift

把 Decimal 乘以精確的十次冪。

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

places: isize/整數 number。正數向右移動小數點，負數向左移動。與 rescale 不同，它會改變數值。非零位移產生超過 i32::MAX 的 scale 時，核心回傳 Overflow。瀏覽器增長及精度檢查也可能在分配前回傳獨立的資源錯誤。即使 shift(0) 也會初始化預設零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
