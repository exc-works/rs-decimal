---
locale: zh-Hans
source_revision: ffab3b21dcd7c80e0222f2cd0457116e79f56aafc95fc2ed3c796e423b9661cc
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

将有理数转换为精确的有限小数。

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

value: &BigRational；JavaScript 接受 {numerator: bigint, denominator: bigint}。分母必须非零，否则为 JavaScript 参数错误；Rust BigRational 规范化符号并约分。约分后分母只能含因子 2 和 5，才能得到有限小数；其他因子返回 InvalidFormat。结果已初始化。浏览器在约分后、调用核心转换前检查输出精度和系数增长。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
