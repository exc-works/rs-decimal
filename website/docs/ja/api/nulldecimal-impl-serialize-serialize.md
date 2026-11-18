---
locale: ja
source_revision: 67d49a92b2e2ca62692f06791c5907e6136e1bf8593cdb08edcf4fb008700159
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::impl Serialize::serialize"
---
# NullDecimal::impl Serialize::serialize

Rust の Serde トレイトでシリアライズします。

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## 引数、戻り値、通常の使い方

serializer: Serializer を実装する S。S::Ok/S::Error を返します。コアは標準の小数文字列または null をシリアライズします。無効な nullable は null、有効で内部が未初期化のゼロなら文字列 "0" になります。小数部分の末尾のゼロは除去します。ブラウザーの serialize() アダプターは serde_json を選び、完全な JSON テキストを返します。任意の Rust serializer は公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize-serialize" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
