---
locale: ja
source_revision: da8cebe24a220cbee0329a50db3c796d6390c270eb7d574cffaac17c4ddc1264
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

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
