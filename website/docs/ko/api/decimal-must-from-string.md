---
locale: ko
source_revision: bb2070c90fbcf81b9f15485debb923fef5e838831e150577f2d283f059db73b6
translation_provenance: en-US-korean-reviewed
api_id: Decimal::must_from_string
---
# Decimal::must_from_string

잘못된 입력을 프로그래밍 오류로 취급할 때 신뢰할 수 있는 소수 텍스트를 파싱합니다.

```rust
fn must_from_string (text : & str) -> Self
```

## 매개변수, 반환값, 사용 시 주의점

text: &str/string. Decimal을 반환합니다. 유효한 입력의 상태는 new_from_string과 같습니다. 잘못된 문법은 Rust에서 DecimalError 패닉을 일으키며 컴파일된 Wasm에서는 복구 가능한 라이브러리 오류 대신 트랩이 됩니다. 사용자 입력에는 new_from_string을 권장합니다. 처리하지 않은 트랩은 현재 실행을 끝내므로 Rust를 다시 쓰기 전에 새 실행을 시작하세요. 매개변수와 리소스 검사는 여전히 Rust 호출 전에 입력을 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-decimal-must-from-string" locale="ko" />

## 제한

bridge는 Rust 이름과 매개변수 순서를 유지합니다. 자유 함수는 playground에서 바로 사용할 수 있습니다. 빌린 입력은 소비하거나 바꾸지 않으며 잘못된 핸들이나 다른 실행의 핸들은 거부합니다. 모든 연산은 [리소스 정책](../resource-policy)을 따릅니다.

## 관련 메서드

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
