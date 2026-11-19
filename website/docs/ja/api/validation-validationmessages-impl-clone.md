---
locale: ja
source_revision: 572356860c801a3a257544cc180e1dca7bec96afecfd793ab3dbbcce432242dd
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationMessages::impl Clone"
---
# validation::ValidationMessages::impl Clone

Rust の Clone 実装を使います。

```rust
impl Clone for validation::ValidationMessages
```

## 引数、戻り値、通常の使い方

Clone は規則全体、テンプレートマップ、または失敗フィールドをコピーします。コピーは独立して所有されます。これはネイティブ Rust のトレイト構文であり、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-impl-clone" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
