---
locale: zh-Hant
source_revision: 922fb0ccad86b971140048111e8d1bdfebf3b2480d201800dddb60a3c6890884
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sub
---
# Decimal::sub

精確相減。

```rust
fn sub (& self , other : & Self) -> Self
```

## 參數、回傳值及常規用法

other: &Decimal；回傳小數位數為兩個輸入中較大者的新 Decimal。對齊是精確的，不丟棄數字。Rust 方法本身不回傳錯誤；瀏覽器仍可能因分配限制回傳資源錯誤。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
