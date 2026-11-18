---
locale: zh-Hans
source_revision: ce7339c2b8115fd42455e27eb624e03a51de3a069de71c01158ea752ddf6833a
api_id: Decimal::bit_len
---
# Decimal::bit_len

统计未缩放系数绝对值的二进制位数。

```rust
fn bit_len (& self) -> usize
```

## 参数、返回值及常规用法

无参数，Rust 返回 usize，bridge 返回 number。不应用 scale，也不计符号位：-1.20 的系数为 -120，位数为 7。零（包括默认零）返回 0。这不是最高置位的索引。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="zh-Hans" />

## 浏览器边界

整数不会经过有损的 JS number 转换。转换元组按 Rust 顺序保留值和标志；标志为 false 时不能假定结果精确。借用输入保持不变，构造结果已初始化。[资源限制](../resource-policy) 限制整数输入、输出精度及有理数系数增长，与库错误分开报告。仅 Rust 导入语法的示例标记为原生测试。

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
