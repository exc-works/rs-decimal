---
locale: zh-Hant
source_revision: b6885bfd89f5c1e43b9c4a8ab207ccde2aef139d913f8cd028eda20cd2081e75
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::unmarshal_text"
---
# NullDecimal::unmarshal_text

將 text 輸入解碼到現有 NullDecimal。

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

bytes: &[u8]/Uint8Array；修改接收者，回傳 Result<(), DecimalError>/undefined。解析十進位文字，包括首尾空白和指數語法。空輸入清除有效旗標並重置內部 Decimal；字面 null 不是合法小數文字。成功解析非空值會初始化小數，保留非零值的末尾精度，並將解析得到的零正規化為 scale 零。同時設定 valid=true。錯誤保留接收者的完整狀態。程式庫解碼錯誤保留 Unmarshal 及底層類別；資源錯誤同樣發生在修改之前。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-text" locale="zh-Hant" />

## 瀏覽器邊界

解碼輸入最多 8192 位元組，參數字串還在複製到 Wasm 前檢查長度。Rust 對文字及 JSON 解碼後的字串再次檢查 [十進位資源限制](../resource-policy)，因此跳脫字元不能繞過指數展開或精度限制。無效 UTF-8 和 JSON/小數語法仍由核心報告。輸入錯誤和資源錯誤都不會部分更新接收者；已移動的物件不可再讀寫。

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
