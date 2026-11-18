---
locale: ja
source_revision: 2558a21d5d46dd2598e1fc940985c24d9012b531617d6e30f108a6afe7d17531
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_int64
---
# Decimal::new_from_int64

符号付き 64 ビット係数と小数点以下の桁数から小数を作成します。数値と状態の動作は `new_with_prec` と同じです。

```rust
fn new_from_int64 (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 引数と戻り値

`value: i64` → `bigint`、`prec: isize` → 整数の `number`。`value × 10^-prec` を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int64" locale="ja" />

## 使用上の注意と制約

値は係数であり、スケール適用済みの金額ではありません。負の精度は `InvalidPrecision`、i64 範囲外の値は bridge の引数エラーです。

ブラウザーは[リソースポリシー](../resource-policy)も適用します。JavaScript 入力は Rust の呼び出し前に検証します。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
