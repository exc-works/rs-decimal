---
locale: ja
source_revision: 5a1796b93575e117064504cd886a99abb8187be7c4bc915a35b33888e3d8ea72
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display::fmt"
---
# NullDecimal::impl std :: fmt :: Display::fmt

Rust の Display で nullable 値を書式設定します。

```rust
fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result
```

## 引数、戻り値、通常の使い方

Display は nullable の string() に委譲し、有効なら内部値の標準テキスト、無効なら null を使います。フォーマッターは JSON トークンやバイトバッファーではなくテキストを書き込みます。format! は Display::fmt を呼び出します。このトレイト構文はネイティブで実行し、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display-fmt" locale="ja" />

## ブラウザーの制約

デコーダー入力は 8192 バイトまでです。パラメーター文字列も Wasm へのコピー前に長さを確認します。Rust はテキストとデコード済み JSON 文字列に[小数のリソースポリシー](../resource-policy)を再適用するため、エスケープで指数やスケールの上限を回避できません。不正な UTF-8、JSON／小数構文の診断はコアが行います。入力エラーやリソースエラーでレシーバーが部分的に更新されることはありません。ムーブ済みのラッパーは読み取りも変更もできません。

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
