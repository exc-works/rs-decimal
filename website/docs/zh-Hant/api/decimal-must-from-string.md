---
locale: zh-Hant
source_revision: 7805847022fc73cbb47e3960021e981cfb8ac4ab47a7b0a542ffa809d986fb9d
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

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
