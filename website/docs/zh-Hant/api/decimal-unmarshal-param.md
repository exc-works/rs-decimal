---
locale: zh-Hant
source_revision: c4d07db94cb802a1abe829e9593804f52c2de0bb0ad545606bf47e6e769fde05
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

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
