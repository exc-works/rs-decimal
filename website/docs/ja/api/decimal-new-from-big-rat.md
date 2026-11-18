---
locale: ja
source_revision: c7909226b96ffde911017d8ead0a3f2edd2a0c894ee744bf3bcbea66574f1bf5
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

有理数を正確な有限小数へ変換します。

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

value: &BigRational。bridge は {numerator: bigint, denominator: bigint} を受け取ります。分母はゼロ以外である必要があり、ゼロなら bridge の引数エラーです。Rust の BigRational は符号と公約数を正規化します。約分後の分母の素因数が 2 と 5 のみの場合に有限小数になり、それ以外の因数は InvalidFormat です。結果は初期化済みです。ブラウザーの検査は約分後に適用し、コア変換の前に出力スケールと係数増大を制限します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="ja" />

## ブラウザーの制約

整数の転送は精度を失う JS number を経由しません。変換タプルは Rust の値／フラグの順序を保持します。false は正確な結果を保証しません。借用入力は変わらず、コンストラクターは結果を初期化します。[リソースポリシー](../resource-policy)はライブラリエラーとは別に、整数入力、出力スケール、有理数の係数増大を制限します。Rust のインポート構文はネイティブで検証するものとして明示します。

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
