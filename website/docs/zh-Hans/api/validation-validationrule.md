---
locale: zh-Hans
source_revision: 4a04c1296f4a9e4d0c1eac8d1707242f3d7b87d7302eac99dc6a590f2c03208e
api_id: "validation::ValidationRule"
---
# validation::ValidationRule

执行精确的小数规则。

```rust
pub struct ValidationRule
```

## 参数、返回值及常规用法

通过 parse(tag, parameter) 创建规则，再复用于 matches 或 validate。构造时检查参数，浏览器句柄属于本次运行，可以显式释放。规则精确比较 Decimal，不经过浮点转换。decimal_required 检查初始化，因此已初始化零通过。eq/ne/gt/gte/lt/lte 按数值比较，包括未初始化零。between 使用包含端点的 lower~upper，且额外要求初始化。positive/negative/nonzero 要求初始化。max_precision 检查存储的 scale，包括保留的末尾零；它不隐含 required。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule" locale="zh-Hans" />

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
