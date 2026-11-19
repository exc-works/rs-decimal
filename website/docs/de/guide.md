---
locale: de
source_revision: f513baf539375e2a99059b44ed14e0bbb75f211d5e47c68b2215892ab1d50f56
---
# Erste Schritte

Eine Kundin bestellt zwei Hefte zu je 4,50. Der Versand kostet 1,25. Berechnen wir den zu zahlenden Betrag und geben ihn auf dem Beleg aus.

## Gesamtbetrag berechnen

Schreiben Sie den Preis im Code mit einem Punkt. `new_from_string("4.50")` liest die Ziffern direkt ein. Multiplizieren Sie den Preis mit `mul_exact` mit zwei. Das ergibt 9,00. Addieren Sie mit `add` die Versandkosten, um 10,25 zu erhalten.

<<< ../../../examples/web_getting_started.rs#calculate

## Den Betrag ausgeben

Für die Zwischensumme liefert `string()` den Text "9", während `string_with_trailing_zeros()` den Text "9.00" liefert. Beide stehen für denselben Betrag. Die zweite Darstellung behält die zwei Nachkommastellen des Stückpreises für den Beleg bei.

<<< ../../../examples/web_getting_started.rs#print

[Die Bestellung im Browser ändern](./playground) — Ändern Sie die Menge von 2 auf 3 und führen Sie den Code erneut aus. Die Zwischensumme steigt auf 13,50 und der Gesamtbetrag auf 14,75.

## Das Rust-Beispiel lokal ausführen

Mit Rust ab Version 1.85 können Sie das Repository herunterladen und das Beispiel mit diesen Befehlen ausführen. Es gibt 10.25 aus.

```sh
git clone https://github.com/exc-works/rs-decimal.git
cd rs-decimal
cargo run --locked --example web_getting_started
```
