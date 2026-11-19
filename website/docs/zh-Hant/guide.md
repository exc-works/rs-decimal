---
locale: zh-Hant
source_revision: f513baf539375e2a99059b44ed14e0bbb75f211d5e47c68b2215892ab1d50f56
---
# 快速入門

客人買了兩本筆記本，每本 4.50 元，運費 1.25 元。我們來算該收多少錢，再把金額印在收據上。

## 算出應付金額

先照訂單上的寫法，用 `new_from_string("4.50")` 讀入單價。用 `mul_exact` 乘以數量 2，商品小計是 9.00 元。再用 `add` 加上運費，應付金額就是 10.25 元。

<<< ../../../examples/web_getting_started.rs#calculate

## 印出收據金額

對商品小計呼叫 `string()` 會得到 "9"，呼叫 `string_with_trailing_zeros()` 會得到 "9.00"。兩者是同一筆錢，後者保留了單價中的兩位小數，適合印在收據上。

<<< ../../../examples/web_getting_started.rs#print

[在瀏覽器裡改數量、看結果](./playground) — 試著把數量從 2 改成 3，再執行一次。商品小計應變成 13.50 元，應付金額應變成 14.75 元。

## 在本機執行 Rust 範例

安裝 Rust 1.85 或更新版本後，執行以下指令下載儲存庫並執行範例。終端機會印出 10.25。

```sh
git clone https://github.com/exc-works/rs-decimal.git
cd rs-decimal
cargo run --locked --example web_getting_started
```
