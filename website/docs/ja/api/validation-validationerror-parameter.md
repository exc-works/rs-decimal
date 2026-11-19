---
locale: ja
source_revision: 9d8774ce76acc603ce438d5ebe120fafe780695ef55b74ea55b1fbe183940981
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

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
