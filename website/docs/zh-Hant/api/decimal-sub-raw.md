---
locale: zh-Hant
source_revision: 4d452c3c2c6d2bed5b36a82866f10106b5603e80ad3916a0ac4e55ced6bfc3f3
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

用 i64 整數改變未縮放的係數。

```rust
fn sub_raw (& self , value : i64) -> Self
```

## 參數、回傳值及常規用法

value: i64，在 bridge 中使用 bigint。保留小數位數：在 scale 為 2 時，加減 1 改變數值 0.01，而非 1。Rust 回傳新 Decimal。bridge 拒絕 number 及超出 i64 範圍的 bigint。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
