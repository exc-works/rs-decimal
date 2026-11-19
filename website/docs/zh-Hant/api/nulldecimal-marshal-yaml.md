---
locale: zh-Hant
source_revision: 634992be26686e49442a948ce62a476a4bcc34db498b408945e4ad08ce9f7530
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::marshal_yaml"
---
# NullDecimal::marshal_yaml

取得 yaml 語義內容。

```rust
fn marshal_yaml (& self) -> Option < String >
```

## 參數、回傳值及常規用法

無參數，回傳 `Option<String>`/string | null。Rust 需要 yaml feature，瀏覽器已啟用。回傳 YAML 語義字串純量，不是序列化後的 YAML 文件。交給 YAML emitter 時按字串處理，由 emitter 負責引號和標籤。規範小數文字移除小數末尾零。無效 nullable 回傳 None/null，表示 YAML null 純量；內部零未初始化的有效值回傳 "0"。讀取不改變初始化或有效旗標；瀏覽器將 Rust None 映射為 null。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="zh-Hant" />

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
