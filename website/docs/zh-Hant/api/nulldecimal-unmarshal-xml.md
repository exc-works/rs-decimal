---
locale: zh-Hant
source_revision: bb5fda951c514c30196177535ed36df93d91d15de5889b17222eea8aae392724
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::unmarshal_xml"
---
# NullDecimal::unmarshal_xml

將 xml 解碼到現有值。

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

source: &str/string；修改接收者，回傳 `Result<(), DecimalError>`/undefined。Rust 需要 xml。解碼首個 XML 元素，組合直接文字、CDATA 和解碼後的實體參照。忽略巢狀子元素的文字，並連接子元素前後的直接文字。忽略後續元素。核心驗證名稱、屬性、字元參照和匹配的結束標籤。去除首尾空白後內容為空時清除 valid 並將內部 Decimal 重置為未初始化零。成功解碼非空值會初始化小數，並保留非零值的末尾 scale。同時設定 valid=true。錯誤保留接收者全部欄位，並保留包含 Unmarshal 的程式庫錯誤類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml" locale="zh-Hant" />

## 瀏覽器邊界

來源輸入最多 8192 個 UTF-8 位元組。Rust 在核心數值解析之前檢查解碼後的首個 YAML 純量或 XML 元素文字，包含跳脫、CDATA、底線正規化及 YAML 浮點補零。YAML 純量最多 4096 個十六進位數字字元，以限制進位整數分配；保守檢查可能拒絕原生程式庫可接受的輸入。scale、係數和呼叫次數遵循 [資源限制](../resource-policy)。已移動的控制代碼不能再讀寫。

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
