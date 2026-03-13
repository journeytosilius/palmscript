# PalmScript-Dokumentation

PalmScript ist eine Sprache fur finanzielle Zeitreihenstrategien. Diese Site
konzentriert sich auf die Sprache selbst: Syntax, Semantik, Builtins und
Codebeispiele.

## Dokumentationsubersicht

- `Lernen` vermittelt die Sprache mit kurzen Beispielen und ausfuhrbaren Ablaufen.
- `Referenz` definiert die akzeptierte Syntax und die Sprachsemantik.

## Hier Beginnen

- Neu bei PalmScript: [Lernen-Uberblick](learn/overview.md)
- Erstes ausfuhrbares Skript: [Schnellstart](learn/quickstart.md)
- Formale Sprachdefinition: [Referenz-Uberblick](reference/overview.md)
- Vertrage der Indikatoren: [Indikatoren-Uberblick](reference/indicators.md)

Die gehostete Browser-IDE bleibt bewusst minimal: ein Editor, eine React- und
TypeScript-Shell mit Monaco, Datumsbereichsauswahl uber dem verfugbaren
BTCUSDT-Verlauf, Live-Diagnosen, Callable-Autovervollstandigungs-Snippets,
Backtest-Panels und Trades-/Orders-Tabellen ohne rohe JSON-Spalte. Die
Werkzeugleiste behalt das PalmScript-Logo im Kopfbereich und bietet einen
Hell/Dunkel-Schalter. Der Dunkelmodus nutzt eine an VS Code angelehnte Shell
mit einem Dracula-artigen Editor-Thema.
Der gehostete Einstiegspunkt ist `/`. [https://palmscript.dev/](https://palmscript.dev/) serviert die Browser-IDE direkt.

## Sprach-Highlights

PalmScript unterstutzt:

- eine verpflichtende Basisdeklaration `interval <...>`
- benannte `source`-Deklarationen fur Marktdaten
- quellqualifizierte Serien wie `spot.close` und `perp.1h.close`
- optionale `use <alias> <interval>`-Deklarationen fur zusatzliche Intervalle
- Literale, Arithmetik, Vergleiche, unare Operatoren, `and` und `or`
- `let`, `const`, `input`, Tupel-Destrukturierung, `export` und `trigger`
- `if / else if / else`
- Serienindexierung mit Literal-Offsets
- Indikatoren, Signal-Helper, Event-Memory-Helper und TA-Lib-artige Builtins
- erstklassige Strategiedeklarationen wie `entry`, `exit`, `order`, `protect` und `target`

## Wie Man Die Dokumentation Liest

Beginne mit `Lernen`, wenn du PalmScript zum ersten Mal schreibst.

Nutze `Referenz`, wenn du exakte Regeln fur Syntax, Semantik, Builtins,
Intervalle oder Ausgaben brauchst.

Der Header-Titel bleibt beim Scrollen auf `PalmScript` und verlinkt zur
Hauptseite der Website.
