---
locale: zh-Hant
source_revision: 240d5cd6140ed74d787cde34f7f3b1ea17105d751eeea756af72039c1ea346ce
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

以指定精度或輸入精度相除。

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳 Result<Decimal, DecimalError>。prec 指定非負的小數位數；負精度在檢查除數前回傳 InvalidPrecision。零除數回傳 DivideByZero。mode 控制捨入；Unnecessary 對不精確結果回傳 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
