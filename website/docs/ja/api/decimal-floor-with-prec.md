---
locale: ja
source_revision: 6ba2cb4a20d5541fd7637c273e7892b620d22f9a54a85ae9d00d1d5387296c75
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::floor_with_prec
---
# Decimal::floor_with_prec

指定した小数点以下の桁数で負の無限大方向へ丸めます。

```rust
fn floor_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

prec: isize/整数 number。負の精度は InvalidPrecision です。負の値では、ゼロ方向の切り捨てと floor は異なります。-1.5 は前者では -1、floor では -2 になります。入力は借用され、変更しません。既定のゼロが入力でも結果は初期化済みです。精度を上げると数値を変えずにゼロを追加します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor-with-prec" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
