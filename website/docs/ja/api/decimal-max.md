---
locale: ja
source_revision: 5956431328357316a87f05c2d1543bb100b12923bb2d50b1e21ff3f96212db45
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::max
---
# Decimal::max

2 つの数値の大きい方をコピーします。

```rust
fn max (& self , other : & Self) -> Self
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、独立した Decimal を返します。数値が等しい場合は最初のオペランドのスケールと初期化状態を保持します。Rust では Decimal::max(&left, &right) でこの借用する固有メソッドを明示的に選択してください。Ord::max は所有権を取得します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
