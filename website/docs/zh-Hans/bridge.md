---
locale: zh-Hans
source_revision: bd473c0431492bbeed88af5f453e401cc80b45775529394fc98a50e453c639ca
---
# 在浏览器中调用 Rust 方法

Playground 使用 JavaScript 调用编译为 Wasm 的 Rust 库。方法名、参数顺序、
舍入规则和十进制状态与 Rust 对应；JavaScript 只负责组织流程，不重新计算
十进制结果。各方法的行为见[方法目录](./api/)，示例可在 [Playground](./playground) 中运行。

```javascript
const price = Decimal.new_from_string('12.350');
const total = price.mul_exact(Decimal.new(3n));
total.string_with_trailing_zeros(); // '37.050'
```

十进制输入使用原始字符串，i64 参数使用 `bigint`（`3n`，不是 `3`）。
`Decimal.new` 拒绝超出 i64 范围的值、数字字符串和 JS number。
precision 使用 wasm32 有符号整数范围内的 JS 整数；小数、非有限值和越界值
在 ABI 转换前被拒绝，避免截断或回绕。负整数 precision 会传入 Rust，
由原方法返回 InvalidPrecision。

## 错误与所有权

Rust 返回错误时抛出 Error，包含 `name: 'DecimalError'`、
`category: 'library'`、原始 `message` 和完整的 `kinds` 字符串数组。
不要只读取一个类别：解码错误可能同时属于多个类别。
JS 类型或范围错误使用 `BridgeError` 和 `category: 'parameter'`，
不伪装成 Rust 算术错误。数值形式的无效舍入模式由 Rust 检查。

借用方法调用后对象仍可使用。`into_rescaled` 会消耗接收者，即使 Rust
返回错误也一样；再次使用已消耗或释放的对象会得到 `moved` BridgeError。
不合法的 JS 参数类型在调用前被拒绝，因此不会消耗接收者。
`clone()` 保留表示和初始化状态。`free()` 释放仍存活的句柄，已消耗后调用
也安全。运行器完成后将销毁整个 Wasm 实例，不依赖垃圾回收终结器释放资源。
不同 JavaScript 实例的对象不能混用。

## 可空值与字节

`NullDecimal.new(decimal)` 消耗参数并设置 `valid=true`，即使参数是
未初始化零。`NullDecimal.default()` 表示缺失。修改 `valid` 只改变有效性，
不会清空内部值。`get_decimal_clone()` 是显式命名的适配方法，返回独立副本；
`set_decimal(value)` 移入新的内部值，并保留 valid。没有会误导为实时引用的
`.decimal` getter。

`coefficient()` 返回不可变的 JS bigint 快照。JavaScript 不复现 Rust 的
借用生命周期和编译期 trait 检查。`ByteBuffer.new(Uint8Array)` 复制输入字节，
`to_uint8_array()` 返回副本，不受 Wasm 内存增长影响。追加文本方法见[方法目录](./api/)。

JavaScript 的 `snapshot(value)` 立即捕获精确状态，包括无效 NullDecimal 中
保留的内部值。返回的数据被冻结，后续修改对象不会改变旧输出。
隔离运行器限制递归输出和执行时间，详见[资源限制](./resource-policy)。
