---
locale: de
source_revision: 284f92d20227e82946bbbcda123bebd9143e64512e1a7f0633cf17755157f389
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
Wenn Sie auf einer Methodenseite einen Parameter ändern, werden Code und Ergebnis aktualisiert. Im Playground können Sie das ganze Skript bearbeiten und teilen.

Beim Teilen werden bis zu 4 KiB UTF-8-Code und der Build-Commit der Bibliothek im URL-Fragment gespeichert. Der Code wird nicht an einen Server hochgeladen. Ein Freigabelink erfordert denselben Build, lädt den Code ohne Ausführung und löscht vorherige Ergebnisse. Ungültige Links lassen den aktuellen Entwurf unverändert. Prüfen Sie den Code und klicken Sie auf Ausführen. Exportieren Sie größere Skripte als Datei; der Editor akzeptiert bis zu 64 KiB. Links enthalten den Text Ihres Skripts. Prüfen Sie ihn vor dem Teilen.
:::

[Ressourcenrichtlinie des Playgrounds](./resource-policy)
