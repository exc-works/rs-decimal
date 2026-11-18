---
locale: zh-Hans
source_revision: 313aa913e617945bd29e0e39f2ee0488ae3c0d32ae43ffdacbbf7badbfb3dbb1
api_id: "DecimalError::impl fmt :: Display"
---
# DecimalError::impl fmt :: Display

使用 Rust fmt  实现。

```rust
impl fmt :: Display for DecimalError 
```

## 参数、返回值及常规用法

Display 只写消息。使用 format! 或 to_string()；机器处理应另行检查 kinds。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
