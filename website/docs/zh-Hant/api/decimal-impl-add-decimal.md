---
locale: zh-Hant
source_revision: ea8ced6433945eb5bf565e52805d90ed9b2cd05d626678d4f8db01d933c5fdd2
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl Add < & Decimal >"
---
# Decimal::impl Add < & Decimal >

使用借用式 Rust Add 語法。

```rust
impl Add < & Decimal > for & Decimal
```

## 常規用法與注意事項

為 &Decimal 實現，運算元仍可使用。精確結果使用較大的運算元 scale，結果已初始化。關聯 Output 為 Decimal。允許負結果，不應用 safe_add/safe_sub 的非負結果要求。trait 委託同一核心具名方法。瀏覽器不多載 JavaScript 運算子，應呼叫 add/sub/neg。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-add-decimal" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
