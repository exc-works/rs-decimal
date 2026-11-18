---
locale: zh-Hant
source_revision: 49e05880780d4157422be53a7d3d5c1a0d8d4e809a77bd78ae251e8f6c7889c8
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sqrt
---
# Decimal::sqrt

計算平方根。Rust 回傳 Result<Decimal, DecimalError>，bridge 回傳 Decimal 或拋出結構化錯誤。沒有 degree 參數。負輸入回傳 NegativeRoot。預設精度為 max(輸入小數位數, 20)，不是 20 位有效位數。使用 HalfEven 捨入；例如 sqrt(6.25) 保留零位時，正中間值 2.5 捨入為 2。雖然名稱包含 approx_root，Rust 會用整數比較驗證捨入；核心計算或迭代預算耗盡時回傳 NoConvergence。輸入僅借用且保持不變；結果（包括預設零）已初始化。

```rust
fn sqrt (& self) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt" locale="zh-Hant" />

## 瀏覽器邊界

[資源策略](../resource-policy) 將輸出 scale 限制為 1000。非平凡根的 degree 最多為 128，並在分配前檢查縮放、Newton 迭代及中點驗證的中間係數預算。零、正負 1 和一次根保留核心快速路徑，但仍受輸出精度限制。定義域錯誤與負精度錯誤保持程式庫行為；資源錯誤與 NoConvergence 不同。輸入範圍合法不代表每個 degree/prec 組合都能透過資源檢查。

## 相關方法

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
