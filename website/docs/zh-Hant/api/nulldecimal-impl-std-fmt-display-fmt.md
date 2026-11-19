---
locale: zh-Hant
source_revision: da8cebe24a220cbee0329a50db3c796d6390c270eb7d574cffaac17c4ddc1264
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display::fmt"
---
# NullDecimal::impl std :: fmt :: Display::fmt

透過 Rust Display 格式化 nullable 值。

```rust
fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result
```

## 參數、回傳值及常規用法

Display 委託 nullable string()：有效時輸出內部規範文字，無效時輸出 null。格式器寫入文字，不是 JSON token 或位元組緩衝區。format! 呼叫 Display::fmt；此 trait 語法在原生環境執行，不是瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display-fmt" locale="zh-Hant" />

## 相關方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
