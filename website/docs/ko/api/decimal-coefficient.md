---
locale: ko
source_revision: b70b6d632eef5a013491ea47ed27e9d8eea36fee70b75cd0db8637b8b6a6ccc5
translation_provenance: en-US-korean-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

스케일 적용 전의 정확한 정수 계수를 확인합니다.

```rust
fn coefficient (& self) -> & BigInt
```

## 매개변수, 반환값, 사용 시 주의점

Rust는 &BigInt를 빌리며 bridge는 불변 bigint 스냅샷을 반환합니다. 변경을 추적하는 참조가 아닙니다. 계수에는 후행 0이 포함되며 소수의 정수 부분과 다릅니다. 예제는 JSON에서 모든 숫자를 유지하도록 bigint를 텍스트로 출력합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
