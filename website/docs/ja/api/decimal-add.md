---
locale: ja
source_revision: 8c45e8f4230346a7c6fcd3cbb920d429b7a02acbcd8eb534c1f096fa8233828d
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::add
---
# Decimal::add

小数を正確に加算します。

```rust
fn add (& self , other : & Self) -> Self
```

## 引数、戻り値、通常の使い方

other: &Decimal。両オペランドの大きい方のスケールで、新しい Decimal を返します。桁揃えは正確で、桁を捨てません。Rust メソッドは失敗しませんが、ブラウザーの割り当て制限によってリソースエラーになることがあります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
