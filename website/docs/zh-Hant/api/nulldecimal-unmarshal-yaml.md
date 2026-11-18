---
locale: zh-Hant
source_revision: e1a6c7014c208ee06566822a33e4d32bdbca21d8076f5cc955ee91bcd5ae501d
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

## 瀏覽器邊界

來源輸入最多 8192 個 UTF-8 位元組。Rust 在核心數值解析之前檢查解碼後的首個 YAML 純量或 XML 元素文字，包含跳脫、CDATA、底線正規化及 YAML 浮點補零。YAML 純量最多 4096 個十六進位數字字元，以限制進位整數分配；保守檢查可能拒絕原生程式庫可接受的輸入。scale、係數和呼叫次數遵循 [資源限制](../resource-policy)。已移動的控制代碼不能再讀寫。

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
