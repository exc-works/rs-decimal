---
locale: zh-Hant
source_revision: f07542c46478fd9edd09bbf3af9bd03d15608eb33e9ed99beb51b022df098d6a
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

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
