---
locale: ja
source_revision: cfe29d83921c9d9fef3ddeed3842336ccc12799d1b0ed231fac999e4d87de7f7
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationError::impl Eq"
---
# validation::ValidationError::impl Eq

Rust の Eq 実装を使います。

```rust
impl Eq for validation::ValidationError
```

## 引数、戻り値、通常の使い方

ValidationError の等価性は翻訳済みメッセージだけでなく、field、tag、元の parameter をまとめて比較します。これはネイティブ Rust のトレイト構文であり、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-eq" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
