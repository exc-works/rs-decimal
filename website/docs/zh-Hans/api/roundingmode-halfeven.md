---
locale: zh-Hans
source_revision: a19e9628caaaa2c66221e7031ee01f75dd38995c9d2873827f8d6308231f84ca
api_id: RoundingMode::HalfEven
---
# RoundingMode::HalfEven

按最接近值，正中间值使保留位为偶数规则舍入。

```rust
HalfEven = 5
```

## 参数、返回值及常规用法

整数判别值为 5，建议使用具名值。规则适用于正数和负数。Down、Up 相对零定义，不是负、正无穷方向。Half 模式仅在正中间值时有区别。没有单独的 Floor 枚举项；该方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfeven" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
