---
locale: ja
source_revision: 845ec62ddce327ca5b797a7a47302c991bc3fbeb2f329134a32bdc0e5d88f53e
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_with_append_prec
---
# Decimal::new_with_append_prec

符号付き整数の数値を保ちながら、小数点以下の桁を追加して保存します。整数の金額を 12.00 のように表現するときに便利です。

```rust
fn new_with_append_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 引数と戻り値

`value: i64` → `bigint`、`prec: isize` → 整数の `number`。保存する係数は `value × 10^prec`、スケールは `prec` になります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-append-prec" locale="ja" />

## 使い方の補足

`string()` は 12、`string_with_trailing_zeros()` は 12.00 を出力します。`new_with_prec` とは異なり、数値は 0.12 になりません。負の精度は `InvalidPrecision` です。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
