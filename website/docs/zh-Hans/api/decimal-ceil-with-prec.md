---
locale: zh-Hans
source_revision: 2d051b010383c0adad95eaeacec22cce35ee5a02d1fddebba48aa1d5a1f4d4ae
api_id: Decimal::ceil_with_prec
---
# Decimal::ceil_with_prec

向正无穷方向，保留指定的小数位数。

```rust
fn ceil_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec: isize/整数 number；Rust 返回 Result<Decimal, DecimalError>，bridge 返回 Decimal 或抛出错误。负精度返回 InvalidPrecision。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil-with-prec" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
