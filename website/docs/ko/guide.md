---
locale: ko
source_revision: b0ab4b73c5319adf6d0f52ca8ad72391d3a1bf1d0a68f2ceac7763706868b54b
---
# 시작하기

입력의 정확도를 유지하려면 문자열에서 십진수를 생성하세요. `mul_exact`는 입력의 소수 자릿수를 더합니다. `string()`은 뒤쪽 0을 생략하고 `string_with_trailing_zeros()`는 보존합니다.

<<< ../../../examples/web_getting_started.rs

precision 인수는 유효 숫자가 아니라 소수 자릿수를 뜻합니다. 중간값에서 `HalfEven`은 마지막 보존 숫자가 짝수인 결과를 선택합니다. `Down`은 0 방향으로 버리며 음의 무한대 방향이 아닙니다.

`Decimal::default()`는 초기화되지 않은 숫자 0입니다. 초기화된 0과 값은 같지만 상태는 다릅니다. `NullDecimal.valid`는 값의 존재 여부를 독립적으로 결정합니다.

실패할 수 있는 일반 메서드는 `Result`를 반환합니다. 오류 종류를 확인하세요. 이름이 `must_*`인 메서드만 예상 가능한 입력 오류에 panic을 발생시킵니다. 브라우저는 JavaScript bridge를 통해 같은 Rust 메서드를 호출하며 타입과 소유권 매핑을 문서로 설명합니다.
