---
locale: zh-Hans
source_revision: fe3bd2e902ba9fe68764646409ba97ea78fcd42d5400a08b8d590625aefe9c57
api_id: "validation::ValidationError::impl Debug"
---
# validation::ValidationError::impl Debug

使用 Rust Debug 实现。

```rust
impl Debug for validation::ValidationError
```

## 参数、返回值及常规用法

Debug 产生诊断文本，不是稳定的验证序列化格式。这是原生 Rust trait 语法，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-debug" locale="zh-Hans" />

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
