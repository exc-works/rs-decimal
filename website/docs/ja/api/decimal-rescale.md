---
locale: ja
source_revision: 0deb49519f3f009c2dba973109de20fc0fcdfe90b9617749c506123c4c387997
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

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
