---
locale: ja
source_revision: d858bd2326ab43e0cc593d11fb421e19dddaa88d9dec2ce79a679a247e556178
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

設定入力 Float32 を選択します。

```rust
Float32 (f32)
```

## 引数、戻り値、通常の使い方

binary32 を保持します。Rust はまずブラウザーの number を f32 に狭め、コア変換を使います。非有限入力や変換時のオーバーフローは Unmarshal を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
