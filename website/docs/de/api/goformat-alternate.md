---
locale: de
source_revision: 711eb90b21b1b79f0303fceb3e98f79a6b9b78a5eb632f30a661f662027c1167
translation_provenance: en-US-german-reviewed
api_id: GoFormat.alternate
---
# GoFormat.alternate

Das Formatfeld alternate setzen.

```rust
alternate: bool
```

## Parameter, Rückgabe und übliche Verwendung

Für f/F/e/E einen Dezimalpunkt beibehalten und für g/G nachgestellte Nullen im allgemeinen Format erhalten. Dies bedeutet keine alternative Zahlenbasis. Standard: false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-alternate" locale="de" />

## Verwandte Methoden

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
