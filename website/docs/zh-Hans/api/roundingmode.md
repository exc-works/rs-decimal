---
locale: zh-Hans
source_revision: 24579cc5f98ad85ad1558995873e9949e2e4e6ae6e01cc4702b2ee7ca18a14ea
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

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
