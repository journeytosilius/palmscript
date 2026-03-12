# Intervalle Und Quellen

Diese Seite definiert die normativen Intervall- und Quellenregeln von
PalmScript.

## Unterstuetzte Intervalle

PalmScript akzeptiert die Intervall-Literale aus der
[Intervalltabelle](intervals.md). Intervalle sind gross-/kleinschreibungs-
sensitiv.

## Basisintervall

Jedes Skript deklariert genau ein Basisintervall:

```palmscript
interval 1m
```

Das Basisintervall definiert den Ausfuehrungstakt.

## Benannte Quellen

Ausfuehrbare Skripte deklarieren eine oder mehrere benannte, exchangegestuetzte
Quellen:

```palmscript
interval 1m
source hl = hyperliquid.perps("BTC")
source bn = binance.spot("BTCUSDT")
use hl 1h

plot(bn.close - hl.1h.close)
```

Regeln:

- mindestens eine `source`-Deklaration ist erforderlich
- Marktserien muessen quellqualifiziert sein
- jede deklarierte Quelle liefert einen Basis-Feed auf dem Basisintervall des
  Skripts
- `use <alias> <interval>` deklariert ein zusaetzliches Intervall fuer diese
  Quelle
- `<alias>.<field>` referenziert diese Quelle auf dem Basisintervall
- `<alias>.<interval>.<field>` referenziert diese Quelle auf dem benannten
  Intervall
- Referenzen auf Intervalle unterhalb des Basisintervalls werden abgelehnt

## Unterstuetzte Quell-Templates

PalmScript unterstuetzt derzeit diese erstklassigen Templates:

- `binance.spot("<symbol>")`
- `binance.usdm("<symbol>")`
- `hyperliquid.spot("<symbol>")`
- `hyperliquid.perps("<symbol>")`

Die Intervall-Unterstuetzung ist template-spezifisch:

- `binance.spot` akzeptiert alle unterstuetzten PalmScript-Intervalle
- `binance.usdm` akzeptiert alle unterstuetzten PalmScript-Intervalle
- `hyperliquid.spot` lehnt `1s` und `6h` ab
- `hyperliquid.perps` lehnt `1s` und `6h` ab

Auch operative Fetch-Beschraenkungen sind template-spezifisch:

- die Hyperliquid-REST-API stellt pro Feed nur die letzten `5000` Kerzen bereit
- Marktmodus lehnt jede Hyperliquid-Feed-Anfrage ab, die dieses
  Aufbewahrungsfenster ueberschreitet
- Binance-Feeds werden intern paginiert und haben nicht dieselbe
  Gesamtfenster-Grenze

## Quellen-Feldmenge

Alle Quell-Templates werden in dieselben kanonischen Marktfelder normalisiert:

- `time`
- `open`
- `high`
- `low`
- `close`
- `volume`

Regeln:

- `time` ist die Kerzen-Oeffnungszeit in Unix-Millisekunden UTC
- Preis- und Volumenfelder sind numerisch
- venue-spezifische Zusatzfelder werden in der Sprache nicht exponiert

## Gleiche, Hoehere Und Niedrigere Intervalle

PalmScript unterscheidet drei Faelle fuer ein referenziertes Intervall relativ
zum Basisintervall:

- gleiches Intervall: gueltig
- hoeheres Intervall: gueltig, wenn mit `use <alias> <interval>` deklariert
- niedrigeres Intervall: abgelehnt

## Laufzeit-Semantik

Im Marktmodus:

- PalmScript laedt die benoetigten `(source, interval)`-Feeds direkt von den
  Venues
- die Basis-Zeitleiste ist die Vereinigung aller Basisintervall-
  Kerzenoeffnungszeiten der deklarierten Quellen
- wenn eine Quelle auf einem Zeitschritt keine Basis-Kerze hat, liefert diese
  Quelle auf diesem Schritt `na`
- langsamere Quellintervalle behalten ihren letzten voll geschlossenen Wert,
  bis ihre naechste Schlussgrenze erreicht ist

## Kein-Lookahead-Garantie

PalmScript darf keine Hoeherintervall-Kerze sichtbar machen, bevor diese Kerze
vollstaendig geschlossen ist.

Das gilt auch fuer quellqualifizierte Zusatzintervalle wie `hl.1h.close`.

## Laufzeit-Ausrichtungsregeln

Vorbereitete Feeds muessen an ihren deklarierten Intervallen ausgerichtet sein.

Die Runtime lehnt Feeds ab, die:

- nicht auf der Intervallgrenze ausgerichtet sind
- unsortiert sind
- bei derselben Intervall-Oeffnungszeit Duplikate enthalten
