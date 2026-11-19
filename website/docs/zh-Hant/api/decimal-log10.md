---
locale: zh-Hant
source_revision: ea6f97f51071f45a8c9bf2f4df22faacd07f9c9b025ed9b3ebf43ae9fab4e095
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::log10
---
# Decimal::log10

計算以 10 為底的對數。無參數。預設輸出精度為 max(輸入小數位數, 20)，不是 20 位有效位數。輸入必須為正；零和負數回傳 InvalidLog。底數的精確冪具有整數對數，並保留輸出小數位數。核心使用區間包圍，僅在捨入已驗證時回傳結果。使用 HalfEven，沒有 mode 參數；核心預算內無法驗證時回傳 NoConvergence，不回傳未經驗證的近似值。輸入保持不變，成功結果已初始化。

```rust
fn log10 (& self) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10" locale="zh-Hant" />

## 相關方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
