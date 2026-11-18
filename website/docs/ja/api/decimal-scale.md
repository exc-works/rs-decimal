---
locale: ja
source_revision: c63b4582e5ea4e7eac387ce70eb29aed40ab3d4cb5d5c76ae95a6cd8fa1f8f69
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::scale
---
# Decimal::scale

保存された非負の小数スケールを読み取ります。

```rust
fn scale (& self) -> usize
```

## 引数、戻り値、使用上の注意

playground のスケール上限内で、usize を正確な number として返します。precision() と同じ値です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
