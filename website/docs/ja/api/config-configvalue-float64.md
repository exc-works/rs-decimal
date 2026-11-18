---
locale: ja
source_revision: 21a2321ebb07dad0e89ac689f0921883a544f6575bbd19e235e51c9aee9deb73
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

設定入力 Float64 を選択します。

```rust
Float64 (f64)
```

## 引数、戻り値、通常の使い方

コアの最短小数変換を通して binary64 を保持します。非有限値は Unmarshal を返します。浮動小数点の丸めですでに失われた桁は復元できません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
