---
locale: zh-Hant
source_revision: 71b55f4d77a06034a780155a4f624e277f9df950e286fd15014d7cc9e03ee581
translation_provenance: zh-Hans-terminology-reviewed
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

覆蓋英文驗證模板。

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

messages: `BTreeMap<String,String>`/字串值物件；回傳 `Result<ValidationMessages, DecimalError>`/訊息控制代碼。只覆蓋提供的鍵，其他規則保留英文預設值。鍵限於十二種 decimal_* 規則。空模板、未知鍵、不匹配花括號及 {0}/{1} 以外的預留位置回傳 InvalidArgument。允許重複有效預留位置。不改變全域或已建構的模板。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="zh-Hant" />

## 相關方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
