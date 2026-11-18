---
locale: zh-Hans
source_revision: 16147351e8fe026905ac983886e4a94a6ce341eb3c301ee1901cd0719ea82ee1
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

验证整数舍入模式编码。

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

value: i32/整数 number。0–6 返回对应模式，其他 i32 值返回 InvalidArgument。浏览器小数、非有限或超出 i32 范围的值会在 Rust 调用前被参数检查拒绝。bridge 调用 Rust 验证器并返回数字模式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
