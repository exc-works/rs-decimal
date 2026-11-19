---
locale: ja
source_revision: eb4bab6a9fdfe771d31eefc8731c88dad81f2f8a3273adea4040fea11f4141ab
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::sign
---
# Decimal::sign

符号を -1、0、1 として読み取ります。

```rust
fn sign (& self) -> i32
```

## 引数、戻り値、使用上の注意

i32/number を返します。初期化済みと未初期化のゼロはいずれも符号 0 で、負のゼロの符号は区別しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
