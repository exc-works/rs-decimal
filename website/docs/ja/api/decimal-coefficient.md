---
locale: ja
source_revision: b70b6d632eef5a013491ea47ed27e9d8eea36fee70b75cd0db8637b8b6a6ccc5
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

スケール適用前の正確な整数係数を確認します。

```rust
fn coefficient (& self) -> & BigInt
```

## 引数、戻り値、使用上の注意

Rust は &BigInt を借用します。bridge は不変の bigint スナップショットを返し、変更を追跡する参照ではありません。係数には末尾のゼロが含まれ、小数の整数部分とは異なります。例では JSON の全桁を保つため bigint をテキストとして表示します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
