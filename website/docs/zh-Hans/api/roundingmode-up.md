---
locale: zh-Hans
source_revision: e0028e1081b0309d3901334cb20b25aa450fe2c46a4c364db5668c9213b4477b
api_id: RoundingMode::Up
---
# RoundingMode::Up

按远离零规则舍入。

```rust
Up = 1
```

## 参数、返回值及常规用法

整数判别值为 1，建议使用具名值。规则适用于正数和负数。Down、Up 相对零定义，不是负、正无穷方向。Half 模式仅在正中间值时有区别。没有单独的 Floor 枚举项；该方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-up" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
