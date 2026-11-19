---
locale: zh-Hant
source_revision: eb4bab6a9fdfe771d31eefc8731c88dad81f2f8a3273adea4040fea11f4141ab
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sign
---
# Decimal::sign

以 -1、0、1 讀取符號。

```rust
fn sign (& self) -> i32
```

## 參數、回傳值及注意事項

回傳 i32/number。已初始化和未初始化的零都回傳 0，不保留單獨的負零符號。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
