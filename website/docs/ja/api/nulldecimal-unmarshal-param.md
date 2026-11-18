---
locale: ja
source_revision: 098847ed019375f939ef5ba1a900bfb15c1c278feadb8bae9f85384666b02bf1
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::unmarshal_param"
---
# NullDecimal::unmarshal_param

param 入力を既存の NullDecimal へデコードします。

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

param: &str/string。レシーバーを変更し、Result<(), DecimalError>/undefined を返します。前後の空白除去と指数構文を含む小数テキストを解析します。空入力は有効性を解除し、内部 Decimal をリセットします。リテラル null は不正な小数テキストです。null 以外の入力の解析に成功すると小数を初期化し、ゼロ以外の数値の末尾のスケールを保持し、解析したゼロはスケール 0 に正規化します。valid=true にします。エラー時はレシーバーの全状態を保持します。ライブラリのデコードエラーは Unmarshal と基底の種類を保持し、リソースの失敗も変更前に起こります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-param" locale="ja" />

## ブラウザーの制約

デコーダー入力は 8192 バイトまでです。パラメーター文字列も Wasm へのコピー前に長さを確認します。Rust はテキストとデコード済み JSON 文字列に[小数のリソースポリシー](../resource-policy)を再適用するため、エスケープで指数やスケールの上限を回避できません。不正な UTF-8、JSON／小数構文の診断はコアが行います。入力エラーやリソースエラーでレシーバーが部分的に更新されることはありません。ムーブ済みのラッパーは読み取りも変更もできません。

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
