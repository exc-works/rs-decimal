---
locale: ko
source_revision: 6c951b2ad37813f4b84e7dc136137d89576bf098308ff5b7d85a1bf0a3c58e09
translation_provenance: en-US-korean-reviewed
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

부동소수점으로 되돌렸을 때 같은 값이 되는 가장 짧은 십진 표현으로 생성합니다.

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: f64를 JS number로 전달합니다. 이진 부동소수점에서 이미 잃은 십진 숫자는 복원하지 않습니다. 정확한 소수 입력에는 new_from_string을 사용하세요. NaN과 양/음의 무한대는 InvalidFormat입니다. 음의 0은 부호 없는 초기화된 Decimal 0이 됩니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
