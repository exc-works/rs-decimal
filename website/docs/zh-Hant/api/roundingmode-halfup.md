---
locale: zh-Hant
source_revision: bb4dd0948b4d4be09c12115b5c13abd8421926284cdbcfed281913fc4636aeb5
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

按最接近值，正中間值遠離零規則捨入。

```rust
HalfUp = 3
```

## 參數、回傳值及常規用法

整數判別值為 3，建議使用具名值。規則適用於正數和負數。Down、Up 相對零定義，不是負、正無窮方向。Half 模式僅在正中間值時有區別。沒有單獨的 Floor 列舉項；該方向使用 floor_with_prec。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
