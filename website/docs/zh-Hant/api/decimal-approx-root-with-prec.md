---
locale: zh-Hant
source_revision: 8eec4d40194be2cd158d623391729d3b1e9d3b0532af669f3499b5168782b68c
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::approx_root_with_prec
---
# Decimal::approx_root_with_prec

計算正整數次根。Rust 回傳 Result<Decimal, DecimalError>，bridge 回傳 Decimal 或拋出結構化錯誤。degree: i64，以 bigint 傳輸；必須為正，否則回傳 InvalidRoot。奇數次根允許負輸入；負數的偶數次根回傳 NegativeRoot。一次根等同於 HalfEven 精度調整。prec: isize，以整數 number 傳輸，表示小數位數。負精度在定義域檢查前回傳 InvalidPrecision。使用 HalfEven 捨入；例如 sqrt(6.25) 保留零位時，正中間值 2.5 捨入為 2。雖然名稱包含 approx_root，Rust 會用整數比較驗證捨入；核心計算或迭代預算耗盡時回傳 NoConvergence。輸入僅借用且保持不變；結果（包括預設零）已初始化。

```rust
fn approx_root_with_prec (& self , degree : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root-with-prec" locale="zh-Hant" />

## 相關方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
