---
locale: zh-Hans
source_revision: 8de86b9c337d17cfc9fdcc766a8c16ae7e2cfcd7e721947844d22305f2992bfa
api_id: RoundingMode::Ceiling
---
# RoundingMode::Ceiling

按向正无穷规则舍入。

```rust
Ceiling = 2
```

## 参数、返回值及常规用法

整数判别值为 2，建议使用具名值。规则适用于正数和负数。Down、Up 相对零定义，不是负、正无穷方向。Half 模式仅在正中间值时有区别。没有单独的 Floor 枚举项；该方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-ceiling" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
