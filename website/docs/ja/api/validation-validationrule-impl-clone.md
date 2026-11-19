---
locale: ja
source_revision: b55dba0922642cda4e225685c47d97a54da01243942c47cb892b8c9136202f1a
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationRule::impl Clone"
---
# validation::ValidationRule::impl Clone

Rust の Clone 実装を使います。

```rust
impl Clone for validation::ValidationRule
```

## 引数、戻り値、通常の使い方

Clone は規則全体、テンプレートマップ、または失敗フィールドをコピーします。コピーは独立して所有されます。これはネイティブ Rust のトレイト構文であり、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-clone" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
