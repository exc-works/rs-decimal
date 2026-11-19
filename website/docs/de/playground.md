---
locale: de
source_revision: d62c9d14668960966b8d4b8ce2f6a65282abf78ff3e9cfea0c0c0f5225f8d9bc
---
# Playground

Starten Sie die Berechnung für zwei Hefte zu je 4,50 plus 1,25 Versand. Zu zahlen sind 10,25. Bei 20,00 erhalten Sie 9,75 zurück. Ändern Sie die Menge von `2n` auf `3n` und führen Sie den Code erneut aus. Der neue Gesamtbetrag ist 14,75.

Falls ein eigener Entwurf geöffnet ist, exportieren Sie ihn, bevor Sie mit **Beispiel zurücksetzen** den Beleg laden.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="de" :labels="locales['de']" />

::: details Methodenbeispiele und Freigabe
Methodenseiten bieten Formulare für literale Argumente ihrer gemeinsamen Beispiele. Die Parameter folgen der JavaScript-Aufrufreihenfolge; Rust-Methodennamen und Argumentreihenfolge bleiben erhalten. Öffnen Sie die Vorschau in diesem Editor, um komplexe Werte zu ändern oder Aufrufe hinzuzufügen. Der erwartete Zustand auf der Methodenseite beschreibt das ursprüngliche Beispiel. Formularänderungen bleiben beim Sprachwechsel in diesem Tab erhalten.

Beim Teilen werden bis zu 4 KiB UTF-8-Code und der Build-Commit der Bibliothek im URL-Fragment gespeichert. Der Code wird nicht an einen Server hochgeladen. Ein Freigabelink erfordert denselben Build, lädt den Code ohne Ausführung und löscht vorherige Ergebnisse. Ungültige Links lassen den aktuellen Entwurf unverändert. Prüfen Sie den Code und klicken Sie auf Ausführen. Exportieren Sie größere Skripte als Datei; der Editor akzeptiert bis zu 64 KiB. Links enthalten den Text Ihres Skripts. Prüfen Sie ihn vor dem Teilen.
:::
