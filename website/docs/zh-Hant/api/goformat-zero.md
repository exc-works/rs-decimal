---
locale: zh-Hant
source_revision: 5fb7d2ba1d450e55c06f4bd243bcd776a0cb6c5e8f2e0f80f16235d6abf720a7
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.zero
---
# GoFormat.zero

設定 zero 格式欄位。

```rust
zero: bool
```

## 參數、回傳值及常規用法

在符號之後用零填充數字左側。left 為 true 或 d 有顯式 precision 時忽略。預設為 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-zero" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
