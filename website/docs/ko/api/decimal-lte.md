---
locale: ko
source_revision: 633e4a423f3366a4dc502e8d35d4ffa6a3d6f7dd5647951875fd3e31e57bf2c5
translation_provenance: en-US-korean-reviewed
api_id: Decimal::lte
---
# Decimal::lte

이 값이 다른 값 이하인지 확인합니다.

```rust
fn lte (& self , other : & Self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 bool/boolean을 반환합니다. 동등성은 수치로 판단하며 소수 자릿수나 초기화 상태와 무관합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
