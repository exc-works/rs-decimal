---
locale: ko
source_revision: 098d219e5e0932c9734847caf4da4f02238ac75ca2e27400d9527165d61283dd
translation_provenance: en-US-korean-reviewed
api_id: Decimal::cmp
---
# Decimal::cmp

두 수치를 비교하여 세 가지 결과 중 하나를 반환합니다.

```rust
fn cmp (& self , other : & Self) -> i32
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal(JavaScript에서는 Decimal)을 빌립니다. i32/number인 -1, 0, 1을 반환합니다. 소수 자릿수나 초기화 상태가 달라도 수치의 동등성에는 영향이 없습니다. Ordering을 반환하는 Ord::cmp가 아니라 고유 메서드입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
