---
locale: zh-Hant
source_revision: 25be20babe80f8d8b33e901842da879899c54ff8a78f97427ea80f9ca18a005d
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::marshal_yaml"
---
# Decimal::marshal_yaml

取得 yaml 語義內容。

```rust
fn marshal_yaml (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string。Rust 需要 yaml feature，瀏覽器已啟用。回傳 YAML 語義字串純量，不是序列化後的 YAML 文件。交給 YAML emitter 時按字串處理，由 emitter 負責引號和標籤。規範小數文字移除小數末尾零。預設未初始化零回傳 "0"。讀取不改變初始化或有效旗標。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-yaml" locale="zh-Hant" />

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
