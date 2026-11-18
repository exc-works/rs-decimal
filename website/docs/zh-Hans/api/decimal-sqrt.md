---
locale: zh-Hans
source_revision: 49e05880780d4157422be53a7d3d5c1a0d8d4e809a77bd78ae251e8f6c7889c8
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

## 浏览器边界

[资源策略](../resource-policy) 将输出 scale 限制为 1000。非平凡根的 degree 最多为 128，并在分配前检查缩放、Newton 迭代及中点验证的中间系数预算。零、正负 1 和一次根保留核心快速路径，但仍受输出精度限制。定义域错误与负精度错误保持库行为；资源错误与 NoConvergence 不同。输入范围合法不代表每个 degree/prec 组合都能通过资源检查。

## 相关方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
