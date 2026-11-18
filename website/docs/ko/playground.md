---
locale: ko
source_revision: fbadb341fac6ded3986e830a05797cf96857f33d92f3224a113dd3d05ba9eb85
---
# 플레이그라운드

JavaScript에서 Rust와 같은 이름의 메서드를 호출하세요. 직접 실행하여 정확한 값과 상태를 확인합니다.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="ko" :labels="locales['ko']" />

메서드 페이지의 양식에서 공통 예제의 리터럴 인수를 편집할 수 있습니다. 매개변수는 JavaScript 호출 순서대로 표시되며 Rust 메서드 이름과 인수 순서는 유지됩니다. 생성된 코드를 이 편집기에서 열어 복잡한 값을 수정하거나 호출을 추가하세요. 메서드 페이지의 예상 상태는 원본 예제에 해당합니다. 같은 탭에서 언어를 변경해도 양식 수정 내용이 유지됩니다.

공유 기능은 최대 4 KiB의 UTF-8 코드와 라이브러리 빌드 커밋을 URL 프래그먼트에 저장합니다. 코드는 서버로 업로드되지 않습니다. 공유 링크는 동일한 빌드가 필요하며, 코드를 실행 없이 불러오고 이전 결과를 지웁니다. 잘못된 링크는 현재 초안을 유지합니다. 코드를 확인한 후 실행하세요. 더 큰 스크립트는 파일로 내보내세요. 편집기는 최대 64 KiB를 지원합니다. 링크에는 스크립트의 텍스트가 포함되므로 공유하기 전에 코드를 확인하세요.
