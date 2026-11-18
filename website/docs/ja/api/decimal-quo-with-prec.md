---
locale: ja
source_revision: 240d5cd6140ed74d787cde34f7f3b1ea17105d751eeea756af72039c1ea346ce
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

明示的な精度、または入力から決まる精度で小数を除算します。

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<Decimal, DecimalError> を返します。prec は非負の小数点以下の桁数です。負の精度は除数の検査前に InvalidPrecision を返します。除数がゼロなら DivideByZero です。mode が丸めを制御し、Unnecessary で不正確な結果になる場合は RoundUnnecessary を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
