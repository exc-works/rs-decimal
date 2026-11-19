---
locale: zh-Hans
source_revision: bb4dd0948b4d4be09c12115b5c13abd8421926284cdbcfed281913fc4636aeb5
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

按最接近值，正中间值远离零规则舍入。

```rust
HalfUp = 3
```

## 参数、返回值及常规用法

整数判别值为 3，建议使用具名值。规则适用于正数和负数。Down、Up 相对零定义，不是负、正无穷方向。Half 模式仅在正中间值时有区别。没有单独的 Floor 枚举项；该方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
