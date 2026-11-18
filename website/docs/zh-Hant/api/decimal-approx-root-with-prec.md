---
locale: zh-Hant
source_revision: 3553acc8fce2edd77d63d567220e36aee7b664b18f15fee50a17c4ee358a3283
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

## 瀏覽器邊界

[資源策略](../resource-policy) 將輸出 scale 限制為 1000。非平凡根的 degree 最多為 128，並在分配前檢查縮放、Newton 迭代及中點驗證的中間係數預算。零、正負 1 和一次根保留核心快速路徑，但仍受輸出精度限制。定義域錯誤與負精度錯誤保持程式庫行為；資源錯誤與 NoConvergence 不同。輸入範圍合法不代表每個 degree/prec 組合都能透過資源檢查。

## 相關方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
