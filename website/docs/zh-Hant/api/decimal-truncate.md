---
locale: zh-Hant
source_revision: d24f0cd3d78b3594804e8def855ad76024ffbd370d84b3931d49427d7730909f
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::truncate
---
# Decimal::truncate

向零方向，得到整數。

```rust
fn truncate (& self) -> Self
```

## 參數、回傳值及常規用法

無參數，回傳小數位數為零的新 Decimal。負數截斷與向下取整不同：-1.5 截斷為 -1，floor 為 -2。輸入僅被借用，保持不變。結果已初始化，包括預設零輸入。提高精度會追加零，不改變數值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate" locale="zh-Hant" />

## 相關方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
