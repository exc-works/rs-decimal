---
locale: ja
source_revision: 71b55f4d77a06034a780155a4f624e277f9df950e286fd15014d7cc9e03ee581
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

英語の検証テンプレートを上書きします。

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

messages: `BTreeMap<String,String>`/文字列値のオブジェクト。`Result<ValidationMessages, DecimalError>`/メッセージハンドルを返します。指定したキーだけを上書きし、他の規則は英語の既定値を保持します。許可されるキーは 12 個の decimal_* 規則です。空のテンプレート、不明なキー、対応しない波括弧、{0}/{1} 以外のプレースホルダーは InvalidArgument です。有効なプレースホルダーの繰り返しは許可します。グローバルなテンプレートや構築済みのテンプレートは変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
