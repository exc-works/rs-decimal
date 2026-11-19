---
locale: zh-Hans
source_revision: db004727284a336c6f0e549dbc589384cfceb8780f95c6036a8917d02c730891
api_id: Decimal::floor
---
# Decimal::floor

向负无穷方向，得到整数。

```rust
fn floor (& self) -> Self
```

## 参数、返回值及常规用法

无参数，返回小数位数为零的新 Decimal。负数截断与向下取整不同：-1.5 截断为 -1，floor 为 -2。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
