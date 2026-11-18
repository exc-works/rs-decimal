---
locale: zh-Hant
source_revision: 06880d717589581395da1d4538089b3e6373aa9e862e6934eb18c5e117dffc70
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

## 瀏覽器邊界

這組方法有額外的 [資源限制](../resource-policy)：輸出精度最多 128，輸入 scale 最多 256，輸入係數最多 4096 個位元。exp 的普通計算還要求絕對值不超過 64；核心可直接證明捨入為零的負輸入保留快速路徑。較大的輸入 scale 可能使無參數方法的預設精度超過 128，可用 with_prec 明確選擇允許的輸出精度。資源錯誤與程式庫的 NoConvergence 不同。執行可透過 Stop 中斷；重新執行使用新的 Wasm 實例。

## 相關方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
