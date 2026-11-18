---
locale: ko
source_revision: f397dc1220fc731959fe63aa3e5a1c46b0d68942bea1f3f5f567773d91995a1b
translation_provenance: en-US-korean-reviewed
api_id: "RoundingMode::impl TryFrom < i32 >::Error"
---
# RoundingMode::impl TryFrom < i32 >::Error

TryFrom으로 원시 반올림 식별값을 검증합니다.

```rust
type Error = DecimalError
```

## 일반적인 사용법과 주의사항

value: i32. `Result<RoundingMode, DecimalError>`을 반환합니다. 코드 0–6은 5=HalfEven을 포함한 고정 매핑을 사용합니다. 알 수 없는 코드는 패닉 없이 InvalidArgument입니다. 연관 Error 형식은 DecimalError입니다. from_raw에 위임하며 브라우저에서는 엄격한 i32 매개변수 검사를 하는 RoundingMode.from_raw(number)를 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-error" locale="ko" />

## 실행

이 항목은 네이티브 Rust 트레이트 구문이며 같은 이름의 JS 연산자나 SQLx 객체를 제공하지 않습니다. 기본 네이티브 문서 테스트 실행기가 이 예제를 실행합니다.

## 관련 메서드

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
