---
locale: ja
source_revision: 7b63356b05d421377bb8ec0a29f08c338eaab7596c00a0c507a722a2de09d089
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::marshal_xml"
---
# NullDecimal::marshal_xml

xml の意味上の内容を取得します。

```rust
fn marshal_xml (& self) -> String
```

## 引数、戻り値、通常の使い方

引数なし。String/string を返します。Rust は xml 機能を必要とし、ブラウザーでは有効です。要素の文字内容を返します。呼び出し側が要素名、名前空間、XML エスケープを処理します。標準の小数テキストは小数部分の末尾のゼロを除去します。無効な nullable は空の内容、有効で内部が未初期化のゼロなら "0" を返します。読み取りでは初期化状態や有効性を変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml" locale="ja" />

## 関連メソッド

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
