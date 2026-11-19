---
locale: zh-Hans
source_revision: 50b84c140827ca9ee14623ef754490d47ed0de164e4d5e55011356b86e00545d
api_id: Decimal::shift
---
# Decimal::shift

把 Decimal 乘以精确的十次幂。

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

places: isize/整数 number。正数向右移动小数点，负数向左移动。与 rescale 不同，它会改变数值。非零位移产生超过 i32::MAX 的 scale 时，核心返回 Overflow。即使 shift(0) 也会初始化默认零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
