---
locale: ko
source_revision: aa32a5479f7baf77d635cc3726288c9b7c2cfa292167bcd9b91265a058234a8b
translation_provenance: en-US-korean-reviewed
api_id: Decimal::abs
---
# Decimal::abs

절댓값을 담은 독립적인 소수를 만듭니다.

```rust
fn abs (& self) -> Self
```

## 매개변수, 반환값, 사용 시 주의점

Decimal을 반환하고 소수 자릿수와 후행 0을 유지합니다. 입력이 기본값이어도 결과는 초기화되며 입력은 변하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
