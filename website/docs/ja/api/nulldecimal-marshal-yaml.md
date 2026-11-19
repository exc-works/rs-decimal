---
locale: ja
source_revision: 634992be26686e49442a948ce62a476a4bcc34db498b408945e4ad08ce9f7530
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::marshal_yaml"
---
# NullDecimal::marshal_yaml

yaml の意味上の内容を取得します。

```rust
fn marshal_yaml (& self) -> Option < String >
```

## 引数、戻り値、通常の使い方

引数なし。`Option<String>`/string | null を返します。Rust は yaml 機能を必要とし、ブラウザーでは有効です。シリアライズされた YAML 文書ではなく、意味上の YAML 文字列スカラーを返します。引用符とタグを正しく処理できるよう、YAML エミッターへ文字列として渡してください。標準の小数テキストは小数部分の末尾のゼロを除去します。無効な nullable は YAML の null スカラーを表す None/null を返します。有効で内部が未初期化のゼロなら "0" を返します。読み取りでは初期化状態や有効性を変更しません。ブラウザーでは Rust の None は null に対応します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="ja" />

## 関連メソッド

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
