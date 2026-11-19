---
locale: ja
source_revision: 7d0c030a450a67fa4b0e618dbc6cb501daeb5fbc64647ae39a02ca84be312b44
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_nil
---
# Decimal::is_nil

小数が未初期化の既定値かどうか調べます。

```rust
fn is_nil (& self) -> bool
```

## 引数、戻り値、使用上の注意

bool/boolean を返します。既定値は nil、構築したゼロは初期化済みですが、両者は数値として等価です。これは NullDecimal の有効性フラグではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
