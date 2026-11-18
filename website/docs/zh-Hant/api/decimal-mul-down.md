---
locale: zh-Hant
source_revision: 968f32ddfefb50259ee32498127a1ae440b4c11919fa84ceb9ba802a6f23f0ec
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::mul_down
---
# Decimal::mul_down

按明確的輸出精度相乘。

```rust
fn mul_down (& self , other : & Self) -> Self
```

## 參數、回傳值及常規用法

other: &Decimal。結果小數位數為兩個輸入中較大者，而非二者之和。保留乘積全部數字請使用 mul_exact。Down 向零截斷，包括負結果。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
