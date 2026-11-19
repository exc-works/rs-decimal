---
locale: zh-Hant
source_revision: 90631f865fb2217f0d8137591d3ab6e82b220d3c98120fa1ec15f91a894f2664
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::marshal_xml"
---
# Decimal::marshal_xml

取得 xml 語義內容。

```rust
fn marshal_xml (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string。Rust 需要 xml feature，瀏覽器已啟用。回傳元素字元內容；呼叫者提供元素名、命名空間並進行 XML 跳脫。規範小數文字移除小數末尾零。未初始化零回傳空內容；已初始化零回傳 "0"。讀取不改變初始化或有效旗標。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml" locale="zh-Hant" />

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
