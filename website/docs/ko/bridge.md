---
locale: ko
source_revision: bd473c0431492bbeed88af5f453e401cc80b45775529394fc98a50e453c639ca
translation_provenance: en-US-korean-reviewed
---
# 브라우저에서 Rust 메서드 호출하기

playground는 Wasm으로 컴파일한 Rust 라이브러리를 호출하는 JavaScript를 실행합니다. 메서드 이름, 인수 순서, 반올림, 소수 상태는 Rust를 따릅니다. JavaScript는 제어 흐름을 제공하며 소수 결과를 계산하지 않습니다. 각 동작은 [메서드 목록](./api/)을 확인하고 [Playground](./playground)에서 예제를 실행하세요.

```javascript
const price = Decimal.new_from_string('12.350');
const total = price.mul_exact(Decimal.new(3n));
total.string_with_trailing_zeros(); // '37.050'
```

소수 입력에는 정확한 문자열을, i64 인수에는 `bigint`를 사용하세요(`3`이 아니라 `3n`). `Decimal.new`는 i64 범위 밖의 값, 숫자 문자열, JS number를 거부합니다. 정밀도는 wasm32 부호 있는 범위의 JS 정수입니다. 소수, 비유한 값, 범위 밖 입력은 ABI 캐스트에서 순환하거나 잘리기 전에 거부합니다. 음수 정수 정밀도는 Rust에 전달되어 원래 InvalidPrecision 오류를 반환합니다.

## 오류와 소유권

Rust가 반환한 오류는 `name: 'DecimalError'`, `category: 'library'`, 원래 `message`, 문자열 배열 `kinds`를 가진 Error로 던집니다. 디코더 오류는 여러 종류에 일치할 수 있으므로 배열 전체를 유지하세요. JS 타입/범위 오류는 `name: 'BridgeError'`, `category: 'parameter'`이며 Rust 산술 오류가 아닙니다. 잘못된 숫자 반올림 판별값은 Rust 검증으로 전달됩니다.

빌림 메서드를 호출한 뒤에도 객체를 사용할 수 있습니다. `into_rescaled`는 Rust가 오류를 반환해도 수신 객체를 소비합니다. 소비되거나 해제된 값을 사용하면 `moved` BridgeError가 발생합니다. 잘못된 JS 인수 타입은 호출 전에 거부하며 수신 객체를 소비하지 않습니다. `clone()`은 표현과 초기화 상태를 유지합니다. `free()`는 활성 핸들을 해제하고 소비 후 호출해도 안전합니다. 각 runner는 종료 시 Wasm 인스턴스를 폐기하며 파이널라이저에 의존하지 않습니다. 다른 JavaScript 인스턴스의 객체는 호출에 전달할 수 없습니다.

## Nullable 값과 바이트

`NullDecimal.new(decimal)`은 인수를 소비하고 미초기화 0이어도 `valid=true`로 설정합니다. `NullDecimal.default()`는 값 없음입니다. `valid` 설정은 유효성만 바꾸며 내부 값을 지우지 않습니다. `get_decimal_clone()`은 독립적인 복사본을 반환하는 명시적 어댑터입니다. `set_decimal(value)`는 유효성을 바꾸지 않고 새 내부 값을 이동합니다. 실시간 참조로 오해할 수 있는 `.decimal` getter는 없습니다.

`coefficient()`도 불변 JS bigint 스냅샷을 반환합니다. JavaScript는 Rust의 빌림 수명과 컴파일 시점 트레이트 검사를 재현하지 않습니다. `ByteBuffer.new(Uint8Array)`는 바이트를 소유 버퍼로 복사하며 `to_uint8_array()`는 Wasm 메모리가 증가해도 유효한 복사본을 반환합니다. 추가 메서드는 [메서드 목록](./api/)에서 설명합니다.

JavaScript의 `snapshot(value)`는 정확한 상태를 즉시 캡처하며 무효인 nullable의 숨겨진 값도 포함합니다. 동결된 데이터를 반환하므로 나중의 변경이 이전 출력을 바꾸지 않습니다. 격리된 runner는 재귀 출력과 실행 시간을 제한합니다. [리소스 정책](./resource-policy)을 확인하세요.
