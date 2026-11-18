---
locale: zh-Hant
source_revision: 7fadd186172ed1d2675ccc16ae4020af71fd55a578a479ca70e021285e76f370
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl From < BigInt >"
---
# Decimal::impl From < BigInt >

透過 Rust From 精確轉換 BigInt。

```rust
impl From < BigInt > for Decimal
```

## 常規用法與注意事項

消耗 BigInt 並回傳已初始化、scale 為零的 Decimal。不經過浮點數，不捨入；對來源整數類型轉換不會失敗。零也會初始化，與 Decimal::default() 不同。BigInt 所有權移入小數。瀏覽器使用明確的具名建構函數或 ConfigValue 有符號/無符號變體，Rust From/Into 語法僅用於原生環境。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-bigint" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
