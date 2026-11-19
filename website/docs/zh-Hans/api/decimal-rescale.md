---
locale: zh-Hans
source_revision: 203acbe634de9e56f24b2f6e3eccc13dcb25f721727e606d7d5a0efdca2a8e02
api_id: Decimal::rescale
---
# Decimal::rescale

使用明确的舍入模式改变小数位数。

```rust
fn rescale (& self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec 表示小数位数，不是有效数字位数；mode 是七种 RoundingMode 值之一。提高精度追加零；降低精度时只对被丢弃的非零数字舍入。负精度返回 InvalidPrecision；Unnecessary 遇到需要舍入时返回 RoundUnnecessary。结果会将默认零初始化；借用的输入保持不变。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
