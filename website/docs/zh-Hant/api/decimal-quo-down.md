---
locale: zh-Hant
source_revision: 96feaba7858db9f7e7efab2f2187c38abada064aba6bc3c66092f84f0b94b74f
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::quo_down
---
# Decimal::quo_down

以指定精度或輸入精度相除。

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳 Result<Decimal, DecimalError>。輸出小數位數為兩個輸入中較大者，因此兩個整數輸入也產生整數：1 / 3 得到 0。需要小數結果請使用 quo_with_prec。零除數回傳 DivideByZero。Down 向零截斷，不是向負無窮。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
