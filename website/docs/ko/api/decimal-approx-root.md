---
locale: ko
source_revision: 4a95a976d839a4b08d68e1a4e5b0d124596f396891cbc4d74282ca10c6649baa
translation_provenance: en-US-korean-reviewed
api_id: Decimal::approx_root
---
# Decimal::approx_root

양의 차수 근을 계산합니다. degree: i64를 bigint로 전달합니다. 차수는 양수여야 하며 그렇지 않으면 InvalidRoot입니다. 홀수 차수는 음수 입력을 허용하고 음수의 짝수 차수 근은 NegativeRoot입니다. 차수 1은 최근접 짝수 반올림으로 자릿수를 바꿉니다. 기본 정밀도는 max(입력 소수 자릿수, 20)이며 유효 숫자 20개가 아닙니다. 최근접 짝수 반올림을 사용합니다. 정밀도 0의 sqrt(6.25)처럼 정확히 중간인 근은 2로 반올림합니다. approx_root라는 이름이어도 Rust는 정수 비교로 반올림을 증명합니다. 코어 계산/반복 예산을 소진하면 NoConvergence를 반환합니다. 입력은 빌려 쓰며 바꾸지 않고 기본 0을 포함한 결과는 초기화됩니다.

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## 일반적인 사용법

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="ko" />

## 관련 메서드

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
