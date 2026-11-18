---
locale: zh-Hans
source_revision: 76aa255f8bc8b60ca22e7c6936a048607e56eb1eaa641962626233731b182883
api_id: "reexport::ErrorKind"
---
# reexport::ErrorKind

从 crate 根路径导入 ErrorKind。

```rust
pub use error::ErrorKind as ErrorKind
```

## 参数、返回值及常规用法

通过 rs_decimal::ErrorKind 访问同一公开条目。这是导入路径，不是额外实现或浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-errorkind" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
