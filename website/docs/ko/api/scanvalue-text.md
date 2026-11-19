---
locale: ko
source_revision: 66f48b07b8a5c080eb29b01047825ba3448ab4e979a526ae96e347e482fef177
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::Text"
---
# ScanValue::Text

SQL 입력 변형 Text을 선택합니다.

```rust
Text (& 'a str)
```

## 매개변수, 반환값, 일반적인 사용법

정확한 소수 텍스트를 빌립니다. 짝이 맞는 바깥 큰따옴표 한 쌍을 제거하지만 이스케이프를 JSON으로 디코딩하지는 않습니다. 0이 아닌 수치의 후행 자릿수는 파싱 후에도 유지합니다. 변형은 scan에 빌려 전달하며 한 브라우저 실행 안에서 재사용할 수 있습니다. 임의로 만든 값이나 다른 실행의 값은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-text" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
