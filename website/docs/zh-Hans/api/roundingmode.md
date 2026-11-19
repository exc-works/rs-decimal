---
locale: zh-Hans
source_revision: ac4a10a855b39a59f0636e888eaecef62de05250e4c407dbf6ae0398e83f87d9
api_id: RoundingMode
---
# RoundingMode

选择七种明确的舍入规则之一。

```rust
pub enum RoundingMode
```

## 参数、返回值及常规用法

Rust 使用具名枚举项，bridge 使用 RoundingMode 字段。Rust 判别值 0–6 与浏览器数字一致。Down 向零，Up 远离零，Ceiling 向正无穷。HalfUp/HalfDown/HalfEven 对正中间值分别远离零、向零、取偶数。Unnecessary 要求精确。不设置隐式的全局舍入方式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
