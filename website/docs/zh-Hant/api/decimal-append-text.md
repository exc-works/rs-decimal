---
locale: zh-Hant
source_revision: bf26b0f6b02acd5afbbeb54a43a96410bc32ae1ee6825c86870266f2a294720f
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::append_text
---
# Decimal::append_text

向現有可變緩衝區追加十進位 ASCII 位元組。

```rust
fn append_text (& self , dst : & mut Vec < u8 >)
```

## 參數、回傳值及常規用法

`dst: &mut Vec<u8>`，以 ByteBuffer 物件表示。Rust 回傳 ()，JS 回傳 undefined。保留已有位元組，原地修改目標。移除小數末尾零。Decimal 和緩衝區都仍可使用。ByteBuffer.new 複製 Uint8Array；to_uint8_array 回傳副本，不是 Wasm 記憶體視圖。資源錯誤發生在修改前，保留原緩衝區。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text" locale="zh-Hant" />

## 瀏覽器邊界

遵循 [資源限制](../resource-policy)：格式寬度最多 65536 位元組，precision 最多 1000，固定格式還檢查係數增長。緩衝區總長最多 65536 位元組，追加前使用保守長度估計，因此可能拒絕實際可容納的臨界操作。資源錯誤不會改變輸入或緩衝區。print 對長文字另有顯示上限；需要檢查完整輸出時可讀取 length 或分段輸出。

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
