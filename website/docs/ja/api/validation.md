---
locale: ja
source_revision: 8a30b5196c371952fcc2508159879f3619eaf7085b839003d31c6721b7f9cf86
translation_provenance: en-US-japanese-reviewed
api_id: "validation"
---
# validation

規則の失敗を順序どおりに収集して翻訳します。

```rust
pub mod validation
```

## 引数、戻り値、通常の使い方

Rust は validation 機能を必要とし、ブラウザービルドでは有効です。このモジュールはフレームワークやグローバルレジストリーを使わず、正確な規則評価と所有メッセージテンプレートをまとめます。ブラウザーでは validation 名前空間を使います。フィールド名は呼び出し側のテキストであり、翻訳はリテラルとして置換します。欠落入力を失敗にする必要がある場合は required を明示してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="ja" />

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
