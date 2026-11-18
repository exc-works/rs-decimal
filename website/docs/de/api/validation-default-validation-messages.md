---
locale: de
source_revision: 9f6709052cffc0b28f6c72e94064cf76722563f48008476b3a2d47db292f5bda
translation_provenance: en-US-german-reviewed
api_id: "validation::default_validation_messages"
---
# validation::default_validation_messages

Eine neue Map mit Gebietsschemavorlagen erhalten.

```rust
fn default_validation_messages (locale : & str) -> BTreeMap < String , String >
```

## Parameter, Rückgabe und übliche Verwendung

locale: &str/string; liefert `BTreeMap<String,String>`/ein gewöhnliches Objekt mit allen zwölf Regelvorlagen. Unbekannte Gebietsschemata verwenden Englisch. Jeder Aufruf besitzt seine Map; Änderungen beeinflussen weder spätere Aufrufe noch andere ValidationMessages-Objekte. Der Reexport ist der öffentliche Modulimportpfad derselben Funktion.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-default-validation-messages" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
