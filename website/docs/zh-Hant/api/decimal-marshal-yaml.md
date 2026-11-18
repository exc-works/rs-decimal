---
locale: zh-Hant
source_revision: 2596ef8441aa7ec726abb87c7af62732ec4f2729da79754eba71b3cb01735596
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

## 瀏覽器邊界

來源輸入最多 8192 個 UTF-8 位元組。Rust 在核心數值解析之前檢查解碼後的首個 YAML 純量或 XML 元素文字，包含跳脫、CDATA、底線正規化及 YAML 浮點補零。YAML 純量最多 4096 個十六進位數字字元，以限制進位整數分配；保守檢查可能拒絕原生程式庫可接受的輸入。scale、係數和呼叫次數遵循 [資源限制](../resource-policy)。已移動的控制代碼不能再讀寫。

## 相關方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
