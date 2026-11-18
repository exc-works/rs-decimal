---
locale: zh-Hans
source_revision: 9cb35d36232b48dfa841cf9ea23e2fcf6d9b36b6ef45c2f68538158901813e1b
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

向最接近值舍入，正中间值取偶数，保留指定的小数位数。

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## 参数

`prec: isize` — 保留的小数位数。bridge 接受 JavaScript 整数 number。

## 返回值

返回已初始化的 Decimal，包括默认零输入。方法借用输入，提高精度会追加尾零。

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). 使用 [`rescale`](./decimal-rescale) 可指定其他舍入模式。

## 错误

负精度返回 `InvalidPrecision`. bridge 会抛出对应错误。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
