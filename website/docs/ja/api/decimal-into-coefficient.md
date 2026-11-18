---
locale: ja
source_revision: fa1a3656d76ca2759cc49aed5b71589136f0ed7e84a57f811be7062ea9aa8860
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

スケール適用前の整数係数をムーブして取り出します。

```rust
fn into_coefficient (self) -> BigInt
```

## 引数、戻り値、通常の使い方

引数なし。Rust は BigInt、JS は bigint を返します。スケールと初期化状態は失われるため、1.2300 は 12300 になります。既定のゼロは 0 を返します。Decimal は消費され、再び読み取れません。bridge で消費せずにスナップショットを得るには coefficient を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
