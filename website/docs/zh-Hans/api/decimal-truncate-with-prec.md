---
locale: zh-Hans
source_revision: d9bfbd2d29924d270b97f6a4251656ca421cb89e51e3bdcdc1ecdf2cdefab10f
api_id: Decimal::truncate_with_prec
---
# Decimal::truncate_with_prec

向零方向，保留指定的小数位数。

```rust
fn truncate_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec: isize/整数 number。负精度返回 InvalidPrecision。负数截断与向下取整不同：-1.5 截断为 -1，floor 为 -2。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate-with-prec" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
