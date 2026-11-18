---
locale: zh-Hant
source_revision: 4b0752061c959ac5ce6e3b7af4bf7c9232d0d1cfb43869372b4caa7a5d6f49f4
translation_provenance: zh-Hans-terminology-reviewed
api_id: NullDecimal::impl Debug
---
# NullDecimal::impl Debug

透過 Rust Debug 查看可空狀態。

```rust
impl Debug for NullDecimal
```

## 常規用法及語義

診斷文字包含有效性旗標及儲存欄位，包括隱藏值。它不是穩定的傳輸格式，瀏覽器使用快照。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-debug" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
