---
locale: ja
source_revision: 356d7409ac746daa69113ece073c46b80c7b34815cbe6ddfc49fc1c80e686226
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

失敗した検証レコードを確認します。

```rust
field: String
```

## 引数、戻り値、通常の使い方

呼び出し側が指定したフィールド名を保持する公開の所有 String/string です。プロパティ参照パスでも HTML 断片でもありません。翻訳時に波括弧はそのまま挿入します。エラーレコードを編集しても規則や小数は変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
