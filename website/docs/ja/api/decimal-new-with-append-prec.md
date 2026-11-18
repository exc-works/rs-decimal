---
locale: ja
source_revision: 7e14f36bf17f5e06d930d4aefd01137eff4589a325eb7eee0237e89f396a3ba8
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

## 使用上の注意と制約

`string()` は 12、`string_with_trailing_zeros()` は 12.00 を出力します。`new_with_prec` とは異なり、数値は 0.12 になりません。負の精度は `InvalidPrecision` です。大きな要求は 10 の累乗を確保する前に拒否します。

ブラウザーは[リソースポリシー](../resource-policy)も適用します。JavaScript 入力は Rust の呼び出し前に検証します。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
