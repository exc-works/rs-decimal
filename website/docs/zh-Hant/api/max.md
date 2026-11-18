---
locale: zh-Hant
source_revision: 0d4a74db932ab126e7566fb630352abba8c528142a3d37c615a2694b7fac9c0a
translation_provenance: zh-Hans-terminology-reviewed
api_id: max
---
# max

透過自由函數複製兩個 Decimal 中較大的一個。

```rust
fn max (a : & Decimal , b : & Decimal) -> Decimal
```

## 參數、回傳值及注意事項

借用 a 和 b: &Decimal，回傳獨立的 Decimal。數值相等時保留 a 的小數位數及初始化狀態，與 Decimal::max 的程式庫語義相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-max" locale="zh-Hant" />

## 邊界

bridge 保留 Rust 方法名及參數順序。自由函數在 Playground 中可直接呼叫。借用的輸入不會被消耗或改變；bridge 拒絕失效或其他執行實例的物件。全部操作遵守 [資源限制](../resource-policy)。

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
