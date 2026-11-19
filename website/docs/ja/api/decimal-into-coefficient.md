---
locale: ja
source_revision: 784a138fd9dfff6c4bbabb3b837f6e63c10c904c5578943fe043993afcf99a55
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

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
