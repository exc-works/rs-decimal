---
locale: en-US
source_revision: 54029652b1ccf0bd92160e138af8c8334fba2feb6f64eea2e573873d94dfb3f7
api_id: "Decimal::unmarshal_xml_attr"
---
# Decimal::unmarshal_xml_attr

Decode xml_attr into an existing value.

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

value: &str/string; mutates the receiver and returns `Result<(), DecimalError>`/undefined. Rust requires xml. Pass the already entity-decoded attribute value, not an XML attribute fragment: "1.23" is valid but "&#49;.23" is not decoded again. Trims surrounding whitespace and parses exact decimal text. Empty content resets the receiver to uninitialized zero. Successful non-null decoding initializes the decimal and preserves nonzero trailing scale. Errors preserve all receiver fields and retain library error kinds including Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml-attr" locale="en-US" />

## Browser boundaries

Source input is capped at 8192 UTF-8 bytes. Before core numeric parsing, Rust checks the decoded first YAML scalar or XML element text, including escapes, CDATA, underscore normalization and YAML float zero insertion. YAML scalars permit at most 4096 hexadecimal digit characters to bound radix integer allocation; conservative checks may reject inputs accepted natively. Scale, coefficient and operation counts follow the [resource policy](../resource-policy). Moved handles cannot be read or mutated.

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
