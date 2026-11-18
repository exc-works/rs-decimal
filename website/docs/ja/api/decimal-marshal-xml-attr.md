---
locale: ja
source_revision: dbdcc64998397a143c045f2ffbee812382399e55664aea5f55e50930e8723a2a
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::marshal_xml_attr"
---
# Decimal::marshal_xml_attr

xml_attr の意味上の内容を取得します。

```rust
fn marshal_xml_attr (& self) -> String
```

## 引数、戻り値、通常の使い方

引数なし。String/string を返します。Rust は xml 機能を必要とし、ブラウザーでは有効です。デコード済みの属性内容を返します。呼び出し側が属性名と XML エスケープを処理します。標準の小数テキストは小数部分の末尾のゼロを除去します。未初期化のゼロは空の内容、初期化済みのゼロは "0" を返します。読み取りでは初期化状態や有効性を変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml-attr" locale="ja" />

## ブラウザーの制約

ソース入力は UTF-8 で 8192 バイトまでです。コアの数値解析前に、Rust は最初の YAML スカラーまたは XML 要素のデコード済みテキストを検査します。エスケープ、CDATA、アンダースコアの正規化、YAML 浮動小数点へのゼロ挿入も含みます。YAML スカラーは基数に応じた整数割り当てを制限するため、16 進数字文字が最大 4096 文字です。保守的な検査によりネイティブでは受理される入力も拒否する場合があります。スケール、係数、操作回数は[リソースポリシー](../resource-policy)に従います。ムーブ済みのハンドルは読み取りも変更もできません。

## 関連メソッド

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
