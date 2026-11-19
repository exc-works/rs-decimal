---
locale: en-US
source_revision: b8d3cac3d5f9251c9158f0efb63e99a5a338182fad355d14b4c984c245247f04
api_id: "NullDecimal::marshal_xml_attr"
---
# NullDecimal::marshal_xml_attr

Get xml_attr semantic content.

```rust
fn marshal_xml_attr (& self) -> Option < String >
```

## Parameters, return and normal usage

No parameters; returns `Option<String>`/string | null. Rust requires the xml feature, enabled in the browser. Returns already decoded attribute content; the caller supplies the attribute name and XML escaping. Canonical decimal text removes fractional trailing zeros. Invalid nullable values return None/null: omit the attribute, rather than writing an empty attribute. Valid inner uninitialized zero returns "0". Reading never changes initialization or validity; the browser maps Rust None to null.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml-attr" locale="en-US" />

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
