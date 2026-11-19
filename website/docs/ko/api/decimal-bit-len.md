---
locale: ko
source_revision: cd78c2dcf4b67b04ef802fd73ba3ef0da02aad24d1bb17be5cc5df5d221df3cd
translation_provenance: en-US-korean-reviewed
api_id: Decimal::bit_len
---
# Decimal::bit_len

스케일 적용 전 계수 절댓값의 비트 수를 셉니다.

```rust
fn bit_len (& self) -> usize
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. Rust는 usize, JavaScript는 number를 반환합니다. 소수 자릿수를 적용하지 않고 부호 비트도 세지 않습니다. -1.20의 계수는 -120이고 비트 길이는 7입니다. 기본 0을 포함한 0은 0을 반환합니다. 최상위 1비트의 위치가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
