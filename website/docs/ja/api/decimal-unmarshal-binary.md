---
locale: ja
source_revision: 8a589312845f2d23d85f3d6da1987ee87f79dfecdc97a705386e9a336b17d7ed
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

Go 互換のバイナリスカラーから小数を置換します。

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

bytes: &[u8]/Uint8Array。レシーバーを変更し、`Result<(), DecimalError>`/undefined を返します。最初の 4 バイトはビッグエンディアンのスケールで、その後に Gob ヘッダー 2（正）または 3（負）と絶対値が続きます。成功時にはエンコードされたスケールを保持します。空入力は既定の未初期化ゼロとは異なり、初期化済みゼロを作ります。Gob ペイロードのない 4 バイトの接頭部も、そのスケールの初期化済みゼロとして受理します。短い接頭部、不正なバージョン、131072 を超えるスケールはコアの Unmarshal エラーを返します。エラー時はレシーバーを保持します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="ja" />

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
