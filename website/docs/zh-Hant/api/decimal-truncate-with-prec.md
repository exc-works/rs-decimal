---
locale: zh-Hant
source_revision: d9bfbd2d29924d270b97f6a4251656ca421cb89e51e3bdcdc1ecdf2cdefab10f
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::truncate_with_prec
---
# Decimal::truncate_with_prec

向零方向，保留指定的小數位數。

```rust
fn truncate_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

prec: isize/整數 number。負精度回傳 InvalidPrecision。負數截斷與向下取整不同：-1.5 截斷為 -1，floor 為 -2。輸入僅被借用，保持不變。結果已初始化，包括預設零輸入。提高精度會追加零，不改變數值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate-with-prec" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
