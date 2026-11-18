---
locale: ja
source_revision: ab458b72e36df078d37dd6de4be21a55e10d1e6c955c47b4abcbe87cbbdd8e23
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::quo_rem
---
# Decimal::quo_rem

整数商と符号付き Decimal 剰余を返します。

```rust
fn quo_rem (& self , other : & Self) -> Result < (Self , Self) , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<(Decimal, Decimal), DecimalError> を返し、[quotient, remainder] に対応します。商はゼロ方向へ切り捨てたスケール 0 の値です。剰余は被除数の符号と両オペランドの大きい方のスケールを持ち、self = quotient * other + remainder を満たします。各結果は独立した初期化済みラッパーで、入力も使用可能なままです。除数がゼロなら桁揃えの前に DivideByZero を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-quo-rem" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
