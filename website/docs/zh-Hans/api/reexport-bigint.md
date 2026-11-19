---
locale: zh-Hans
source_revision: f07542c46478fd9edd09bbf3af9bd03d15608eb33e9ed99beb51b022df098d6a
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

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
