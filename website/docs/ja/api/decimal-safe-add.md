---
locale: ja
source_revision: afcf4f720c241e6c6d4a096ef63464bb4e6b4cfb060705652f6b168cfc059cbe
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::safe_add
---
# Decimal::safe_add

小数を正確に加算し、結果が非負であることを要求します。

```rust
fn safe_add (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。両オペランドの大きい方のスケールで、新しい Decimal を返します。桁揃えは正確で、桁を捨てません。Rust は Result を返します。非負である必要があるのは結果だけで、各オペランドではありません。結果が負なら InvalidArgument、ゼロは許可されます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-add" locale="ja" />

## ブラウザーの制約

入力は借用され、成功後も通常のエラー後も使用できます。返された Decimal は独立したデータを所有し、演算結果は初期化済みです。ブラウザーはコアの呼び出し前に[リソースポリシー](../resource-policy)を確認し、桁揃え、中間積、除算時のスケール調整、出力精度を制限します。保守的な事前検査により、最終結果が小さくても拒否される場合があります。リソースエラーと数値ライブラリのエラーは区別され、パニックは実行を終了します。

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
