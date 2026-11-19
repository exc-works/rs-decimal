---
locale: ja
source_revision: eafc21e4a88e29bea4168f39bfe3b5db899f2a28bccbd47f88733ffdaab27a0c
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::min
---
# Decimal::min

2 つの数値の小さい方をコピーします。

```rust
fn min (& self , other : & Self) -> Self
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、独立した Decimal を返します。数値が等しい場合は最初のオペランドのスケールと初期化状態を保持します。Rust では Decimal::min(&left, &right) を使い、値を消費する Ord::min トレイトメソッドの選択を避けてください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
