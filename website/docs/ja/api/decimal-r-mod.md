---
locale: ja
source_revision: 93dfb354f7906fd9c23e3b9ad99dace81c54c1bb0757216e3e8aa83d274f8bb9
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::r#mod
---
# Decimal::r#mod

ゼロ方向に切り捨てる除算の符号付き剰余を返します。

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<Decimal, DecimalError> を返します。剰余は被除数の符号と両オペランドの大きい方のスケールを持ち、非負のユークリッド剰余ではありません。除数がゼロなら DivideByZero です。入力は借用します。mod は予約語なので Rust では r#mod、JS では .mod(other) を使います。どちらも同じコアの動作を呼び出します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
