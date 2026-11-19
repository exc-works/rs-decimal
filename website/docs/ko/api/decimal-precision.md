---
locale: ko
source_revision: 0ad97f33b356cd281b88951f90d8d3c10dff06f728f22f7136b85c34bc769e4e
translation_provenance: en-US-korean-reviewed
api_id: Decimal::precision
---
# Decimal::precision

값에 저장된 소수 자릿수를 읽습니다.

```rust
fn precision (& self) -> isize
```

## 매개변수, 반환값, 사용 시 주의점

isize(브라우저에서는 정수 number)를 반환합니다. 유효 숫자의 개수가 아니라 소수 자릿수이며 후행 0도 포함합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
