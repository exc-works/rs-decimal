---
locale: zh-Hant
source_revision: f7f692709bde775f34d7f5feb756c59944ff77c763bae1798b2b4306d9770c41
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

複製精確的未縮放整數係數。

```rust
fn big_int (& self) -> BigInt
```

## 參數、回傳值及注意事項

Rust 回傳獨立的 BigInt，JavaScript 回傳 bigint。它包含末尾零，不是截斷小數的整數轉換。範例為 JSON 輸出把 bigint 轉成文字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
