---
locale: zh-Hant
source_revision: c98261b50ba3febe344cea19fd963c1739045fe7ff1872b278b97c36e7fef20b
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

對整數位分組，並選擇小數分隔符。

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## 參數、回傳值及常規用法

thousands: char；decimal: char。回傳 String/string，整數位每三位一組，保留所有儲存的小數位。每個 JS 參數必須恰好是一個 Unicode 純量，可使用補充平面字元；孤立代理項和多字元字串被拒絕。NUL thousands 禁用分組，NUL decimal 使用小數點。分隔符是字面字元，不是語言區域名稱。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
