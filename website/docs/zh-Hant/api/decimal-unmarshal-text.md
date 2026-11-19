---
locale: zh-Hant
source_revision: 6fa89e7d9fd5a43bce6d83be2495a38047592a77e9611dcd781da000b62e4862
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::unmarshal_text"
---
# Decimal::unmarshal_text

將 text 輸入解碼到現有 Decimal。

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

bytes: &[u8]/Uint8Array；修改接收者，回傳 Result<(), DecimalError>/undefined。解析十進位文字，包括首尾空白和指數語法。空文字和字面 null 都不是合法小數文字；空值表示無值時使用 nullable 解碼。成功解析非空值會初始化小數，保留非零值的末尾精度，並將解析得到的零正規化為 scale 零。錯誤保留接收者的完整狀態。程式庫解碼錯誤保留 Unmarshal 及底層類別；資源錯誤同樣發生在修改之前。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-text" locale="zh-Hant" />

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
