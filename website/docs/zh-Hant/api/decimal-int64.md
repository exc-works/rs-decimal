---
locale: zh-Hant
source_revision: f83d4eab7c44d20bd9f7f112876d8de08078fd38cb0bbaaf09e60fe2891d5d1e
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::int64
---
# Decimal::int64

轉換為固定位寬整數並回傳成功旗標。

```rust
fn int64 (& self) -> (i64 , bool)
```

## 參數、回傳值及常規用法

無參數；回傳 (i64, bool)，映射為 [bigint, boolean]。範圍內的整數回傳對應整數和 true。小數或溢出回傳 (0, false)，不會截斷或拋出錯誤。不能僅憑 scale 判斷是否為整數：12.00 轉換成功。應檢查旗標，區分失敗與有效零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-int64" locale="zh-Hant" />

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
