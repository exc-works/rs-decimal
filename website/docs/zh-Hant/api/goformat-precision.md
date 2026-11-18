---
locale: zh-Hant
source_revision: b5c0056c42f3f4e6d3613af84d12321baa6359b54407d77f1aacdb7d5d883dd7
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.precision
---
# GoFormat.precision

設定 precision 格式欄位。

```rust
precision: Option < usize >
```

## 參數、回傳值及常規用法

精度含義隨動詞變化：f/F 為小數位，e/E 為首位之後的位數，g/G 為有效位數（零按一處理），d 為最少整數位。None/null 使用動詞預設規則；f 保留 scale，e 使用六位。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
