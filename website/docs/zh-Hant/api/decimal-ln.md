---
locale: zh-Hant
source_revision: d7c263dcbf87f527f8688d8c67f916ab9df964fed5eeb802c12da2b176a5e991
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::ln
---
# Decimal::ln

計算自然對數；Rust 回傳 Result<Decimal, DecimalError>，bridge 回傳 Decimal 或拋出結構化錯誤。無參數。預設輸出精度為 max(輸入小數位數, 20)，不是 20 位有效位數。輸入必須為正；零和負數回傳 InvalidLog。ln(1) 精確為零。核心使用區間包圍，僅在捨入已驗證時回傳結果。使用 HalfEven，沒有 mode 參數；核心預算內無法驗證時回傳 NoConvergence，不回傳未經驗證的近似值。輸入保持不變，成功結果已初始化。

```rust
fn ln (& self) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln" locale="zh-Hant" />

## 相關方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
