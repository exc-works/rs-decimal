---
locale: zh-Hans
source_revision: 92b27b2e44b2095f2d604aead4c4f01fb922094a0083fe1c3c5e2509eaa4b3d7
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

按规则验证一个字段。

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## 参数、返回值及常规用法

field: &str/string，value: &Decimal/Decimal；返回 `Result<(), ValidationError>`。JS 成功返回 undefined；失败抛出 ValidationError，带 category="validation" 及 field/tag/parameter 属性，区别于 DecimalError。保留字段名和原参数文本，包括花括号与空白。借用规则及数值。需要无异常收集多个错误时使用 validate_decimal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="zh-Hans" />

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
