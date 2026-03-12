# Typen Und Werte

PalmScript arbeitet mit skalaren Zahlen, skalaren Booleschen Werten,
typisierten Enum-Literalen, Serien dieser Werte, `na` und `void`.

## Konkrete Typen

Die Implementierung unterscheidet diese konkreten Typen:

- `float`
- `bool`
- `ma_type`
- `tif`
- `trigger_ref`
- `position_side`
- `exit_kind`
- `series<float>`
- `series<bool>`
- `void`

`void` ist der Ergebnistyp von Ausdruecken wie `plot(...)`, die keinen
wiederverwendbaren Wert liefern.

## Primitive Werte

PalmScript-Werte haben in der Runtime die folgenden Formen:

- numerische Werte sind `f64`
- boolesche Werte sind `true` oder `false`
- `ma_type.<variant>`-Werte sind typisierte Enum-Literale
- `tif.<variant>`-Werte sind typisierte Enum-Literale
- `trigger_ref.<variant>`-Werte sind typisierte Enum-Literale
- `position_side.<variant>`-Werte sind typisierte Enum-Literale
- `exit_kind.<variant>`-Werte sind typisierte Enum-Literale
- `na` ist der Missing-Value-Sentinel
- `void` ist kein vom Benutzer schreibbares Literal

Aktuelle typisierte Enum-Oberflaeche:

- `ma_type.sma`
- `ma_type.ema`
- `ma_type.wma`
- `ma_type.dema`
- `ma_type.tema`
- `ma_type.trima`
- `ma_type.kama`
- `ma_type.mama`
- `ma_type.t3`
- `tif.gtc`
- `tif.ioc`
- `tif.fok`
- `tif.gtd`
- `trigger_ref.last`
- `trigger_ref.mark`
- `trigger_ref.index`
- `position_side.long`
- `position_side.short`
- `exit_kind.protect`
- `exit_kind.target`
- `exit_kind.signal`
- `exit_kind.reversal`

Alle aktuellen `ma_type`-Varianten sind ueber die TA-Lib-artigen
Moving-Average-Builtins ausfuehrbar; siehe
[TA-Lib-Oberflache](ta-lib.md). `tif`, `trigger_ref`, `position_side` und
`exit_kind` dienen derzeit dazu, Backtest-Orderdeklarationen sowie den
backtestgetriebenen Positions- und Exit-Zustand zu parametrisieren.

## Serien-Typen

Serienwerte sind zeitindizierte Stroeme.

Ein Serien-Typ:

- schreitet auf einem Aktualisierungstakt fort
- behaelt begrenzte Historie
- exponiert sein aktuelles Sample, wenn er in Ausdruecken verwendet wird
- kann bei einem bestimmten Sample `na` liefern

Marktfelder sind Serienwerte. Indikatoren, Signal-Helper und
Event-Memory-Builtins koennen ebenfalls Serienwerte zurueckgeben.

Einige Builtins koennen auch Tupel fester Groesse aus Serienwerten
zurueckgeben. In der aktuellen Implementierung werden Tupelergebnisse nur als
unmittelbare Builtin-Ergebnisse unterstuetzt und muessen mit `let (...) = ...`
destrukturiert werden.

Beispiel:

```palm
let (line, signal, hist) = macd(spot.close, 12, 26, 9)
plot(hist)
```

Aktuelle Grenzen der Tupel-Unterstuetzung:

- Tupelwerte werden nur von bestimmten Builtins erzeugt
- Tupelwerte koennen nicht als gewoehnliche wiederverwendbare Werte gespeichert
  werden
- tupelwertige Ausdruecke koennen nicht direkt an `plot`, `export`, `trigger`,
  Bedingungen oder weitere Ausdruecke uebergeben werden
- Tupel-Destrukturierung ist die einzige unterstuetzte Art, ein Tupelergebnis
  zu konsumieren

## `na`

`na` ist Teil der normalen Sprachsemantik. Es ist keine Laufzeit-Exception.

`na` kann entstehen durch:

- unzureichende Historie fuer Indexierung
- Indikator-Warmup
- fehlende Daten auf einem quellenbewussten Basis-Taktschritt
- Arithmetik oder Vergleiche, bei denen ein Operand bereits `na` ist
- explizite Verwendung des Literals `na`

PalmScript exponiert ausserdem `na(value)` als Builtin-Praedikat, getrennt vom
nackten Literal `na`:

- `na` allein ist das Missing-Value-Literal
- `na(expr)` liefert je nach Argument `bool` oder `series<bool>`
- `nz(value[, fallback])` und `coalesce(value, fallback)` sind die primaeren
  Null-Handling-Helper

## Kombination Von Serien Und Skalaren

PalmScript erlaubt das Mischen von Skalaren und Serien in Ausdruecken, wenn der
zugrunde liegende Operator diese Operanden-Kategorien akzeptiert.

Regeln:

- wenn einer der akzeptierten Operanden `series<float>` ist, liefert Arithmetik
  `series<float>`
- wenn einer der akzeptierten Operanden `series<bool>` ist, liefern logische
  Operationen `series<bool>`
- wenn einer der akzeptierten Operanden `series<float>` ist, liefern numerische
  Vergleiche `series<bool>`
- Gleichheit ueber beliebige Serien-Operanden liefert `series<bool>`

Dies ist Value-Lifting, keine implizite Materialisierung einer unbegrenzten
Serie. Die Auswertung folgt weiterhin den Aktualisierungstakten aus
[Auswertungssemantik](evaluation-semantics.md).

## `na` Im Type-Checking

`na` wird an jeder Stelle akzeptiert, an der spaeter ein numerischer oder
boolescher Ausdruck verlangt werden kann, vorbehaltlich des umgebenden
Konstrukts.

Beispiele:

- `plot(na)` ist gueltig
- `export x = na` ist gueltig
- `trigger t = na` ist gueltig
- `if na { ... } else { ... }` ist gueltig
- `ma(spot.close, 20, ma_type.ema)` ist gueltig

## Boolesche Logik

`and` und `or` verwenden PalmScripts dreiwertige Logik.

Sie zwingen `na` nicht zu `false`. Ihre Laufzeit-Wahrheitstabelle ist in
[Auswertungssemantik](evaluation-semantics.md) definiert.

## Ausgabe-Normalisierung

Ausgabedeklarationen normalisieren ihre Wertetypen wie folgt:

- `export` ueber numerisch, numerische Serie oder `na` ergibt `series<float>`
- `export` ueber bool oder boolesche Serie ergibt `series<bool>`
- `trigger`-, `entry`- und `exit`-Ausgaben liefern immer `series<bool>`

Siehe [Ausgaben](outputs.md) fuer das exakte Ausgabeverhalten.
