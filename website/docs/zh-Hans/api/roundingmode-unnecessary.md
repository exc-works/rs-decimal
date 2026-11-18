---
locale: zh-Hans
source_revision: 1d47fa6ae1a12d0a01d3c07d39948787bbb4209bfed6d0edc8d1a4933e938a92
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

按只允许精确结果规则舍入。

```rust
Unnecessary = 6
```

## 参数、返回值及常规用法

整数判别值为 6，建议使用具名值。规则适用于正数和负数。丢弃零数字仍属精确操作；丢弃任何非零数字都返回 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
