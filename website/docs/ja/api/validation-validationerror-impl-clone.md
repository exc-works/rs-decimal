---
locale: ja
source_revision: 262ffddf12b7585e3d9a91337def5d4995343306864bd291c3fadc2686ec5701
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationError::impl Clone"
---
# validation::ValidationError::impl Clone

Rust の Clone 実装を使います。

```rust
impl Clone for validation::ValidationError
```

## 引数、戻り値、通常の使い方

Clone は規則全体、テンプレートマップ、または失敗フィールドをコピーします。コピーは独立して所有されます。これはネイティブ Rust のトレイト構文であり、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-clone" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
