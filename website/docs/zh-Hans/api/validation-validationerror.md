---
locale: zh-Hans
source_revision: 377152e0483ddcbebcd02f161fe043c129efdc1f915e3eb70b399c28e3d3906b
api_id: "validation::ValidationError"
---
# validation::ValidationError

查看失败验证记录。

```rust
pub struct ValidationError
```

## 参数、返回值及常规用法

失败记录包含公开 field、tag、parameter 属性。ValidationRule.validate 失败时返回，validate_decimal 返回其列表。JS 提供可编辑字段记录，用于翻译。它们不是 DecimalError 类别或异常消息。修改错误记录不会改变规则或小数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="zh-Hans" />

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
