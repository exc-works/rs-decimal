---
locale: en-US
source_revision: 8ca020b2d56652a32165b4756446630d2a88be60a068a89e4f65ce218930abe5
api_id: "Decimal::marshal_xml_attr"
---
# Decimal::marshal_xml_attr

Get xml_attr semantic content.

```rust
fn marshal_xml_attr (& self) -> String
```

## Parameters, return and normal usage

No parameters; returns String/string. Rust requires the xml feature, enabled in the browser. Returns already decoded attribute content; the caller supplies the attribute name and XML escaping. Canonical decimal text removes fractional trailing zeros. Uninitialized zero returns empty content; initialized zero returns "0". Reading never changes initialization or validity.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml-attr" locale="en-US" />

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
