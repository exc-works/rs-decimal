---
locale: zh-Hant
source_revision: 8ca020b2d56652a32165b4756446630d2a88be60a068a89e4f65ce218930abe5
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::marshal_xml_attr"
---
# Decimal::marshal_xml_attr

取得 xml_attr 語義內容。

```rust
fn marshal_xml_attr (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string。Rust 需要 xml feature，瀏覽器已啟用。回傳已解碼的屬性內容；呼叫者提供屬性名並進行 XML 跳脫。規範小數文字移除小數末尾零。未初始化零回傳空內容；已初始化零回傳 "0"。讀取不改變初始化或有效旗標。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml-attr" locale="zh-Hant" />

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
