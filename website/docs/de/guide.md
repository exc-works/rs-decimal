---
locale: de
source_revision: b0ab4b73c5319adf6d0f52ca8ad72391d3a1bf1d0a68f2ceac7763706868b54b
---
# Erste Schritte

Erzeugen Sie Dezimalwerte aus Zeichenketten, wenn die Eingabe exakt bleiben soll. `mul_exact` addiert die Skalen der Eingaben. `string()` lässt nachgestellte Nullen weg, `string_with_trailing_zeros()` erhält sie.

<<< ../../../examples/web_getting_started.rs

Präzisionsparameter zählen Nachkommastellen, nicht signifikante Stellen. Bei Gleichstand wählt `HalfEven` eine gerade letzte beibehaltene Ziffer. `Down` schneidet in Richtung null ab, nicht in Richtung minus unendlich.

`Decimal::default()` ist eine nicht initialisierte numerische Null. Eine initialisierte Null ist numerisch gleich, hat aber einen anderen Zustand. `NullDecimal.valid` bestimmt unabhängig davon, ob ein Wert vorhanden ist.

Gewöhnliche fehlbare Methoden geben `Result` zurück. Prüfen Sie die Fehlerkategorie; nur ausdrücklich als `must_*` benannte Methoden lösen bei erwartbaren Eingabefehlern einen panic aus. Der Browser ruft dieselben Rust-Methoden über eine JavaScript-Bridge auf; Typ- und Eigentumszuordnungen werden dokumentiert.
