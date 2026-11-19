---
locale: ja
source_revision: 9937fd6977e8a466e16b6a35082706b1183e93c9c873b5eb8e6c66d5ab917b4f
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationMessages"
---
# validation::ValidationMessages

所有するテンプレートで検証失敗を翻訳します。

```rust
pub struct ValidationMessages
```

## 引数、戻り値、通常の使い方

ValidationError レコードの翻訳用テンプレートを所有します。new(locale) または with_messages(map) を使います。別のテンプレートマップの変更はこのハンドルに影響しません。ロケール照合は大文字小文字を区別せず、ハイフン／アンダースコアと言語・地域の接尾辞を受け付けます。zh-Hant/zh-TW は繁体字中国語、en-US/en-GB は英語を選び、不明なロケールは英語へ戻ります。{0} にフィールド、{1} に元の引数を 1 回の処理で挿入し、挿入した波括弧は再展開しません。結果はプレーンテキストです。HTML ではなくテキストとして表示してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
