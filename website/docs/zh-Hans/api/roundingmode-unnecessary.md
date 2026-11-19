---
locale: zh-Hans
source_revision: 50372ca1028f4ab77e1e376da48c589b0fdad5a066b7c436d3b5780901b17b77
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

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
