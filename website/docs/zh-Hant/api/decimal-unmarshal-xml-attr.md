---
locale: zh-Hant
source_revision: 8e284228528617403ea37afb52c965d21d076aee976dac5cd366e305507e8e05
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::unmarshal_xml_attr"
---
# Decimal::unmarshal_xml_attr

將 xml_attr 解碼到現有值。

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

value: &str/string；修改接收者，回傳 `Result<(), DecimalError>`/undefined。Rust 需要 xml。傳入已完成實體解碼的屬性值，而不是 XML 屬性片段："1.23" 有效，但 `"&#49;.23"` 不會再次解碼。去除首尾空白後精確解析小數文字。空內容將接收者重置為未初始化零。成功解碼非空值會初始化小數，並保留非零值的末尾 scale。錯誤保留接收者全部欄位，並保留包含 Unmarshal 的程式庫錯誤類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml-attr" locale="zh-Hant" />

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
