---
locale: ja
source_revision: 9ffad200b8eee0be119e53ec6189311fb6b54cada31774d04149bdc2b868ac4e
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_negative
---
# Decimal::is_negative

数値が厳密に負か調べます。

```rust
fn is_negative (& self) -> bool
```

## 引数、戻り値、使用上の注意

bool/boolean を返します。未初期化の既定値を含むゼロには false を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
