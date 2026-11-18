---
locale: ja
source_revision: 4488da41f235921047e8f85b8a68b47bc497bf199529326773333d8f22d38585
translation_provenance: en-US-japanese-reviewed
api_id: between
---
# between

小数が両端を含む数値範囲内にあるか調べます。

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## 引数、戻り値、使用上の注意

v、lower、upper を &Decimal として借用し、lower <= v <= upper の bool/boolean を返します。等価性は表現の違いを無視します。境界は並べ替えないため、lower > upper では false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="ja" />

## 制約

bridge は Rust の名前と引数の順序を保持します。自由関数は playground で直接使用できます。借用入力を消費したり変更したりせず、不正なハンドルや別の実行のハンドルは拒否します。すべての操作は[リソースポリシー](../resource-policy)に従います。

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
