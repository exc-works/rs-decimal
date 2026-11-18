---
locale: zh-Hant
source_revision: 751751d0b535375fecff33d2510df43e92ffca892248b1be4dba2a34ce1d0f99
translation_provenance: zh-Hans-terminology-reviewed
api_id: zero
---
# zero

建構已初始化且小數位數為零的零值。

```rust
fn zero () -> Decimal
```

## 參數、回傳值及注意事項

無參數，回傳新的 Decimal。與 Decimal::default() 不同，結果已初始化；兩者的數值仍然相等。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="zh-Hant" />

## 邊界

bridge 保留 Rust 方法名及參數順序。自由函數在 Playground 中可直接呼叫。借用的輸入不會被消耗或改變；bridge 拒絕失效或其他執行實例的物件。全部操作遵守 [資源限制](../resource-policy)。

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
