---
locale: zh-Hans
source_revision: be7e92c9ff29d88bdd7b2eadc18fb40911d696405ee369af6496615cdbec068e
api_id: RoundingMode::Down
---
# RoundingMode::Down

按向零规则舍入。

```rust
Down = 0
```

## 参数、返回值及常规用法

整数判别值为 0，建议使用具名值。规则适用于正数和负数。Down、Up 相对零定义，不是负、正无穷方向。Half 模式仅在正中间值时有区别。没有单独的 Floor 枚举项；该方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-down" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
