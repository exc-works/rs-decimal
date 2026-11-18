---
locale: ja
source_revision: ce941de7d4ef355a538d2c2c33a609ab4eeb3b4645eb801e81a24755fcf0ca5a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

指定した出力精度で小数を乗算します。

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。結果のスケールは prec 桁で、非負である必要があります。負の prec は InvalidPrecision を返します。明示的な mode が丸めを制御し、Unnecessary でゼロ以外の桁を捨てる必要がある場合は RoundUnnecessary を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
