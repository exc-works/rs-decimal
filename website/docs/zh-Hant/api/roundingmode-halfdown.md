---
locale: zh-Hant
source_revision: 80ca29e7b285c6bbe14fbaae400143c560d7d0375268641d09bff74efd458b6d
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::HalfDown
---
# RoundingMode::HalfDown

按最接近值，正中間值向零規則捨入。

```rust
HalfDown = 4
```

## 參數、回傳值及常規用法

整數判別值為 4，建議使用具名值。規則適用於正數和負數。Down、Up 相對零定義，不是負、正無窮方向。Half 模式僅在正中間值時有區別。沒有單獨的 Floor 列舉項；該方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfdown" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
