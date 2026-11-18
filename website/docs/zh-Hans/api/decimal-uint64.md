---
locale: zh-Hans
source_revision: 309b64e5487e4ac1d2ea8cf88852216653b617cb1ed53a7d6bdd1a0dffa2d494
api_id: Decimal::uint64
---
# Decimal::uint64

转换为固定位宽整数并返回成功标志。

```rust
fn uint64 (& self) -> (u64 , bool)
```

## 参数、返回值及常规用法

无参数；返回 (u64, bool)，映射为 [bigint, boolean]。范围内的整数返回对应整数和 true。小数或溢出返回 (0, false)，不会截断或抛出错误。负数同样返回 (0, false)。不能仅凭 scale 判断是否为整数：12.00 转换成功。应检查标志，区分失败与有效零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-uint64" locale="zh-Hans" />

## 浏览器边界

整数不会经过有损的 JS number 转换。转换元组按 Rust 顺序保留值和标志；标志为 false 时不能假定结果精确。借用输入保持不变，构造结果已初始化。[资源限制](../resource-policy) 限制整数输入、输出精度及有理数系数增长，与库错误分开报告。仅 Rust 导入语法的示例标记为原生测试。

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
