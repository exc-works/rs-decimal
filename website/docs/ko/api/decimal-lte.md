---
locale: ko
source_revision: 60ca5295cf77ea0e6cccdadb5d937c5adbdd69051a5dbac42387fec9ee90828c
translation_provenance: en-US-korean-reviewed
api_id: Decimal::lte
---
# Decimal::lte

이 값이 다른 값 이하인지 확인합니다.

```rust
fn lte (& self , other : & Self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 bool/boolean을 반환합니다. 동등성은 수치로 판단하며 소수 자릿수나 초기화 상태와 무관합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
