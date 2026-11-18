---
locale: ja
source_revision: c870cbfccba48189ea2c1f73c60ffc0835f379cdb13fc7e1adf9905fa90662b8
translation_provenance: en-US-japanese-reviewed
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

Unmarshal の失敗を識別します。

```rust
Unmarshal
```

## 引数、戻り値、通常の使い方

デコードに失敗しました。InvalidFormat など別の種類が原因を表す場合があるため、1 種類だけと仮定せず包含を調べてください。ErrorKind はメッセージ文字列ではなく、安定した分類です。ブラウザーの列挙値は Rust のバリアント名です。構築したエラーでは matches() を使い、捕捉した bridge 例外では kinds 配列を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
