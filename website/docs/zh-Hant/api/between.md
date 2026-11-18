---
locale: zh-Hant
source_revision: 4488da41f235921047e8f85b8a68b47bc497bf199529326773333d8f22d38585
translation_provenance: zh-Hans-terminology-reviewed
api_id: between
---
# between

判斷 Decimal 是否位於包含兩端點的數值區間內。

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## 參數、回傳值及注意事項

借用 v、lower 和 upper: &Decimal；回傳 bool/boolean，判斷 lower <= v <= upper。相等按數值判斷，忽略表示差異。不會重排邊界，lower > upper 時回傳 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="zh-Hant" />

## 邊界

bridge 保留 Rust 方法名及參數順序。自由函數在 Playground 中可直接呼叫。借用的輸入不會被消耗或改變；bridge 拒絕失效或其他執行實例的物件。全部操作遵守 [資源限制](../resource-policy)。

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
