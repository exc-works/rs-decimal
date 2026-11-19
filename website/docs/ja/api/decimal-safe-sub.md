---
locale: ja
source_revision: b834c8dbe54adc1326f4715bae0b5a7e4a551bc4817e951238d3a76cb1960aa5
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

小数を正確に減算し、結果が非負であることを要求します。

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。両オペランドの大きい方のスケールで、新しい Decimal を返します。桁揃えは正確で、桁を捨てません。Rust は Result を返します。非負である必要があるのは結果だけで、各オペランドではありません。結果が負なら InvalidArgument、ゼロは許可されます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
