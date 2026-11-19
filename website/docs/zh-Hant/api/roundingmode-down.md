---
locale: zh-Hant
source_revision: be7e92c9ff29d88bdd7b2eadc18fb40911d696405ee369af6496615cdbec068e
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::Down
---
# RoundingMode::Down

按向零規則捨入。

```rust
Down = 0
```

## 參數、回傳值及常規用法

整數判別值為 0，建議使用具名值。規則適用於正數和負數。Down、Up 相對零定義，不是負、正無窮方向。Half 模式僅在正中間值時有區別。沒有單獨的 Floor 列舉項；該方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-down" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
