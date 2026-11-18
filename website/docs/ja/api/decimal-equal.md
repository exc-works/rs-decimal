---
locale: ja
source_revision: 6d86e4a4d53be90de2e3c79ca0231980d419a71194ef67f300c06ca432f017c2
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::equal
---
# Decimal::equal

2 つの小数が同じ数値を表すか調べます。

```rust
fn equal (& self , other : & Self) -> bool
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、bool/boolean を返します。スケールと初期化状態は無視します。JavaScript の === はラッパーの同一性を比較します。Rust の数値としての等価性には equal() を使ってください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
