---
locale: zh-Hans
source_revision: 4ba98590c5087494de7de79070360a5295cf980481866e0fbdd09d39a5d16846
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

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
