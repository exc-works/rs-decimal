---
locale: zh-Hans
source_revision: 816a5e8449654efb901339ad5516b3fdb94be2af25e3c709a61823154b83be91
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

## 相关方法

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
