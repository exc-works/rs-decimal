---
locale: ja
source_revision: 3d99b7dec25de936ed85161e3eb8cf4565f56b88649e6c879a6468d051308404
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

指定した小数点以下の桁数に有理数を丸めます。

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

value: &BigRational、prec: 非負の isize、mode: RoundingMode。JavaScript は numerator/denominator の正確な bigint オブジェクトを使います。正確な変換とは異なり、1/3 などの無限小数も許可します。負の精度は InvalidPrecision、Unnecessary で丸めが必要なら RoundUnnecessary を返します。ゼロ分母は Rust の呼び出し前に JavaScript の不正入力として拒否します。ゼロ分子はスケール prec の初期化済みゼロを返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="ja" />

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
