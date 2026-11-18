---
locale: ja
source_revision: 9c252d1ff038e1dac4451e4dd728020f00ea8f88567265b756b2f63f4474b26e
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

非負であることを要求し、初期化済みのコピーを返します。

```rust
fn must_non_negative (& self) -> Self
```

## 引数、戻り値、通常の使い方

引数なし。ゼロを含む非負の入力では Decimal を返します。負の入力では Rust は文字列ペイロードでパニックします。Wasm では回復可能な DecimalError ではなくトラップとなり、playground の実行が終了します。新しい実行で Wasm の状態を作り直してください。これは数値の表明であり、絶対値への変換ではありません。符号の変更には abs を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
