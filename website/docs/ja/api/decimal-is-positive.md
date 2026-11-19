---
locale: ja
source_revision: 2ea2866a9b7493c17d529f3372e5bbb238200663f6cd52e7c63d668cc43d0ffe
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_positive
---
# Decimal::is_positive

数値が厳密に正か調べます。

```rust
fn is_positive (& self) -> bool
```

## 引数、戻り値、使用上の注意

bool/boolean を返します。未初期化の既定値を含むゼロには false を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-positive" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
