---
locale: zh-Hans
source_revision: 788e3facaf7bb31d022529aaea08540d91c8a4001537b0085ab4e604ef0e3ae8
api_id: "validation::ValidationRule::matches"
---
# validation::ValidationRule::matches

执行精确的小数规则。

```rust
fn matches (& self , value : & Decimal) -> bool
```

## 参数、返回值及常规用法

value: &Decimal/Decimal；返回 bool/boolean，并保留规则与数值。规则精确比较 Decimal，不经过浮点转换。decimal_required 检查初始化，因此已初始化零通过。eq/ne/gt/gte/lt/lte 按数值比较，包括未初始化零。between 使用包含端点的 lower~upper，且额外要求初始化。positive/negative/nonzero 要求初始化。max_precision 检查存储的 scale，包括保留的末尾零；它不隐含 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-matches" locale="zh-Hans" />

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
