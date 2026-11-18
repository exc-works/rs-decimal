---
locale: zh-Hant
source_revision: 753b858d717076ef46e98ae0c0b43697fb31b0651286396a37e7378758f67207
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

## 瀏覽器邊界

解碼輸入最多 8192 位元組，參數字串還在複製到 Wasm 前檢查長度。Rust 對文字及 JSON 解碼後的字串再次檢查 [十進位資源限制](../resource-policy)，因此跳脫字元不能繞過指數展開或精度限制。無效 UTF-8 和 JSON/小數語法仍由核心報告。輸入錯誤和資源錯誤都不會部分更新接收者；已移動的物件不可再讀寫。

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
