---
locale: ja
source_revision: aa764a04d58bbac840fe64bca919cb70175f064a9010f9a840a1f56ffacabc1c
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

値に依存する操作の前に、数値がゼロか調べます。

```rust
fn is_zero (& self) -> bool
```

## 引数、戻り値、使用上の注意

bool/boolean を返し、未初期化の既定値にも true を返します。初期化状態が重要なら is_nil() を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
