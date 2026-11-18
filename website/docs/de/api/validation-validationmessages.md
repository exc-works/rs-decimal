---
locale: de
source_revision: 7d029f4b4a98b484b183c91199768a9635c9e85851c7f98513e0ef2565e4aa2e
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationMessages"
---
# validation::ValidationMessages

Validierungsfehler mit eigenen Vorlagen übersetzen.

```rust
pub struct ValidationMessages
```

## Parameter, Rückgabe und übliche Verwendung

Besitzt Vorlagen zur Übersetzung von ValidationError-Datensätzen. Verwenden Sie new(locale) oder with_messages(map); Änderungen an einer anderen Vorlagenmap beeinflussen dieses Handle nicht. Gebietsschemaabgleich ignoriert Groß-/Kleinschreibung und akzeptiert Bindestriche/Unterstriche sowie Sprach-/Regionssuffixe; zh-Hant/zh-TW wählen traditionelles Chinesisch. en-US/en-GB verwenden Englisch; unbekannte Gebietsschemata fallen auf Englisch zurück. {0} fügt das Feld und {1} den ursprünglichen Parameter in einem Durchgang ein; eingefügte Klammern werden nicht erneut expandiert. Ergebnisse sind Klartext; stellen Sie sie als Text dar, nicht als HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
