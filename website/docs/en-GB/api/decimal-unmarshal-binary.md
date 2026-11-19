---
locale: en-GB
source_revision: 8a589312845f2d23d85f3d6da1987ee87f79dfecdc97a705386e9a336b17d7ed
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

Replace a decimal from a Go-compatible binary scalar.

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## Parameters, return and normal usage

bytes: &[u8]/Uint8Array; mutates the receiver and returns `Result<(), DecimalError>`/undefined. The first four bytes encode big-endian scale, followed by Gob header 2 (positive) or 3 (negative) and magnitude. Successful decoding preserves encoded scale. Empty input creates initialized zero, unlike default uninitialized zero. A four-byte prefix without a Gob payload is also accepted as initialized zero at that scale. Short prefixes, invalid versions and scale above 131072 return core Unmarshal errors. Errors preserve the receiver.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="en-GB" />

## Related methods

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
