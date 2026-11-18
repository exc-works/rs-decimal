---
locale: ja
source_revision: 19a39638a4734aeba49de6f5af4a9f79e89b842f7b78ba2cf7aeb511f0ebcd6c
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

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
