---
locale: zh-Hans
source_revision: b1d5b92283e0de37c46d70318c8e896d60bc04df8806e365913a4c12caa7a8d9
api_id: Decimal::ceil
---
# Decimal::ceil

向正无穷方向，得到整数。

```rust
fn ceil (& self) -> Self
```

## 参数、返回值及常规用法

无参数，返回小数位数为零的新 Decimal。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
