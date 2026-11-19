---
locale: ja
source_revision: 521743c60e4285630a1e202d92aebd0d0b9f4764a73c56a223f9ec0c5662a6d6
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

失敗した検証レコードを確認します。

```rust
pub struct ValidationError
```

## 引数、戻り値、通常の使い方

公開プロパティ field、tag、parameter を持つ失敗レコードです。ValidationRule.validate は失敗時にこれを返し、validate_decimal はリストを返します。JavaScript は翻訳に使える編集可能なフィールドレコードを公開します。DecimalError の種類や例外メッセージではありません。レコードを編集しても規則や小数は変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
