---
locale: ko
source_revision: 78277d1082dac8a98f79b7018413113c09afb0c224a6c82ae6c179432b0517f1
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::Unsupported"
---
# ScanValue::Unsupported

SQL 입력 변형 Unsupported을 선택합니다.

```rust
Unsupported
```

## 매개변수, 반환값, 일반적인 사용법

미지원 드라이버 입력을 강제 변환 없이 나타냅니다. scan은 Unmarshal을 반환하며 기존 값을 유지합니다. 변형은 scan에 빌려 전달하며 한 브라우저 실행 안에서 재사용할 수 있습니다. 임의로 만든 값이나 다른 실행의 값은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-unsupported" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
