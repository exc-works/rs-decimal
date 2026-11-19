---
locale: zh-Hans
source_revision: d24f0cd3d78b3594804e8def855ad76024ffbd370d84b3931d49427d7730909f
api_id: Decimal::truncate
---
# Decimal::truncate

向零方向，得到整数。

```rust
fn truncate (& self) -> Self
```

## 参数、返回值及常规用法

无参数，返回小数位数为零的新 Decimal。负数截断与向下取整不同：-1.5 截断为 -1，floor 为 -2。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
