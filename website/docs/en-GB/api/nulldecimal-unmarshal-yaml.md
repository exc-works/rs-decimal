---
locale: en-GB
source_revision: dbc15c20de89c4c0e7f4a73778b0f1b410d8c87e2c8de57f1e13798c66980cb2
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="yaml-xml-nulldecimal-unmarshal-yaml" locale="en-GB" />

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
