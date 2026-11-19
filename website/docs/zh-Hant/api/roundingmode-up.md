---
locale: zh-Hant
source_revision: e0028e1081b0309d3901334cb20b25aa450fe2c46a4c364db5668c9213b4477b
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::Up
---
# RoundingMode::Up

按遠離零規則捨入。

```rust
Up = 1
```

## 參數、回傳值及常規用法

整數判別值為 1，建議使用具名值。規則適用於正數和負數。Down、Up 相對零定義，不是負、正無窮方向。Half 模式僅在正中間值時有區別。沒有單獨的 Floor 列舉項；該方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-up" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
