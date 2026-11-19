---
locale: zh-Hant
source_revision: 284f92d20227e82946bbbcda123bebd9143e64512e1a7f0633cf17755157f389
---
# Playground

按「執行」，算算兩本單價 4.50 元的筆記本加上 1.25 元運費，要付多少錢。輸出中的應付金額是 10.25 元，付 20.00 元應找 9.75 元。把數量從 `2n` 改成 `3n` 再執行一次，應付金額就會變成 14.75 元。

如果開啟的是自己的草稿，可以先匯出保存，再按「重設範例」載入收據。

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="zh-Hant" :labels="locales['zh-Hant']" />

::: details 方法範例與分享
在方法頁修改參數，範例程式碼與預期結果會自動更新。要編輯或分享整段程式碼，可在 Playground 中開啟。

分享功能將最多 4 KiB 的 UTF-8 程式碼與程式庫建置提交編號放入 URL 片段，不會將程式碼上傳至伺服器。分享連結需要相同的建置版本，載入時會清除先前的結果，但不會自動執行。無效連結會保留目前的草稿。檢查程式碼後按下執行。較大的腳本請匯出為檔案；編輯器最多接受 64 KiB。連結會包含腳本中的文字，分享前請檢查程式碼。
:::

[Playground 資源限制](./resource-policy)
