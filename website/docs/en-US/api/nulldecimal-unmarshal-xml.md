---
locale: en-US
source_revision: e94c40441044730df649ba261de89daead4c5ce018d30a42e07e0ff558def35c
api_id: "NullDecimal::unmarshal_xml"
---
# NullDecimal::unmarshal_xml

Decode xml into an existing value.

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

source: &str/string; mutates the receiver and returns `Result<(), DecimalError>`/undefined. Rust requires xml. Decodes the first XML element, combining its direct text, CDATA and decoded entity references. Nested child text is ignored; text before/after a child is concatenated. Later elements are ignored. Names, attributes, character references and matched closing tags are validated by the core. Trimmed empty content clears valid and resets the inner Decimal to uninitialized zero. Successful non-null decoding initializes the decimal and preserves nonzero trailing scale. It sets valid=true. Errors preserve all receiver fields and retain library error kinds including Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml" locale="en-US" />

## Related methods

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
