---
locale: en-GB
source_revision: 7b63356b05d421377bb8ec0a29f08c338eaab7596c00a0c507a722a2de09d089
translation_provenance: en-US-reviewed-reuse
api_id: "NullDecimal::marshal_xml"
---
# NullDecimal::marshal_xml

Get xml semantic content.

```rust
fn marshal_xml (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Rust requires the xml feature, enabled in the browser. Returns element character content; the caller supplies the element name, namespace and XML escaping. Canonical decimal text removes fractional trailing zeros. Invalid nullable values return empty content; valid inner uninitialized zero returns "0". Reading never changes initialization or validity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml" locale="en-GB" />

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
