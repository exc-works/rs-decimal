---
locale: ja
source_revision: e8f2b09a741088cc99921865f54ef434d0d70ac376d4e29b7efc3c00a51c3956
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

## 使用上の注意と制約

精度を失った JavaScript number を後から bigint に変換しないでください。末尾が `n` の整数リテラルか、正確な整数テキストから始めてください。playground の入力は最大 4096 桁です。ネイティブライブラリにはこの playground 固有の制限はありません。

ブラウザーは[リソースポリシー](../resource-policy)も適用します。JavaScript 入力は Rust の呼び出し前に検証します。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
