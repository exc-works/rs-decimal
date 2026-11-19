---
locale: ja
source_revision: 49724190e7f35b58dc598adc543c9f85eecedcc93db53ee4b759bbb126244282
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationRule::impl Debug"
---
# validation::ValidationRule::impl Debug

Rust の Debug 実装を使います。

```rust
impl Debug for validation::ValidationRule
```

## 引数、戻り値、通常の使い方

Debug は診断用テキストを出力し、検証結果の安定したシリアライズ形式ではありません。これはネイティブ Rust のトレイト構文であり、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-debug" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
