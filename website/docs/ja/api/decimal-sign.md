---
locale: ja
source_revision: 828ccfc55974951d7ced64d269a1959b7a45efe557a7f4202f79dcfc3632ea3f
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::sign
---
# Decimal::sign

符号を -1、0、1 として読み取ります。

```rust
fn sign (& self) -> i32
```

## 引数、戻り値、使用上の注意

i32/number を返します。初期化済みと未初期化のゼロはいずれも符号 0 で、負のゼロの符号は区別しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
