// Keys are shared by the task directory, the type navigation and the catalogue.
const keys = 'types construct compare arithmetic precision convert fields variants traits members functions exports all result example'.split(' ');
const labels = {
  'en-US': 'Types and modules|Construction and parsing|Comparison|Arithmetic|Precision and rounding|Formatting and conversion|Fields|Variants|Trait implementations|Members|Functions and constants|Re-exports|All symbols|Result|Example',
  'zh-Hans': '类型与模块|构造与解析|比较|算术|精度与舍入|格式化与转换|字段|枚举值|Trait 实现|成员|函数与常量|重新导出|全部符号|结果|示例',
  'zh-Hant': '型別與模組|建構與解析|比較|算術|精度與捨入|格式化與轉換|欄位|列舉值|Trait 實作|成員|函式與常數|重新匯出|全部符號|結果|範例',
  fr: 'Types et modules|Construction et analyse|Comparaison|Arithmétique|Précision et arrondi|Formatage et conversion|Champs|Variantes|Implémentations de traits|Membres|Fonctions et constantes|Réexportations|Tous les symboles|Résultat|Exemple',
  de: 'Typen und Module|Erstellung und Einlesen|Vergleich|Arithmetik|Genauigkeit und Rundung|Formatierung und Konvertierung|Felder|Varianten|Trait-Implementierungen|Elemente|Funktionen und Konstanten|Reexporte|Alle Symbole|Ergebnis|Beispiel',
  ja: '型とモジュール|生成と解析|比較|算術|精度と丸め|書式と変換|フィールド|列挙子|トレイト実装|メンバー|関数と定数|再エクスポート|すべてのシンボル|結果|使用例',
  ko: '타입과 모듈|생성과 파싱|비교|산술|정밀도와 반올림|서식과 변환|필드|열거형 값|트레이트 구현|멤버|함수와 상수|다시 내보내기|전체 심볼|결과|예제',
};
labels['en-GB'] = labels['en-US'];
export const referenceLabels = Object.fromEntries(Object.entries(labels).map(([lang, row]) =>
  [lang, Object.fromEntries(row.split('|').map((label, index) => [keys[index], label]))]));
