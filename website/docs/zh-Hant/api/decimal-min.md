---
locale: zh-Hant
source_revision: eafc21e4a88e29bea4168f39bfe3b5db899f2a28bccbd47f88733ffdaab27a0c
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::min
---
# Decimal::min

複製兩個數值中較小的一個。

```rust
fn min (& self , other : & Self) -> Self
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳獨立的 Decimal。相等時保留第一個運算元的小數位數及初始化狀態。Rust 中使用 Decimal::min(&left, &right)，避免選中會取得所有權的 Ord::min trait 方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
