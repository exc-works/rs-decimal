---
locale: zh-Hant
source_revision: e228552c41e4946b118a075a0bdaa811f6ff511347af55baa0908ccb2c361609
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::rescale_down
---
# Decimal::rescale_down

透過向零截斷改變小數位數。

```rust
fn rescale_down (& self , prec : isize) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

prec: isize/整數 number；回傳 Decimal，負精度回傳 InvalidPrecision。Down 表示向零，不是向負無窮。提高精度追加零。輸入不變，結果已初始化。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale-down" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
