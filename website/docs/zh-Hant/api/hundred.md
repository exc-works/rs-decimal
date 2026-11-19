---
locale: zh-Hant
source_revision: 57e2d57e143cb2ba5e54935f1231d3095826fead303cbd95c70c73feba2c6bfd
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

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
