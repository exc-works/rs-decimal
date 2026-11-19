---
locale: zh-Hant
source_revision: 73057fa1830878a540ff385b7411b32617a1d65647f7c283e5495a0cce3aca84
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

從 crate 根路徑匯入捨入模式。

```rust
pub use rounding::RoundingMode as RoundingMode
```

## 參數、回傳值及常規用法

使用 rs_decimal::RoundingMode，這是同一個列舉，不是另一份實現。瀏覽器直接提供具名的 RoundingMode 物件，不使用 Rust 匯入語法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
