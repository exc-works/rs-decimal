---
locale: ja
source_revision: dabe3e417d155ec62acc296ebb36a3f87bdc044baf1a1ffd4eb27cbaf056b77d
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

Decimal を text バイト列へエンコードします。

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## 引数、戻り値、通常の使い方

引数なし。標準の ASCII 小数テキストを含む `Vec<u8>`/Uint8Array を返します。小数部分の末尾のゼロを除去します。既定の Decimal ゼロは ASCII 0 を出力します。返されたバイト列は独立したコピーで、読み取りは状態を変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
