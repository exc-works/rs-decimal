---
locale: zh-Hant
source_revision: 329a8011aecf030884cb84a14d70c08abd0f17ed41927f957f56a638648169ac
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl PgHasArrayType"
---
# NullDecimal::impl PgHasArrayType

使用原生 SQLx PostgreSQL NUMERIC 整合。

```rust
impl PgHasArrayType for NullDecimal
```

## 常規用法與注意事項

需要 postgres feature 和 SQLx 0.8 PostgreSQL 類型。此實現僅適用於原生環境：瀏覽器 Playground 不連接資料庫。array_type_info() 回傳 NUMERIC[] 的 OID 1231，使 SQLx 可以綁定陣列。NullDecimal 陣列元素透過有效性旗標保留 SQL NULL；完整陣列的編解碼由原生 SQLx 查詢執行。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-pghasarraytype" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
