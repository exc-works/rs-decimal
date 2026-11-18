---
locale: zh-Hant
source_revision: ce7339c2b8115fd42455e27eb624e03a51de3a069de71c01158ea752ddf6833a
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

統計未縮放係數絕對值的位元數。

```rust
fn bit_len (& self) -> usize
```

## 參數、回傳值及常規用法

無參數，Rust 回傳 usize，bridge 回傳 number。不應用 scale，也不計符號位：-1.20 的係數為 -120，位數為 7。零（包括預設零）回傳 0。這不是最高置位的索引。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="zh-Hant" />

## 瀏覽器邊界

整數不會經過有損的 JS number 轉換。轉換元組按 Rust 順序保留值和旗標；旗標為 false 時不能假定結果精確。借用輸入保持不變，建構結果已初始化。[資源限制](../resource-policy) 限制整數輸入、輸出精度及有理數係數增長，與程式庫錯誤分開報告。僅 Rust 匯入語法的範例標記為原生測試。

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
