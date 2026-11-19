---
locale: ja
source_revision: dcbc410f5dabf4a62b4cb667b765d034201be05823aeeb413148caad325fb23f
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

スケール適用前の正確な整数係数をコピーします。

```rust
fn big_int (& self) -> BigInt
```

## 引数、戻り値、使用上の注意

Rust は独立した BigInt、bridge は bigint を返します。切り捨てる整数変換とは異なり、末尾のゼロを含みます。例では JSON のため bigint をテキストとして表示します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
