---
locale: ja
source_revision: 181ceff066af430ef0b4f061e0d4890d14b967a20ab1d2fe96ac03e3b4f024fe
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::add_owned
---
# Decimal::add_owned

左オペランドを消費して正確な演算を行います。

```rust
fn add_owned (self , other : & Self) -> Self
```

## 引数、戻り値、通常の使い方

other: &Decimal。入力の大きい方のスケールで初期化済み Decimal を返します。レシーバーはムーブされ、other は引き続き使用できます。記憶領域は再利用できますが、増大時には割り当てが発生することがあります。JavaScript は同じラッパーをレシーバーと other の両方に渡すことをムーブ前に拒否します。Rust に入ると、リソースエラーでもレシーバーを消費します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-add-owned" locale="ja" />

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
