---
locale: zh-Hans
source_revision: 4e9262b5d73b0bfa6dbb9a564894dae85973da389d298cf19e5836d09170ed0d
api_id: "NullDecimal::impl std :: fmt :: Display"
---
# NullDecimal::impl std :: fmt :: Display

通过 Rust Display 格式化 nullable 值。

```rust
impl std :: fmt :: Display for NullDecimal 
```

## 参数、返回值及常规用法

Display 委托 nullable string()：有效时输出内部规范文本，无效时输出 null。格式器写入文本，不是 JSON token 或字节缓冲区。format! 调用 Display::fmt；此 trait 语法在原生环境执行，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-impl-std-fmt-display" locale="zh-Hans" />

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
