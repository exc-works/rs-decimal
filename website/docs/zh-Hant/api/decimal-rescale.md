---
locale: zh-Hant
source_revision: 203acbe634de9e56f24b2f6e3eccc13dcb25f721727e606d7d5a0efdca2a8e02
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::rescale
---
# Decimal::rescale

使用明確的捨入模式改變小數位數。

```rust
fn rescale (& self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

prec 表示小數位數，不是有效位數位數；mode 是七種 RoundingMode 值之一。提高精度追加零；降低精度時只對被丟棄的非零數字捨入。負精度回傳 InvalidPrecision；Unnecessary 遇到需要捨入時回傳 RoundUnnecessary。結果會將預設零初始化；借用的輸入保持不變。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
