---
locale: zh-Hant
source_revision: 46f70cc4972a4c4ca833f0191402c07a03bbeeb19c10706e521c3bfe03276dbf
translation_provenance: zh-Hans-terminology-reviewed
api_id: one
---
# one

建構值為一的 Decimal 整數。

```rust
fn one () -> Decimal
```

## 參數、回傳值及注意事項

無參數，回傳新的已初始化 Decimal，小數位數為零。修改或釋放一次呼叫的結果不會影響後續呼叫。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-one" locale="zh-Hant" />

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
