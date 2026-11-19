---
locale: zh-Hant
source_revision: aa67b8e6e6f3fb7b6a66f11573d9907fb9dc057abd9547e4d0c0474969528084
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

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
