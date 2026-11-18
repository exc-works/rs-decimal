---
locale: ja
source_revision: 0ee4a72ff112d52d1fe74af33099195201a24147830d9b414e5d9feccaa1d9af
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationMessages::impl Debug"
---
# validation::ValidationMessages::impl Debug

Rust の Debug 実装を使います。

```rust
impl Debug for validation::ValidationMessages
```

## 引数、戻り値、通常の使い方

Debug は診断用テキストを出力し、検証結果の安定したシリアライズ形式ではありません。これはネイティブ Rust のトレイト構文であり、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-impl-debug" locale="ja" />

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
