---
locale: zh-Hans
source_revision: 8eec4d40194be2cd158d623391729d3b1e9d3b0532af669f3499b5168782b68c
api_id: Decimal::approx_root_with_prec
---
# Decimal::approx_root_with_prec

计算正整数次根。Rust 返回 Result<Decimal, DecimalError>，bridge 返回 Decimal 或抛出结构化错误。degree: i64，以 bigint 传输；必须为正，否则返回 InvalidRoot。奇数次根允许负输入；负数的偶数次根返回 NegativeRoot。一次根等同于 HalfEven 精度调整。prec: isize，以整数 number 传输，表示小数位数。负精度在定义域检查前返回 InvalidPrecision。使用 HalfEven 舍入；例如 sqrt(6.25) 保留零位时，正中间值 2.5 舍入为 2。虽然名称包含 approx_root，Rust 会用整数比较验证舍入；核心计算或迭代预算耗尽时返回 NoConvergence。输入仅借用且保持不变；结果（包括默认零）已初始化。

```rust
fn approx_root_with_prec (& self , degree : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 常规示例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root-with-prec" locale="zh-Hans" />

## 相关方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
