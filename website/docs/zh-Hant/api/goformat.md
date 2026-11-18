---
locale: zh-Hant
source_revision: e9b605c9c2b762d25f61c081bee35aa1c375be0cb3cab1e12e2bd0938c7a3c70
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat
---
# GoFormat

使用欄位物件描述一次格式化操作。

```rust
pub struct GoFormat
```

## 參數、回傳值及常規用法

欄位對應 Rust 結構體：verb、width、precision、left、plus、space、zero、alternate。bridge 的 GoFormat.default() 從 Rust 讀取預設值，回傳新的可變物件；修改它不會設定全域格式。width/precision 使用 null 表示 None，非負 wasm32 整數 number 表示 Some。Rust trait 仍屬於原生語法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
