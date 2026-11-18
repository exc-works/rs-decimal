---
locale: zh-Hant
source_revision: fbadb341fac6ded3986e830a05797cf96857f33d92f3224a113dd3d05ba9eb85
---
# Playground

用 JavaScript 呼叫與 Rust 同名的方法。點選執行，檢查精確數值和狀態。

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="zh-Hant" :labels="locales['zh-Hant']" />

方法頁面提供共用範例的常值參數表單。參數依 JavaScript 呼叫順序排列，保留 Rust 的方法名稱與參數順序。產生的程式碼可在此編輯器中繼續修改複雜值或新增呼叫。方法頁面的預期狀態對應原始範例。在本分頁切換語言後會保留表單修改。

分享功能將最多 4 KiB 的 UTF-8 程式碼與程式庫建置提交編號放入 URL 片段，不會將程式碼上傳至伺服器。分享連結需要相同的建置版本，載入時會清除先前的結果，但不會自動執行。無效連結會保留目前的草稿。檢查程式碼後按下執行。較大的腳本請匯出為檔案；編輯器最多接受 64 KiB。連結會包含腳本中的文字，分享前請檢查程式碼。
