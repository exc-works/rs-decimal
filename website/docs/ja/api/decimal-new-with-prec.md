---
locale: ja
source_revision: 2cba3a645b36c53756ae013b43e89956dee5ee0f736cbeccecbeb94e8966c7b4
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_with_prec
---
# Decimal::new_with_prec

符号付き整数を、指定した小数点以下の桁数に対応する係数として解釈します。例えば 12 と精度 2 は 0.12 を意味します。

```rust
fn new_with_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 引数と戻り値

`value: i64` → `bigint`、`prec: isize` → 整数の `number`。`value × 10^-prec` を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-prec" locale="ja" />

## 使い方の補足

精度は有効数字ではなく、小数点以下の桁数です。整数の数値を保ってゼロを追加するものではありません。12.00 には `new_with_append_prec` を使います。負の精度は `InvalidPrecision` です。係数がゼロでも指定したスケールを保持します。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
