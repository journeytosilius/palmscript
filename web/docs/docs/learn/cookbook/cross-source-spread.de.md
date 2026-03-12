# Rezept: Spread Zwischen Quellen

Dieses Muster vergleicht zwei benannte Markte auf demselben Basis-Takt.

```palmscript
interval 1m

source spot = binance.spot("BTCUSDT")
source perp = binance.usdm("BTCUSDT")

let spread = spot.close - perp.close
plot(spread)
```

## Warum Das Wichtig Ist

Die quellenbewusste Ausfuhrung baut den Basis-Takt aus der Vereinigung der
Basis-Zeitstempel aller deklarierten Quellen auf.

Das bedeutet:

- die Strategie lauft weiterhin einmal pro Basisintervall-Schritt
- wenn einer Quelle auf einem Schritt Daten fehlen, liefert diese Quelle `na`
- Ausdruecke, die von diesem fehlenden Eingang abhaengen, propagieren `na`
  gemaess der normalen Semantik weiter

Referenz:

- [Auswertungssemantik](../../reference/evaluation-semantics.md)
- [Intervalle und Quellen](../../reference/intervals-and-sources.md)
