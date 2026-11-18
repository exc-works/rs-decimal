---
locale: ja
source_revision: 805fc62e495d042f613bdfd51817251dea41dcde63bd630737a6a7dd4ec14e8a
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::scan"
---
# Decimal::scan

ドライバーに依存しない SQL スカラーを読み取ります。

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

value: ScanValue。レシーバーを変更し、`Result<(), DecimalError>`/undefined を返します。バリアントを明示的に選択してください。Int64 は正確な bigint、Float32 は binary32 への変換、Text/Bytes は元の小数桁の保持を使います。SQL テキストの外側には対応する二重引用符 1 組を置け、JSON エスケープ解除なしで除去します。NULL はレシーバーを未初期化のゼロにリセットします。未対応の型と非有限の浮動小数点は Unmarshal です。他のデコード失敗は基底の種類を保持します。すべてのエラーでレシーバーの状態を保持します。このアダプターにデータベース接続は不要です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-scan" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
