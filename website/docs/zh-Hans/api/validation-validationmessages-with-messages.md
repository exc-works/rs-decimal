---
locale: zh-Hans
source_revision: 71b55f4d77a06034a780155a4f624e277f9df950e286fd15014d7cc9e03ee581
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

覆盖英文验证模板。

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

messages: `BTreeMap<String,String>`/字符串值对象；返回 `Result<ValidationMessages, DecimalError>`/消息句柄。只覆盖提供的键，其他规则保留英文默认值。键限于十二种 decimal_* 规则。空模板、未知键、不匹配花括号及 {0}/{1} 以外的占位符返回 InvalidArgument。允许重复有效占位符。不改变全局或已构造的模板。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
