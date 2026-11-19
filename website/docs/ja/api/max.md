---
locale: ja
source_revision: f49d2dc4c074cf4f52b2a3a197eb1d13f1144942ab8a1df7ea4bf51caaaa350f
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

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
