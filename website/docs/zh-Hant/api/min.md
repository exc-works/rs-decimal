---
locale: zh-Hant
source_revision: 9a05736a78d12045af2704500028fdc4fdec0490c797ca6d8ba9130b66fbc153
translation_provenance: zh-Hans-terminology-reviewed
api_id: min
---
# min

透過自由函數複製兩個 Decimal 中較小的一個。

```rust
fn min (a : & Decimal , b : & Decimal) -> Decimal
```

## 參數、回傳值及注意事項

借用 a 和 b: &Decimal，回傳獨立的 Decimal。數值相等時保留 a 的小數位數及初始化狀態，與 Decimal::min 的程式庫語義相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-min" locale="zh-Hant" />

## 邊界

bridge 保留 Rust 方法名及參數順序。自由函數在 Playground 中可直接呼叫。借用的輸入不會被消耗或改變；bridge 拒絕失效或其他執行實例的物件。全部操作遵守 [資源限制](../resource-policy)。

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
