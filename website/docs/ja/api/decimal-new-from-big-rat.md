---
locale: ja
source_revision: ffab3b21dcd7c80e0222f2cd0457116e79f56aafc95fc2ed3c796e423b9661cc
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

有理数を正確な有限小数へ変換します。

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

value: &BigRational。JavaScript は {numerator: bigint, denominator: bigint} を受け取ります。分母はゼロ以外である必要があり、ゼロなら JavaScript の引数エラーです。Rust の BigRational は符号と公約数を正規化します。約分後の分母の素因数が 2 と 5 のみの場合に有限小数になり、それ以外の因数は InvalidFormat です。結果は初期化済みです。ブラウザーの検査は約分後に適用し、コア変換の前に出力スケールと係数増大を制限します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="ja" />

## 関連メソッド

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
