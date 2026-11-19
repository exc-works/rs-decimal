---
locale: zh-Hans
source_revision: 2e472bd0598b8ec796fb5adb09d98ce669a8d1defb11e5244f124c816e8d2cd8
api_id: Decimal::approx_root
---
# Decimal::approx_root

计算正整数次根。Rust 返回 Result<Decimal, DecimalError>，bridge 返回 Decimal 或抛出结构化错误。degree: i64，以 bigint 传输；必须为正，否则返回 InvalidRoot。奇数次根允许负输入；负数的偶数次根返回 NegativeRoot。一次根等同于 HalfEven 精度调整。默认精度为 max(输入小数位数, 20)，不是 20 位有效数字。使用 HalfEven 舍入；例如 sqrt(6.25) 保留零位时，正中间值 2.5 舍入为 2。虽然名称包含 approx_root，Rust 会用整数比较验证舍入；核心计算或迭代预算耗尽时返回 NoConvergence。输入仅借用且保持不变；结果（包括默认零）已初始化。

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## 常规示例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="zh-Hans" />

## 相关方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
