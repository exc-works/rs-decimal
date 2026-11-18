---
locale: de
source_revision: 38cd98cd0eb4f9b158b56009f272819540665b187722c4eef35585af26929132
translation_provenance: en-US-german-reviewed
api_id: COMPATIBILITY_REVISION
---
# COMPATIBILITY_REVISION

Die für Kompatibilitätsprüfungen verwendete Upstream-Go-Revision untersuchen.

```rust
const COMPATIBILITY_REVISION: & str = "a8791208e2d15483cf652cd2814b74b0c1a84187"
```

## Übliche Verwendung und Semantik

Dieses unveränderliche &str bezeichnet eine Quellrevision, nicht den aktuellen Rust-/Web-Build-Commit. Der Browser liest dieselbe Konstante aus kompiliertem Rust. Sie ist weder Paketversion noch Bereitstellungskennung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-compatibility-revision" locale="de" />

Das gemeinsame Beispiel läuft in nativem Rust und im Browser.

## Weiterführende Dokumentation

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
