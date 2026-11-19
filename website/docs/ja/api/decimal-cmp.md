---
locale: ja
source_revision: 098d219e5e0932c9734847caf4da4f02238ac75ca2e27400d9527165d61283dd
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::cmp
---
# Decimal::cmp

2 つの数値を比較し、3 通りの結果を返します。

```rust
fn cmp (& self , other : & Self) -> i32
```

## 引数、戻り値、使用上の注意

other: &Decimal（JavaScript では Decimal）を借用します。i32/number の -1、0、1 を返します。スケールや初期化状態の違いは数値の等価性に影響しません。これは固有メソッドであり、Ordering を返す Ord::cmp ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
