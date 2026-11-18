---
locale: ja
source_revision: ce07dfd04d96401a00570a69b98dc03abf2e70bda05348ea2e8e6bff0225c443
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

## ブラウザーの制約

デコーダー入力は 8192 バイトまでです。パラメーター文字列も Wasm へのコピー前に長さを確認します。Rust はテキストとデコード済み JSON 文字列に[小数のリソースポリシー](../resource-policy)を再適用するため、エスケープで指数やスケールの上限を回避できません。不正な UTF-8、JSON／小数構文の診断はコアが行います。入力エラーやリソースエラーでレシーバーが部分的に更新されることはありません。ムーブ済みのラッパーは読み取りも変更もできません。

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
