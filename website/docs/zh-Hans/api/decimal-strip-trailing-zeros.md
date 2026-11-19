---
locale: zh-Hans
source_revision: 584886d027ae1ffb7977be9193069c421cfe24c70d485c0551ed585a1d8e7407
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

去掉多余的小数末尾零，不改变数值。

```rust
fn strip_trailing_zeros (& self) -> Self
```

## 参数、返回值及常规用法

无参数，返回独立的 Decimal。仅在 scale 为正时移除十因子，因此不会去掉 1200 这样的整数末尾零。任何数值零都会变为已初始化且 scale 为零的零值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
