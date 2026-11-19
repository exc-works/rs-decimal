---
locale: ja
source_revision: 0d6262b2eb7c6955f187e4d63b88d16a39d035f514ca55c738d74b10a8d69203
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::quo
---
# Decimal::quo

明示的な精度、または入力から決まる精度で小数を除算します。

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<Decimal, DecimalError> を返します。出力スケールは両オペランドの大きい方です。そのため両入力が整数なら結果も整数となり、1 / 3 は Down では 0、Up では 1 です。小数の出力には quo_with_prec を使います。除数がゼロなら DivideByZero を返します。mode が丸めを制御し、Unnecessary で不正確な結果になる場合は RoundUnnecessary を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
