---
locale: zh-Hant
source_revision: e9dbb721c8987afdaedb4f74b013156db52924dc6bfa528e95669ba4396942ff
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

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
