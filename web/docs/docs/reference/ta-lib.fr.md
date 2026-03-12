# TA-Lib Surface

PalmScript inclut maintenant une couche d'integration TA-Lib typee ancree sur
le commit TA-Lib amont `1bdf54384036852952b8b4cb97c09359ae407bd0`.

PalmScript n'expose pas encore l'integralite du catalogue de fonctions TA-Lib
comme surface de langage executable, mais il reserve ce catalogue plus large et
utilise deja les fonctionnalites de langage typees necessaires a cette
surface :

- les litteraux enum `ma_type.<variant>`
- la destructuration de tuples pour les builtins TA-Lib a sorties multiples
- un snapshot de metadata TA-Lib fige
- un catalogue genere de 161 fonctions

Comportement actuel de la surface pilotee par les metadata :

- les 161 noms de fonctions TA-Lib sont reserves comme noms builtin
- l'autocompletion et le hover IDE peuvent afficher les signatures et resumes
  TA-Lib generes
- appeler une fonction du catalogue qui n'est pas encore implementee produit un
  diagnostic de compilation deterministe au lieu d'etre traite comme un
  identifiant inconnu
- les fixtures oracle versionnees sous `tests/data/ta_lib/` valident
  maintenant le sous-ensemble implemente contre la bibliotheque C amont

Autrement dit :

- la couverture des noms reserves est plus large que la couverture
  d'execution runtime
- la visibilite IDE / catalogue n'implique pas qu'une fonction soit executable
- [Builtins](builtins.md) et la section [Indicators](indicators.md) sont la
  documentation de reference pour le sous-ensemble executable

Builtins de style TA-Lib implementes :

- `ma(series, length, ma_type)`
- `apo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])`
- `ppo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])`
- `macd(series, fast_length, slow_length, signal_length)`
- `macdfix(series[, signal_length=9])`
- `macdext(series[, fast_length=12[, fast_ma=ma_type.sma[, slow_length=26[, slow_ma=ma_type.sma[, signal_length=9[, signal_ma=ma_type.sma]]]]]])`
- transformations mathematiques unaires : `acos`, `asin`, `atan`, `ceil`,
  `cos`, `cosh`, `exp`, `floor`, `ln`, `log10`, `sin`, `sinh`, `sqrt`, `tan`,
  `tanh`
- operateurs mathematiques : `add`, `div`, `mult`, `sub`, `max`, `min`, `sum`,
  `maxindex`, `minindex`, `minmax`, `minmaxindex`
- transformations de prix : `avgprice`, `medprice`, `typprice`, `wclprice`
- helpers de recouvrement : `accbands`, `bbands`, `dema`, `ema`, `kama`, `ma`,
  `mavp`, `midpoint`, `midprice`, `sar`, `sarext`, `sma`, `t3`, `tema`,
  `trima`, `wma`
- helpers de cycle : `ht_dcperiod`, `ht_dcphase`, `ht_phasor`, `ht_sine`,
  `ht_trendline`, `ht_trendmode`, `mama`
- helpers statistiques : `avgdev`, `stddev`, `var`, `linearreg`,
  `linearreg_angle`, `linearreg_intercept`, `linearreg_slope`, `tsf`, `beta`,
  `correl`
- helpers de momentum : `adx`, `adxr`, `apo`, `aroon`, `aroonosc`, `bop`,
  `cci`, `cmo`, `dx`, `imi`, `mfi`, `minus_di`, `minus_dm`, `mom`, `plus_di`,
  `plus_dm`, `ppo`, `roc`, `rocp`, `rocr`, `rocr100`, `stoch`, `stochf`,
  `stochrsi`, `trix`, `willr`
- helpers de volume et de volatilite : `ad`, `adosc`, `atr`, `natr`, `obv`,
  `trange`

Variantes `ma_type` actuelles :

- `ma_type.sma`
- `ma_type.ema`
- `ma_type.wma`
- `ma_type.dema`
- `ma_type.tema`
- `ma_type.trima`
- `ma_type.kama`
- `ma_type.mama`
- `ma_type.t3`

Toutes les variantes `ma_type` actuelles sont executables via `ma(...)`,
`apo(...)`, `ppo(...)`, `bbands(...)`, `macdext(...)`, `mavp(...)`,
`stoch(...)`, `stochf(...)` et `stochrsi(...)`. Pour la famille generique de
moyennes mobiles TA-Lib, `ma_type.mama` suit le comportement amont de TA-Lib :
il ignore le parametre `length` explicite et utilise les valeurs MAMA
`fast_limit=0.5` et `slow_limit=0.05`.

Valeurs par defaut TA-Lib actuellement respectees dans la surface executable :

- `max`, `min` et `sum` ont une fenetre par defaut de `30`
- `midpoint` et `midprice` ont une fenetre par defaut de `14`
- `wma`, `maxindex`, `minindex`, `minmax` et `minmaxindex` ont une fenetre par
  defaut de `30`
- `avgdev` a une fenetre par defaut de `14`
- `stddev` et `var` utilisent `length=5` par defaut
- `linearreg`, `linearreg_angle`, `linearreg_intercept`, `linearreg_slope` et
  `tsf` utilisent `length=14` par defaut
- `beta` utilise `length=5` par defaut et applique le calcul beta fonde sur les
  rendements de TA-Lib
- `correl` utilise `length=30` par defaut
- `apo` et `ppo` utilisent `fast_length=12`, `slow_length=26` et
  `ma_type.sma` par defaut
- `macdfix` utilise `signal_length=9` par defaut
- `macdext` utilise `fast_length=12`, `slow_length=26`, `signal_length=9` et
  `ma_type.sma` pour les trois roles de moyenne mobile
- `bbands` utilise `length=5`, `deviations_up=2`, `deviations_down=2` et
  `ma_type.sma` par defaut
- `accbands` utilise `length=20` par defaut
- `mavp` exige `minimum_period`, `maximum_period` et `ma_type` explicites
- `sar` utilise `acceleration=0.02` et `maximum=0.2` par defaut
- `sarext` utilise `start_value=0`, `offset_on_reverse=0`, `af_init_long=0.02`,
  `af_long=0.02`, `af_max_long=0.2`, `af_init_short=0.02`, `af_short=0.02`,
  `af_max_short=0.2` par defaut
- `aroon` et `aroonosc` utilisent `length=14` par defaut
- `atr`, `natr`, `plus_dm`, `minus_dm`, `plus_di`, `minus_di`, `dx`, `adx`,
  `adxr`, `mfi` et `imi` utilisent `length=14` par defaut
- `adosc` utilise `fast_length=3` et `slow_length=10` par defaut
- `cci` utilise `length=14` par defaut
- `cmo` utilise `length=14` par defaut
- `dema`, `tema`, `trima`, `kama` et `trix` utilisent `length=30` par defaut
- `t3` utilise `length=5` et `volume_factor=0.7` par defaut
- `mama` utilise `fast_limit=0.5` et `slow_limit=0.05` par defaut
- `mom`, `roc`, `rocp`, `rocr` et `rocr100` utilisent `length=10` par defaut
- `stoch` utilise `fast_k=5`, `slow_k=3`, `slow_d=3` et `ma_type.sma` pour les
  deux etapes de lissage
- `stochf` utilise `fast_k=5`, `fast_d=3` et `ma_type.sma` par defaut
- `stochrsi` utilise `time_period=14`, `fast_k=5`, `fast_d=3` et
  `ma_type.sma` par defaut
- `willr` utilise `length=14` par defaut

Rafraichissement des fixtures oracle pour le sous-ensemble implemente :

```bash
python3 tools/generate_talib_fixtures.py
cargo test --test ta_lib_parity
```

Exemple de retour tuple :

```palm
interval 1m
source spot = binance.spot("BTCUSDT")

let (line, signal, hist) = macd(spot.close, 12, 26, 9)
plot(line)
plot(signal)
plot(hist)
```
