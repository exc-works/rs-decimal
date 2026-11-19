---
locale: zh-Hant
source_revision: 663e2e4ad6a9b90feffe429f81f13e798f3070cf1d9ea7de1079b0dc756cf5d2
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_negative
---
# Decimal::is_negative

判斷數值是否嚴格小於零。

```rust
fn is_negative (& self) -> bool
```

## 參數、回傳值及注意事項

回傳 bool/boolean。零（包括未初始化的預設值）回傳 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
