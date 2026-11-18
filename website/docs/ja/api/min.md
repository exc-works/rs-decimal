---
locale: ja
source_revision: 9a05736a78d12045af2704500028fdc4fdec0490c797ca6d8ba9130b66fbc153
translation_provenance: en-US-japanese-reviewed
api_id: min
---
# min

自由関数を使って 2 つの小数の小さい方をコピーします。

```rust
fn min (a : & Decimal , b : & Decimal) -> Decimal
```

## 引数、戻り値、使用上の注意

a と b を &Decimal として借用し、独立した Decimal を返します。数値が等しい場合は a のスケールと初期化状態を保持します。Decimal::min と同じライブラリの意味規則を使用します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-min" locale="ja" />

## 制約

bridge は Rust の名前と引数の順序を保持します。自由関数は playground で直接使用できます。借用入力を消費したり変更したりせず、不正なハンドルや別の実行のハンドルは拒否します。すべての操作は[リソースポリシー](../resource-policy)に従います。

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
