---
locale: zh-Hant
source_revision: cee1a90bda7ee828acf5a855ac4b338f7c02ca9ef05b0b3742b98db12c7cf78a
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

## 瀏覽器邊界

來源輸入最多 8192 個 UTF-8 位元組。Rust 在核心數值解析之前檢查解碼後的首個 YAML 純量或 XML 元素文字，包含跳脫、CDATA、底線正規化及 YAML 浮點補零。YAML 純量最多 4096 個十六進位數字字元，以限制進位整數分配；保守檢查可能拒絕原生程式庫可接受的輸入。scale、係數和呼叫次數遵循 [資源限制](../resource-policy)。已移動的控制代碼不能再讀寫。

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
