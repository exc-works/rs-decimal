---
locale: zh-Hant
source_revision: c526a43d69654fb21b49f56d858016f45f88e7f7f6ec8eaa0c13c3e5e939cd03
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::uint64
---
# Decimal::uint64

轉換為固定位寬整數並回傳成功旗標。

```rust
fn uint64 (& self) -> (u64 , bool)
```

## 參數、回傳值及常規用法

無參數；回傳 (u64, bool)，映射為 [bigint, boolean]。範圍內的整數回傳對應整數和 true。小數或溢出回傳 (0, false)，不會截斷或拋出錯誤。負數同樣回傳 (0, false)。不能僅憑 scale 判斷是否為整數：12.00 轉換成功。應檢查旗標，區分失敗與有效零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-uint64" locale="zh-Hant" />

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
