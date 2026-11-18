---
locale: zh-Hant
source_revision: 8110d2fd30808fd737c3f5730a5be2d19cf89f407f5d688859de476689342458
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::BigInt
---
# reexport::BigInt

從 crate 根路徑匯入 BigInt。

```rust
pub use num_bigint::BigInt as BigInt
```

## 參數、回傳值及常規用法

這是 Rust 的公開重新匯出，不是新類型或新實現。匯入語法在原生環境測試。瀏覽器對應項分別為 bigint、精確有理數欄位物件以及全域 most_significant_bit 函數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigint" locale="zh-Hant" />

## 瀏覽器邊界

整數不會經過有損的 JS number 轉換。轉換元組按 Rust 順序保留值和旗標；旗標為 false 時不能假定結果精確。借用輸入保持不變，建構結果已初始化。[資源限制](../resource-policy) 限制整數輸入、輸出精度及有理數係數增長，與程式庫錯誤分開報告。僅 Rust 匯入語法的範例標記為原生測試。

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
