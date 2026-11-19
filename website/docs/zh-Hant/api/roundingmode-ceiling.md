---
locale: zh-Hant
source_revision: 8de86b9c337d17cfc9fdcc766a8c16ae7e2cfcd7e721947844d22305f2992bfa
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::Ceiling
---
# RoundingMode::Ceiling

按向正無窮規則捨入。

```rust
Ceiling = 2
```

## 參數、回傳值及常規用法

整數判別值為 2，建議使用具名值。規則適用於正數和負數。Down、Up 相對零定義，不是負、正無窮方向。Half 模式僅在正中間值時有區別。沒有單獨的 Floor 列舉項；該方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-ceiling" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
