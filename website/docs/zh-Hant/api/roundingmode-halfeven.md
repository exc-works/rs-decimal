---
locale: zh-Hant
source_revision: 84ba3002f85664068a9d9ececc408a7d6f0214a486be55b19a41d101ddf845eb
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::HalfEven
---
# RoundingMode::HalfEven

按最接近值，正中間值使保留位為偶數規則捨入。

```rust
HalfEven = 5
```

## 參數、回傳值及常規用法

整數判別值為 5，建議使用具名值。規則適用於正數和負數。Down、Up 相對零定義，不是負、正無窮方向。Half 模式僅在正中間值時有區別。沒有單獨的 Floor 列舉項；該方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfeven" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
