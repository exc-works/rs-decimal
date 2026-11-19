---
locale: zh-Hant
source_revision: b2646b52b58adc30058be6fedbac290221f3b2b947946b06b25fe8783f697afc
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::unmarshal_json"
---
# Decimal::unmarshal_json

將 json 輸入解碼到現有 Decimal。

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

bytes: &[u8]/Uint8Array；修改接收者，回傳 Result<(), DecimalError>/undefined。Rust 需要 json feature。傳入原始 UTF-8 JSON 位元組：數字不經過浮點數，字串先反跳脫再精確解析小數。不要先把數字 token 用 JSON.parse 解析為 JS number。JSON 空白僅包括空格、tab、CR、LF。JSON null 保留現有 Decimal 的完整狀態，包括初始化狀態。成功解析非空值會初始化小數，保留非零值的末尾精度，並將解析得到的零正規化為 scale 零。錯誤保留接收者的完整狀態。程式庫解碼錯誤保留 Unmarshal 及底層類別；資源錯誤同樣發生在修改之前。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-json" locale="zh-Hant" />

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
