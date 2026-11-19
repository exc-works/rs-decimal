---
locale: ja
source_revision: b2363400afff1e4c740938a16816a148faaff0faef10b304c45a776ef2dcc1ee
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

指定した出力精度で小数を乗算します。

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。結果のスケールは prec 桁で、非負である必要があります。負の prec は InvalidPrecision を返します。明示的な mode が丸めを制御し、Unnecessary でゼロ以外の桁を捨てる必要がある場合は RoundUnnecessary を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
