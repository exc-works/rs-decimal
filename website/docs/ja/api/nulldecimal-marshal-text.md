---
locale: ja
source_revision: 2b341a7995c77dbb72c2087390bcd56b4208d2c9ec45de8ea22c2ce3f209501a
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::marshal_text"
---
# NullDecimal::marshal_text

NullDecimal を text バイト列へエンコードします。

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## 引数、戻り値、通常の使い方

引数なし。標準の ASCII 小数テキストを含む `Vec<u8>`/Uint8Array を返します。有効な値は標準小数テキスト、無効な値は単語 null ではなく空のバイト列になります。返されたバイト列は独立したコピーで、読み取りは状態を変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-text" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
