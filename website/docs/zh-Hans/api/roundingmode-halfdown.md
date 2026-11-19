---
locale: zh-Hans
source_revision: 80ca29e7b285c6bbe14fbaae400143c560d7d0375268641d09bff74efd458b6d
api_id: RoundingMode::HalfDown
---
# RoundingMode::HalfDown

按最接近值，正中间值向零规则舍入。

```rust
HalfDown = 4
```

## 参数、返回值及常规用法

整数判别值为 4，建议使用具名值。规则适用于正数和负数。Down、Up 相对零定义，不是负、正无穷方向。Half 模式仅在正中间值时有区别。没有单独的 Floor 枚举项；该方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfdown" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
