---
locale: zh-Hans
source_revision: 5acf1a9800c0d02bd1cb1fa68c2c1f7068c1bb36cf20034e0a1e404758ce2bc4
api_id: "ErrorKind::InvalidRoot"
---
# ErrorKind::InvalidRoot

识别 InvalidRoot 错误。

```rust
InvalidRoot
```

## 参数、返回值及常规用法

根次数不是正数。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidroot" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
