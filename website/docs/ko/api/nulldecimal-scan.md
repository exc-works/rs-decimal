---
locale: ko
source_revision: 3c6ebaee1d6c2d15a7801caee0a8597128eea9b110eb2748ac00d60cf69e23fd
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::scan"
---
# NullDecimal::scan

드라이버에 독립적인 SQL 스칼라를 읽습니다.

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: ScanValue. 수신자를 수정하고 `Result<(), DecimalError>`/undefined를 반환합니다. 변형을 명시적으로 선택하세요. Int64는 정확한 bigint를, Float32는 binary32로 좁힌 값을 사용하며 Text/Bytes는 원래 소수 자릿수를 보존합니다. SQL 텍스트의 짝이 맞는 바깥 큰따옴표 한 쌍은 JSON 이스케이프 해석 없이 제거합니다. NULL은 유효성을 지우고 내부 Decimal을 미초기화 0으로 재설정합니다. null 이외의 성공한 입력은 valid=true로 설정합니다. 미지원 형식과 유한하지 않은 부동소수점은 Unmarshal입니다. 다른 디코딩 오류는 원래 종류를 유지합니다. 모든 오류에서 수신자 상태를 보존합니다. 이 어댑터는 데이터베이스 연결이 필요하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-scan" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
