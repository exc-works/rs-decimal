---
locale: ko
source_revision: 1933adb169124e0ed9d429b21c95687e4fd59240647c4b0f684ca7bc76aabd2b
translation_provenance: en-US-korean-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

스케일 적용 전의 정확한 정수 계수를 복사합니다.

```rust
fn big_int (& self) -> BigInt
```

## 매개변수, 반환값, 사용 시 주의점

Rust는 독립적인 BigInt, bridge는 bigint를 반환합니다. 정수로 버림 변환하는 것과 달리 후행 0이 포함됩니다. 예제는 JSON을 위해 bigint를 텍스트로 출력합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
