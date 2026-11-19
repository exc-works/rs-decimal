---
locale: zh-Hans
source_revision: 536ad1418de3fbdb10070d55eda3a68aef79b3b4a2bc3507ce307a0421f5b595
api_id: RoundingMode
---
# RoundingMode

选择七种明确的舍入规则之一。

```rust
pub enum RoundingMode
```

## 参数、返回值及常规用法

Rust 使用具名枚举项，JavaScript 使用 RoundingMode 字段。Rust 判别值 0–6 与浏览器数字一致。Down 向零，Up 远离零，Ceiling 向正无穷。HalfUp/HalfDown/HalfEven 对正中间值分别远离零、向零、取偶数。Unnecessary 要求精确。不设置隐式的全局舍入方式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
