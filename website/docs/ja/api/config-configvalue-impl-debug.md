---
locale: ja
source_revision: 76e0ede0f8f2ecf84ebf0544bf21902acebb1483558125958da665053c84f11f
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::impl Debug"
---
# config::ConfigValue::impl Debug

ネイティブの Debug 実装を使います。

```rust
impl Debug for config::ConfigValue
```

## 引数、戻り値、通常の使い方

ConfigValue は小さな型付き列挙型です。Clone/Copy は参照先を深くコピーせず、借用参照を保持します。Debug は診断用テキストであり、安定した通信形式ではありません。ネイティブ Rust のトレイト構文は別に検証し、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-debug" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
