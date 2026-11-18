---
locale: de
source_revision: 9459cb9d77c7f6ad22b96853baef01b1a9461529cecff18a116b5d2dee9166d3
translation_provenance: en-US-german-reviewed
api_id: Decimal
---
# Decimal

Einen exakten Koeffizienten und eine nichtnegative Dezimalskala speichern.

```rust
pub struct Decimal
```

## Übliche Verwendung und Semantik

Der Wert ist Koeffizient × 10^-scale. Die Skala zählt Nachkommastellen einschließlich erhaltener nachgestellter Nullen. Default ist eine nicht initialisierte numerische Null. Konstruktoren erzeugen initialisierte Werte; ausleihende Methoden erhalten Eingaben, konsumierende Methoden machen deren Browserwrapper ungültig. Verwenden Sie benannte Konstruktoren der Bridge; new Decimal() ist kein öffentlicher Konstruktor.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal" locale="de" />

Das gemeinsame Beispiel läuft in nativem Rust und im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
