---
locale: zh-Hant
source_revision: 2ea2866a9b7493c17d529f3372e5bbb238200663f6cd52e7c63d668cc43d0ffe
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_positive
---
# Decimal::is_positive

判斷數值是否嚴格大於零。

```rust
fn is_positive (& self) -> bool
```

## 參數、回傳值及注意事項

回傳 bool/boolean。零（包括未初始化的預設值）回傳 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-positive" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
