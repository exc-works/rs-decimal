---
locale: ja
source_revision: 25be20babe80f8d8b33e901842da879899c54ff8a78f97427ea80f9ca18a005d
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::marshal_yaml"
---
# Decimal::marshal_yaml

yaml の意味上の内容を取得します。

```rust
fn marshal_yaml (& self) -> String
```

## 引数、戻り値、通常の使い方

引数なし。String/string を返します。Rust は yaml 機能を必要とし、ブラウザーでは有効です。シリアライズされた YAML 文書ではなく、意味上の YAML 文字列スカラーを返します。引用符とタグを正しく処理できるよう、YAML エミッターへ文字列として渡してください。標準の小数テキストは小数部分の末尾のゼロを除去します。既定の未初期化ゼロは "0" を返します。読み取りでは初期化状態や有効性を変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-yaml" locale="ja" />

## 関連メソッド

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
