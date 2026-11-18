---
locale: zh-Hant
source_revision: 46411f1a8f7889f0ec3654fae25da003c49b8d268105e0431ed703cb61c031fe
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_integer
---
# Decimal::is_integer

檢查數值的小數部分。

```rust
fn is_integer (& self) -> bool
```

## 參數、回傳值及常規用法

無參數，回傳 bool/boolean。檢查數值而非 scale 是否為零：12.00 仍是整數。預設零也是整數，沒有小數部分。查詢不會初始化或修改輸入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
