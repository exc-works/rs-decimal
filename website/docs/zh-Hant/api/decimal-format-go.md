---
locale: zh-Hant
source_revision: f9b1eaa0c6a10e9f32908baa1f2c09a9aafeb3265bffa095bb0a0b210e1e1f05
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::format_go
---
# Decimal::format_go

應用一個相容 Go 的格式動詞及選項。

```rust
fn format_go (& self , f : GoFormat) -> Result < String , DecimalError >
```

## 參數、回傳值及常規用法

f: GoFormat；回傳 Result<String, DecimalError>/string。接受單個動詞，不接受 %.2f 這樣的 printf 模板。支援 v/s/q/d/f/F/e/E/g/G/b。捨入使用 HalfEven。未知動詞及對非整數使用 d 會產生 %! 文字標記，不拋出錯誤。定點精度超出原生有符號範圍回傳 InvalidPrecision；瀏覽器另有寬度、精度和分配限制。JS 省略的欄位使用 Rust 預設值，未知欄位和錯誤欄位類型會被拒絕。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-go" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
