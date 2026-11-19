---
locale: ja
source_revision: 11b8aa485f155a658e5960045fe97e0878f07fd6eb371a471f1440f29be38903
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::ceil_with_prec
---
# Decimal::ceil_with_prec

指定した小数点以下の桁数で正の無限大方向へ丸めます。

```rust
fn ceil_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

prec: isize/整数 number。負の精度は InvalidPrecision です。入力は借用され、変更しません。既定のゼロが入力でも結果は初期化済みです。精度を上げると数値を変えずにゼロを追加します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil-with-prec" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
