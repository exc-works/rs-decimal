---
locale: zh-Hans
source_revision: 0deb49519f3f009c2dba973109de20fc0fcdfe90b9617749c506123c4c387997
api_id: Decimal::rescale
---
# Decimal::rescale

使用明确的舍入模式改变小数位数。

```rust
fn rescale (& self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec 表示小数位数，不是有效数字位数；mode 是七种 RoundingMode 值之一。提高精度追加零；降低精度时只对被丢弃的非零数字舍入。负精度返回 InvalidPrecision；Unnecessary 遇到需要舍入时返回 RoundUnnecessary。结果会将默认零初始化；借用的输入保持不变。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
