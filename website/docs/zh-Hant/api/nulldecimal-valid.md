---
locale: zh-Hant
source_revision: d57c1e8ddb07024ed3c5c002f095b2cee086c1a0719df70a0b4ba8761f29418d
translation_provenance: zh-Hans-terminology-reviewed
api_id: NullDecimal.valid
---
# NullDecimal.valid

讀取或修改是否存在值，不改變內部 Decimal。

```rust
valid: bool
```

## 常規用法及語義

Rust 公開的 bool 欄位映射為 boolean getter/setter。設定 false 不會清除 decimal，設定 true 也不會將其初始化。bridge 會在修改前拒絕非 boolean 輸入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="zh-Hant" />

這份共用範例同時在原生 Rust 和瀏覽器中執行。

## 相關文件

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
