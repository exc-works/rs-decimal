---
locale: ja
source_revision: 7d029f4b4a98b484b183c91199768a9635c9e85851c7f98513e0ef2565e4aa2e
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

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
