---
locale: zh-Hans
source_revision: b417c8ff7bf7c7086d2d314348cd490ed506bea3f1d87b65cb817d90defb82d7
api_id: Decimal::big_rat
---
# Decimal::big_rat

读取精确约分后的有理数值。

```rust
fn big_rat (& self) -> BigRational
```

## 参数、返回值及常规用法

无参数，Rust 返回 BigRational。bridge 返回不可变的 {numerator: bigint, denominator: bigint} 快照，规范化后分母为正。Decimal 的精度和末尾零不会作为有理数元数据保留；零变为 0/1。输入仅被借用，保持不变。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="zh-Hans" />

## 浏览器边界

整数不会经过有损的 JS number 转换。转换元组按 Rust 顺序保留值和标志；标志为 false 时不能假定结果精确。借用输入保持不变，构造结果已初始化。[资源限制](../resource-policy) 限制整数输入、输出精度及有理数系数增长，与库错误分开报告。仅 Rust 导入语法的示例标记为原生测试。

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
