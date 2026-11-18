---
locale: zh-Hans
source_revision: bae692e3c5c12038fd1ef26d04bf1880cf624ae83c8c2ec0ad9f2f071f1b9f3a
api_id: Decimal::significant_figures
---
# Decimal::significant_figures

舍入到不超过指定数量的十进制有效数字。

```rust
fn significant_figures (& self , figures : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

figures: 正 isize/整数 number；mode 指定舍入方式。与小数位精度不同，有效数字按系数的数字位数计算。此方法不会增加精度，也可对整数位舍入（1234.5 保留三位得到 1230）。非正 figures 返回 InvalidArgument；Unnecessary 遇到需要舍入时返回 RoundUnnecessary。浏览器会预先检查可能的系数进位。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-significant-figures" locale="zh-Hans" />

## 浏览器边界

所有整数参数必须位于 wasm32 对应范围。浏览器还施加 [资源限制](../resource-policy)，包括目标小数位数、系数增长及调用次数。资源错误与数值库错误分开报告。借用方法不会消耗输入；仅 Rust 的 trait 语法明确标注为原生测试。

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
