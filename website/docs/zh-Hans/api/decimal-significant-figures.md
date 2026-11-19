---
locale: zh-Hans
source_revision: 6155fe36a4ec8e1b03a89c20c5c7247a068af20681f3b463de98b38c447a2097
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

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
