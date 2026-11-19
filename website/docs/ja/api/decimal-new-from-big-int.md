---
locale: ja
source_revision: cc346dbc73e3d203c64f61c0e3e68ec08682dec8479c74ee4ea48b4441aef5c2
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_big_int
---
# Decimal::new_from_big_int

任意精度の整数を、桁を失わずにスケール 0 の初期化済み Decimal へコピーします。

```rust
fn new_from_big_int (value : & BigInt) -> Self
```

## 引数と戻り値

Rust は `&BigInt` を借用し、JavaScript は不変の `bigint` を渡します。返される係数は元の値から独立しています。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int" locale="ja" />

## 使い方の補足

精度を失った JavaScript number を後から bigint に変換しないでください。末尾が `n` の整数リテラルか、正確な整数テキストから始めてください。playground の入力は最大 4096 桁です。ネイティブライブラリにはこの playground 固有の制限はありません。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
