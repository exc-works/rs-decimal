---
locale: ja
source_revision: 77287e6c4550f824e46a4556e0aaf001ccf92f4e78984f337df5b480f5f48874
translation_provenance: en-US-japanese-reviewed
api_id: "config::decode_nullable"
---
# config::decode_nullable

任意の設定値をデコードします。

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## 引数、戻り値、通常の使い方

value: ConfigValue。`Result<NullDecimal, DecimalError>`/新しい nullable ハンドルを返します。Null、厳密に空の Text、空の Bytes は無効になり、内部は未初期化のゼロです。空白だけの Text や空の Number は小数解析に失敗します。既存の Nullable は無効でも有効性と隠れた内部データの両方を複製します。借用 Decimal は強制的に初期化せず有効になります。他の入力は decode_decimal に従い、成功すると有効になります。入力は消費しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
