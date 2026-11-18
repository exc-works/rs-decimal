---
locale: ja
source_revision: 92b27b2e44b2095f2d604aead4c4f01fb922094a0083fe1c3c5e2509eaa4b3d7
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

1 つのフィールドを規則で検証します。

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## 引数、戻り値、通常の使い方

field: &str/string、value: &Decimal/Decimal。`Result<(), ValidationError>` を返します。JavaScript の成功時は undefined です。失敗時は category="validation" と field/tag/parameter プロパティを持つ JS ValidationError を送出し、DecimalError とは区別します。フィールドと元の引数テキストは波括弧や空白も含め保持します。規則と値は借用します。例外を送出せず複数の失敗を集めるには validate_decimal を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="ja" />

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
