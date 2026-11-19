---
locale: en-US
source_revision: 90631f865fb2217f0d8137591d3ab6e82b220d3c98120fa1ec15f91a894f2664
api_id: "Decimal::marshal_xml"
---
# Decimal::marshal_xml

Get xml semantic content.

```rust
fn marshal_xml (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Rust requires the xml feature, enabled in the browser. Returns element character content; the caller supplies the element name, namespace and XML escaping. Canonical decimal text removes fractional trailing zeros. Uninitialized zero returns empty content; initialized zero returns "0". Reading never changes initialization or validity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml" locale="en-US" />

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
