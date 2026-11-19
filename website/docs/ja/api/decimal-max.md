---
locale: ja
source_revision: c7a338ce433accfeb61051f993009577dfae7589a629a17bae6043234f27ca38
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

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
