---
locale: ko
source_revision: f3c9953fc7c2ab0c097e714a9d8217e98239ea7e625d7a02d8c88afee82038af
translation_provenance: en-US-korean-reviewed
api_id: Decimal::gt
---
# Decimal::gt

이 값이 다른 값보다 큰지 확인합니다.

```rust
fn gt (& self , other : & Self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 bool/boolean을 반환합니다. 저장된 소수 자릿수가 달라도 수치가 같으면 false입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gt" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
