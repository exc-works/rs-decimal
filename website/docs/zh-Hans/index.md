---
locale: zh-Hans
source_revision: 92be186c5740ac6d22b3ca742b8ced1f5ab7822e9ab835f05db11a750bdadf79
aside: false
---
# rs-decimal

rs-decimal 是一个用 Rust 处理十进制数的库。你可以用它把几笔金额相加、将单价乘以数量，也可以决定小票上的金额要显示几位小数。

## 一张小票是怎么算出来的？

两本笔记本，每本 4.50 元，再加 1.25 元运费，应付 10.25 元。从订单上读入单价，到最后打印小票，都能用 Decimal 完成。

<div class="receipt-equation">4.50 × 2 + 1.25 = <strong>10.25</strong></div>

<div class="ref-actions">

[跟着算一张小票](/zh-Hans/guide)
[在浏览器里改数量、看结果](/zh-Hans/playground)

</div>
