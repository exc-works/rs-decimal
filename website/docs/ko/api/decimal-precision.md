---
locale: ko
source_revision: c8a99895a920cbd49f4dba891779055021c167a0addb6791872293807a1937f8
translation_provenance: en-US-korean-reviewed
api_id: Decimal::precision
---
# Decimal::precision

값에 저장된 소수 자릿수를 읽습니다.

```rust
fn precision (& self) -> isize
```

## 매개변수, 반환값, 사용 시 주의점

isize(브라우저에서는 정수 number)를 반환합니다. 유효 숫자의 개수가 아니라 소수 자릿수이며 후행 0도 포함합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
