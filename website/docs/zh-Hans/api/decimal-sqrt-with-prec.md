---
locale: zh-Hans
source_revision: 1364e2c70db38e747614af95f661a59bb822316d59013022933d2987f410c846
api_id: Decimal::sqrt_with_prec
---
# Decimal::sqrt_with_prec

计算平方根。Rust 返回 Result<Decimal, DecimalError>，bridge 返回 Decimal 或抛出结构化错误。没有 degree 参数。负输入返回 NegativeRoot。prec: isize，以整数 number 传输，表示小数位数。负精度在定义域检查前返回 InvalidPrecision。使用 HalfEven 舍入；例如 sqrt(6.25) 保留零位时，正中间值 2.5 舍入为 2。虽然名称包含 approx_root，Rust 会用整数比较验证舍入；核心计算或迭代预算耗尽时返回 NoConvergence。输入仅借用且保持不变；结果（包括默认零）已初始化。

```rust
fn sqrt_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 常规示例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt-with-prec" locale="zh-Hans" />

## 相关方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
