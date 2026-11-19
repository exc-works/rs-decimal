---
locale: ja
source_revision: 5f7a180e14ebabfd6785a42604765a6ece20be12513e46794eb1f0299f911006
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::string"
---
# NullDecimal::string

nullable の標準テキストを読み取ります。

```rust
fn string (& self) -> String
```

## 引数、戻り値、通常の使い方

引数なし。String/string を返します。有効な値は内部 Decimal の標準テキストを使い、小数部分の末尾のゼロを除去します。無効な値は隠れた内部データに関係なくリテラル null を返します。無効な値に空のバイト列を返す marshal_text とは異なります。読み取りでは有効性や初期化状態を変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
