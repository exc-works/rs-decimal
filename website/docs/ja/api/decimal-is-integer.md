---
locale: ja
source_revision: e73578c50b0181c9f243a7f97c0c8dbdcdf0b57587c7ac98a36dc6043a2ce666
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_integer
---
# Decimal::is_integer

数値に小数部分がないか（整数かどうか）調べます。

```rust
fn is_integer (& self) -> bool
```

## 引数、戻り値、通常の使い方

引数なし。bool/boolean を返します。スケールが 0 かどうかではなく数値を調べるため、12.00 は整数です。既定のゼロも整数で、小数部分はありません。これらの問い合わせは入力を初期化も変更もしません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
