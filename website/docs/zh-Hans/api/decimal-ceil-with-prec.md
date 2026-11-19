---
locale: zh-Hans
source_revision: 11b8aa485f155a658e5960045fe97e0878f07fd6eb371a471f1440f29be38903
api_id: Decimal::ceil_with_prec
---
# Decimal::ceil_with_prec

向正无穷方向，保留指定的小数位数。

```rust
fn ceil_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec: isize/整数 number。负精度返回 InvalidPrecision。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil-with-prec" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
