---
locale: ja
source_revision: 7388877c082aab40b6cea1eaae69162f25e5c07d8d3673378f0fdd23fef3e494
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::scale
---
# Decimal::scale

保存された非負の小数スケールを読み取ります。

```rust
fn scale (& self) -> usize
```

## 引数、戻り値、使用上の注意

playground のスケール上限内で、usize を正確な number として返します。precision() と同じ値です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
