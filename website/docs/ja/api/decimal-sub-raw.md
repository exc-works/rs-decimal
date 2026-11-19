---
locale: ja
source_revision: 31042d691902e2dc4bdadd92d4688638c315fb08be40c79d11b031b854afc6bc
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

スケール適用前の係数を i64 整数分だけ変更します。

```rust
fn sub_raw (& self , value : i64) -> Self
```

## 引数、戻り値、通常の使い方

value: i64。bridge では bigint で表します。スケールは保持するため、スケール 2 で 1 を加減すると数値は 1 ではなく 0.01 変わります。Rust は新しい Decimal を返します。number と i64 範囲外の bigint は bridge が拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="ja" />

## 関連メソッド

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
