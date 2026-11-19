---
locale: ja
source_revision: f90f9a719348f32512d576fc2ef23076c6492acd11b0dbe87ae7f8c72a7c2eab
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

NullDecimal を json バイト列へエンコードします。

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 引数、戻り値、通常の使い方

引数なし。JSON バイト列を含む `Vec<u8>`/Uint8Array を返します。Rust は json 機能を必要とし、ブラウザービルドでは有効です。無効な nullable は JSON null、有効な値は内部 Decimal が未初期化でも引用符付きの標準小数文字列になります。返されたバイト列は独立したコピーで、読み取りは状態を変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
