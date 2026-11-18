---
locale: ja
source_revision: 60ca5295cf77ea0e6cccdadb5d937c5adbdd69051a5dbac42387fec9ee90828c
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::lte
---
# Decimal::lte

この値が他の値以下か調べます。

```rust
fn lte (& self , other : & Self) -> bool
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、bool/boolean を返します。等価性は数値で判定し、スケールや初期化状態には依存しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
