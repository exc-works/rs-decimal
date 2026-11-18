---
locale: zh-Hant
source_revision: be3c0e366eb3a51881c8080b2a8de1d5f2faa06f8be431e48c0342e9c8d4c0e5
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::HalfDown
---
# RoundingMode::HalfDown

按最接近值，正中間值向零規則捨入。

```rust
HalfDown = 4
```

## 參數、回傳值及常規用法

整數判別值為 4，建議使用具名值。規則適用於正數和負數。Down、Up 相對零定義，不是負、正無窮方向。Half 模式僅在正中間值時有區別。沒有單獨的 Floor 列舉項；該方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfdown" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
