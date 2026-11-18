---
locale: zh-Hant
source_revision: df5bdc8fba1feff0933958fdceab2cd0bf8d1a541a0f2e47f17bffe6a6338d47
translation_provenance: zh-Hans-terminology-reviewed
api_id: ten
---
# ten

建構值為十的 Decimal 整數。

```rust
fn ten () -> Decimal
```

## 參數、回傳值及注意事項

無參數，回傳新的已初始化 Decimal，小數位數為零。需要小數位數時使用 rescale。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="zh-Hant" />

## 邊界

bridge 保留 Rust 方法名及參數順序。自由函數在 Playground 中可直接呼叫。借用的輸入不會被消耗或改變；bridge 拒絕失效或其他執行實例的物件。全部操作遵守 [資源限制](../resource-policy)。

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
