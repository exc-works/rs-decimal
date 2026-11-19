---
locale: zh-Hant
source_revision: b8d3cac3d5f9251c9158f0efb63e99a5a338182fad355d14b4c984c245247f04
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::marshal_xml_attr"
---
# NullDecimal::marshal_xml_attr

取得 xml_attr 語義內容。

```rust
fn marshal_xml_attr (& self) -> Option < String >
```

## 參數、回傳值及常規用法

無參數，回傳 `Option<String>`/string | null。Rust 需要 xml feature，瀏覽器已啟用。回傳已解碼的屬性內容；呼叫者提供屬性名並進行 XML 跳脫。規範小數文字移除小數末尾零。無效 nullable 回傳 None/null：應省略屬性，而不是輸出空屬性。內部零未初始化的有效值回傳 "0"。讀取不改變初始化或有效旗標；瀏覽器將 Rust None 映射為 null。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml-attr" locale="zh-Hant" />

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
