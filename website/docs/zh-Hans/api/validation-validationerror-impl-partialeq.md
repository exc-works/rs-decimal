---
locale: zh-Hans
source_revision: 88d2cfb170b637934d794950f5a609aa140f9cabce333b9eba44daa4d52bc8fc
api_id: "validation::ValidationError::impl PartialEq"
---
# validation::ValidationError::impl PartialEq

使用 Rust PartialEq 实现。

```rust
impl PartialEq for validation::ValidationError
```

## 参数、返回值及常规用法

ValidationError 相等同时比较 field、tag 和原始 parameter，不是只比较翻译后的消息。这是原生 Rust trait 语法，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="zh-Hans" />

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
