---
locale: zh-Hant
source_revision: 584886d027ae1ffb7977be9193069c421cfe24c70d485c0551ed585a1d8e7407
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

去掉多餘的小數末尾零，不改變數值。

```rust
fn strip_trailing_zeros (& self) -> Self
```

## 參數、回傳值及常規用法

無參數，回傳獨立的 Decimal。僅在 scale 為正時移除十因子，因此不會去掉 1200 這樣的整數末尾零。任何數值零都會變為已初始化且 scale 為零的零值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
