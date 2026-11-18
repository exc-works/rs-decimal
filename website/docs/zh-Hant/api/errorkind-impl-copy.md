---
locale: zh-Hant
source_revision: a66891e5a69e79ad1f199a0dd34da946899f3deff1662608f76ae3c569145632
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

使用 Rust Copy 實現。

```rust
impl Copy for ErrorKind
```

## 參數、回傳值及常規用法

Copy 複製此小型列舉，不深拷貝借用的純量資料。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
