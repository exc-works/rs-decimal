---
locale: zh-Hant
source_revision: 40283a0b4501fabd3b307e9ac7d1a0dd292a797503fe33899ba506e5b43fdac8
translation_provenance: zh-Hans-terminology-reviewed
api_id: "reexport::PRECISION_FIXED_SIZE"
---
# reexport::PRECISION_FIXED_SIZE

讀取二進位精度前綴的位元組數。

```rust
pub use binary::PRECISION_FIXED_SIZE as PRECISION_FIXED_SIZE
```

## 參數、回傳值及常規用法

Rust usize 常數，值為 4：相容 Go 的二進位格式中大端 scale 前綴的位元組數。它不是小數位數或精度上限。瀏覽器提供來自 Rust 的唯讀 number。crate 根重匯出屬於 Rust 匯入語法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-reexport-precision-fixed-size" locale="zh-Hant" />

## 瀏覽器邊界

輸入必須為 Uint8Array，kind 必須是 0–255 的整數。複製到 Wasm 前限制輸入為 65536 位元組；Rust 再檢查二進位 scale 不超過 1000、實際係數不超過 16384 位。BSON 字串需滿足 8192 位元組文字限制，Decimal128 的指數展開也接受 [資源檢查](../resource-policy)。這些限制可能比原生程式庫更嚴格。所有檢查在接收者改變之前完成；Core 錯誤與資源錯誤保持區分，已移動的控制代碼不可重複使用。

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
