---
locale: de
source_revision: 637474c992943a95b792c86f50bfa04821ba9ba29fe9dab7696d42ada69cbe84
translation_provenance: en-US-german-reviewed
---
# Ressourcenrichtlinie des Playgrounds

Überschreitet eine Berechnung einen der folgenden Grenzwerte, meldet der Playground `category: "resource"`.

| Grenze | Aktueller Wert |
| --- | --- |
| Dezimaleingabe | 8192 UTF-8-Bytes, 4096 Mantissen-/expandierte Koeffizientenziffern |
| Gespeicherte Skala und Betrag der Eingabeskala | 1000 Nachkommastellen |
| Geschätzter Ergebniskoeffizient | 16384 Bits (höchstens 4933 Dezimalziffern) |
| Binär-/BSON- und ByteBuffer-Eingabe | 65536 Bytes |
| Validierungsregel-/Fehlerlisten und Nachrichtenmaps | 64 Einträge; 8192 Bytes je Zeichenfolge; 65536 Bytes JSON-Transport |
| Ersetzter Validierungstext | konservative Gesamtgrenze von 65536 Bytes vor der Übersetzung |
| Aufrufe in eine Wasm-Instanz | 10000, einschließlich Getter, Snapshots und Metadaten |
| Nichttrivialer Wurzelgrad | 128 |
| Log-/exp-Ausgabepräzision, Eingabeskala, Eingabekoeffizient | 128 Stellen, 256 Stellen, 4096 Bits |
| Betrag einer gewöhnlichen exp-Eingabe | 64 (bewiesener schneller Pfad für negative, auf null gerundete Eingaben bleibt erhalten) |
| Maximum des linearen Speichers im gebauten Wasm | 64 MiB |

Eingabeskala und Expansion positiver Exponenten werden vor dem Rust-Parsing geprüft. Konfigurationskonvertierungen prüfen ursprünglichen Dezimaltext und decodierte JSON-Zeichenfolgen; 128-Bit-Ganzzahlvarianten erzwingen vor dem Transport exakte vorzeichenbehaftete/vorzeichenlose Grenzen. Generische Serde-Adapter behalten ihre eigenen Fehlertypen und weisen Gleitkomma-Visitor-Eingaben ab.

SQL-Scanning prüft Text nach kernkompatiblem Entfernen äußerer Anführungszeichen. Die Validierung prüft dezimale Regelparameter vor der Konstruktion und begrenzt wiederholte Platzhalterexpansion vor der Übersetzung; Felder und eingefügte Parameter bleiben wörtlicher Text. Text-/Parameter-/JSON-Decoder begrenzen Quellbytes auf 8192 und prüfen decodierten Dezimaltext einschließlich JSON-Escapes vor einer Änderung. Ursprüngliche JSON-Zahlenbytes bleiben exakt; der Browser parst sie niemals über JavaScript-Zahlen.

Binärdecodierung begrenzt tatsächliche Koeffizientenbits und codierte Skala vor der BigInt-Allokation. YAML-/XML-Quellen sind auf 8192 Bytes begrenzt. Decodierter Skalar-/Elementtext wird vor dem numerischen Kernparser geprüft, einschließlich Escapes, CDATA, Unterstrichnormalisierung und Nulleinfügung in YAML-Gleitkommazahlen. Die basisabhängige Ganzzahlallokation für YAML-Skalare ist durch konservativ höchstens 4096 hexadezimale Ziffernzeichen begrenzt. BSON-Zeichenfolgen unterliegen den Textgrenzen; Decimal128-Exponentenexpansion wird vor dem Kernparser geprüft. Jede fehlgeschlagene Decoder-Vorprüfung erhält den Empfängerzustand.

Exakte Produkte prüfen die Summe der Skalen, gerundete Produkte die Ausgabeskala. Beide begrenzen Zwischenkoeffizientenbits vor der Multiplikation. Addition/Subtraktion prüfen Wachstum bei der Skalenausrichtung und einen Übertrag; Division prüft Zähler- oder Nennerskalierung vor der Allokation. Quotient/Rest prüfen die Ausrichtung nach der Nulldivisorprüfung. Potenzen begrenzen normalisierte Zwischenskala und Koeffizientenwachstum einschließlich Kehrwertskalierung; schnelle Kernpfade für Null/Einheit und Overflow vor Allokation bleiben erhalten. Besitzübernehmende Arithmetik konsumiert den Empfänger auch bei fehlgeschlagener Vorprüfung.

Exakte rationale Konvertierung prüft Faktoren des gekürzten Nenners und Koeffizientenwachstum; gerundete rationale Konvertierung prüft Zählerskalierung vor der Multiplikation. Der Nenner wird vor der rationalen Konstruktion auf ungleich null geprüft. Formatierung begrenzt Breite (65536 Bytes), Präzision (1000) und Wachstum des Festkommakoeffizienten. Append prüft vorhandenen Puffer plus konservative Dezimaltextgrenze vor einer Änderung; auch eine gerade noch passende Operation kann abgewiesen werden.

Wurzeln begrenzen skalierte Werte, Newton-Potenzen und exakte Mittelpunktzertifizierung vor dem Kernaufruf. Nichttriviale Grade sind höchstens 128; schnelle Pfade für Null, Einheit und Grad eins bleiben innerhalb des Ausgabeskalenbudgets verfügbar. Log/exp haben engere Eingabe- und Präzisionsbudgets für rationale Kürzung und alle Versuche der Intervallverfeinerung, einschließlich des Kernmaximums von 8192 Guard-Bits. Exp begrenzt zusätzlich den Betrag gewöhnlicher Argumente; ausreichend negative Eingaben, deren Rundung auf null der Kern beweist, behalten den sicheren schnellen Pfad. Negative Präzision und Nulldivisoren behalten ihre Kernfehler, wenn der Kern sie vor Allokation abweisen kann.

Rescale prüft angeforderte Skala und konservatives Wachstum vor dem Anlegen von Zehnerpotenzen. Präzisionsspezifische Rundung nutzt dieselben Prüfungen. Shift prüft Ergebnisskala und Koeffizientenwachstum; Rundung auf signifikante Stellen reserviert einen möglichen Übertrag. Shift-Overflow des Kerns bleibt erhalten, wenn Rust ihn vor Allokation liefern kann. Daher kann eine Folge akzeptierter Operationen weder einen unbegrenzten Koeffizienten noch eine unbegrenzt lange formatierte Dezimalzeichenfolge erzeugen. Konservative Vorprüfungen können Operationen mit kleinerem Endwert abweisen, etwa Produkte mit einem Nulloperanden; diese Grenze ist keine Regel der Kernbibliothek. Gewöhnlicher fehlerhafter Text, negative Präzision und unbekannte Rundungsmodi behalten Bibliotheksfehler, sofern sie nicht unabhängig eine Ressourcengrenze verletzen.

Eine konsumierende Methode konsumiert ihren Wert auch dann, wenn Rust eine Ressourcengrenze ablehnt. Ausgeliehene Operanden bleiben unverändert. Nach Ausschöpfen des Instanzbudgets scheitern alle exportierten Aufrufe außer Freigabe weiter; eine neue JS-Fassade setzt es nicht zurück. Erst eine frische Wasm-Instanz setzt das Budget zurück.

Die JS-Fassade prüft außerdem Zeichenfolgen- und Pufferlängen, bevor wasm-bindgen Argumente kopiert. Rust prüft den empfangenen Slice vor dem Kernparser oder weiterer Allokation erneut. Direkte Aufrufer der rohen ABI können die temporäre Eingabekopie vor der Rust-Prüfung anlegen, aber das Speichermaximum des gebauten Moduls nicht entfernen. Der Runner muss die rohe ABI privat halten. Dies ist keine harte Quote für JavaScript-Heap oder gesamten Browserprozessspeicher. Der Runner erzwingt Worker-Beendigung, Quell-/Ausgabelimits und Netzwerkisolation.

Ein lokaler Node-26-Lauf las einen Koeffizienten mit 4096 Ziffern ein und formatierte einen Wert mit Skala 1000 in etwa 1,2 ms. Das ist eine Entwicklungsreferenz, keine Latenzzusage. CI gibt dieselbe Messung aus und prüft Ablehnungsgrenzen. Das Wasm-Entwicklungsartefakt war etwa 196 KB groß (73 KB gzip); `build.json` dokumentiert exakte Größe, Hashes und Speichermaximum des finalen Commits.
