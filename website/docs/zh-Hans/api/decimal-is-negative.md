---
locale: zh-Hans
source_revision: 663e2e4ad6a9b90feffe429f81f13e798f3070cf1d9ea7de1079b0dc756cf5d2
api_id: Decimal::is_negative
---
# Decimal::is_negative

判断数值是否严格小于零。

```rust
fn is_negative (& self) -> bool
```

## 参数、返回值及注意事项

返回 bool/boolean。零（包括未初始化的默认值）返回 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
