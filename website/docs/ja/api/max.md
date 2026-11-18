---
locale: ja
source_revision: 0d4a74db932ab126e7566fb630352abba8c528142a3d37c615a2694b7fac9c0a
translation_provenance: en-US-japanese-reviewed
api_id: max
---
# max

自由関数を使って 2 つの小数の大きい方をコピーします。

```rust
fn max (a : & Decimal , b : & Decimal) -> Decimal
```

## 引数、戻り値、使用上の注意

a と b を &Decimal として借用し、独立した Decimal を返します。数値が等しい場合は a のスケールと初期化状態を保持します。Decimal::max と同じライブラリの意味規則を使用します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-max" locale="ja" />

## 制約

bridge は Rust の名前と引数の順序を保持します。自由関数は playground で直接使用できます。借用入力を消費したり変更したりせず、不正なハンドルや別の実行のハンドルは拒否します。すべての操作は[リソースポリシー](../resource-policy)に従います。

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
