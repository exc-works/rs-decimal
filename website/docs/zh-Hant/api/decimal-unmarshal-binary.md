---
locale: zh-Hant
source_revision: 8a589312845f2d23d85f3d6da1987ee87f79dfecdc97a705386e9a336b17d7ed
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

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
