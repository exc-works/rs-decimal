---
locale: zh-Hans
source_revision: f513baf539375e2a99059b44ed14e0bbb75f211d5e47c68b2215892ab1d50f56
---
# 快速开始

顾客买了两本笔记本，每本 4.50 元，运费 1.25 元。我们来算收银台该收多少钱，再把金额印到小票上。

## 算出应付金额

先照订单上的写法，用 `new_from_string("4.50")` 读入单价。用 `mul_exact` 乘以数量 2，商品小计是 9.00 元。再用 `add` 加上运费，应付就是 10.25 元。

<<< ../../../examples/web_getting_started.rs#calculate

## 把金额印在小票上

对商品小计调用 `string()` 会得到 "9"，调用 `string_with_trailing_zeros()` 会得到 "9.00"。两者是同一笔钱，后者保留了单价里的两位小数，适合印在小票上。

<<< ../../../examples/web_getting_started.rs#print

[在浏览器里改数量、看结果](./playground) — 试着把数量从 2 改成 3，再运行一次。商品小计应变成 13.50 元，应付金额应变成 14.75 元。

## 在本机运行 Rust 示例

安装 Rust 1.85 或更高版本后，运行以下命令下载仓库并执行示例。终端会打印 10.25。

```sh
git clone https://github.com/exc-works/rs-decimal.git
cd rs-decimal
cargo run --locked --example web_getting_started
```
