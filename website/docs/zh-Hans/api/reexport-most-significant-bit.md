---
locale: zh-Hans
source_revision: fe0a2cbd7e86aa455f6274096dc78e80786ab081e8fd78826b22f34970a7dfa9
api_id: reexport::most_significant_bit
---
# reexport::most_significant_bit

从 crate 根路径导入 most_significant_bit。

```rust
pub use conversion::most_significant_bit as most_significant_bit
```

## 参数、返回值及常规用法

这是 Rust 的公开重新导出，不是新类型或新实现。导入语法在原生环境测试。浏览器对应项分别为 bigint、精确有理数字段对象以及全局 most_significant_bit 函数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-most-significant-bit" locale="zh-Hans" />

## 浏览器边界

整数不会经过有损的 JS number 转换。转换元组按 Rust 顺序保留值和标志；标志为 false 时不能假定结果精确。借用输入保持不变，构造结果已初始化。[资源限制](../resource-policy) 限制整数输入、输出精度及有理数系数增长，与库错误分开报告。仅 Rust 导入语法的示例标记为原生测试。

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
