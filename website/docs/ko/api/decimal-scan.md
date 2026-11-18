---
locale: ko
source_revision: 805fc62e495d042f613bdfd51817251dea41dcde63bd630737a6a7dd4ec14e8a
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::scan"
---
# Decimal::scan

드라이버에 독립적인 SQL 스칼라를 읽습니다.

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: ScanValue. 수신자를 수정하고 `Result<(), DecimalError>`/undefined를 반환합니다. 변형을 명시적으로 선택하세요. Int64는 정확한 bigint를, Float32는 binary32로 좁힌 값을 사용하며 Text/Bytes는 원래 소수 자릿수를 보존합니다. SQL 텍스트의 짝이 맞는 바깥 큰따옴표 한 쌍은 JSON 이스케이프 해석 없이 제거합니다. NULL은 수신자를 미초기화 0으로 재설정합니다. 미지원 형식과 유한하지 않은 부동소수점은 Unmarshal입니다. 다른 디코딩 오류는 원래 종류를 유지합니다. 모든 오류에서 수신자 상태를 보존합니다. 이 어댑터는 데이터베이스 연결이 필요하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-scan" locale="ko" />

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
