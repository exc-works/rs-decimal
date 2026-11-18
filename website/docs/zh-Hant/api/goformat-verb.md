---
locale: zh-Hant
source_revision: 349cbd7b8c032b6b9b799b90d2d1981de5ba6641cd94e6d5d2f6cdc9c436f5e2
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.verb
---
# GoFormat.verb

設定 verb 格式欄位。

```rust
verb: char
```

## 參數、回傳值及常規用法

選擇單個 Unicode 純量格式動詞。預設 v 使用緊湊定點文字；e/E 使用科學記數法，q 加引號，b 輸出二進位係數及 p-scale（不是 IEEE 浮點格式）。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-verb" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
