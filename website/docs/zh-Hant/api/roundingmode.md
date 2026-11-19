---
locale: zh-Hant
source_revision: 536ad1418de3fbdb10070d55eda3a68aef79b3b4a2bc3507ce307a0421f5b595
translation_provenance: zh-Hans-terminology-reviewed
api_id: RoundingMode
---
# RoundingMode

選擇七種明確的捨入規則之一。

```rust
pub enum RoundingMode
```

## 參數、回傳值及常規用法

Rust 使用具名列舉項，JavaScript 使用 RoundingMode 欄位。Rust 判別值 0–6 與瀏覽器數字一致。Down 向零，Up 遠離零，Ceiling 向正無窮。HalfUp/HalfDown/HalfEven 對正中間值分別遠離零、向零、取偶數。Unnecessary 要求精確。不設定隱式的全域捨入方式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
