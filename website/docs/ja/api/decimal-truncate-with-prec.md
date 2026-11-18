---
locale: ja
source_revision: cce5cfd2a6bfd3edbc32da84fa417cbea83f61f7b1bcf9e77c7c5b49aaed3994
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

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
