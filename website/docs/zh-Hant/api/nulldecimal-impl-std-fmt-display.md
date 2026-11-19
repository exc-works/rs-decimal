---
locale: zh-Hant
source_revision: 4e9262b5d73b0bfa6dbb9a564894dae85973da389d298cf19e5836d09170ed0d
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

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
