---
locale: zh-Hant
source_revision: 53a165ed47764f5f661387617a1b68ba2abdefa181cac1cde270d551da87797c
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sqrt_with_prec
---
# Decimal::sqrt_with_prec

計算平方根。沒有 degree 參數。負輸入回傳 NegativeRoot。prec: isize，以整數 number 傳輸，表示小數位數。負精度在定義域檢查前回傳 InvalidPrecision。使用 HalfEven 捨入；例如 sqrt(6.25) 保留零位時，正中間值 2.5 捨入為 2。雖然名稱包含 approx_root，Rust 會用整數比較驗證捨入；核心計算或迭代預算耗盡時回傳 NoConvergence。輸入僅借用且保持不變；結果（包括預設零）已初始化。

```rust
fn sqrt_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt-with-prec" locale="zh-Hant" />

## 相關方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
