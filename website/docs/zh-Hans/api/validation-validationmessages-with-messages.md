---
locale: zh-Hans
source_revision: 4973633c1cdc3ddfb696d250ca871028b9835fa4187520f393245e4690f6a0c3
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

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
