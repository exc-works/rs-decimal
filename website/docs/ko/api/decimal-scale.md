---
locale: ko
source_revision: c63b4582e5ea4e7eac387ce70eb29aed40ab3d4cb5d5c76ae95a6cd8fa1f8f69
translation_provenance: en-US-korean-reviewed
api_id: Decimal::scale
---
# Decimal::scale

저장된 음수가 아닌 소수 자릿수를 읽습니다.

```rust
fn scale (& self) -> usize
```

## 매개변수, 반환값, 사용 시 주의점

playground 소수 자릿수 제한 안에서 usize를 정확한 number로 반환합니다. precision()과 값이 같습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="ko" />

## 오류와 제한

이 메서드들은 입력을 빌리며 소비하거나 바꾸지 않습니다. 라이브러리 연산은 Result를 반환하지 않습니다. bridge는 해제되거나 이동된 핸들, 다른 실행의 핸들을 거부하고 [리소스 정책](../resource-policy)을 적용합니다. 예제는 스칼라 출력을 `value` 필드로 감싸며 Decimal 출력에는 전체 스냅샷을 사용합니다.

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
