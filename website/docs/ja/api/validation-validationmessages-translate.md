---
locale: ja
source_revision: d21e2ccb9059c66b32b65cffe1a9b447ab9a7fc0664cf5fa020b0d334fe294fd
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationMessages::translate"
---
# validation::ValidationMessages::translate

所有するテンプレートで検証失敗を翻訳します。

```rust
fn translate (& self , errors : & [ValidationError]) -> Vec < String >
```

## 引数、戻り値、通常の使い方

errors: &[ValidationError]/配列。同じ順序で文字列配列を返します。ロケール照合は大文字小文字を区別せず、ハイフン／アンダースコアと言語・地域の接尾辞を受け付けます。zh-Hant/zh-TW は繁体字中国語、en-US/en-GB は英語を選び、不明なロケールは英語へ戻ります。{0} にフィールド、{1} に元の引数を 1 回の処理で挿入し、挿入した波括弧は再展開しません。結果はプレーンテキストです。HTML ではなくテキストとして表示してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-translate" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
