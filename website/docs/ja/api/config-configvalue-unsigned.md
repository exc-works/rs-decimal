---
locale: ja
source_revision: b6da66eb1330fd7e72548ba45454893fec7b7bdb425b5a72d19b09887a6620cd
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

設定入力 Unsigned を選択します。

```rust
Unsigned (u128)
```

## 引数、戻り値、通常の使い方

正確な符号なし u128 を保持します。ブラウザーは [0, 2^128-1] の bigint を要求し、浮動小数点を経由しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
