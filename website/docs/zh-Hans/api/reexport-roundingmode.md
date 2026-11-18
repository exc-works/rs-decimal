---
locale: zh-Hans
source_revision: 340ee9c355dc0ac1e0c13e6d43aee222d595260389706ec4d9398cbe1a49147c
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

从 crate 根路径导入舍入模式。

```rust
pub use rounding::RoundingMode as RoundingMode
```

## 参数、返回值及常规用法

使用 rs_decimal::RoundingMode，这是同一个枚举，不是另一份实现。浏览器直接提供具名的 RoundingMode 对象，不使用 Rust 导入语法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
