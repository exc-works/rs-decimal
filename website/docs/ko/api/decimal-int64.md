---
locale: ko
source_revision: f83d4eab7c44d20bd9f7f112876d8de08078fd38cb0bbaaf09e60fe2891d5d1e
translation_provenance: en-US-korean-reviewed
api_id: Decimal::int64
---
# Decimal::int64

성공 플래그와 함께 고정 너비 정수로 변환합니다.

```rust
fn int64 (& self) -> (i64 , bool)
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. (i64, bool)을 반환하며 [bigint, boolean]에 대응합니다. 범위 안의 정수 값은 정수와 true를 반환합니다. 소수나 오버플로는 버린 값 또는 예외가 아니라 (0, false)를 반환합니다. 소수 자릿수만으로 정수 여부가 정해지지 않으므로 12.00은 성공합니다. 실패와 유효한 0을 구분하도록 항상 플래그를 확인하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-int64" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
