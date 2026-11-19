---
locale: ja
source_revision: aa67b8e6e6f3fb7b6a66f11573d9907fb9dc057abd9547e4d0c0474969528084
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

Go 互換の precision/Gob 通信形式へエンコードします。

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## 引数、戻り値、通常の使い方

引数なし。`Result<Vec<u8>, DecimalError>`/Uint8Array を返します。初期化済みの値では小数部分の末尾のゼロを除去してから、4 バイトのビッグエンディアンのスケール、big.Int Gob のバージョン／符号バイト、ビッグエンディアンの絶対値を書き込みます。これは小数スカラーのペイロードであり、IEEE 浮動小数点ではありません。未初期化のゼロは空のバイト列、初期化済みのゼロは [0,0,0,0,2] になります。返されたコピーを変更しても Decimal は変わりません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="ja" />

## 関連メソッド

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
