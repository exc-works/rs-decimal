---
locale: zh-Hant
source_revision: 5aa2d7144a94e759cd81dc307aff2c18ce7db36a83dc3e91c6d08804e1cdf2c5
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::impl PartialEq"
---
# DecimalError::impl PartialEq

使用 Rust PartialEq 實現。

```rust
impl PartialEq for DecimalError
```

## 參數、回傳值及常規用法

相等比較完整錯誤訊息和類別，或確切列舉變體。DecimalError 相等比匹配一個類別更嚴格。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-partialeq" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
