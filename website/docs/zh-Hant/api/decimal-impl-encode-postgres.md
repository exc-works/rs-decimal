---
locale: zh-Hant
source_revision: ca5166a5653606ecffcf807579adb23570a7902ad8051649236d87831df9c02a
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl Encode < '_ , Postgres >"
---
# Decimal::impl Encode < '_ , Postgres >

使用原生 SQLx PostgreSQL NUMERIC 整合。

```rust
impl Encode < '_ , Postgres > for Decimal
```

## 常規用法與注意事項

需要 postgres feature 和 SQLx 0.8 PostgreSQL 類型。此實現僅適用於原生環境：瀏覽器 Playground 不連接資料庫。encode_by_ref(&self, &mut PgArgumentBuffer) 回傳 `Result<IsNull, BoxDynError>`。借用保留數值，成功編碼後才向已有參數緩衝區追加位元組。即使未初始化 Decimal 也編碼為非 NULL 的 NUMERIC 零。二進位格式使用大端 base-10000 分組，保留顯示 scale，不經過浮點數或 BigDecimal。scale 超過 16383 或整數位超過 131072 時拒絕；編碼錯誤保留目標緩衝區。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-encode-postgres" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
