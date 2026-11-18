---
locale: ja
source_revision: 65b82f1df9828e1e76dbeae405f27c44161cfdcb090640f8812099c14a0bd130
translation_provenance: en-US-japanese-reviewed
api_id: "validation::validate_decimal"
---
# validation::validate_decimal

規則の失敗を順序どおりに収集して翻訳します。

```rust
fn validate_decimal (field : & str , value : & Decimal , rules : & [ValidationRule] ,) -> Vec < ValidationError >
```

## 引数、戻り値、通常の使い方

Rust は validation 機能を必要とし、ブラウザービルドでは有効です。field: &str/string、value: &Decimal/Decimal、rules: &[ValidationRule]/配列。失敗したすべての ValidationError を呼び出し側の順序で返します。規則の重複は失敗の重複につながる場合があります。リフレクション、レジストリー、早期終了は使いません。空の規則は空配列を返します。すべての入力を借用します。フィールド名は呼び出し側のテキストであり、翻訳はリテラルとして置換します。欠落入力を失敗にする必要がある場合は required を明示してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validate-decimal" locale="ja" />

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
