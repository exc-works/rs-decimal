---
locale: de
source_revision: 4824e4dc634e3ef762c96aa330a4aec14949a376f5c5d2b586754496c1c0639f
translation_provenance: en-US-german-reviewed
api_id: "NullDecimal::impl Deserialize < 'de >::deserialize"
---
# NullDecimal::impl Deserialize < 'de >::deserialize

Mit dem generischen Serde-Visitor deserialisieren.

```rust
fn deserialize < D : Deserializer < 'de > > (d : D) -> Result < Self , D :: Error >
```

## Parameter, Rückgabe und übliche Verwendung

deserializer: D mit Implementierung von Deserializer; liefert Self oder D::Error. Akzeptiert exakte Dezimalzeichenfolgen, ganzzahlige Visitor-Eingaben und null. Gleitkomma-Visitor-Eingaben werden abgewiesen: Eine JSON-Zahl mit Bruchteil/Exponent muss den speziellen rohen JSON-/config-Adapter verwenden. JSON null ergibt einen ungültigen nullable-Wert mit innerer nicht initialisierter Null. Leere Zeichenfolgen sind ungültig. Der statische Browseradapter deserialize(source) führt serde_json auf vollständigem ursprünglichem JSON-Text aus; sein Umfang ist enger als die Unterstützung beliebiger Rust-Deserializer. Fehler stellen category="serde" und die ursprüngliche Meldung bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-deserialize-de-deserialize" locale="de" />

## Grenzen im Browser

Die Features config und serde sind aktiviert. Quelltext/-bytes sind auf 8192 UTF-8-Bytes begrenzt; Rust prüft Dezimaltext und decodierte JSON-Zeichenfolgen vor dem Parsen, sodass Escapes die [Ressourcenrichtlinie](../resource-policy) nicht umgehen. 128-Bit-Ganzzahlen benötigen bigint im gültigen Bereich. Ausgeliehene Decimal-/NullDecimal-Handles müssen beim Decodieren noch aktiv sein; die Konvertierung erzeugt unabhängige Werte ohne Eingabekonsum. Generisches Serde-Trait-Verhalten wird durch konkrete JSON-Adapter gezeigt.

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
