---
locale: ko
source_revision: 762606c51dd255e44f3c362ba14134e460bd14c0850c5cfb4f977a9c95e0eb98
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_float32
---
# Decimal::new_from_float32

부동소수점으로 되돌렸을 때 같은 값이 되는 가장 짧은 십진 표현으로 생성합니다.

```rust
fn new_from_float32 (value : f32) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: f32를 JS number로 전달합니다. bridge는 먼저 binary64를 binary32로 좁히므로 16777217은 16777216이 됩니다. 이진 부동소수점에서 이미 잃은 십진 숫자는 복원하지 않습니다. 정확한 소수 입력에는 new_from_string을 사용하세요. NaN과 양/음의 무한대는 InvalidFormat입니다. 유한한 f64라도 f32로 좁힐 때 오버플로하면 InvalidFormat입니다. 음의 0은 부호 없는 초기화된 Decimal 0이 됩니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float32" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
