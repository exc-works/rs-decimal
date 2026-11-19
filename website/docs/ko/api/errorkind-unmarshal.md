---
locale: ko
source_revision: 274359ec57012f0ef20672a2e83c98afd2d82df0c75391b519022c4925e09ca8
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

Unmarshal 오류를 식별합니다.

```rust
Unmarshal
```

## 매개변수, 반환값, 일반적인 사용법

디코딩에 실패했습니다. InvalidFormat 같은 추가 종류가 원인을 설명할 수 있으므로 하나라고 가정하지 말고 포함 여부를 검사하세요. ErrorKind는 메시지 문자열이 아니라 안정적인 분류입니다. 브라우저 열거값은 Rust 변형 이름입니다. 생성한 오류는 matches()를 사용하고 잡은 bridge 예외는 kinds 배열을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
