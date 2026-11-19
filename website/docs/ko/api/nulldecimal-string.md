---
locale: ko
source_revision: 5f7a180e14ebabfd6785a42604765a6ece20be12513e46794eb1f0299f911006
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::string"
---
# NullDecimal::string

nullable의 정규 텍스트를 읽습니다.

```rust
fn string (& self) -> String
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. String/string을 반환합니다. 유효한 값은 내부 Decimal의 정규 텍스트를 사용하여 소수 부분의 후행 0을 제거합니다. 무효인 값은 숨겨진 내부 데이터와 관계없이 리터럴 null을 반환합니다. 무효인 값에 빈 바이트를 반환하는 marshal_text와 다릅니다. 읽기는 유효성이나 초기화 상태를 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="ko" />

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
