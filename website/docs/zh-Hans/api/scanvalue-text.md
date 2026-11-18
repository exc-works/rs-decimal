---
locale: zh-Hans
source_revision: b5a3d9a8d6370fd602bde442c0194ed8033b601f5d907d38c4cf9f51317197c6
api_id: "ScanValue::Text"
---
# ScanValue::Text

选择 Text SQL 输入变体。

```rust
Text (& 'a str)
```

## 参数、返回值及常规用法

借用精确小数文本。移除一对匹配的首尾双引号，但不按 JSON 解码转义。解析后保留非零值的末尾 scale。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-text" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
