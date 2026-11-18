---
locale: zh-Hans
source_revision: 52c061e67f4d78f9abff6cb38b422cdd828bc88de770dccac8c7ef157f4fde94
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

通过能往返还原浮点数的最短十进制表示构造。

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

value: f64，以 JS number 传入。无法恢复之前因二进制浮点而丢失的十进制数字。精确十进制输入请使用 new_from_string。NaN 和正负无穷返回 InvalidFormat。负零变成已初始化且不带符号的 Decimal 零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="zh-Hans" />

## 浏览器边界

整数不会经过有损的 JS number 转换。转换元组按 Rust 顺序保留值和标志；标志为 false 时不能假定结果精确。借用输入保持不变，构造结果已初始化。[资源限制](../resource-policy) 限制整数输入、输出精度及有理数系数增长，与库错误分开报告。仅 Rust 导入语法的示例标记为原生测试。

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
