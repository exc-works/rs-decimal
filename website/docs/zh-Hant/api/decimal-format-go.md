---
locale: zh-Hant
source_revision: 0d692c90930959d30189d9870456cce3ed78a046b51b42c8004981e031486270
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

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
