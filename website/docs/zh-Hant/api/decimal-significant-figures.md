---
locale: zh-Hant
source_revision: bae692e3c5c12038fd1ef26d04bf1880cf624ae83c8c2ec0ad9f2f071f1b9f3a
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::significant_figures
---
# Decimal::significant_figures

捨入到不超過指定數量的十進位有效位數。

```rust
fn significant_figures (& self , figures : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

figures: 正 isize/整數 number；mode 指定捨入方式。與小數位精度不同，有效位數按係數的數字位數計算。此方法不會增加精度，也可對整數位捨入（1234.5 保留三位得到 1230）。非正 figures 回傳 InvalidArgument；Unnecessary 遇到需要捨入時回傳 RoundUnnecessary。瀏覽器會預先檢查可能的係數進位。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-significant-figures" locale="zh-Hant" />

## 瀏覽器邊界

所有整數參數必須位於 wasm32 對應範圍。瀏覽器還施加 [資源限制](../resource-policy)，包括目標小數位數、係數增長及呼叫次數。資源錯誤與數值程式庫錯誤分開報告。借用方法不會消耗輸入；僅 Rust 的 trait 語法明確標注為原生測試。

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
