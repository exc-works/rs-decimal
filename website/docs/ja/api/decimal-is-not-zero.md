---
locale: ja
source_revision: 19e13f576512aa6ef6c12c54900338d87cb485da07e25d15d575e4c5353b491a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_not_zero
---
# Decimal::is_not_zero

数値がゼロと異なるか調べます。

```rust
fn is_not_zero (& self) -> bool
```

## 引数、戻り値、使用上の注意

is_zero() の反対の bool/boolean を返します。未初期化の既定値には false を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-not-zero" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
