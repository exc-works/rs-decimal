---
locale: zh-Hans
source_revision: 73057fa1830878a540ff385b7411b32617a1d65647f7c283e5495a0cce3aca84
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

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
