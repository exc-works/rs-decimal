---
locale: zh-Hant
source_revision: 5f7a180e14ebabfd6785a42604765a6ece20be12513e46794eb1f0299f911006
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::string"
---
# NullDecimal::string

讀取 nullable 的規範文字。

```rust
fn string (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string。有效值使用內部 Decimal 的規範文字，移除小數末尾零。無效值始終回傳字面 null，與隱藏的內部資料無關。這與 marshal_text 不同：後者對無效值回傳空位元組。讀取不改變有效性或初始化狀態。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="zh-Hant" />

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
