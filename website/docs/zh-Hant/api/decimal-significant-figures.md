---
locale: zh-Hant
source_revision: 6155fe36a4ec8e1b03a89c20c5c7247a068af20681f3b463de98b38c447a2097
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::significant_figures
---
# Decimal::significant_figures

捨入到不超過指定數量的十進位有效位數。

```rust
fn significant_figures (& self , figures : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

figures: 正 isize/整數 number；mode 指定捨入方式。與小數位精度不同，有效位數按係數的數字位數計算。此方法不會增加精度，也可對整數位捨入（1234.5 保留三位得到 1230）。非正 figures 回傳 InvalidArgument；Unnecessary 遇到需要捨入時回傳 RoundUnnecessary。瀏覽器會預先檢查可能的係數進位。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-significant-figures" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
