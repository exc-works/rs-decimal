---
locale: ko
source_revision: 602dcc7d7a4e6e51eaa040b5b95f43c52285a62967f721872ad5548725d33f50
translation_provenance: en-US-korean-reviewed
api_id: Decimal::not_equal
---
# Decimal::not_equal

두 소수가 서로 다른 수치를 나타내는지 확인합니다.

```rust
fn not_equal (& self , other : & Self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 bool/boolean을 반환합니다. equal()의 반대이며 계수나 소수 자릿수 스냅샷을 비교하는 것이 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
