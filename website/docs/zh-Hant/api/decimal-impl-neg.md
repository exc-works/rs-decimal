---
locale: zh-Hant
source_revision: 42bb4c692d19b0113bb004bfa29bdbc283b0bb157f90295cf679a7adf7e3388f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl Neg"
---
# Decimal::impl Neg

使用借用式 Rust Neg 語法。

```rust
impl Neg for & Decimal
```

## 常規用法與注意事項

為 &Decimal 實現，運算元仍可使用。一元 - 保留 scale 並初始化結果，包括預設零。trait 委託同一核心具名方法。瀏覽器不多載 JavaScript 運算子，應呼叫 add/sub/neg。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
