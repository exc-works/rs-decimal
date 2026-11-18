---
locale: zh-Hant
source_revision: b417c8ff7bf7c7086d2d314348cd490ed506bea3f1d87b65cb817d90defb82d7
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

讀取精確約分後的有理數值。

```rust
fn big_rat (& self) -> BigRational
```

## 參數、回傳值及常規用法

無參數，Rust 回傳 BigRational。bridge 回傳不可變的 {numerator: bigint, denominator: bigint} 快照，正規化後分母為正。Decimal 的精度和末尾零不會作為有理數中繼資料保留；零變為 0/1。輸入僅被借用，保持不變。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="zh-Hant" />

## 瀏覽器邊界

整數不會經過有損的 JS number 轉換。轉換元組按 Rust 順序保留值和旗標；旗標為 false 時不能假定結果精確。借用輸入保持不變，建構結果已初始化。[資源限制](../resource-policy) 限制整數輸入、輸出精度及有理數係數增長，與程式庫錯誤分開報告。僅 Rust 匯入語法的範例標記為原生測試。

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
