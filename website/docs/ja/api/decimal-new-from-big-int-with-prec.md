---
locale: ja
source_revision: bf618b8d345d150746683bfe2d04e5a166f48e4917e60db9f1d6a2719994d91a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_big_int_with_prec
---
# Decimal::new_from_big_int_with_prec

任意精度の係数をコピーし、小数点以下の桁数を指定します。大きな整数の入力も正確に保持します。

```rust
fn new_from_big_int_with_prec (value : & BigInt , prec : isize) -> Result < Self , DecimalError >
```

## 引数と戻り値

`value: &BigInt` → `bigint`、`prec: isize` → 整数の `number`。Rust 入力を借用し、`value × 10^-prec` を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int-with-prec" locale="ja" />

## 使用上の注意と制約

係数の数字自体ではなく、係数の解釈を変更します。負の精度は `InvalidPrecision` です。playground は入力桁数とスケールを独立に制限します。`resource` エラーは数値精度の損失を意味しません。

ブラウザーは[リソースポリシー](../resource-policy)も適用します。JavaScript 入力は Rust の呼び出し前に検証します。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
