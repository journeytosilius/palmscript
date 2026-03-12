# TA-Lib-Oberflache

PalmScript enthaelt jetzt eine typisierte TA-Lib-Integrationsschicht, die an
den Upstream-TA-Lib-Commit `1bdf54384036852952b8b4cb97c09359ae407bd0`
gebunden ist.

PalmScript exponiert den vollstaendigen TA-Lib-Funktionskatalog noch nicht als
ausfuehrbare Sprachoberflaeche, reserviert aber bereits den breiteren Katalog
und nutzt die typisierten Sprachmerkmale, die fuer diese Oberflaeche noetig
sind:

- `ma_type.<variant>`-Enum-Literale
- Tupel-Destrukturierung fuer Multi-Output-TA-Lib-Builtins
- einen fixierten TA-Lib-Metadaten-Snapshot
- einen generierten Katalog mit 161 Funktionen

Aktuelles metadatengetriebenes Oberflaechenverhalten:

- alle 161 TA-Lib-Funktionsnamen sind als Builtin-Namen reserviert
- IDE-Completion und Hover koennen generierte TA-Lib-Signaturen und
  Zusammenfassungen anzeigen
- der Aufruf einer Katalogfunktion, die noch nicht implementiert ist, erzeugt
  eine deterministische Compile-Diagnose, statt als unbekannter Identifier
  behandelt zu werden
- eingecheckte Oracle-Fixtures unter `tests/data/ta_lib/` validieren die
  implementierte Teilmenge gegen die Upstream-C-Bibliothek

Mit anderen Worten:

- die Abdeckung reservierter Namen ist groesser als die Laufzeitabdeckung
- Sichtbarkeit in IDE oder Katalog bedeutet nicht, dass eine Funktion heute
  ausfuehrbar ist
- [Builtins](builtins.md) und der Abschnitt [Indikatoren](indicators.md) sind
  die autoritativen Dokumente fuer die ausfuehrbare Teilmenge

Implementierte TA-Lib-artige Builtins:

- `ma(series, length, ma_type)`
- `apo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])`
- `ppo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])`
- `macd(series, fast_length, slow_length, signal_length)`
- `macdfix(series[, signal_length=9])`
- `macdext(series[, fast_length=12[, fast_ma=ma_type.sma[, slow_length=26[, slow_ma=ma_type.sma[, signal_length=9[, signal_ma=ma_type.sma]]]]]])`
- unäre Mathematik-Transformationen: `acos`, `asin`, `atan`, `ceil`, `cos`,
  `cosh`, `exp`, `floor`, `ln`, `log10`, `sin`, `sinh`, `sqrt`, `tan`, `tanh`
- Mathe-Operatoren: `add`, `div`, `mult`, `sub`, `max`, `min`, `sum`,
  `maxindex`, `minindex`, `minmax`, `minmaxindex`
- Preis-Transformationen: `avgprice`, `medprice`, `typprice`, `wclprice`
- Overlap-Helper: `accbands`, `bbands`, `dema`, `ema`, `kama`, `ma`, `mavp`,
  `midpoint`, `midprice`, `sar`, `sarext`, `sma`, `t3`, `tema`, `trima`,
  `wma`
- Cycle-Helper: `ht_dcperiod`, `ht_dcphase`, `ht_phasor`, `ht_sine`,
  `ht_trendline`, `ht_trendmode`, `mama`
- Statistik-Helper: `avgdev`, `stddev`, `var`, `linearreg`,
  `linearreg_angle`, `linearreg_intercept`, `linearreg_slope`, `tsf`, `beta`,
  `correl`
- Momentum-Helper: `adx`, `adxr`, `apo`, `aroon`, `aroonosc`, `bop`, `cci`,
  `cmo`, `dx`, `imi`, `mfi`, `minus_di`, `minus_dm`, `mom`, `plus_di`,
  `plus_dm`, `ppo`, `roc`, `rocp`, `rocr`, `rocr100`, `stoch`, `stochf`,
  `stochrsi`, `trix`, `willr`
- Volumen- und Volatilitaets-Helper: `ad`, `adosc`, `atr`, `natr`, `obv`,
  `trange`

Aktuelle `ma_type`-Varianten:

- `ma_type.sma`
- `ma_type.ema`
- `ma_type.wma`
- `ma_type.dema`
- `ma_type.tema`
- `ma_type.trima`
- `ma_type.kama`
- `ma_type.mama`
- `ma_type.t3`

Alle aktuellen `ma_type`-Varianten sind ueber `ma(...)`, `apo(...)`, `ppo(...)`,
`bbands(...)`, `macdext(...)`, `mavp(...)`, `stoch(...)`, `stochf(...)` und
`stochrsi(...)` ausfuehrbar. Fuer die generische TA-Lib-Moving-Average-Familie
folgt `ma_type.mama` dem Upstream-TA-Lib-Verhalten: der explizite
`length`-Parameter wird ignoriert und stattdessen MAMA-Defaults
`fast_limit=0.5` und `slow_limit=0.05` verwendet.

Aktuell in der ausfuehrbaren Oberflaeche eingehaltene TA-Lib-Defaults:

- `max`, `min` und `sum` verwenden standardmaessig ein Fenster von `30`
- `midpoint` und `midprice` verwenden standardmaessig ein Fenster von `14`
- `wma`, `maxindex`, `minindex`, `minmax` und `minmaxindex` verwenden
  standardmaessig ein Fenster von `30`
- `avgdev` verwendet standardmaessig ein Fenster von `14`
- `stddev` und `var` verwenden standardmaessig `length=5`
- `linearreg`, `linearreg_angle`, `linearreg_intercept`, `linearreg_slope`
  und `tsf` verwenden standardmaessig `length=14`
- `beta` verwendet standardmaessig `length=5` und die renditebasierte
  Beta-Berechnung von TA-Lib
- `correl` verwendet standardmaessig `length=30`
- `apo` und `ppo` verwenden standardmaessig `fast_length=12`,
  `slow_length=26` und `ma_type.sma`
- `macdfix` verwendet standardmaessig `signal_length=9`
- `macdext` verwendet standardmaessig `fast_length=12`, `slow_length=26`,
  `signal_length=9` und `ma_type.sma` fuer alle drei MA-Rollen
- `bbands` verwendet standardmaessig `length=5`, `deviations_up=2`,
  `deviations_down=2` und `ma_type.sma`
- `accbands` verwendet standardmaessig `length=20`
- `mavp` erfordert explizite `minimum_period`, `maximum_period` und `ma_type`
- `sar` verwendet standardmaessig `acceleration=0.02` und `maximum=0.2`
- `sarext` verwendet standardmaessig `start_value=0`, `offset_on_reverse=0`,
  `af_init_long=0.02`, `af_long=0.02`, `af_max_long=0.2`,
  `af_init_short=0.02`, `af_short=0.02` und `af_max_short=0.2`
- `aroon` und `aroonosc` verwenden standardmaessig `length=14`
- `atr`, `natr`, `plus_dm`, `minus_dm`, `plus_di`, `minus_di`, `dx`, `adx`,
  `adxr`, `mfi` und `imi` verwenden standardmaessig `length=14`
- `adosc` verwendet standardmaessig `fast_length=3` und `slow_length=10`
- `cci` verwendet standardmaessig `length=14`
- `cmo` verwendet standardmaessig `length=14`
- `dema`, `tema`, `trima`, `kama` und `trix` verwenden standardmaessig
  `length=30`
- `t3` verwendet standardmaessig `length=5` und `volume_factor=0.7`
- `mama` verwendet standardmaessig `fast_limit=0.5` und `slow_limit=0.05`
- `mom`, `roc`, `rocp`, `rocr` und `rocr100` verwenden standardmaessig
  `length=10`
- `stoch` verwendet standardmaessig `fast_k=5`, `slow_k=3`, `slow_d=3` und
  `ma_type.sma` fuer beide Glattungsstufen
- `stochf` verwendet standardmaessig `fast_k=5`, `fast_d=3` und
  `ma_type.sma`
- `stochrsi` verwendet standardmaessig `time_period=14`, `fast_k=5`,
  `fast_d=3` und `ma_type.sma`
- `willr` verwendet standardmaessig `length=14`

Oracle-Fixture-Aktualisierung fuer die implementierte Teilmenge:

```bash
python3 tools/generate_talib_fixtures.py
cargo test --test ta_lib_parity
```

Beispiel fuer Tupel-Rueckgabe:

```palm
interval 1m
source spot = binance.spot("BTCUSDT")

let (line, signal, hist) = macd(spot.close, 12, 26, 9)
plot(line)
plot(signal)
plot(hist)
```
