---
locale: zh-Hans
source_revision: 8106ef71942ee657518a42cba33a247697b1ac2ebfcf011160e32c4adeb6e0b5
api_id: "reexport::ScanValue"
---
# reexport::ScanValue

从 crate 根路径导入 ScanValue。

```rust
pub use sql::ScanValue as ScanValue
```

## 参数、返回值及常规用法

通过 rs_decimal::ScanValue 访问同一公开条目。这是导入路径，不是额外实现或浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-scanvalue" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
