---
locale: zh-Hant
source_revision: cbc68f86b580dd7b41c6b741193a12b44e7d04cdf771d632d510adf82d84c1ba
translation_provenance: zh-Hans-terminology-reviewed
api_id: most_significant_bit
---
# most_significant_bit

查找從零起算的最高置位索引。

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## 參數、回傳值及常規用法

value: &BigInt，以 bigint 表示。回傳 Result<usize, DecimalError>/number。零和一都回傳 0，120 回傳 6。負輸入回傳 InvalidArgument。這是自由函數，不是 Decimal 方法；需要係數時應傳入 coefficient()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="zh-Hant" />

## 瀏覽器邊界

整數不會經過有損的 JS number 轉換。轉換元組按 Rust 順序保留值和旗標；旗標為 false 時不能假定結果精確。借用輸入保持不變，建構結果已初始化。[資源限制](../resource-policy) 限制整數輸入、輸出精度及有理數係數增長，與程式庫錯誤分開報告。僅 Rust 匯入語法的範例標記為原生測試。

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
