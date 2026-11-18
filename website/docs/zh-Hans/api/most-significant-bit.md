---
locale: zh-Hans
source_revision: cbc68f86b580dd7b41c6b741193a12b44e7d04cdf771d632d510adf82d84c1ba
api_id: most_significant_bit
---
# most_significant_bit

查找从零起算的最高置位索引。

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## 参数、返回值及常规用法

value: &BigInt，以 bigint 表示。返回 Result<usize, DecimalError>/number。零和一都返回 0，120 返回 6。负输入返回 InvalidArgument。这是自由函数，不是 Decimal 方法；需要系数时应传入 coefficient()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="zh-Hans" />

## 浏览器边界

整数不会经过有损的 JS number 转换。转换元组按 Rust 顺序保留值和标志；标志为 false 时不能假定结果精确。借用输入保持不变，构造结果已初始化。[资源限制](../resource-policy) 限制整数输入、输出精度及有理数系数增长，与库错误分开报告。仅 Rust 导入语法的示例标记为原生测试。

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
