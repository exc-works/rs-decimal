---
locale: ja
source_revision: 753b858d717076ef46e98ae0c0b43697fb31b0651286396a37e7378758f67207
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

## ブラウザーの制約

デコーダー入力は 8192 バイトまでです。パラメーター文字列も Wasm へのコピー前に長さを確認します。Rust はテキストとデコード済み JSON 文字列に[小数のリソースポリシー](../resource-policy)を再適用するため、エスケープで指数やスケールの上限を回避できません。不正な UTF-8、JSON／小数構文の診断はコアが行います。入力エラーやリソースエラーでレシーバーが部分的に更新されることはありません。ムーブ済みのラッパーは読み取りも変更もできません。

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
