---
locale: zh-Hant
source_revision: bb2070c90fbcf81b9f15485debb923fef5e838831e150577f2d283f059db73b6
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::must_from_string
---
# Decimal::must_from_string

在非法輸入屬於程式錯誤的場景下解析可信的小數文字。

```rust
fn must_from_string (text : & str) -> Self
```

## 參數、回傳值及注意事項

text: &str/string，回傳 Decimal。合法輸入與 new_from_string 得到相同狀態。非法語法在 Rust 中觸發包含 DecimalError 的 panic；編譯後的 Wasm 發生 trap，不會回傳可恢復的程式庫錯誤。使用者輸入應優先使用 new_from_string。未捕獲的 trap 會結束目前執行；再次使用 Rust 前應開始全新的執行。參數及資源檢查仍會在呼叫 Rust 前拒絕超限輸入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-decimal-must-from-string" locale="zh-Hant" />

## 邊界

bridge 保留 Rust 方法名及參數順序。自由函數在 Playground 中可直接呼叫。借用的輸入不會被消耗或改變；bridge 拒絕失效或其他執行實例的物件。全部操作遵守 [資源限制](../resource-policy)。

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
