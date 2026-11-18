---
locale: ko
source_revision: 16b33be7c90300f9c4b236f60591e3b54c1078c3a213fb54df0f571ca8f60e09
translation_provenance: en-US-korean-reviewed
api_id: Decimal::neg
---
# Decimal::neg

부호가 반대인 독립적인 소수를 만듭니다.

```rust
fn neg (& self) -> Self
```

## 매개변수, 반환값, 사용 시 주의점

Decimal을 반환하고 소수 자릿수와 후행 0을 유지합니다. 0은 수치상 0을 유지하며 기본 입력이어도 결과는 초기화됩니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
