---
locale: zh-Hant
source_revision: 73595bceae9a4435e67075be3cf3874619847ef2c8e049d0faeae85eae4feb26
translation_provenance: zh-Hans-terminology-reviewed
api_id: "PRECISION_FIXED_SIZE"
---
# PRECISION_FIXED_SIZE

讀取二進位精度前綴的位元組數。

```rust
const PRECISION_FIXED_SIZE: usize = 4
```

## 參數、回傳值及常規用法

Rust usize 常數，值為 4：相容 Go 的二進位格式中大端 scale 前綴的位元組數。它不是小數位數或精度上限。瀏覽器提供來自 Rust 的唯讀 number。crate 根重匯出屬於 Rust 匯入語法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-precision-fixed-size" locale="zh-Hant" />

## 瀏覽器邊界

輸入必須為 Uint8Array，kind 必須是 0–255 的整數。複製到 Wasm 前限制輸入為 65536 位元組；Rust 再檢查二進位 scale 不超過 1000、實際係數不超過 16384 位。BSON 字串需滿足 8192 位元組文字限制，Decimal128 的指數展開也接受 [資源檢查](../resource-policy)。這些限制可能比原生程式庫更嚴格。所有檢查在接收者改變之前完成；Core 錯誤與資源錯誤保持區分，已移動的控制代碼不可重複使用。

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
