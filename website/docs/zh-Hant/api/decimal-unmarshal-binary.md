---
locale: zh-Hant
source_revision: 404c61af98ea51d383b516ba1e691c0c718c44923614dbeaca2d2a7efd410ffc
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

從相容 Go 的二進位純量替換小數。

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

bytes: &[u8]/Uint8Array；修改接收者，回傳 `Result<(), DecimalError>`/undefined。前四位元組是大端 scale，之後為 Gob 頭 2（正）或 3（負）和絕對值。成功解碼保留編碼中的 scale。空輸入產生已初始化零，與預設未初始化零不同。只有四位元組前綴而無 Gob 負載時也接受，產生該 scale 的已初始化零。過短前綴、無效版本或超過 131072 的 scale 回傳核心 Unmarshal 錯誤。錯誤保留接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="zh-Hant" />

## 瀏覽器邊界

輸入必須為 Uint8Array，kind 必須是 0–255 的整數。複製到 Wasm 前限制輸入為 65536 位元組；Rust 再檢查二進位 scale 不超過 1000、實際係數不超過 16384 位。BSON 字串需滿足 8192 位元組文字限制，Decimal128 的指數展開也接受 [資源檢查](../resource-policy)。這些限制可能比原生程式庫更嚴格。所有檢查在接收者改變之前完成；Core 錯誤與資源錯誤保持區分，已移動的控制代碼不可重複使用。

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
