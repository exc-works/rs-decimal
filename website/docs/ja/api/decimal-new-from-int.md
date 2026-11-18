---
locale: ja
source_revision: aad24e4a42d458d995751545010904db4f3e9453abb0fa3c66514c8c59b55494
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_int
---
# Decimal::new_from_int

ネイティブ幅の符号付き整数から初期化済み整数を作成します。

```rust
fn new_from_int (value : isize) -> Self
```

## 引数と戻り値

`value: isize` → 整数の JavaScript `number`。結果のスケールは 0 です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int" locale="ja" />

## 使用上の注意と制約

ブラウザーのターゲットは wasm32 なので、ネイティブ Rust が 64 ビット環境でも、受け付ける範囲は -2147483648 から 2147483647 です。小数、NaN、無限大は引数エラーになります。

ブラウザーは[リソースポリシー](../resource-policy)も適用します。JavaScript 入力は Rust の呼び出し前に検証します。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
