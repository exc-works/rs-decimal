---
locale: ja
source_revision: 4e9262b5d73b0bfa6dbb9a564894dae85973da389d298cf19e5836d09170ed0d
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::impl std :: fmt :: Display"
---
# NullDecimal::impl std :: fmt :: Display

Rust の Display で nullable 値を書式設定します。

```rust
impl std :: fmt :: Display for NullDecimal
```

## 引数、戻り値、通常の使い方

Display は nullable の string() に委譲し、有効なら内部値の標準テキスト、無効なら null を使います。フォーマッターは JSON トークンやバイトバッファーではなくテキストを書き込みます。format! は Display::fmt を呼び出します。このトレイト構文はネイティブで実行し、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
