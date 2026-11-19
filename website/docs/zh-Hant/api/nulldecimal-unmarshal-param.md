---
locale: zh-Hant
source_revision: 7c807773ded0adcdcd91027cab2d90a4029760f6cf1ca4fd49648d1e294b17d6
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::unmarshal_param"
---
# NullDecimal::unmarshal_param

將 param 輸入解碼到現有 NullDecimal。

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

param: &str/string；修改接收者，回傳 Result<(), DecimalError>/undefined。解析十進位文字，包括首尾空白和指數語法。空輸入清除有效旗標並重置內部 Decimal；字面 null 不是合法小數文字。成功解析非空值會初始化小數，保留非零值的末尾精度，並將解析得到的零正規化為 scale 零。同時設定 valid=true。錯誤保留接收者的完整狀態。程式庫解碼錯誤保留 Unmarshal 及底層類別；資源錯誤同樣發生在修改之前。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-param" locale="zh-Hant" />

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
