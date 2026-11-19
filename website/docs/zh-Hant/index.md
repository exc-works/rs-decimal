---
locale: zh-Hant
source_revision: 9dbdcebee0aa8b717d5990e56743976e5e18fb73cbd32737b093a37a0ebe40de
sidebar: false
aside: false
---
# rs-decimal

rs-decimal 是用 Rust 處理十進位數的函式庫。你可以用它加總金額、將單價乘以數量，也可以決定收據上的金額要顯示幾位小數。

## 一張收據怎麼算？

兩本筆記本，每本 4.50 元，再加 1.25 元運費，應付 10.25 元。從訂單讀入單價到印出收據，都可以用 Decimal 完成。

<div class="receipt-equation">4.50 × 2 + 1.25 = <strong>10.25</strong></div>

- [動手算一張收據](/zh-Hant/guide)
- [在瀏覽器裡改數量、看結果](/zh-Hant/playground)
- [依操作查找 API 方法](/zh-Hant/api/)
