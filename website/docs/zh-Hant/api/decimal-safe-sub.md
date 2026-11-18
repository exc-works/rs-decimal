---
locale: zh-Hant
source_revision: 8969e894369c10d2e27ead76f07916d45a77105ffad604c0d4f46fcf59b84751
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

精確相減，要求結果非負。

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；回傳小數位數為兩個輸入中較大者的新 Decimal。對齊是精確的，不丟棄數字。Rust 回傳 Result：僅要求結果非負，不要求每個輸入非負。負結果回傳 InvalidArgument，允許零結果。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
