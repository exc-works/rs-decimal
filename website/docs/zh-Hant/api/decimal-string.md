---
locale: zh-Hant
source_revision: 84f61035965dcc9b605c00e7c877afbafcb7870fb2b3b89e0902348c80e03dd7
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::string
---
# Decimal::string

格式化為定點十進位文字。

```rust
fn string (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string，不使用科學記數法。移除多餘的小數末尾零，保留整數末尾零。使用小數點，不分組。負零沒有單獨的表示。預設零輸出 0，不因此被初始化；格式化僅借用輸入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
