---
locale: ja
source_revision: 0da87c25bd0ca11269721d2b8d815d7bac612f73d7110e87ce497372ebbaf01e
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationError::impl Debug"
---
# validation::ValidationError::impl Debug

Rust の Debug 実装を使います。

```rust
impl Debug for validation::ValidationError
```

## 引数、戻り値、通常の使い方

Debug は診断用テキストを出力し、検証結果の安定したシリアライズ形式ではありません。これはネイティブ Rust のトレイト構文であり、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-debug" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
