---
locale: ko
source_revision: 828ccfc55974951d7ced64d269a1959b7a45efe557a7f4202f79dcfc3632ea3f
translation_provenance: en-US-korean-reviewed
api_id: Decimal::sign
---
# Decimal::sign

부호를 -1, 0, 1로 읽습니다.

```rust
fn sign (& self) -> i32
```

## 매개변수, 반환값, 사용 시 주의점

i32/number를 반환합니다. 초기화된 0과 미초기화 0의 부호는 모두 0이며 음의 0을 별도로 구분하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
