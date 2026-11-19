---
locale: zh-Hans
source_revision: 6e97ceecb692f3b30f694df6cf3812a7bc93e427ba0e4c690447e33c151380a9
api_id: Decimal::round
---
# Decimal::round

向最接近值舍入，正中间值取偶数，得到整数。

```rust
fn round (& self) -> Self
```

## 参数、返回值及常规用法

无参数，返回小数位数为零的新 Decimal。这里默认使用 HalfEven，与 JavaScript Math.round 的正中间值处理规则不同。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
