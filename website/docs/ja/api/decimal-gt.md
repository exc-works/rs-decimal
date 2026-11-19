---
locale: ja
source_revision: f3c9953fc7c2ab0c097e714a9d8217e98239ea7e625d7a02d8c88afee82038af
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::gt
---
# Decimal::gt

この値が他の値より大きいか調べます。

```rust
fn gt (& self , other : & Self) -> bool
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、bool/boolean を返します。保存されたスケールが異なっても、数値が等しければ false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gt" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
