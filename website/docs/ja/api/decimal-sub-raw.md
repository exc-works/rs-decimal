---
locale: ja
source_revision: 4d452c3c2c6d2bed5b36a82866f10106b5603e80ad3916a0ac4e55ced6bfc3f3
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

スケール適用前の係数を i64 整数分だけ変更します。

```rust
fn sub_raw (& self , value : i64) -> Self
```

## 引数、戻り値、通常の使い方

value: i64。bridge では bigint で表します。スケールは保持するため、スケール 2 で 1 を加減すると数値は 1 ではなく 0.01 変わります。Rust は新しい Decimal を返します。number と i64 範囲外の bigint は bridge が拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
