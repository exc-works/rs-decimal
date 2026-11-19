---
locale: zh-Hant
source_revision: 7b63356b05d421377bb8ec0a29f08c338eaab7596c00a0c507a722a2de09d089
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::marshal_xml"
---
# NullDecimal::marshal_xml

取得 xml 語義內容。

```rust
fn marshal_xml (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string。Rust 需要 xml feature，瀏覽器已啟用。回傳元素字元內容；呼叫者提供元素名、命名空間並進行 XML 跳脫。規範小數文字移除小數末尾零。無效 nullable 回傳空內容；內部零未初始化的有效值回傳 "0"。讀取不改變初始化或有效旗標。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml" locale="zh-Hant" />

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
