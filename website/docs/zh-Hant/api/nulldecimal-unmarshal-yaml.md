---
locale: zh-Hant
source_revision: dbc15c20de89c4c0e7f4a73778b0f1b410d8c87e2c8de57f1e13798c66980cb2
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::unmarshal_yaml"
---
# NullDecimal::unmarshal_yaml

將 yaml 解碼到現有值。

```rust
fn unmarshal_yaml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

source: &str/string；修改接收者，回傳 `Result<(), DecimalError>`/undefined。Rust 需要 yaml。傳入原始 YAML 來源文字，使 Rust 能處理引號、標籤和精確數字。普通 012 按八進位解析為 10，而帶引號的 "012" 或 !!str 012 解析為十進位 12。數字標籤、進位和底線遵循核心解析規則：普通 1_000 為 1000，但帶引號的 "1_000" 無效。布林值、集合和區塊純量被拒絕。僅首個文件提供值，但後續文件仍須語法有效。null 或空文件清除 valid 並將內部 Decimal 重置為未初始化零。成功解碼非空值會初始化小數，並保留非零值的末尾 scale。同時設定 valid=true。錯誤保留接收者全部欄位，並保留包含 Unmarshal 的程式庫錯誤類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-yaml" locale="zh-Hant" />

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
