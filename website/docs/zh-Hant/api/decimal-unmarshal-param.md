---
locale: zh-Hant
source_revision: 00df85dbc78a408d1e3a1357be14942f9ab79794d46b2e7624e5bd2c1ceeaef3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::unmarshal_param"
---
# Decimal::unmarshal_param

將 param 輸入解碼到現有 Decimal。

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

param: &str/string；修改接收者，回傳 Result<(), DecimalError>/undefined。解析十進位文字，包括首尾空白和指數語法。空文字和字面 null 都不是合法小數文字；空值表示無值時使用 nullable 解碼。成功解析非空值會初始化小數，保留非零值的末尾精度，並將解析得到的零正規化為 scale 零。錯誤保留接收者的完整狀態。程式庫解碼錯誤保留 Unmarshal 及底層類別；資源錯誤同樣發生在修改之前。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-param" locale="zh-Hant" />

## 瀏覽器邊界

解碼輸入最多 8192 位元組，參數字串還在複製到 Wasm 前檢查長度。Rust 對文字及 JSON 解碼後的字串再次檢查 [十進位資源限制](../resource-policy)，因此跳脫字元不能繞過指數展開或精度限制。無效 UTF-8 和 JSON/小數語法仍由核心報告。輸入錯誤和資源錯誤都不會部分更新接收者；已移動的物件不可再讀寫。

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
