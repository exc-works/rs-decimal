---
locale: zh-Hans
source_revision: ce0e6eba90e03ef0b3ea77c71d25c9216e58acb759f414a585091bbcd0fa1215
api_id: RoundingMode::impl Copy
---
# RoundingMode::impl Copy

使用舍入模式的 Rust Copy 实现。

```rust
impl Copy for RoundingMode
```

## 参数、返回值及常规用法

RoundingMode 是小型 Copy 枚举，复制不会改变所选规则。相等比较枚举项，Debug 输出名称。此示例验证原生 Rust trait 语法；浏览器模式是不可变的数字值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-copy" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
