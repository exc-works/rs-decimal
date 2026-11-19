---
locale: zh-Hans
source_revision: d588c3aa025efc49b40f1c2c9f6f76c0ff1df249830b32d566fb6a6a5794cdc5
api_id: RoundingMode::impl Eq
---
# RoundingMode::impl Eq

使用舍入模式的 Rust Eq 实现。

```rust
impl Eq for RoundingMode
```

## 参数、返回值及常规用法

RoundingMode 是小型 Copy 枚举，复制不会改变所选规则。相等比较枚举项，Debug 输出名称。此示例验证原生 Rust trait 语法；浏览器模式是不可变的数字值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-eq" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
