---
locale: zh-Hans
source_revision: 823f73234f73cba4c9bfd0b5f1d418fa482625a8365337d1d27eb5d7306b4e6d
api_id: RoundingMode::impl Debug
---
# RoundingMode::impl Debug

使用舍入模式的 Rust Debug 实现。

```rust
impl Debug for RoundingMode
```

## 参数、返回值及常规用法

RoundingMode 是小型 Copy 枚举，复制不会改变所选规则。相等比较枚举项，Debug 输出名称。此示例验证原生 Rust trait 语法；浏览器模式是不可变的数字值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-debug" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
