---
locale: zh-Hans
source_revision: cce5cfd2a6bfd3edbc32da84fa417cbea83f61f7b1bcf9e77c7c5b49aaed3994
api_id: Decimal::truncate_with_prec
---
# Decimal::truncate_with_prec

向零方向，保留指定的小数位数。

```rust
fn truncate_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec: isize/整数 number；Rust 返回 Result<Decimal, DecimalError>，bridge 返回 Decimal 或抛出错误。负精度返回 InvalidPrecision。负数截断与向下取整不同：-1.5 截断为 -1，floor 为 -2。输入仅被借用，保持不变。结果已初始化，包括默认零输入。提高精度会追加零，不改变数值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate-with-prec" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
