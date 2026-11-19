---
locale: zh-Hans
source_revision: 53cafbe90f3ab017866bcf2afb140d94d6b1a293f7aa6ca2a32a775110c8f82c
api_id: Decimal::ceil
---
# Decimal::ceil

向正无穷方向，得到整数。

```rust
fn ceil (& self) -> Self
```

## 参数、返回值及常规用法

无参数，返回小数位数为零的新 Decimal。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
