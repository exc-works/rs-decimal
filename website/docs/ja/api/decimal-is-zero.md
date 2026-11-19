---
locale: ja
source_revision: 3a7d890770468c269032ca95ee3365a1f5fdeb70051590bf674c733d1e1658d6
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

値に依存する操作の前に、数値がゼロか調べます。

```rust
fn is_zero (& self) -> bool
```

## 引数、戻り値、使用上の注意

bool/boolean を返し、未初期化の既定値にも true を返します。初期化状態が重要なら is_nil() を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
