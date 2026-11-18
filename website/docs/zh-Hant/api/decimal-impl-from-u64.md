---
locale: zh-Hant
source_revision: fc3ed611d72b0cb1f91a16643e1645524bf0ecd7713a4a9f4adaaf5279855e77
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl From < u64 >"
---
# Decimal::impl From < u64 >

透過 Rust From 精確轉換 u64。

```rust
impl From<u64> for Decimal
```

## 常規用法與注意事項

消耗 u64 並回傳已初始化、scale 為零的 Decimal。不經過浮點數，不捨入；對來源整數類型轉換不會失敗。零也會初始化，與 Decimal::default() 不同。來源範圍由 Rust 整數類型決定；isize/usize 寬度依賴原生目標。瀏覽器使用明確的具名建構函數或 ConfigValue 有符號/無符號變體，Rust From/Into 語法僅用於原生環境。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-u64" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
