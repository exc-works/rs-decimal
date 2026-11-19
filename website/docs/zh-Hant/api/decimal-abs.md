---
locale: zh-Hant
source_revision: c71c6b4ce225ea223944d584a5ec03b1adfe1dbd2adeb04f6beb38800f1e252e
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::abs
---
# Decimal::abs

建立儲存絕對值的獨立 Decimal。

```rust
fn abs (& self) -> Self
```

## 參數、回傳值及注意事項

回傳 Decimal，保留小數位數及末尾零。即使輸入為預設值，結果也會初始化；輸入保持不變。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
