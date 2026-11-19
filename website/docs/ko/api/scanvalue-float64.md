---
locale: ko
source_revision: b276bcddff5985f20e7905654e4b3cef6f352b3cfa139550a92b9425bd777e62
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

SQL 입력 변형 Float64을 선택합니다.

```rust
Float64 (f64)
```

## 매개변수, 반환값, 일반적인 사용법

binary64를 담습니다. 유한 값은 코어의 최단 소수 변환을 사용하며 NaN과 무한대는 Unmarshal입니다. 이미 반올림된 JS number로 원래 숫자를 복원할 수 없습니다. 변형은 scan에 빌려 전달하며 한 브라우저 실행 안에서 재사용할 수 있습니다. 임의로 만든 값이나 다른 실행의 값은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
