---
locale: ja
source_revision: 7f1c2832917badced6e78a9c287520234452b297109e8406a028c3703581c859
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationError::impl PartialEq"
---
# validation::ValidationError::impl PartialEq

Rust の PartialEq 実装を使います。

```rust
impl PartialEq for validation::ValidationError
```

## 引数、戻り値、通常の使い方

ValidationError の等価性は翻訳済みメッセージだけでなく、field、tag、元の parameter をまとめて比較します。これはネイティブ Rust のトレイト構文であり、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
