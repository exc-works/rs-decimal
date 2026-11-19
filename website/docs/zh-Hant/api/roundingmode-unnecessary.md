---
locale: zh-Hant
source_revision: 50372ca1028f4ab77e1e376da48c589b0fdad5a066b7c436d3b5780901b17b77
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

按只允許精確結果規則捨入。

```rust
Unnecessary = 6
```

## 參數、回傳值及常規用法

整數判別值為 6，建議使用具名值。規則適用於正數和負數。丟棄零數字仍屬精確操作；丟棄任何非零數字都回傳 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
