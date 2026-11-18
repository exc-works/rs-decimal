---
locale: ja
source_revision: bae692e3c5c12038fd1ef26d04bf1880cf624ae83c8c2ec0ad9f2f071f1b9f3a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::significant_figures
---
# Decimal::significant_figures

指定した最大有効桁数へ丸めます。

```rust
fn significant_figures (& self , figures : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

figures: 正の isize/整数 number。mode は丸め規則です。小数点以下の桁数とは異なり、有効数字は係数の桁を数えます。精度を追加することはなく、整数部分も丸める場合があります（有効数字 3 桁で 1234.5 → 1230）。figures が非正なら InvalidArgument、Unnecessary で丸めが必要なら RoundUnnecessary を返します。係数の桁上がりの可能性をブラウザーの上限と事前に照合します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-significant-figures" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
