---
locale: de
source_revision: 8de700fe2bea77817d4cc64ce868f77fd5301e630577d698cc1cf5a3b4f76ce5
translation_provenance: en-US-german-reviewed
api_id: "reexport::PRECISION_FIXED_SIZE"
---
# reexport::PRECISION_FIXED_SIZE

Die Bytelänge des binären Präzisionspräfixes lesen.

```rust
pub use binary::PRECISION_FIXED_SIZE as PRECISION_FIXED_SIZE
```

## Parameter, Rückgabe und übliche Verwendung

Eine Rust-usize-Konstante mit Wert 4: die Bytelänge des Big-Endian-Skalenpräfixes im Go-kompatiblen Binärformat. Sie ist weder Dezimalziffernzahl noch Präzisionsgrenze. Der Browser stellt eine schreibgeschützte number aus Rust bereit. Der Reexport an der Crate-Wurzel ist Rust-Importsyntax.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-reexport-precision-fixed-size" locale="de" />

## Verwandte Methoden

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
