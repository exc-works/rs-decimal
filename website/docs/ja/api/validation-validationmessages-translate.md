---
locale: ja
source_revision: cba611f08bedeba221d0757fa2de9adc9759708b4a544bb24d89caed0ec075de
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

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
