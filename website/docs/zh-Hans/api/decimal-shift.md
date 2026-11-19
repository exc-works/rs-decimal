---
locale: zh-Hans
source_revision: 6d142adfffddbbc67008e17859ebe097a896ff7983bb306596fd170c8b867b28
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

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
