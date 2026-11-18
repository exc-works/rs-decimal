---
locale: en-US
source_revision: e1a6c7014c208ee06566822a33e4d32bdbca21d8076f5cc955ee91bcd5ae501d
api_id: "NullDecimal::unmarshal_yaml"
---
# NullDecimal::unmarshal_yaml

Decode yaml into an existing value.

```rust
fn unmarshal_yaml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

source: &str/string; mutates the receiver and returns `Result<(), DecimalError>`/undefined. Rust requires yaml. Pass original YAML source so quoting, tags and exact digits remain available to Rust. Plain 012 resolves as octal 10, whereas quoted "012" or !!str 012 parses decimal 12. Numeric tags/radix/underscores follow the core resolver: plain 1_000 is 1000 but quoted "1_000" is invalid. Booleans, collections and block scalars are rejected. Only the first document supplies the value; later documents still must be syntactically valid. Null or an empty document clears valid and resets the inner Decimal to uninitialized zero. Successful non-null decoding initializes the decimal and preserves nonzero trailing scale. It sets valid=true. Errors preserve all receiver fields and retain library error kinds including Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-yaml" locale="en-US" />

## Browser boundaries

Source input is capped at 8192 UTF-8 bytes. Before core numeric parsing, Rust checks the decoded first YAML scalar or XML element text, including escapes, CDATA, underscore normalization and YAML float zero insertion. YAML scalars permit at most 4096 hexadecimal digit characters to bound radix integer allocation; conservative checks may reject inputs accepted natively. Scale, coefficient and operation counts follow the [resource policy](../resource-policy). Moved handles cannot be read or mutated.

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
