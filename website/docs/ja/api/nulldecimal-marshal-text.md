---
locale: ja
source_revision: d7c6490115eacc09ab0edd5dff3653c7f3971bc72919c598a24d41b9dd4bd07a
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

## ブラウザーの制約

デコーダー入力は 8192 バイトまでです。パラメーター文字列も Wasm へのコピー前に長さを確認します。Rust はテキストとデコード済み JSON 文字列に[小数のリソースポリシー](../resource-policy)を再適用するため、エスケープで指数やスケールの上限を回避できません。不正な UTF-8、JSON／小数構文の診断はコアが行います。入力エラーやリソースエラーでレシーバーが部分的に更新されることはありません。ムーブ済みのラッパーは読み取りも変更もできません。

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
