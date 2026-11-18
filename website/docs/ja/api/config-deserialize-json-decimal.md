---
locale: ja
source_revision: a26edb51e2fbe0ae6bac5abd8ce3e12bd13315bb3e836a0d554d5f19af0430ce
translation_provenance: en-US-japanese-reviewed
api_id: "config::deserialize_json_decimal"
---
# config::deserialize_json_decimal

正確な JSON 設定フィールドをデシリアライズします。

```rust
fn deserialize_json_decimal < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Decimal , D :: Error >
```

## 引数、戻り値、通常の使い方

deserializer: Serde Deserializer を実装する D。#[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")] 向けです。serde_json ソースと config 機能が必要です。浮動小数点の往復なしに元の数値トークンを保持します。ブラウザーは任意の deserializer ではなく、完全な JSON スカラーのソース文字列 1 つを受け取る具体的なアダプターを提供します。非 nullable の対象では JSON null と空文字列を拒否します。エラーは Serde deserializer のエラー型です。ブラウザーの category="serde" は DecimalError の種類を作らず、元のメッセージを保持します。末尾の JSON 空白以外の入力は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-decimal" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
