---
locale: ko
source_revision: 7388877c082aab40b6cea1eaae69162f25e5c07d8d3673378f0fdd23fef3e494
translation_provenance: en-US-korean-reviewed
api_id: Decimal::scale
---
# Decimal::scale

저장된 음수가 아닌 소수 자릿수를 읽습니다.

```rust
fn scale (& self) -> usize
```

## 매개변수, 반환값, 사용 시 주의점

playground 소수 자릿수 제한 안에서 usize를 정확한 number로 반환합니다. precision()과 값이 같습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
