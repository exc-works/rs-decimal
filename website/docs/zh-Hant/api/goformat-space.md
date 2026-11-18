---
locale: zh-Hant
source_revision: 6ee970e3e2507cdd225eb4c314175b5a95b9aa45ac2e64ed4be5fded0c11b425
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.space
---
# GoFormat.space

設定 space 格式欄位。

```rust
space: bool
```

## 參數、回傳值及常規用法

plus 為 false 時，為非負數字輸出添加前導空格。負數保留符號。預設為 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
