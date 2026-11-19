---
locale: ja
source_revision: 9f58b0f5c6b478fabd100e00297f6dfcdae4ff2a32ead701d8b208c279651c42
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

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
