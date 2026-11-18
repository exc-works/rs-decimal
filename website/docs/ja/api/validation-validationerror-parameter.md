---
locale: ja
source_revision: 0e561c16634157ba3586cee74da3c90f15491d42428e944428a8ee980c8ddeb4
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

失敗した検証レコードを確認します。

```rust
parameter: String
```

## 引数、戻り値、通常の使い方

空白も含め元の規則引数を正確に保持する公開の所有 String/string です。{1} として 1 回だけ置換し、テンプレートを再帰的には解釈しません。エラーレコードを編集しても規則や小数は変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="ja" />

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
