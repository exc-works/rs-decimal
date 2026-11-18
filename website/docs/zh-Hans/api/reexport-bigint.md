---
locale: zh-Hans
source_revision: 8110d2fd30808fd737c3f5730a5be2d19cf89f407f5d688859de476689342458
api_id: reexport::BigInt
---
# reexport::BigInt

从 crate 根路径导入 BigInt。

```rust
pub use num_bigint::BigInt as BigInt
```

## 参数、返回值及常规用法

这是 Rust 的公开重新导出，不是新类型或新实现。导入语法在原生环境测试。浏览器对应项分别为 bigint、精确有理数字段对象以及全局 most_significant_bit 函数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigint" locale="zh-Hans" />

## 浏览器边界

整数不会经过有损的 JS number 转换。转换元组按 Rust 顺序保留值和标志；标志为 false 时不能假定结果精确。借用输入保持不变，构造结果已初始化。[资源限制](../resource-policy) 限制整数输入、输出精度及有理数系数增长，与库错误分开报告。仅 Rust 导入语法的示例标记为原生测试。

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
