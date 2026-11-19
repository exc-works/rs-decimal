---
locale: ja
source_revision: 663e2e4ad6a9b90feffe429f81f13e798f3070cf1d9ea7de1079b0dc756cf5d2
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_negative
---
# Decimal::is_negative

数値が厳密に負か調べます。

```rust
fn is_negative (& self) -> bool
```

## 引数、戻り値、使用上の注意

bool/boolean を返します。未初期化の既定値を含むゼロには false を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
