---
locale: zh-Hant
source_revision: 13b73a8f1a7c329ddb10d60fb148021dcab79998f09acc291519f00e02fa1abd
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl Type < Postgres >::compatible"
---
# NullDecimal::impl Type < Postgres >::compatible

使用原生 SQLx PostgreSQL NUMERIC 整合。

```rust
fn compatible (ty : & PgTypeInfo) -> bool
```

## 常規用法與注意事項

需要 postgres feature 和 SQLx 0.8 PostgreSQL 類型。此實現僅適用於原生環境：瀏覽器 Playground 不連接資料庫。type_info() 回傳 NUMERIC 的 OID 1700。compatible(ty) 只接受該確切 OID，不接受 float8、整數或任意文字類型，不承諾隱式浮點轉換。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-type-postgres-compatible" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
