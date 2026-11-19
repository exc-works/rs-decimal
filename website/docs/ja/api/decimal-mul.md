---
locale: ja
source_revision: adde9dcc9ce010498ad91d2410621453cfaa363aae0ee4c3d4598482bb6eead0
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::mul
---
# Decimal::mul

指定した出力精度で小数を乗算します。

```rust
fn mul (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。結果のスケールは両オペランドの大きい方であり、合計ではありません。積の全桁を保持するには mul_exact を使います。明示的な mode が丸めを制御し、Unnecessary でゼロ以外の桁を捨てる必要がある場合は RoundUnnecessary を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
