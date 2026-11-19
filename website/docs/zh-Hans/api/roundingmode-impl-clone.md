---
locale: zh-Hans
source_revision: be26937ee4bcd1a30b10deb30b4ced946f0371e3015f1e537f8c85709084d801
api_id: RoundingMode::impl Clone
---
# RoundingMode::impl Clone

使用舍入模式的 Rust Clone 实现。

```rust
impl Clone for RoundingMode
```

## 参数、返回值及常规用法

RoundingMode 是小型 Copy 枚举，复制不会改变所选规则。相等比较枚举项，Debug 输出名称。此示例验证原生 Rust trait 语法；浏览器模式是不可变的数字值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-clone" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
