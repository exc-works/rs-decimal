---
locale: zh-Hans
source_revision: a66a15086c7943a9a541c1451a0fe544a096af2a9b938eb44bd20730eac8975d
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

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
