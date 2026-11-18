---
locale: ja
source_revision: 38cd98cd0eb4f9b158b56009f272819540665b187722c4eef35585af26929132
translation_provenance: en-US-japanese-reviewed
api_id: COMPATIBILITY_REVISION
---
# COMPATIBILITY_REVISION

互換性検証に使う上流 Go のリビジョンを確認します。

```rust
const COMPATIBILITY_REVISION: & str = "a8791208e2d15483cf652cd2814b74b0c1a84187"
```

## 通常の使い方と意味

この不変の &str はソースリビジョンであり、現在の Rust/web ビルドのコミットではありません。ブラウザーはコンパイルされた Rust から同じ定数を読み取ります。パッケージのバージョンやデプロイ識別子として解釈しないでください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-compatibility-revision" locale="ja" />

共通の例をネイティブ Rust とブラウザーで実行します。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
