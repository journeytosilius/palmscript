# Schnellstart

## 1. Offne Die Browser-IDE

Nutze die gehostete IDE unter [https://palmscript.dev/](https://palmscript.dev/).

## 2. Fuge Ein Skript Ein

```palmscript
interval 1m
source spot = binance.spot("BTCUSDT")

let fast = ema(spot.close, 5)
let slow = sma(spot.close, 10)

export trend = fast > slow
plot(spot.close)
```

## 3. Diagnosen Prufen

Der Editor validiert das Skript wahrend der Eingabe und zeigt eventuelle
Kompilierungsdiagnosen im rechten Bereich an.

## 4. Einen Backtest Starten

Wahle einen Datumsbereich und drucke `Run Backtest`, um das Skript gegen den
verfugbaren BTCUSDT-Verlauf in der App auszufuhren.

Weiter: [Erste Strategie](first-strategy.md)
