---
locale: zh-Hant
source_revision: a269819e7b61d5a164cb35c2c28efef7424a30b1113a1fd7abc0f0da7fe8a5f9
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::approx_root
---
# Decimal::approx_root

計算正整數次根。Rust 回傳 Result<Decimal, DecimalError>，bridge 回傳 Decimal 或拋出結構化錯誤。degree: i64，以 bigint 傳輸；必須為正，否則回傳 InvalidRoot。奇數次根允許負輸入；負數的偶數次根回傳 NegativeRoot。一次根等同於 HalfEven 精度調整。預設精度為 max(輸入小數位數, 20)，不是 20 位有效位數。使用 HalfEven 捨入；例如 sqrt(6.25) 保留零位時，正中間值 2.5 捨入為 2。雖然名稱包含 approx_root，Rust 會用整數比較驗證捨入；核心計算或迭代預算耗盡時回傳 NoConvergence。輸入僅借用且保持不變；結果（包括預設零）已初始化。

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="zh-Hant" />

## 瀏覽器邊界

[資源策略](../resource-policy) 將輸出 scale 限制為 1000。非平凡根的 degree 最多為 128，並在分配前檢查縮放、Newton 迭代及中點驗證的中間係數預算。零、正負 1 和一次根保留核心快速路徑，但仍受輸出精度限制。定義域錯誤與負精度錯誤保持程式庫行為；資源錯誤與 NoConvergence 不同。輸入範圍合法不代表每個 degree/prec 組合都能透過資源檢查。

## 相關方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
