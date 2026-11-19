---
locale: zh-Hans
source_revision: 609e506e194d4140e441e114e8a66b7a118a2cb443c002b71a5be2c8260307ad
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

将有理数转换为精确的有限小数。

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

value: &BigRational；bridge 接受 {numerator: bigint, denominator: bigint}。分母必须非零，否则为 bridge 参数错误；Rust BigRational 规范化符号并约分。约分后分母只能含因子 2 和 5，才能得到有限小数；其他因子返回 InvalidFormat。结果已初始化。浏览器在约分后、调用核心转换前检查输出精度和系数增长。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
