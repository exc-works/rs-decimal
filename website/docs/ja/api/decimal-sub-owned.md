---
locale: ja
source_revision: ddfedf2b2a7b0914238752b884265b0c43b55c647cf39e728c8143e706a5b5ba
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

左オペランドを消費して正確な演算を行います。

```rust
fn sub_owned (self , other : & Self) -> Self
```

## 引数、戻り値、通常の使い方

other: &Decimal。入力の大きい方のスケールで初期化済み Decimal を返します。レシーバーはムーブされ、other は引き続き使用できます。記憶領域は再利用できますが、増大時には割り当てが発生することがあります。bridge は同じラッパーをレシーバーと other の両方に渡すことをムーブ前に拒否します。Rust に入ると、リソースエラーでもレシーバーを消費します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="ja" />

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
