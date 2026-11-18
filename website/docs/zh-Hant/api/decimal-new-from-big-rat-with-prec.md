---
locale: zh-Hant
source_revision: 75750ddb09ce29d94b77a840b238d39bf569cfb662c01d997d14bb79d57564c3
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

將有理數捨入到指定小數位數。

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

value: &BigRational；prec: 非負 isize；mode: RoundingMode。bridge 使用精確的 bigint 分子分母物件。與精確轉換不同，允許 1/3 等無限小數。負精度回傳 InvalidPrecision；Unnecessary 遇到必須捨入時回傳 RoundUnnecessary。零分母在呼叫 Rust 前被 bridge 拒絕；零分子回傳 prec 精度的已初始化零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="zh-Hant" />

## 瀏覽器邊界

整數不會經過有損的 JS number 轉換。轉換元組按 Rust 順序保留值和旗標；旗標為 false 時不能假定結果精確。借用輸入保持不變，建構結果已初始化。[資源限制](../resource-policy) 限制整數輸入、輸出精度及有理數係數增長，與程式庫錯誤分開報告。僅 Rust 匯入語法的範例標記為原生測試。

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
