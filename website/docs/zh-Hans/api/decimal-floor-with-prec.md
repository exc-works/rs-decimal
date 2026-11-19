---
locale: zh-Hans
source_revision: 6ba2cb4a20d5541fd7637c273e7892b620d22f9a54a85ae9d00d1d5387296c75
api_id: Decimal::floor_with_prec
---
# Decimal::floor_with_prec

向负无穷方向，保留指定的小数位数。

```rust
fn floor_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec: isize/整数 number。负精度返回 InvalidPrecision。负数截断与向下取整不同：-1.5 截断为 -1，floor 为 -2。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor-with-prec" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
