---
locale: ja
source_revision: 1933adb169124e0ed9d429b21c95687e4fd59240647c4b0f684ca7bc76aabd2b
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

スケール適用前の正確な整数係数をコピーします。

```rust
fn big_int (& self) -> BigInt
```

## 引数、戻り値、使用上の注意

Rust は独立した BigInt、bridge は bigint を返します。切り捨てる整数変換とは異なり、末尾のゼロを含みます。例では JSON のため bigint をテキストとして表示します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
