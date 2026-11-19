---
locale: ja
source_revision: d67309b22e8bef25fa612b0b77506dc49f39a3b10f47781eb6058acd17162dd3
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::unmarshal_json"
---
# NullDecimal::unmarshal_json

json 入力を既存の NullDecimal へデコードします。

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

bytes: &[u8]/Uint8Array。レシーバーを変更し、Result<(), DecimalError>/undefined を返します。Rust は json 機能を必要とします。元の UTF-8 JSON バイト列を渡してください。数値は浮動小数点の往復なしに検証し、文字列はエスケープ解除後に正確な小数解析を行います。先に JSON.parse で数値トークンを JS number にしないでください。JSON の空白は space/tab/CR/LF のみです。JSON null は有効性を解除し、内部 Decimal を未初期化のゼロにリセットします。null 以外の入力の解析に成功すると小数を初期化し、ゼロ以外の数値の末尾のスケールを保持し、解析したゼロはスケール 0 に正規化します。valid=true にします。エラー時はレシーバーの全状態を保持します。ライブラリのデコードエラーは Unmarshal と基底の種類を保持し、リソースの失敗も変更前に起こります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-json" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
