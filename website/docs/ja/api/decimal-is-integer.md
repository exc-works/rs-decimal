---
locale: ja
source_revision: 46411f1a8f7889f0ec3654fae25da003c49b8d268105e0431ed703cb61c031fe
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_integer
---
# Decimal::is_integer

数値に小数部分があるか調べます。

```rust
fn is_integer (& self) -> bool
```

## 引数、戻り値、通常の使い方

引数なし。bool/boolean を返します。スケールが 0 かどうかではなく数値を調べるため、12.00 は整数です。既定のゼロも整数で、小数部分はありません。これらの問い合わせは入力を初期化も変更もしません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
