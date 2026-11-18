---
locale: ja
source_revision: f2e44024c5b91fa0d6b53bc06008f77209b308fe37fd208a3ffa67a94ede4eb1
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Deserialize < 'de >"
---
# Decimal::impl Deserialize < 'de >

汎用 Serde visitor でデシリアライズします。

```rust
impl< 'de > Deserialize < 'de > for Decimal
```

## 引数、戻り値、通常の使い方

deserializer: Deserializer を実装する D。Self または D::Error を返します。正確な小数文字列、整数 visitor 入力、null を受け取ります。浮動小数点 visitor 入力は拒否します。小数や指数を含む JSON 数値には専用の生 JSON/config アダプターを使う必要があります。JSON null は未初期化の Decimal ゼロになります。空文字列は不正です。ブラウザーの静的 deserialize(source) アダプターは元の完全な JSON テキストに serde_json を実行し、任意の Rust deserializer の対応範囲より狭くなります。エラーは category="serde" と元のメッセージを公開します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-deserialize-de" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
