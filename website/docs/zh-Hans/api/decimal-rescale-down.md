---
locale: zh-Hans
source_revision: e9dbb721c8987afdaedb4f74b013156db52924dc6bfa528e95669ba4396942ff
api_id: Decimal::rescale_down
---
# Decimal::rescale_down

通过向零截断改变小数位数。

```rust
fn rescale_down (& self , prec : isize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec: isize/整数 number；返回 Decimal，负精度返回 InvalidPrecision。Down 表示向零，不是向负无穷。提高精度追加零。输入不变，结果已初始化。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale-down" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
