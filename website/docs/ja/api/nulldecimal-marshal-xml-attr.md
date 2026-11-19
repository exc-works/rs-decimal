---
locale: ja
source_revision: b8d3cac3d5f9251c9158f0efb63e99a5a338182fad355d14b4c984c245247f04
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::marshal_xml_attr"
---
# NullDecimal::marshal_xml_attr

xml_attr の意味上の内容を取得します。

```rust
fn marshal_xml_attr (& self) -> Option < String >
```

## 引数、戻り値、通常の使い方

引数なし。`Option<String>`/string | null を返します。Rust は xml 機能を必要とし、ブラウザーでは有効です。デコード済みの属性内容を返します。呼び出し側が属性名と XML エスケープを処理します。標準の小数テキストは小数部分の末尾のゼロを除去します。無効な nullable は None/null を返します。空の属性を書かず、属性を省略してください。有効で内部が未初期化のゼロなら "0" を返します。読み取りでは初期化状態や有効性を変更しません。ブラウザーでは Rust の None は null に対応します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml-attr" locale="ja" />

## 関連メソッド

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
