---
locale: zh-Hant
source_revision: 2d051b010383c0adad95eaeacec22cce35ee5a02d1fddebba48aa1d5a1f4d4ae
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::ceil_with_prec
---
# Decimal::ceil_with_prec

向正無窮方向，保留指定的小數位數。

```rust
fn ceil_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

prec: isize/整數 number；Rust 回傳 Result<Decimal, DecimalError>，bridge 回傳 Decimal 或拋出錯誤。負精度回傳 InvalidPrecision。輸入僅被借用，保持不變。結果已初始化，包括預設零輸入。提高精度會追加零，不改變數值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil-with-prec" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
