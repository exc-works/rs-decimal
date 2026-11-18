---
locale: ko
source_revision: a269819e7b61d5a164cb35c2c28efef7424a30b1113a1fd7abc0f0da7fe8a5f9
translation_provenance: en-US-korean-reviewed
api_id: Decimal::approx_root
---
# Decimal::approx_root

양의 차수 근을 계산합니다. Rust는 Result<Decimal, DecimalError>를, bridge는 Decimal 또는 구조화된 예외를 반환합니다. degree: i64를 bigint로 전달합니다. 차수는 양수여야 하며 그렇지 않으면 InvalidRoot입니다. 홀수 차수는 음수 입력을 허용하고 음수의 짝수 차수 근은 NegativeRoot입니다. 차수 1은 최근접 짝수 반올림으로 자릿수를 바꿉니다. 기본 정밀도는 max(입력 소수 자릿수, 20)이며 유효 숫자 20개가 아닙니다. 최근접 짝수 반올림을 사용합니다. 정밀도 0의 sqrt(6.25)처럼 정확히 중간인 근은 2로 반올림합니다. approx_root라는 이름이어도 Rust는 정수 비교로 반올림을 증명합니다. 코어 계산/반복 예산을 소진하면 NoConvergence를 반환합니다. 입력은 빌려 쓰며 바꾸지 않고 기본 0을 포함한 결과는 초기화됩니다.

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## 일반적인 사용법

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="ko" />

## 브라우저 제한

[리소스 정책](../resource-policy)은 출력 소수 자릿수를 1000으로 제한합니다. 비자명한 근의 차수는 최대 128이며 스케일 조정, Newton 거듭제곱, 중간점 증명의 중간값을 사전 검사합니다. 0, 부호 있는 단위 값, 차수 1의 근은 코어의 빠른 경로를 유지하지만 출력 정밀도 제한을 따릅니다. 정의역과 음수 정밀도 오류는 라이브러리 동작을 유지하며 리소스 오류는 NoConvergence와 다릅니다. 각 입력이 유효해도 모든 차수/정밀도 조합이 브라우저 예산에 맞는 것은 아닙니다.

## 관련 메서드

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
