# PalmScript Lernen

Die offentliche PalmScript-Dokumentation ist um zwei Dinge herum aufgebaut:

- die Sprache zum Schreiben von Strategien
- Beispiele, die zeigen, wie Skripte geschrieben und verwendet werden

## Was Du Mit PalmScript Tust

Typischer Ablauf:

1. ein `.ps`-Skript schreiben
2. ein Basis-`interval` deklarieren
3. eine oder mehrere `source`-Bindungen deklarieren
4. es in der Browser-IDE validieren
5. es in der App uber historische Daten laufen lassen

## Lange Optimierungen

Fur lange CLI-Tuning-Laufe:

- nutze `palmscript run optimize ...`, wenn du das Ergebnis im Vordergrund willst
- nutze `palmscript run optimize ...` fuer direkte Optimierung in der CLI
- speichere brauchbare Kandidaten mit `--preset-out best.json`, damit du sie mit `run backtest` oder `run walk-forward` erneut pruefen kannst
- lasse den standardmaessigen unangetasteten Holdout aktiv, sofern du diesen Schutz nicht bewusst abschaltest

## Was Du Als Nachstes Lesen Solltest

- Erster ausfuhrbarer Ablauf: [Schnellstart](quickstart.md)
- Erste vollstandige Strategiefuhrung: [Erste Strategie](first-strategy.md)
- Sprachuberblick: [Sprachuberblick](language-overview.md)
- Exakte Regeln und Semantik: [Referenz-Uberblick](../reference/overview.md)

## Rollen Der Dokumentation

- `Lernen` erklart, wie man PalmScript effektiv einsetzt.
- `Referenz` definiert, was PalmScript bedeutet.
