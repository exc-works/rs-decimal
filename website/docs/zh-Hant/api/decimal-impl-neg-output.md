---
locale: zh-Hant
source_revision: d8d63d1c0e264a86d3f7608f1fd35c5b3c0175c1d05354d8cfc9826d8215e864
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl Neg::Output"
---
# Decimal::impl Neg::Output

使用借用式 Rust Neg 語法。

```rust
type Output = Decimal
```

## 常規用法與注意事項

為 &Decimal 實現，運算元仍可使用。一元 - 保留 scale 並初始化結果，包括預設零。trait 委託同一核心具名方法。瀏覽器不多載 JavaScript 運算子，應呼叫 bridge 的 add/sub/neg。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-output" locale="zh-Hant" />

## 執行方式

此條目是 Rust 原生 trait 語法，不提供同名 JavaScript 運算子或 SQLx 物件。 此範例由預設原生文件測試執行。

## 相關方法

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
