---
locale: zh-Hant
source_revision: 2a36a5c46f57052b68644728359c44f15465f6bfca2c60d1540e40c603cc8c5b
translation_provenance: zh-Hans-terminology-reviewed
api_id: NullDecimal.valid
---
# NullDecimal.valid

讀取或修改是否存在值，不改變內部 Decimal。

```rust
valid: bool
```

## 常規用法及語義

Rust 公開的 bool 欄位映射為 boolean getter/setter。設定 false 不會清除 decimal，設定 true 也不會將其初始化。JavaScript 會在修改前拒絕非 boolean 輸入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="zh-Hant" />

這份共用範例同時在原生 Rust 和瀏覽器中執行。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
