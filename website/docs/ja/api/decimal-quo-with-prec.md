---
locale: ja
source_revision: eb9bcc338f948ddd5ec14bfd00bb5bb38695bb5e5eba9b781fe855d89569b0a0
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

明示的な精度、または入力から決まる精度で小数を除算します。

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<Decimal, DecimalError> を返します。prec は非負の小数点以下の桁数です。負の精度は除数の検査前に InvalidPrecision を返します。除数がゼロなら DivideByZero です。mode が丸めを制御し、Unnecessary で不正確な結果になる場合は RoundUnnecessary を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
