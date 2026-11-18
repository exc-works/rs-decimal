---
locale: zh-Hant
source_revision: 150ae4511881de6cce3960793291dd735b00fc7aa5175f86d5957a93c70d0c13
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::impl Debug
---
# Decimal::impl Debug

在 Rust 除錯時查看 Decimal。

```rust
impl Debug for Decimal
```

## 常規用法及語義

派生的診斷表示包含內部欄位，不是穩定的序列化格式。瀏覽器輸出使用明確的快照。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-debug" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
