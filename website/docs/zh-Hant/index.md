---
locale: zh-Hant
source_revision: 92be186c5740ac6d22b3ca742b8ced1f5ab7822e9ab835f05db11a750bdadf79
aside: false
---
# rs-decimal

rs-decimal 是用 Rust 處理十進位數的函式庫。你可以用它加總金額、將單價乘以數量，也可以決定收據上的金額要顯示幾位小數。

## 一張收據怎麼算？

兩本筆記本，每本 4.50 元，再加 1.25 元運費，應付 10.25 元。從訂單讀入單價到印出收據，都可以用 Decimal 完成。

<div class="receipt-equation">4.50 × 2 + 1.25 = <strong>10.25</strong></div>

<div class="ref-actions">

[動手算一張收據](/zh-Hant/guide)
[在瀏覽器裡改數量、看結果](/zh-Hant/playground)

</div>
