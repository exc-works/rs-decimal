---
locale: ja
source_revision: 76052d80c386b89ac2428898d57b9ce525f22161fd5cd3ee7956cceafe3b11e9
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

Decimal を json バイト列へエンコードします。

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 引数、戻り値、通常の使い方

引数なし。JSON バイト列を含む `Vec<u8>`/Uint8Array を返します。Rust は json 機能を必要とし、ブラウザービルドでは有効です。初期化済みの Decimal は引用符付きの標準文字列、既定の未初期化ゼロは JSON null になります。返されたバイト列は独立したコピーで、読み取りは状態を変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
