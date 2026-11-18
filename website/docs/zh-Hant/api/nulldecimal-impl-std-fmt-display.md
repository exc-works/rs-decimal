---
locale: zh-Hant
source_revision: 425ca68f655481c858e4668ea2e1df0ddc734454c90f72d356190ed7ac4eb22e
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display"
---
# NullDecimal::impl std :: fmt :: Display

透過 Rust Display 格式化 nullable 值。

```rust
impl std :: fmt :: Display for NullDecimal
```

## 參數、回傳值及常規用法

Display 委託 nullable string()：有效時輸出內部規範文字，無效時輸出 null。格式器寫入文字，不是 JSON token 或位元組緩衝區。format! 呼叫 Display::fmt；此 trait 語法在原生環境執行，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display" locale="zh-Hant" />

## 瀏覽器邊界

解碼輸入最多 8192 位元組，參數字串還在複製到 Wasm 前檢查長度。Rust 對文字及 JSON 解碼後的字串再次檢查 [十進位資源限制](../resource-policy)，因此跳脫字元不能繞過指數展開或精度限制。無效 UTF-8 和 JSON/小數語法仍由核心報告。輸入錯誤和資源錯誤都不會部分更新接收者；已移動的物件不可再讀寫。

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
