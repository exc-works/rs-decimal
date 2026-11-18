---
locale: ja
source_revision: 96feaba7858db9f7e7efab2f2187c38abada064aba6bc3c66092f84f0b94b74f
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::quo_down
---
# Decimal::quo_down

明示的な精度、または入力から決まる精度で小数を除算します。

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<Decimal, DecimalError> を返します。出力スケールは両オペランドの大きい方です。そのため両入力が整数なら結果も整数となり、1 / 3 は 0 です。小数の出力には quo_with_prec を使います。除数がゼロなら DivideByZero を返します。Down は負の無限大ではなくゼロ方向に切り捨てます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
