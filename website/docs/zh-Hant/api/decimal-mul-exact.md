---
locale: zh-Hant
source_revision: 5fb085491b7b8fca3cac801efd640150df3603d315a453ad87ad49323fbb68e2
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

按精確方式相乘。

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal。結果小數位數為兩個輸入之和，保留乘積的所有數字。核心精度溢出回傳 Overflow。沒有捨入模式參數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
