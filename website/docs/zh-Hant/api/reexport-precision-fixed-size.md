---
locale: zh-Hant
source_revision: 8de700fe2bea77817d4cc64ce868f77fd5301e630577d698cc1cf5a3b4f76ce5
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

## 相關方法

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
