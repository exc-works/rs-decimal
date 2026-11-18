---
locale: ja
source_revision: 968f32ddfefb50259ee32498127a1ae440b4c11919fa84ceb9ba802a6f23f0ec
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::mul_down
---
# Decimal::mul_down

指定した出力精度で小数を乗算します。

```rust
fn mul_down (& self , other : & Self) -> Self
```

## 引数、戻り値、通常の使い方

other: &Decimal。結果のスケールは両オペランドの大きい方であり、合計ではありません。積の全桁を保持するには mul_exact を使います。Down は負の結果もゼロ方向に切り捨てます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
