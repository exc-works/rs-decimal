---
locale: zh-Hant
source_revision: 4beaa8ccdf2751ba7bce74d134b8de29fabd5c0d82d0d4eebee1575a204be12d
translation_provenance: zh-Hans-terminology-reviewed
api_id: reexport::GoFormat
---
# reexport::GoFormat

從 crate 根路徑匯入 GoFormat。

```rust
pub use formatting::GoFormat as GoFormat
```

## 參數、回傳值及常規用法

這是 Rust 結構體的公開重新匯出，不是另一份實現。瀏覽器提供欄位物件適配器和呼叫 Rust 的預設值工廠。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-reexport-goformat" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
