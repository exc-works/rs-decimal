---
locale: zh-Hant
source_revision: f662d4a18feb3987ba22a3e426a493d661feab0dd71696533c5ec29aefece86d
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::exp
---
# Decimal::exp

計算指數 e^x。無參數。預設輸出精度為 max(輸入小數位數, 20)，不是 20 位有效位數。核心接受所有實數輸入。exp(0) 是已初始化的 1；足夠負的輸入可在指定精度下正確捨入為已初始化的零。核心使用區間包圍，僅在捨入已驗證時回傳結果。使用 HalfEven，沒有 mode 參數；核心預算內無法驗證時回傳 NoConvergence，不回傳未經驗證的近似值。輸入保持不變，成功結果已初始化。

```rust
fn exp (& self) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp" locale="zh-Hant" />

## 相關方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
