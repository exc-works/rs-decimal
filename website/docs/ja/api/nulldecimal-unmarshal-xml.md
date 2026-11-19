---
locale: ja
source_revision: e94c40441044730df649ba261de89daead4c5ce018d30a42e07e0ff558def35c
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::unmarshal_xml"
---
# NullDecimal::unmarshal_xml

xml を既存の値へデコードします。

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

source: &str/string。レシーバーを変更し、`Result<(), DecimalError>`/undefined を返します。Rust は xml を必要とします。最初の XML 要素をデコードし、直接のテキスト、CDATA、デコード済み実体参照を結合します。入れ子の子要素のテキストは無視し、子の前後のテキストを連結します。後続の要素は無視します。名前、属性、文字参照、終了タグの対応はコアが検証します。空白除去後の内容が空なら valid を解除し、内部 Decimal を未初期化のゼロにリセットします。null 以外の入力のデコードが成功すると小数を初期化し、ゼロ以外の数値の末尾のスケールを保持します。valid=true にします。エラー時はレシーバーの全フィールドを保持し、Unmarshal を含むライブラリのエラー種別を保持します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml" locale="ja" />

## 関連メソッド

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
