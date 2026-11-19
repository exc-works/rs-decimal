---
locale: ja
source_revision: e9dbb721c8987afdaedb4f74b013156db52924dc6bfa528e95669ba4396942ff
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::rescale_down
---
# Decimal::rescale_down

ゼロ方向への切り捨てで小数点以下の桁数を変更します。

```rust
fn rescale_down (& self , prec : isize) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

prec: isize/整数 number。Decimal を返し、負の精度では InvalidPrecision になります。Down は負の無限大ではなくゼロ方向です。精度を上げるとゼロを追加します。入力は変わらず、結果は初期化済みです。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale-down" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
