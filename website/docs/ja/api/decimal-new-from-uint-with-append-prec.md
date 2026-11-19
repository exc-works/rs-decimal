---
locale: ja
source_revision: 276b5ccc2b5a962810bbd0b08cd40698f00e48d79a517b96783f1d6cae65ea86
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_uint_with_append_prec
---
# Decimal::new_from_uint_with_append_prec

符号なし整数の数値を保ちながら、保存する小数点以下の桁を追加します。

```rust
fn new_from_uint_with_append_prec (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## 引数と戻り値

`value: u64` → 非負の `bigint`、`prec: isize` → 整数の `number`。結果はスケール `prec` で整数の数値を保持します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint-with-append-prec" locale="ja" />

## 使い方の補足

負の値や範囲外の値は bridge の引数エラーです。負の精度は `InvalidPrecision` になり、大きな精度は別途 playground のリソース制限に達します。末尾のゼロを保持する出力で追加された桁を確認できます。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
