---
locale: zh-Hant
source_revision: 71bc56362fd9f1c25a96932ba1ffd795db889517f58a951726bcd78e58fe2bc7
translation_provenance: zh-Hans-terminology-reviewed
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

透過 Rust Default 建立缺失的可空值。

```rust
impl Default for NullDecimal
```

## 常規用法及語義

有效性旗標與內部初始化狀態均為 false。bridge 提供 NullDecimal.default()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="zh-Hant" />

此 Rust 語法範例由原生測試執行，不計作瀏覽器 trait 支持。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
