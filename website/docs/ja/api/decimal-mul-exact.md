---
locale: ja
source_revision: 5fb085491b7b8fca3cac801efd640150df3603d315a453ad87ad49323fbb68e2
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

小数を正確に乗算します。

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。結果のスケールは両オペランドのスケールの合計で、積の全桁を保持します。コアのスケールオーバーフローは Overflow を返します。丸めモードの引数はありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
