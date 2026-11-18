---
locale: zh-Hant
source_revision: 18a49744a56694725a1fe7d677be386e97a4c76f0669871a289c86113d20f3e2
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::marshal_bson_value"
---
# Decimal::marshal_bson_value

編碼 BSON 類型位元組和純量負載。

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## 參數、回傳值及常規用法

無參數，回傳 `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]。Rust 需要 bson feature，瀏覽器建置已啟用。元組包含 BSON 純量，而非完整 BSON 文件。已初始化值編碼為 BSON 字串（類型 2），不是 Decimal128；未初始化零編碼為類型 10 和空負載。字串負載先寫小端 i32 位元組長度（包含結尾 NUL），隨後寫無小數末尾零的規範文字和 NUL。回傳位元組是獨立副本；編碼不改變初始化或有效旗標。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-bson-value" locale="zh-Hant" />

## 瀏覽器邊界

輸入必須為 Uint8Array，kind 必須是 0–255 的整數。複製到 Wasm 前限制輸入為 65536 位元組；Rust 再檢查二進位 scale 不超過 1000、實際係數不超過 16384 位。BSON 字串需滿足 8192 位元組文字限制，Decimal128 的指數展開也接受 [資源檢查](../resource-policy)。這些限制可能比原生程式庫更嚴格。所有檢查在接收者改變之前完成；Core 錯誤與資源錯誤保持區分，已移動的控制代碼不可重複使用。

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
