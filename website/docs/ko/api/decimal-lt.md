---
locale: ko
source_revision: ea763f949f6ea9085a5bdbf656ce6fbf3e8731a7f67b0b4db3858116c77c3d15
translation_provenance: en-US-korean-reviewed
api_id: Decimal::lt
---
# Decimal::lt

이 값이 다른 값보다 작은지 확인합니다.

```rust
fn lt (& self , other : & Self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 bool/boolean을 반환합니다. 저장된 소수 자릿수가 달라도 수치가 같으면 false입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
