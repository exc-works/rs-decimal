---
locale: zh-Hant
source_revision: ce941de7d4ef355a538d2c2c33a609ab4eeb3b4645eb801e81a24755fcf0ca5a
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

按明確的輸出精度相乘。

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal。結果保留 prec 個小數位；prec 必須非負，否則回傳 InvalidPrecision。顯式 mode 控制捨入；Unnecessary 在必須丟棄非零數字時回傳 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
