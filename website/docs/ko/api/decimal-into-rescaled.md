---
locale: ko
source_revision: aed80a094ad154f3abfbbdc72125d175f579082056ef3b224d8dda331e64cff5
translation_provenance: en-US-korean-reviewed
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

소수를 소비하면서 소수 자릿수를 바꿉니다.

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

prec: isize, mode: RoundingMode. rescale처럼 반올림하거나 0을 추가하며 결과는 항상 초기화됩니다. Rust가 InvalidPrecision, RoundUnnecessary 또는 브라우저 리소스 오류를 반환해도 수신 객체는 이동됩니다. 잘못된 JS 타입/범위는 Rust에 들어가기 전에 거부하므로 소비하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="ko" />

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
