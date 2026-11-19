---
locale: zh-Hans
source_revision: 3ce42c12330a0659d9f932da918ccfd356a91e7dc3aa65b330f45700001e6ba2
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

消耗错误并增加类别。

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## 参数、返回值及常规用法

kind: ErrorKind；消耗 self 并返回新 DecimalError。保留原类别和消息，忽略重复类别。原浏览器句柄失效。无效 JS 类别在移动前被拒绝。浏览器具名构造函数创建可释放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。抛出的库异常是原生 JS Error，带 category="library" 和 kinds 数组；此时使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved JavaScript 错误与库错误分开，不带库类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
