---
locale: zh-Hant
source_revision: 075546f79884d636c4e552add1629511a5f2dca740186a5700262f084c5bd8a5
translation_provenance: zh-Hans-terminology-reviewed
api_id: hundred
---
# hundred

建構值為一百的 Decimal 整數。

```rust
fn hundred () -> Decimal
```

## 參數、回傳值及注意事項

無參數，回傳新的已初始化 Decimal，小數位數為零。它是數值建構函數，不是共享的可變常數。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="zh-Hant" />

## 邊界

bridge 保留 Rust 方法名及參數順序。自由函數在 Playground 中可直接呼叫。借用的輸入不會被消耗或改變；bridge 拒絕失效或其他執行實例的物件。全部操作遵守 [資源限制](../resource-policy)。

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
