---
locale: zh-Hans
source_revision: 0e561c16634157ba3586cee74da3c90f15491d42428e944428a8ee980c8ddeb4
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

查看失败验证记录。

```rust
parameter: String
```

## 参数、返回值及常规用法

公开的自有 String/string，精确保留原始规则参数，包括空白。作为 {1} 只替换一次，不递归解释模板。修改错误记录不会改变规则或小数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="zh-Hans" />

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
