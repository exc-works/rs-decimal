---
locale: ja
source_revision: 536eae84e3e9c7c0c702cbcb339f82c64bf0aafab561d4a3f87590939cb52d8e
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

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
