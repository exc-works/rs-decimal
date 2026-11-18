---
locale: zh-Hant
source_revision: c70b9c986ae1c2d1db6f94784745de5a4d3bb8a7cf3f6af3550e9be48e0f551f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

建立包含一個類別的錯誤。

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## 參數、回傳值及常規用法

kind: ErrorKind，message: impl `Into<String>`/string；回傳新 DecimalError。訊息供人閱讀，不決定類別成員關係。瀏覽器具名建構函數建立可釋放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。拋出的程式庫異常是原生 JS Error，帶 category="library" 和 kinds 陣列；此時使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved bridge 錯誤與程式庫錯誤分開，不帶程式庫類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
