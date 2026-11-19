---
locale: zh-Hant
source_revision: e94c40441044730df649ba261de89daead4c5ce018d30a42e07e0ff558def35c
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

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
