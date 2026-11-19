---
locale: ja
source_revision: 0b44060089288355d3eba23edfcc95ed4174aef13953ff7dc6b74df01f566b11
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationError.tag"
---
# validation::ValidationError.tag

失敗した検証レコードを確認します。

```rust
tag: & 'static str
```

## 引数、戻り値、通常の使い方

失敗した規則を識別する公開 Rust 静的文字列です。ブラウザーでは所有する文字列プロパティです。不明なタグの翻訳ではコアの既定テンプレート "{0} is invalid" を使います。エラーレコードを編集しても規則や小数は変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-tag" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
