---
locale: zh-Hant
source_revision: 8a9067139f7128cd65beb0f6ffcecf849b824634ed652fd6778b7004ed84c520
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl Debug"
---
# ErrorKind::impl Debug

使用 Rust Debug 實現。

```rust
impl Debug for ErrorKind
```

## 參數、回傳值及常規用法

Debug 提供診斷文字，不是穩定的傳輸序列化。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-debug" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
