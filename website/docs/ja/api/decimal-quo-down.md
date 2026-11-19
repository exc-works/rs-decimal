---
locale: ja
source_revision: 24cee5102dc473cb8fed8902e9829dd352863649ca89859b6c19bb3f001aa710
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::quo_down
---
# Decimal::quo_down

明示的な精度、または入力から決まる精度で小数を除算します。

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<Decimal, DecimalError> を返します。出力スケールは両オペランドの大きい方です。そのため両入力が整数なら結果も整数となり、1 / 3 は 0 です。小数の出力には quo_with_prec を使います。除数がゼロなら DivideByZero を返します。Down は負の無限大ではなくゼロ方向に切り捨てます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
