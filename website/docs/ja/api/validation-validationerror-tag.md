---
locale: ja
source_revision: 6fae0dc88a563ce1dfaf67cbb1ece08366df9aad8969e9320dabf2cf06fa5933
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

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
