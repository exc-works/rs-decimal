---
locale: de
source_revision: 60b71a4e750d3835abdec3e2e546f11a77759013f3f84ad31d530c433bd67b11
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
Auf der Methodenseite können Sie die Argumente ändern und das Beispiel ausführen. Öffnen Sie es im Playground, um den Code zu bearbeiten oder zu teilen.

Beim Teilen werden bis zu 4 KiB UTF-8-Code und der Build-Commit der Bibliothek im URL-Fragment gespeichert. Der Code wird nicht an einen Server hochgeladen. Ein Freigabelink erfordert denselben Build, lädt den Code ohne Ausführung und löscht vorherige Ergebnisse. Ungültige Links lassen den aktuellen Entwurf unverändert. Prüfen Sie den Code und klicken Sie auf Ausführen. Exportieren Sie größere Skripte als Datei; der Editor akzeptiert bis zu 64 KiB. Links enthalten den Text Ihres Skripts. Prüfen Sie ihn vor dem Teilen.
:::

[Ressourcenrichtlinie des Playgrounds](./resource-policy)
