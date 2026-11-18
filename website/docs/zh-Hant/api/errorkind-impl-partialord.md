---
locale: zh-Hant
source_revision: cd40a54006cbf363c2c029f006d9d975f79108b87d1ec7bdd4a2b4ab26efe940
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl PartialOrd"
---
# ErrorKind::impl PartialOrd

使用 Rust PartialOrd 實現。

```rust
impl PartialOrd for ErrorKind
```

## 參數、回傳值及常規用法

按 Rust 聲明順序排列 ErrorKind，不按嚴重程度或名稱字母序。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialord" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
