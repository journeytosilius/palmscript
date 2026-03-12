# Referenz-Uberblick

Dieser Abschnitt ist die normative offentliche Definition von PalmScript.

Falls sich eine Leitseite und eine Referenzseite jemals unterscheiden, ist die
Referenz verbindlich.

## Was Dieser Abschnitt Definiert

- lexikalische Struktur
- Grammatik
- Regeln fur Deklarationen und Gultigkeitsbereich
- Typen und Werte
- Semantik von Serien und Indexierung
- Auswertungssemantik
- Regeln fur Intervalle und Quellen
- Vertrage fur Builtins und Indikatoren
- Semantik der Ausgaben
- Diagnoseklassen

## Heute Implementiert

Die aktuelle PalmScript-Oberflache umfasst:

- genau eine Top-Level-Basisdirektive `interval <...>` pro Skript
- eine oder mehrere benannte `source`-Aliase pro ausfuhrbarem Skript
- quellqualifizierte Serien wie `spot.close` oder `bb.1h.close`
- zusatzliche Intervalle uber `use <alias> <interval>`
- Top-Level-`fn`-Deklarationen mit Ausdruckskorper
- `let`, `const`, `input`, Tupel-Destrukturierung, `export`, `trigger`, erstklassige `entry` / `exit` und `order`
- ausschliesslich literale Serienindexierung, typisierte Enum-Literale `ma_type.<variant>`, `tif.<variant>`, `trigger_ref.<variant>`, `position_side.<variant>` und `exit_kind.<variant>` sowie deterministische dreiwertige boolesche Logik
- eine TA-Lib-artige Builtin-Oberflache, bei der einige Namen heute ausfuhrbar sind und weitere reservierte Namen uber Diagnosen sichtbar werden

## Aktuelle Grenzen

- `interval`, `source`, `use`, `fn`, `const`, `input`, `export`, `trigger`, `entry`, `exit` und `order` sind nur auf Top-Level erlaubt
- nackte Marktidentifikatoren wie `close` sind in ausfuhrbaren Skripten nicht gultig
- hohere Quellintervalle erfordern `use <alias> <interval>`
- nur Identifikatoren sind aufrufbar
- String-Literale sind nur innerhalb von `source`-Deklarationen gultig
- Serienindexierung erfordert ein nicht-negatives Integer-Literal
- Tupelwertige Builtin-Ergebnisse mussen sofort mit `let (...) = ...` destrukturiert werden, bevor sie weiterverwendet werden

## Wie Man Es Liest

- beginne mit [Lexikalische Struktur](lexical-structure.md) und [Grammatik](grammar.md) fur die akzeptierte Syntax
- nutze [Deklarationen und Gultigkeitsbereich](declarations-and-scope.md) fur Bindungs- und Sichtbarkeitsregeln
- nutze [Auswertungssemantik](evaluation-semantics.md) und [Intervalle und Quellen](intervals-and-sources.md) fur die Bedeutung der Sprache
- nutze [Builtins](builtins.md), [Indikatoren](indicators.md) und [Ausgaben](outputs.md) fur das Verhalten von Aufrufen und Ausgaben
