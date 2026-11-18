---
locale: zh-Hant
source_revision: b225904d6e477d16ac1df6ba10bcfee3249227a343581851ba1cc46cc84f5863
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

編碼相容 Go 的 precision/Gob 二進位格式。

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## 參數、回傳值及常規用法

無參數，回傳 `Result<Vec<u8>, DecimalError>`/Uint8Array。已初始化值先移除小數末尾零，再寫入四位元組大端 scale、big.Int Gob 版本/符號位元組和大端絕對值。這是小數純量格式，不是 IEEE 浮點數。未初始化零編碼為空位元組，已初始化零編碼為 [0,0,0,0,2]。回傳的是獨立副本，修改位元組不改變 Decimal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="zh-Hant" />

## 瀏覽器邊界

輸入必須為 Uint8Array，kind 必須是 0–255 的整數。複製到 Wasm 前限制輸入為 65536 位元組；Rust 再檢查二進位 scale 不超過 1000、實際係數不超過 16384 位。BSON 字串需滿足 8192 位元組文字限制，Decimal128 的指數展開也接受 [資源檢查](../resource-policy)。這些限制可能比原生程式庫更嚴格。所有檢查在接收者改變之前完成；Core 錯誤與資源錯誤保持區分，已移動的控制代碼不可重複使用。

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
