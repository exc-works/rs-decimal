---
locale: zh-Hant
source_revision: 7388877c082aab40b6cea1eaae69162f25e5c07d8d3673378f0fdd23fef3e494
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::scale
---
# Decimal::scale

讀取儲存的非負小數位數。

```rust
fn scale (& self) -> usize
```

## 參數、回傳值及注意事項

回傳 usize，在 Playground 精度限制內映射為精確的 number；數值與 precision() 相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
