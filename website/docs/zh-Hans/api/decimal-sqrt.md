---
locale: zh-Hans
source_revision: 0bbd6c78da40405c6e9ad9c68a14878f98d98b4601eb8ff1296abba0bbf809ae
api_id: Decimal::sqrt
---
# Decimal::sqrt

计算平方根。Rust 返回 Result<Decimal, DecimalError>，bridge 返回 Decimal 或抛出结构化错误。没有 degree 参数。负输入返回 NegativeRoot。默认精度为 max(输入小数位数, 20)，不是 20 位有效数字。使用 HalfEven 舍入；例如 sqrt(6.25) 保留零位时，正中间值 2.5 舍入为 2。虽然名称包含 approx_root，Rust 会用整数比较验证舍入；核心计算或迭代预算耗尽时返回 NoConvergence。输入仅借用且保持不变；结果（包括默认零）已初始化。

```rust
fn sqrt (& self) -> Result < Self , DecimalError >
```

## 常规示例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt" locale="zh-Hans" />

## 相关方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
