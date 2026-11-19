---
locale: ja
source_revision: 6f37682196df5b201aacde1b52aa54d192c469e0d50cfead6f78c24b12704f29
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::truncate_with_prec
---
# Decimal::truncate_with_prec

指定した小数点以下の桁数でゼロ方向へ丸めます。

```rust
fn truncate_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

prec: isize/整数 number。Rust は Result<Decimal, DecimalError>、bridge は Decimal または例外を返します。負の精度は InvalidPrecision です。負の値では、ゼロ方向の切り捨てと floor は異なります。-1.5 は前者では -1、floor では -2 になります。入力は借用され、変更しません。既定のゼロが入力でも結果は初期化済みです。精度を上げると数値を変えずにゼロを追加します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate-with-prec" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
