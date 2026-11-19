---
locale: zh-Hant
source_revision: 37b57334dcd3fd469d86572a66aa34315b91919270af861524b7fae399085f7c
translation_provenance: zh-Hans-terminology-reviewed
api_id: ten
---
# ten

建構值為十的 Decimal 整數。

```rust
fn ten () -> Decimal
```

## 參數、回傳值及注意事項

無參數，回傳新的已初始化 Decimal，小數位數為零。需要小數位數時使用 rescale。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="zh-Hant" />

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
