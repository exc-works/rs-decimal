---
locale: zh-Hant
source_revision: 939b86ba367c0b35cb69b9f2664eb74fafe4279f8b1b81c20315cc4e07b18382
translation_provenance: zh-Hans-terminology-reviewed
api_id: one
---
# one

建構值為一的 Decimal 整數。

```rust
fn one () -> Decimal
```

## 參數、回傳值及注意事項

無參數，回傳新的已初始化 Decimal，小數位數為零。修改或釋放一次呼叫的結果不會影響後續呼叫。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-one" locale="zh-Hant" />

## 邊界

bridge 保留 Rust 方法名及參數順序。自由函數在 Playground 中可直接呼叫。借用的輸入不會被消耗或改變；bridge 拒絕失效或其他執行實例的物件。全部操作遵守 [資源限制](../resource-policy)。

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
