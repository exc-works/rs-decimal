---
locale: ja
source_revision: b283786af672117d0c3017e513f97f5df201a379427acab824b383812aa4cf93
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

設定入力 Decimal を選択します。

```rust
Decimal (& 'a Decimal)
```

## 引数、戻り値、通常の使い方

既存の小数を借用し、デコード時に係数、スケール、初期化状態を複製します。元の値は引き続き使用できます。ブラウザーでデコード前に元の値を解放またはムーブすると、この借用バリアントは無効になります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
