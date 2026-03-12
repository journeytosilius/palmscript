# Rezept: Quellen Mit Exchange-Daten

Verwende benannte Quellen, wenn die Strategie historische Kerzen direkt von
unterstuetzten Exchanges laden soll.

```palmscript
interval 1m

source bn = binance.spot("BTCUSDT")
source hl = hyperliquid.perps("BTC")
use hl 1h

plot(bn.close)
plot(hl.1h.close)
```

## Probiere Es In Der Browser-IDE

Oeffne [https://palmscript.dev/app/](https://palmscript.dev/app/), fuege das
Beispiel in den Editor ein und fuehre es mit der verfuegbaren BTCUSDT-Historie
in der App aus.

## Worauf Du Achten Solltest

- quellenbewusste Skripte muessen quellqualifizierte Marktserien verwenden
- `use hl 1h` ist erforderlich, bevor `hl.1h.close` gueltig ist
- das Skript hat weiterhin genau ein globales Basis-`interval`
- der Runtime loest jeden benoetigten `(source, interval)`-Feed vor der
  Ausfuehrung auf

Referenz:

- [Intervalle und Quellen](../../reference/intervals-and-sources.md)
