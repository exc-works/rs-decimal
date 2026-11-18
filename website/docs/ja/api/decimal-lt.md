---
locale: ja
source_revision: fe8ae2df48c06b20c3ac1c7bbd224d12734d4193d035a1a96ed7e524e49fd64b
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::lt
---
# Decimal::lt

この値が他の値より小さいか調べます。

```rust
fn lt (& self , other : & Self) -> bool
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、bool/boolean を返します。保存されたスケールが異なっても、数値が等しければ false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
