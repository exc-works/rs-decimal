---
locale: zh-Hant
source_revision: c7a338ce433accfeb61051f993009577dfae7589a629a17bae6043234f27ca38
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::max
---
# Decimal::max

複製兩個數值中較大的一個。

```rust
fn max (& self , other : & Self) -> Self
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳獨立的 Decimal。相等時保留第一個運算元的小數位數及初始化狀態。Rust 中使用 Decimal::max(&left, &right) 明確選擇借用參數的固有方法；Ord::max 會取得所有權。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
