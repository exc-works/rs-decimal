---
locale: zh-Hans
source_revision: e175d4d7de2a038dff837a099025c66d8337b4480eacf9bc8d2de1f1d5950025
api_id: "validation::ValidationRule::parse"
---
# validation::ValidationRule::parse

执行精确的小数规则。

```rust
fn parse (tag : & str , parameter : & str) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

tag: &str/string，parameter: &str/string；返回 `Result<ValidationRule, DecimalError>`/规则句柄。使用 decimal_gt 等完整标签。比较参数为小数文本，between 使用 lower~upper，max_precision 使用非负 isize。未知标签、错误参数和反向区间返回 InvalidArgument。在 wasm32 上 isize 为有符号 32 位。不需要参数的规则保留但不解释参数文本。规则精确比较 Decimal，不经过浮点转换。decimal_required 检查初始化，因此已初始化零通过。eq/ne/gt/gte/lt/lte 按数值比较，包括未初始化零。between 使用包含端点的 lower~upper，且额外要求初始化。positive/negative/nonzero 要求初始化。max_precision 检查存储的 scale，包括保留的末尾零；它不隐含 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-parse" locale="zh-Hans" />

## 浏览器边界

validation feature 已启用。单个字符串最多 8192 个 UTF-8 字节；规则列表、错误列表和消息映射最多 64 项，JSON 传输最多 65536 字节。小数规则参数遵循 [十进制资源限制](../resource-policy)。翻译前保守估计占位符展开后的总文本，超过 65536 字节时拒绝；可能拒绝实际刚好容纳的结果。自定义类别和字段是数据，不执行代码。验证失败与配置错误、bridge 资源错误分开报告。

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
