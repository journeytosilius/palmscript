# Rezept: Multi-Intervall-Strategie

Dieses Muster fuegt langsameren Kontext zu einer schnelleren oder gleich
schnellen Basisstrategie hinzu.

```palmscript
interval 1d
source spot = binance.spot("BTCUSDT")
use spot 1w

let weekly_basis = ema(spot.1w.close, 8)

if spot.close > weekly_basis {
    plot(1)
} else {
    plot(0)
}
```

## Probiere Es In Der Browser-IDE

Oeffne [https://palmscript.dev/](https://palmscript.dev/), fuege das
Beispiel in den Editor ein und fuehre es ueber einen Datumsbereich aus, der
mehrere Wochenschluesse abdeckt.

## Worauf Du Achten Solltest

- `use spot 1w` ist erforderlich, bevor `spot.1w.close` gueltig ist
- hoehere Intervallwerte erscheinen erst, wenn die hoehere Kerze vollstaendig
  geschlossen ist
- eine teilweise Wochenkerze wird nie sichtbar gemacht
- die Indexierung arbeitet auf dem langsameren Intervall-Takt, nicht auf dem
  Basis-Takt

Referenz:

- [Intervalle und Quellen](../../reference/intervals-and-sources.md)
- [Serien und Indexierung](../../reference/series-and-indexing.md)
- [Auswertungssemantik](../../reference/evaluation-semantics.md)
