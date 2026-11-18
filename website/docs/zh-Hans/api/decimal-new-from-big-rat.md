---
locale: zh-Hans
source_revision: c7909226b96ffde911017d8ead0a3f2edd2a0c894ee744bf3bcbea66574f1bf5
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

## 浏览器边界

整数不会经过有损的 JS number 转换。转换元组按 Rust 顺序保留值和标志；标志为 false 时不能假定结果精确。借用输入保持不变，构造结果已初始化。[资源限制](../resource-policy) 限制整数输入、输出精度及有理数系数增长，与库错误分开报告。仅 Rust 导入语法的示例标记为原生测试。

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
