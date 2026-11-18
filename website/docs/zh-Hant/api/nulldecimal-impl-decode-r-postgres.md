---
locale: zh-Hant
source_revision: 54175f7d24af1ed024b6094baba67179a4412d7b895f4da6835c5302cb6117c7
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl Decode < 'r , Postgres >"
---
# NullDecimal::impl Decode < 'r , Postgres >

使用原生 SQLx PostgreSQL NUMERIC 整合。

```rust
impl< 'r > Decode < 'r , Postgres > for NullDecimal
```

## 常規用法與注意事項

需要 postgres feature 和 SQLx 0.8 PostgreSQL 類型。此實現僅適用於原生環境：瀏覽器 Playground 不連接資料庫。decode(PgValueRef) 回傳 `Result<Self, BoxDynError>`。SQLx 提供借用的資料庫值，不應偽造其私有傳輸表示。二進位及文字 NUMERIC 保留數值和顯示 scale，包括帶 scale 的零。SQL NULL 回傳無效 NullDecimal，內部零未初始化；非空值變為有效。拒絕格式錯誤和非有限 NUMERIC，不做捨入。此範例在 CI 的專用 PostgreSQL 服務中執行，瀏覽器不需要外部資料庫。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-decode-r-postgres" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 使用專用 PostgreSQL 服務和 DATABASE_URL，執行 `cargo run --locked -p decimal-web-examples --features postgres-live` 執行此範例。未啟用該 profile 時會明確報告跳過，不能算作資料庫驗證透過。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
