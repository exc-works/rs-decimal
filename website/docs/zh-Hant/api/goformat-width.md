---
locale: zh-Hant
source_revision: 0ad883afdae2ec6d62af55006e383eb095f5c7928dcdbf6c2f7baf446a49d945
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.width
---
# GoFormat.width

設定 width 格式欄位。

```rust
width: Option < usize >
```

## 參數、回傳值及常規用法

最小寬度包含符號和小數點；透過填充補足，不截斷更長文字。None/null 禁用最小寬度。核心按輸出位元組數計算。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
