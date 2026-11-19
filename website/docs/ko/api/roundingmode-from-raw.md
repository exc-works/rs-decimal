---
locale: ko
source_revision: e9709f45197c4a553d9eef583ec3ace47a8ab16090d52890478f7fffb689b911
translation_provenance: en-US-korean-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

정수 반올림 방식 코드를 검사합니다.

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: i32/정수 number. 0–6에는 해당 방식을 반환하고 다른 i32 값에는 InvalidArgument를 반환합니다. 브라우저의 소수, 비유한 값, i32 범위 밖의 값은 Rust 호출 전 매개변수 검사에서 실패합니다. bridge는 Rust 검증 함수를 호출하여 숫자 형태의 방식을 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
