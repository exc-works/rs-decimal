---
locale: zh-Hant
source_revision: 4edb7e0a9fed4047c94ce74a3f224028a33e2b9c87f41b5e744d6d68a1abd2e7
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl Neg::neg"
---
# Decimal::impl Neg::neg

使用借用式 Rust Neg 語法。

```rust
fn neg (self) -> Decimal
```

## 常規用法與注意事項

為 &Decimal 實現，運算元仍可使用。一元 - 保留 scale 並初始化結果，包括預設零。trait 委託同一核心具名方法。瀏覽器不多載 JavaScript 運算子，應呼叫 bridge 的 add/sub/neg。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-neg" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
