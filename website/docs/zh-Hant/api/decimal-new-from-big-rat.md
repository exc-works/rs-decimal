---
locale: zh-Hant
source_revision: c7909226b96ffde911017d8ead0a3f2edd2a0c894ee744bf3bcbea66574f1bf5
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_big_rat
---
# Decimal::new_from_big_rat

將有理數轉換為精確的有限小數。

```rust
fn new_from_big_rat (value : & BigRational) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

value: &BigRational；bridge 接受 {numerator: bigint, denominator: bigint}。分母必須非零，否則為 bridge 參數錯誤；Rust BigRational 正規化符號並約分。約分後分母只能含因子 2 和 5，才能得到有限小數；其他因子回傳 InvalidFormat。結果已初始化。瀏覽器在約分後、呼叫核心轉換前檢查輸出精度和係數增長。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat" locale="zh-Hant" />

## 瀏覽器邊界

整數不會經過有損的 JS number 轉換。轉換元組按 Rust 順序保留值和旗標；旗標為 false 時不能假定結果精確。借用輸入保持不變，建構結果已初始化。[資源限制](../resource-policy) 限制整數輸入、輸出精度及有理數係數增長，與程式庫錯誤分開報告。僅 Rust 匯入語法的範例標記為原生測試。

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
