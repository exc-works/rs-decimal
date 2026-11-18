---
locale: ja
source_revision: 9743ba1e89051026e818a070b86183de419a65ca646afc51459afa17731968b9
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

ゼロ方向に切り捨てて整数部分を取り出します。

```rust
fn int_part (& self) -> BigInt
```

## 引数、戻り値、通常の使い方

引数なし。Rust は BigInt、bridge は bigint を返し、浮動小数点変換は行いません。-12.75 の結果は -12 であり、floor(-12.75) = -13 ではありません。既定のゼロはゼロを返します。例では JSON でも任意精度を保つため整数を文字列として出力します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
