---
locale: ja
source_revision: e228552c41e4946b118a075a0bdaa811f6ff511347af55baa0908ccb2c361609
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

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
