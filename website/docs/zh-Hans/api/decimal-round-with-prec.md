---
locale: zh-Hans
source_revision: 636d6e9ec2d1b0cb7be987deb57dc622bb8084f92601482a3b6418a00ec64c8b
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

向最接近值舍入，正中间值取偶数，保留指定的小数位数。

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## 参数

`prec: isize` — 保留的小数位数。bridge 接受 JavaScript 整数 number。

## 返回值

返回已初始化的 Decimal，包括默认零输入。方法借用输入，提高精度会追加尾零。

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). 使用 [`rescale`](./decimal-rescale) 可指定其他舍入模式。

## 错误

负精度返回 `InvalidPrecision`. bridge 会抛出对应错误。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
