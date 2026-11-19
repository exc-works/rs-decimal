---
locale: ja
source_revision: 8ffc267a7c40a9915ad1544633aa5847696df2f114f2a6275b83514da10320eb
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

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
