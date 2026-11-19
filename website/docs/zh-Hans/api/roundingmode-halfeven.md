---
locale: zh-Hans
source_revision: 84ba3002f85664068a9d9ececc408a7d6f0214a486be55b19a41d101ddf845eb
api_id: RoundingMode::HalfEven
---
# RoundingMode::HalfEven

按最接近值，正中间值使保留位为偶数规则舍入。

```rust
HalfEven = 5
```

## 参数、返回值及常规用法

整数判别值为 5，建议使用具名值。规则适用于正数和负数。Down、Up 相对零定义，不是负、正无穷方向。Half 模式仅在正中间值时有区别。没有单独的 Floor 枚举项；该方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfeven" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
