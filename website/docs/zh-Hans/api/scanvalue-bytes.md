---
locale: zh-Hans
source_revision: 4a909248a2e14bd655a50dd72e826f6d87a3347b735e76ddfe31ae9a4d4b4671
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

选择 Bytes SQL 输入变体。

```rust
Bytes (& 'a [u8])
```

## 参数、返回值及常规用法

借用小数文本字节，不是 binary/BSON 编码。浏览器将 Uint8Array 保留至 scan，之前的修改会被观察到。scan 不修改输入。无效 UTF-8 导致小数解析失败。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
