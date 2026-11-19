---
locale: ja
source_revision: 2b24eac398b765662bae8967979a2547332ee566c8d7e9c0bb785ba0d1f9e478
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationMessages::new"
---
# validation::ValidationMessages::new

所有するテンプレートで検証失敗を翻訳します。

```rust
fn new (locale : & str) -> Self
```

## 引数、戻り値、通常の使い方

locale: &str/string。新しい所有メッセージテンプレートを返します。ロケール照合は大文字小文字を区別せず、ハイフン／アンダースコアと言語・地域の接尾辞を受け付けます。zh-Hant/zh-TW は繁体字中国語、en-US/en-GB は英語を選び、不明なロケールは英語へ戻ります。{0} にフィールド、{1} に元の引数を 1 回の処理で挿入し、挿入した波括弧は再展開しません。結果はプレーンテキストです。HTML ではなくテキストとして表示してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-new" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
