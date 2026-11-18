---
locale: ja
source_revision: d543ee5c1ea405eaf1deafc27267bde1bfdd8a1593aa6059be6405a03110ee1f
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

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
