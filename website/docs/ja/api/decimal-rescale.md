---
locale: ja
source_revision: 203acbe634de9e56f24b2f6e3eccc13dcb25f721727e606d7d5a0efdca2a8e02
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::rescale
---
# Decimal::rescale

明示的な丸めモードで小数点以下の桁数を変更します。

```rust
fn rescale (& self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

prec は有効数字ではなく小数点以下の桁数、mode は 7 つの RoundingMode 値のいずれかです。精度を上げるとゼロを追加します。精度を下げるときは、捨てるゼロ以外の桁だけが丸めに影響します。負の精度は InvalidPrecision、Unnecessary で丸めが必要なら RoundUnnecessary を返します。結果では既定のゼロも初期化され、借用入力は変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
