---
locale: zh-Hans
source_revision: 69f5b453b56bcf2350f3a3cf2e3d29b1035a464f2105cefeff33c4c49c926524
api_id: Decimal::shift
---
# Decimal::shift

把 Decimal 乘以精确的十次幂。

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

places: isize/整数 number。正数向右移动小数点，负数向左移动。与 rescale 不同，它会改变数值。非零位移产生超过 i32::MAX 的 scale 时，核心返回 Overflow。浏览器增长及精度检查也可能在分配前返回独立的资源错误。即使 shift(0) 也会初始化默认零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
